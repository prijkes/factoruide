use crate::{fixture::Fixture, job, plan::Planner, xp::Edition};
use crate::{
    model::select_test_sources,
    plan::{Effort, Kind, Stop},
    testing::{At, Obj, Q, Scenario},
};

fn select(mut p: job::Prepared, roots: &[u32]) -> job::Prepared {
    select_test_sources(&mut p, roots, &[]);
    p
}

fn plan(p: &job::Prepared) -> Planner<'_> {
    Planner::new(&p.model, &p.world, &p.params, &p.profile)
}

const HERE: At = At {
    zone: 1,
    x: 50.0,
    y: 50.0,
};

fn prepared(mode: &str) -> job::Prepared {
    prepared_with(mode, false)
}

fn prepared_with(mode: &str, held: bool) -> job::Prepared {
    let conn = Fixture::new(Edition::Classic).toml(r#"
[[npc]]
id = 990001
name = "Gear giver"
friendly_to = "AH"
at = [{ zone = 12, x = 48.0, y = 42.0 }]
[[quest]]
id = 990002
name = "Source prerequisite"
level = 1
min_level = 1
giver = 990001
[[quest]]
id = 990003
name = "Slow supported source"
pre_all = [990002]
level = 1
min_level = 1
giver = 990001
rewards = [{ item = 990010 }]
[[item]]
id = 990010
name = "Supported cap"
level = 1
class = 4
[[row]]
table = "m_item_stats"
values = { id = 990010, quality = 1, inventory_type = 1, class = 4, subclass = 1, allowable_class = 0, required_level = 1, str = 10.0, agi = 0.0, sta = 0.0, int = 0.0, spi = 0.0, armor = 0.0, ap = 0.0, rap = 0.0, sp = 0.0, heal = 0.0, crit = 0.0, spell_crit = 0.0, hit = 0.0, mp5 = 0.0, dps = 0.0, speed = 0.0 }
"#).open();
    if held {
        conn.execute_batch("UPDATE m_quest_link SET kind='pre_held' WHERE quest_id=990003;")
            .unwrap();
    }
    let overrides = job::Overrides::load_edition(
        &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../overrides"),
        Edition::Classic,
    )
    .unwrap();
    let request = serde_json::from_value(serde_json::json!({
        "race":"human", "class":"warrior", "from_level":1,"to_level":5,
        "required_class_quests":[], "params":{"edition":"classic","bis":mode}
    }))
    .unwrap();
    let mut p = job::prepare(&conn, &overrides, &request, &|_| {}).unwrap();
    p.params.mandatory_penalty = 1.0;
    p.params.stop_overhead = 1000.0;
    for q in &mut p.model.quests {
        q.xp = 0;
    }
    p
}

#[test]
fn required_slow_source_and_prerequisite_are_actually_completed() {
    for mode in ["required", "prefer"] {
        let p = prepared(mode);
        let planner = Planner::new(&p.model, &p.world, &p.params, &p.profile);
        let route = job::finalize_route(&planner, Vec::new()).unwrap();
        let (cost, end) = planner.simulate_state(&route, None).unwrap();
        let root = p.model.index[&990003];
        let parent = p.model.index[&990002];
        if mode == "required" {
            assert!(end.rewarded(root), "Required omitted the selected slow source");
            assert!(end.rewarded(parent), "source prerequisite was not completed");
            assert!(cost > planner.simulate(&[], None).unwrap() + p.params.mandatory_penalty);
        } else {
            assert!(!end.rewarded(root), "Prefer does not create hard goals");
        }
    }
}

#[test]
fn held_only_parent_and_global_cross_root_order_survive_nearby_turnins() {
    for roots in [&[1][..], &[0, 1][..]] {
        let p = select(
            Scenario::new()
                .levels(1, 10)
                .quest(Q::new(1, "Held parent").xp(0))
                .quest(Q::new(2, "Child source").xp(0).while_in_log(&[1]))
                .params(|p| p.camp_radius = 100.0)
                .build(),
            roots,
        );
        let planner = plan(&p);
        let route = planner.repair_required_gear(Vec::new());
        planner.validate_required(&route).unwrap_or_else(|e| {
            panic!(
                "{e}; route {route:?}; actions {:?}; initial {:?}",
                p.model.bis.source_actions(),
                p.model.initial.accepted
            )
        });
        let (_, end) = planner.simulate_state(&route, None).unwrap();
        assert!(end.rewarded(1));
        assert_eq!(end.rewarded(0), roots.len() == 2);
    }
}

#[test]
fn handed_parent_objective_follows_child_completion() {
    let p = select(
        Scenario::new()
            .levels(1, 10)
            .quest(
                Q::new(1, "Handed source")
                    .xp(0)
                    .objective(Obj::visit("Handed item", HERE).handed_by(&[2])),
            )
            .quest(Q::new(2, "Item provider").xp(0).while_in_log(&[1]))
            .params(|p| p.camp_radius = 100.0)
            .build(),
        &[0],
    );
    let planner = plan(&p);
    let route = planner.repair_required_gear(Vec::new());
    planner.validate_required(&route).unwrap();
    let (_, end) = planner.simulate_state(&route, None).unwrap();
    assert!(end.rewarded(0) && end.rewarded(1));
}

#[test]
fn selected_two_of_three_support_is_actually_completed() {
    let mut p = Scenario::new()
        .levels(1, 10)
        .quest(Q::new(1, "First").xp(0))
        .quest(Q::new(2, "Second").xp(0))
        .quest(Q::new(3, "Third").xp(0))
        .quest(Q::new(4, "Collection source").xp(0).after_any(&[1, 2, 3]))
        .build();
    p.model.quests[3].pre_need = 2;
    select_test_sources(&mut p, &[3], &[]);
    let planner = plan(&p);
    let route = planner.repair_required_gear(Vec::new());
    planner.validate_required(&route).unwrap();
    let (_, end) = planner.simulate_state(&route, None).unwrap();
    assert!(end.rewarded(0) && end.rewarded(1) && end.rewarded(3));
    assert!(!end.rewarded(2));
}

#[test]
fn recorded_unbegun_source_and_held_parent_keep_real_acceptance() {
    for held in [false, true] {
        let mut p = Scenario::new()
            .levels(1, 10)
            .quest(Q::new(1, "Logged participant").xp(0).after(&[999]).members(1))
            .quest(Q::new(2, "Child source").xp(0).while_in_log(&[1]).members(1))
            .build();
        let start = job::StartState {
            level: 1,
            log: vec![job::LogQuest {
                id: 1.into(),
                ..Default::default()
            }],
            ..Default::default()
        };
        select_test_sources(&mut p, &[u32::from(held)], &[&start]);
        p.model.initial = start.initial(&p.model, &p.world, &mut p.params, &mut Vec::new());
        assert_eq!(p.model.initial.accepted, vec![0]);
        let planner = plan(&p);
        let manual = vec![
            Stop::quest(0, Kind::Accept),
            Stop::quest(u32::from(held), Kind::Accept),
            Stop::quest(u32::from(held), Kind::TurnIn),
        ];
        assert!(
            planner.simulate(&manual, None).is_some(),
            "manual {:?}",
            planner.diagnose(&manual)
        );
        let route = planner.repair_required_gear(Vec::new());
        planner
            .validate_required(&route)
            .unwrap_or_else(|e| panic!("{e}; route {route:?}; actions {:?}", p.model.bis.source_actions()));
        assert!(!route.iter().any(|s| s.index == 999));
    }
    let p = prepared("off");
    let start = job::StartState {
        log: vec![job::LogQuest {
            id: 990003.into(),
            ..Default::default()
        }],
        ..Default::default()
    };
    assert!(
        start
            .initial(&p.model, &p.world, &mut p.params.clone(), &mut Vec::new())
            .accepted
            .is_empty()
    );
    let p = prepared("prefer");
    assert_eq!(
        start
            .initial(&p.model, &p.world, &mut p.params.clone(), &mut Vec::new())
            .accepted,
        vec![p.model.index[&990003]]
    );
}

#[test]
fn dungeon_entry_credit_reuses_one_forced_run() {
    let mut p = Scenario::new()
        .levels(1, 10)
        .dungeon("Run", 1, 50.0, 50.0)
        .quest(
            Q::new(1, "Entry source")
                .xp(0)
                .objective(Obj::visit("Run credit", HERE).inside(0)),
        )
        .params(|p| p.forced_dungeons = vec![900])
        .build();
    p.model.dungeons[0].kills = 0.0;
    p.model.quests[0].objectives[0].dungeon = Some(0);
    p.model.quests[0].objectives[0].loc.inside = None;
    select_test_sources(&mut p, &[0], &[]);
    let planner = plan(&p);
    let route = planner.repair_required_gear(vec![Stop {
        index: 0,
        kind: Kind::Dungeon,
    }]);
    planner.validate_required(&route).unwrap();
    assert!(!route.iter().any(|s| matches!(s.kind, Kind::Objective(_))));
    let (_, end) = planner.simulate_state(&route, None).unwrap();
    assert_eq!(end.spent.dungeon_runs, 1);
    assert_eq!(route.iter().filter(|s| s.kind == Kind::Dungeon).count(), 1);
}

#[test]
fn direct_inside_objectives_stay_in_one_continuous_visit() {
    let mut p = Scenario::new()
        .levels(1, 10)
        .dungeon("Run", 1, 50.0, 50.0)
        .quest(
            Q::new(1, "First source")
                .xp(0)
                .objective(Obj::visit("Inside", HERE).inside(0)),
        )
        .quest(
            Q::new(2, "Second source")
                .xp(0)
                .objective(Obj::visit("Inside too", HERE).inside(0)),
        )
        .build();
    p.model.dungeons[0].kills = 0.0;
    select_test_sources(&mut p, &[0, 1], &[]);
    let planner = plan(&p);
    let route = planner.repair_required_gear(Vec::new());
    planner.validate_required(&route).unwrap();
    let (_, end) = planner.simulate_state(&route, None).unwrap();
    assert_eq!(end.spent.dungeon_runs, 1);
    assert_eq!(route.iter().filter(|s| matches!(s.kind, Kind::Objective(_))).count(), 2);
}

#[test]
fn planned_after_target_and_exclusive_alternative_do_not_count_as_sources() {
    let mut p = select(
        Scenario::new()
            .levels(1, 2)
            .quest(Q::new(1, "Actual source").xp(0))
            .quest(Q::new(2, "Fast leveling").xp(100000))
            .build(),
        &[0],
    );
    let route = vec![
        Stop::quest(1, Kind::Accept),
        Stop::quest(1, Kind::TurnIn),
        Stop::quest(0, Kind::Accept),
        Stop::quest(0, Kind::TurnIn),
    ];
    p.params.camp_radius = 0.0;
    let planner = plan(&p);
    assert!(
        planner
            .validate_required(&route)
            .unwrap_err()
            .to_string()
            .contains("Actual source")
    );
    let mut trace = Vec::new();
    planner.simulate(&route, Some(&mut trace)).unwrap();
    assert!(planner.validate_required_trace(&trace).is_err());
    p.model.quests[0].exclusive = vec![2];
    assert!(plan(&p).validate_required(&route[..2]).is_err());
    p.model.initial.turned = vec![0];
    assert!(plan(&p).validate_required(&[]).is_ok());
    assert!(plan(&p).validate_required_trace(&[]).is_ok());
}

#[test]
fn repair_and_cleanup_preserve_exact_sources_and_forced_runs() {
    let mut p = select(
        Scenario::new()
            .levels(1, 10)
            .dungeon("Forced", 1, 50.0, 50.0)
            .quest(Q::new(1, "Source").xp(0))
            .quest(Q::new(2, "Exact").xp(0))
            .required(2)
            .params(|p| {
                p.forced_dungeons = vec![900];
                p.stop_overhead = 1000.0;
                p.mandatory_penalty = 1.0;
            })
            .build(),
        &[0],
    );
    p.model.dungeons[0].kills = 0.0;
    p.model.dungeons[0].min_level = 1;
    p.model.dungeons[0].max_level = 9;
    let planner = plan(&p);
    let base = vec![
        Stop::quest(1, Kind::Accept),
        Stop::quest(1, Kind::TurnIn),
        Stop {
            index: 0,
            kind: Kind::Dungeon,
        },
    ];
    assert!(
        planner.simulate(&base, None).is_some(),
        "base {:?}",
        planner.diagnose(&base)
    );
    let mut manual = vec![Stop::quest(0, Kind::Accept), Stop::quest(0, Kind::TurnIn)];
    manual.extend_from_slice(&base);
    assert!(
        planner.simulate(&manual, None).is_some(),
        "manual {:?}",
        planner.diagnose(&manual)
    );
    let repaired = planner.repair_required_gear(base);
    planner.validate_required(&repaired).unwrap_or_else(|e| {
        panic!(
            "{e}; route {repaired:?}; invalid {:?}; actions {:?}",
            planner.diagnose(&repaired),
            p.model.bis.source_actions()
        )
    });
    assert_eq!(planner.keep_hard_goals(repaired.clone(), Vec::new()), repaired);
    let improved = planner.improve_while_observed(
        repaired,
        Effort::Tries(30),
        || true,
        |_| {},
        |_, _, _| {},
        |_, route| planner.validate_required(route).unwrap(),
    );
    planner.validate_required(&improved).unwrap();
}

#[test]
fn incompatible_continuous_visit_windows_are_rejected_transactionally() {
    let mut p = Scenario::new()
        .levels(1, 30)
        .dungeon("Run", 1, 50.0, 50.0)
        .quest(
            Q::new(1, "Early source")
                .xp(0)
                .objective(Obj::visit("Early inside", HERE).inside(0)),
        )
        .quest(Q::new(2, "Late outside prerequisite").xp(0).level(20).min_level(20))
        .quest(
            Q::new(3, "Late source")
                .xp(0)
                .level(20)
                .after(&[2])
                .objective(Obj::visit("Late inside", HERE).inside(0)),
        )
        .build();
    p.model.dungeons[0].kills = 0.0;
    p.model.dungeons[0].min_level = 1;
    p.model.dungeons[0].max_level = 29;
    p.model.quests[0].ends[0].inside = Some(0);
    p.model.quests[0].completion_window.max_level = Some(10);
    select_test_sources(&mut p, &[0, 2], &[]);
    let planner = plan(&p);
    let repaired = planner.repair_required_gear(Vec::new());
    assert!(planner.validate_required(&repaired).is_err());
    assert!(planner.simulate(&repaired, None).is_some());
}

#[test]
fn gear_joins_existing_exact_quest_in_the_same_forced_dungeon_visit() {
    let mut p = Scenario::new()
        .levels(1, 10)
        .dungeon("Shared run", 1, 50.0, 50.0)
        .quest(
            Q::new(1, "Gear source")
                .xp(0)
                .objective(Obj::visit("Gear", HERE).inside(0)),
        )
        .quest(
            Q::new(2, "Exact dungeon quest")
                .xp(0)
                .objective(Obj::visit("Exact", HERE).inside(0)),
        )
        .required(2)
        .params(|p| p.forced_dungeons = vec![900])
        .build();
    p.model.dungeons[0].kills = 0.0;
    p.model.dungeons[0].min_level = 1;
    p.model.dungeons[0].max_level = 9;
    select_test_sources(&mut p, &[0], &[]);
    let planner = plan(&p);
    let base = vec![
        Stop::quest(1, Kind::Accept),
        Stop::quest(1, Kind::Objective(0)),
        Stop::quest(1, Kind::TurnIn),
    ];
    assert!(planner.simulate(&base, None).is_some());
    let repaired = planner.repair_required_gear(base);
    planner.validate_required(&repaired).unwrap();
    assert_eq!(planner.simulate_state(&repaired, None).unwrap().1.spent.dungeon_runs, 1);
}

#[test]
fn failed_timer_or_log_capacity_does_not_commit_partial_gear_work() {
    for timed in [false, true] {
        let mut p = Scenario::new()
            .levels(1, 10)
            .quest(Q::new(1, "Held parent").xp(0))
            .quest(
                Q::new(2, "Timed source")
                    .xp(0)
                    .while_in_log(&[1])
                    .objective(Obj::visit("Far work", At { x: 90.0, ..HERE })),
            )
            .build();
        if timed {
            p.model.quests[1].time_limit = Some(1.0);
        } else {
            p.params.quest_log_size = 1;
        }
        select_test_sources(&mut p, &[1], &[]);
        let planner = plan(&p);
        assert_eq!(planner.repair_required_gear(Vec::new()), Vec::<Stop>::new());
        assert!(planner.validate_required(&[]).is_err());
    }
}

#[test]
fn selected_inside_alternative_giver_preserves_the_continuous_visit() {
    let mut p = Scenario::new()
        .levels(1, 10)
        .dungeon("Run", 1, 50.0, 50.0)
        .quest(
            Q::new(1, "Inside predecessor")
                .xp(0)
                .objective(Obj::visit("Inside", HERE).inside(0)),
        )
        .quest(
            Q::new(2, "Alternative giver source")
                .xp(0)
                .after(&[1])
                .objective(Obj::visit("Inside again", HERE).inside(0)),
        )
        .params(|p| p.camp_radius = 0.0)
        .build();
    p.model.dungeons[0].kills = 0.0;
    p.model.dungeons[0].min_level = 1;
    p.model.dungeons[0].max_level = 9;
    p.model.quests[0].ends[0].inside = Some(0);
    let mut inside = p.model.quests[1].starts[0].clone();
    inside.inside = Some(0);
    inside.pos.x += 20.0;
    p.model.quests[1].starts.push(inside);
    select_test_sources(&mut p, &[1], &[]);
    let planner = plan(&p);
    let repaired = planner.repair_required_gear(Vec::new());
    planner.validate_required(&repaired).unwrap();
    assert_eq!(planner.simulate_state(&repaired, None).unwrap().1.spent.dungeon_runs, 1);
}

#[test]
fn prefer_retains_recorded_held_parent_without_fabricating_work() {
    let mut p = prepared_with("prefer", true);
    let start = job::StartState {
        level: 1,
        log: vec![job::LogQuest {
            id: 990002.into(),
            ..Default::default()
        }],
        ..Default::default()
    };
    p.model.initial = start.initial(&p.model, &p.world, &mut p.params, &mut Vec::new());
    let root = p.model.index[&990003] as u32;
    let parent = p.model.index[&990002];
    let planner = plan(&p);
    let route = vec![Stop::quest(root, Kind::Accept), Stop::quest(root, Kind::TurnIn)];
    let (_, end) = planner.simulate_state(&route, None).unwrap();
    assert!(end.rewarded(root as usize));
    assert!(!end.rewarded(parent));
    assert!(p.model.bis.required_roots().is_empty());
}

#[test]
fn selected_inside_objective_alternative_overrides_a_cheaper_outside_spot() {
    let mut p = Scenario::new()
        .levels(1, 10)
        .dungeon("Run", 1, 50.0, 50.0)
        .quest(
            Q::new(1, "Predecessor")
                .xp(0)
                .objective(Obj::visit("Inside", HERE).inside(0)),
        )
        .quest(
            Q::new(2, "Alternative objective source")
                .xp(0)
                .after(&[1])
                .objective(Obj::visit("Alternative work", HERE)),
        )
        .params(|p| p.camp_radius = 0.0)
        .build();
    p.model.dungeons[0].kills = 0.0;
    p.model.dungeons[0].min_level = 1;
    p.model.dungeons[0].max_level = 9;
    p.model.quests[0].ends[0].inside = Some(0);
    p.model.quests[1].starts[0].inside = Some(0);
    p.model.quests[1].ends[0].inside = Some(0);
    let mut inside = p.model.quests[1].objectives[0].clone();
    inside.loc.inside = Some(0);
    inside.loc.pos.x += 20.0;
    p.model.quests[1].objectives[0].alts.push(inside);
    select_test_sources(&mut p, &[1], &[]);
    let planner = plan(&p);
    let mut s = planner.sim().initial_state();
    s.spot = Some((1, 0, 0));
    assert_eq!(planner.sim().spot(&s, 1, 0).loc.inside, Some(0));
    let repaired = planner.repair_required_gear(Vec::new());
    planner.validate_required(&repaired).unwrap();
    assert_eq!(planner.simulate_state(&repaired, None).unwrap().1.spent.dungeon_runs, 1);
}

#[test]
fn nearby_source_npc_actions_do_not_cross_a_dungeon_boundary_without_entry() {
    let mut p = Scenario::new()
        .levels(1, 10)
        .dungeon("Run", 1, 50.0, 50.0)
        .quest(Q::new(1, "Outside source").xp(0))
        .quest(Q::new(2, "Inside source").xp(0))
        .params(|p| p.camp_radius = 100.0)
        .build();
    p.model.dungeons[0].kills = 0.0;
    p.model.dungeons[0].min_level = 1;
    p.model.dungeons[0].max_level = 9;
    p.model.quests[1].starts[0].inside = Some(0);
    p.model.quests[1].ends[0].inside = Some(0);
    select_test_sources(&mut p, &[0, 1], &[]);
    let planner = plan(&p);
    let repaired = planner.repair_required_gear(Vec::new());
    planner.validate_required(&repaired).unwrap();
    assert_eq!(planner.simulate_state(&repaired, None).unwrap().1.spent.dungeon_runs, 1);
}
