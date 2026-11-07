//! Selected gear sources are actual completion goals, independent of their soft penalty.

use super::{Kind, Planner, Simulator, State, Stop};
use crate::model::BisQuestAction;
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn source_visit(model: &crate::model::Model, stop: Stop) -> Option<Option<usize>> {
    if model.bis.required_roots().is_empty() {
        return None;
    }
    let action = match stop.kind {
        Kind::Accept => BisQuestAction::Accept(stop.index),
        Kind::TurnIn => BisQuestAction::TurnIn(stop.index),
        Kind::Objective(objective) => BisQuestAction::Objective {
            quest: stop.index,
            objective,
        },
        _ => return None,
    };
    model.bis.source_visit(action)
}

/// A small terminal rank, carried instead of keeping another run's states alive.
/// With no Required sources it is always zero, preserving ordinary scalar ordering.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct HardGoals {
    pub(crate) exact: u32,
    pub(crate) dungeons: u32,
    pub(crate) zones: u32,
    pub(crate) gear: u32,
}

pub(super) fn action_stop(action: BisQuestAction) -> Stop {
    match action {
        BisQuestAction::Accept(q) => Stop::quest(q, Kind::Accept),
        BisQuestAction::Objective { quest, objective } => Stop::quest(quest, Kind::Objective(objective)),
        BisQuestAction::TurnIn(q) => Stop::quest(q, Kind::TurnIn),
    }
}

pub(super) fn action_done(s: &State, action: BisQuestAction) -> bool {
    match action {
        BisQuestAction::Accept(q) => s.accepted[q as usize] || s.rewarded(q as usize),
        BisQuestAction::Objective { quest, objective } => {
            s.rewarded(quest as usize) || s.objectives[quest as usize] & (1 << objective) != 0
        }
        BisQuestAction::TurnIn(q) => s.rewarded(q as usize),
    }
}

impl Simulator<'_> {
    /// Automatic nearby actions must respect residual cross-source held/breadcrumb edges.
    pub(super) fn source_action_ready(&self, s: &State, stop: Stop) -> bool {
        if self.model.bis.required_roots().is_empty() {
            return true;
        }
        let action = match stop.kind {
            Kind::Accept => BisQuestAction::Accept(stop.index),
            Kind::TurnIn => BisQuestAction::TurnIn(stop.index),
            Kind::Objective(objective) => BisQuestAction::Objective {
                quest: stop.index,
                objective,
            },
            _ => return true,
        };
        self.model
            .bis
            .source_edges()
            .iter()
            .filter(|(_, after)| *after == action)
            .all(|(before, _)| action_done(s, *before))
    }
}

impl Planner<'_> {
    pub(crate) fn hard_goals(&self, s: &State) -> HardGoals {
        if self.model.bis.required_roots().is_empty() {
            return HardGoals::default();
        }
        HardGoals {
            exact: s.required_left,
            zones: s.forced_left,
            dungeons: (0..self.model.dungeons.len())
                .filter(|&d| {
                    self.availability().dungeon_forced(d)
                        && self.model.dungeons[d].min_level < self.profile.to_level
                        && s.dungeons & (1u128 << d) == 0
                })
                .count() as u32,
            gear: self
                .model
                .bis
                .required_roots()
                .iter()
                .filter(|&&q| !s.rewarded(q as usize))
                .count() as u32,
        }
    }

    /// Accepted specific sources not actually completed, including recorded initial history.
    pub fn missing_required_gear(&self, route: &[Stop]) -> Vec<u32> {
        if self.model.bis.required_roots().is_empty() {
            return Vec::new();
        }
        let state = self.simulate_state(route, None).map(|(_, s)| s);
        self.model
            .bis
            .required_roots()
            .iter()
            .copied()
            .filter(|&q| state.as_ref().is_none_or(|s| !s.rewarded(q as usize)))
            .collect()
    }

    pub(crate) fn scored_goals(&self, route: &[Stop]) -> Option<(f64, f64, HardGoals)> {
        self.simulate_state(route, None)
            .map(|(score, s)| (score, s.spent.total, self.hard_goals(&s)))
    }

    /// Whether a transformation preserves every previously satisfied hard goal.
    fn preserves_goals(&self, before: &State, after: &State) -> bool {
        after.forced_left <= before.forced_left
            && self
                .model
                .quests
                .iter()
                .enumerate()
                .all(|(q, quest)| !quest.required || !before.rewarded(q) || after.rewarded(q))
            && self
                .model
                .bis
                .required_roots()
                .iter()
                .all(|&q| !before.rewarded(q as usize) || after.rewarded(q as usize))
            && (0..self.model.dungeons.len()).all(|d| {
                !self.availability().dungeon_forced(d)
                    || before.dungeons & (1u128 << d) == 0
                    || after.dungeons & (1u128 << d) != 0
            })
    }

    /// Keep a known hard-feasible route across cosmetic cleanup; Off does no extra replay.
    pub(crate) fn keep_hard_goals(&self, before: Vec<Stop>, after: Vec<Stop>) -> Vec<Stop> {
        if self.model.bis.required_roots().is_empty() || before == after {
            return after;
        }
        let Some((_, old)) = self.simulate_state(&before, None) else {
            return after;
        };
        if self
            .simulate_state(&after, None)
            .is_some_and(|(_, new)| self.preserves_goals(&old, &new))
        {
            after
        } else {
            before
        }
    }

    /// Insert selected residual closures transactionally. Global edges and the contiguous
    /// dungeon witness govern ordering; local root plans can predate cross-root held edges.
    pub fn repair_required_gear(&self, mut route: Vec<Stop>) -> Vec<Stop> {
        if self.model.bis.required_roots().is_empty() || self.aborted() {
            return route;
        }
        let Some((_, mut end)) = self.simulate_state(&route, None) else {
            return route;
        };
        if self.hard_goals(&end).gear == 0 {
            return route;
        }
        // Try the complete graph first, then individual residual ancestors when only part can
        // be scheduled. A dungeon's whole selected visit is kept together in either case.
        let roots = self.model.bis.required_roots();
        let attempts = std::iter::once(roots.to_vec()).chain(roots.iter().map(|&q| vec![q]));
        for goals in attempts {
            if self.aborted() || self.hard_goals(&end).gear == 0 {
                break;
            }
            if goals.iter().all(|&q| end.rewarded(q as usize)) {
                continue;
            }
            let mut actions = self.source_closure(&goals);
            let visits: BTreeSet<_> = actions.iter().filter_map(|&a| self.source_dungeon(a)).collect();
            let mut edges = self.model.bis.source_edges().to_vec();
            // Existing work in a visit travels with that visit, especially exact dungeon
            // quests. Keep its own action sequence while allowing outside accepts before
            // the shared entry and outside hand-ins afterwards.
            let companions: BTreeSet<_> = route
                .iter()
                .filter(|s| s.is_quest())
                .filter_map(|s| {
                    let q = self.quest(s.index);
                    (q.objectives
                        .iter()
                        .filter_map(|o| o.dungeon_of())
                        .any(|d| visits.contains(&d))
                        || q.starts
                            .iter()
                            .chain(&q.ends)
                            .filter_map(|l| l.inside)
                            .any(|d| visits.contains(&usize::from(d))))
                    .then_some(s.index)
                })
                .collect();
            let mut previous = BTreeMap::new();
            for stop in route.iter().filter(|s| s.is_quest() && companions.contains(&s.index)) {
                let action = match stop.kind {
                    Kind::Accept => BisQuestAction::Accept(stop.index),
                    Kind::TurnIn => BisQuestAction::TurnIn(stop.index),
                    Kind::Objective(objective) => BisQuestAction::Objective {
                        quest: stop.index,
                        objective,
                    },
                    _ => unreachable!(),
                };
                actions.insert(action);
                if let Some(before) = previous.insert(stop.index, action)
                    && before != action
                {
                    edges.push((before, action));
                }
            }
            let quests: BTreeSet<_> = actions.iter().map(|&a| action_stop(a).index).collect();
            let Some(ordered) = self.source_order(&actions, &edges) else {
                continue;
            };
            let mut inserted = Vec::new();
            let mut dungeons = BTreeSet::new();
            for action in ordered {
                let stop = action_stop(action);
                if let Kind::Objective(k) = stop.kind
                    && let Some(d) = self.quest(stop.index).objectives[k as usize].dungeon
                {
                    if dungeons.insert(d) {
                        inserted.push(Stop {
                            index: d as u32,
                            kind: Kind::Dungeon,
                        });
                    }
                } else {
                    if let Some(d) = self.places().usual_inside(stop) {
                        dungeons.insert(d);
                    }
                    inserted.push(stop);
                }
            }
            // Recorded held-only parents have no residual Accept node, but an explicit no-op
            // anchor retains their real log state through start_state's abandonment policy.
            let anchors: Vec<_> = self
                .model
                .bis
                .source_quests()
                .iter()
                .copied()
                .filter(|q| self.in_log_at_start(*q) && !quests.contains(q))
                .map(|q| Stop::quest(q, Kind::Accept))
                .collect();
            inserted.splice(0..0, anchors);
            let removed = |s: &Stop| {
                s.is_quest() && quests.contains(&s.index)
                    || s.kind == Kind::Dungeon && dungeons.contains(&(s.index as usize))
            };
            let original_at = route.iter().take_while(|s| !removed(s)).count();
            let base: Vec<_> = route.iter().copied().filter(|s| !removed(s)).collect();
            // Try the original neighborhood, the beginning, and bounded route landmarks.
            // Insertion never relies on post-target intent.
            // Compact replay states are discarded between attempts; no second Run is retained.
            let mut placements = vec![original_at.min(base.len()), 0];
            placements.extend((1..base.len()).step_by(base.len().div_ceil(24).max(1)));
            placements.push(base.len());
            let mut tried = BTreeSet::new();
            placements.retain(|at| tried.insert(*at));
            for at in placements {
                let mut candidate = base.clone();
                candidate.splice(at..at, inserted.iter().copied());
                let Some((_, next)) = self.simulate_state(&candidate, None) else {
                    continue;
                };
                if self.preserves_goals(&end, &next) && self.hard_goals(&next).gear < self.hard_goals(&end).gear {
                    (route, end) = (candidate, next);
                    break;
                }
            }
        }
        route
    }

    fn source_closure(&self, roots: &[u32]) -> BTreeSet<BisQuestAction> {
        let mut selected: BTreeSet<_> = roots.iter().map(|&q| BisQuestAction::TurnIn(q)).collect();
        loop {
            let old = selected.len();
            for &(before, after) in self.model.bis.source_edges() {
                if selected.contains(&after) {
                    selected.insert(before);
                }
            }
            let visits: BTreeSet<_> = selected.iter().filter_map(|&a| self.source_dungeon(a)).collect();
            for &a in self.model.bis.source_actions() {
                if self.source_dungeon(a).is_some_and(|d| visits.contains(&d)) {
                    selected.insert(a);
                }
            }
            if old == selected.len() {
                break;
            }
        }
        selected.retain(|a| self.model.bis.source_actions().contains(a));
        selected
    }

    fn source_dungeon(&self, action: BisQuestAction) -> Option<usize> {
        if let Some(visit) = self.model.bis.source_visit(action) {
            return visit;
        }
        let stop = action_stop(action);
        if let Kind::Objective(k) = stop.kind {
            return self.quest(stop.index).objectives[k as usize].dungeon_of();
        }
        self.places().usual_inside(stop)
    }

    /// Dependency-first traversal avoids opening every independent quest before finishing
    /// any. Contracted visit blocks retain the selector's internal witness order.
    fn source_order(
        &self,
        selected: &BTreeSet<BisQuestAction>,
        edges: &[(BisQuestAction, BisQuestAction)],
    ) -> Option<Vec<BisQuestAction>> {
        let mut blocks: Vec<Vec<BisQuestAction>> = Vec::new();
        let mut visits = BTreeMap::new();
        let mut block_of = BTreeMap::new();
        let mut witness: Vec<_> = self
            .model
            .bis
            .source_actions()
            .iter()
            .copied()
            .filter(|a| selected.contains(a))
            .collect();
        witness.extend(
            selected
                .iter()
                .copied()
                .filter(|a| !self.model.bis.source_actions().contains(a)),
        );
        for action in witness {
            let block = if let Some(d) = self.source_dungeon(action) {
                *visits.entry(d).or_insert_with(|| {
                    blocks.push(Vec::new());
                    blocks.len() - 1
                })
            } else {
                blocks.push(Vec::new());
                blocks.len() - 1
            };
            blocks[block].push(action);
            block_of.insert(action, block);
        }
        let mut parents = vec![Vec::new(); blocks.len()];
        for &(before, after) in edges {
            if let (Some(&a), Some(&b)) = (block_of.get(&before), block_of.get(&after))
                && a != b
            {
                parents[b].push(a);
            }
        }
        fn visit(i: usize, parents: &[Vec<usize>], seen: &mut [u8], order: &mut Vec<usize>) -> Option<()> {
            let mut stack = vec![(i, false)];
            while let Some((i, exiting)) = stack.pop() {
                if exiting {
                    seen[i] = 2;
                    order.push(i);
                    continue;
                }
                if seen[i] == 2 {
                    continue;
                }
                if seen[i] == 1 {
                    return None;
                }
                seen[i] = 1;
                stack.push((i, true));
                stack.extend(parents[i].iter().rev().map(|&p| (p, false)));
            }
            Some(())
        }
        let mut seen = vec![0; blocks.len()];
        let mut order = Vec::new();
        for &q in self.model.bis.required_roots() {
            if let Some(&i) = block_of.get(&BisQuestAction::TurnIn(q)) {
                visit(i, &parents, &mut seen, &mut order)?;
            }
        }
        for i in 0..blocks.len() {
            visit(i, &parents, &mut seen, &mut order)?;
        }
        Some(order.into_iter().flat_map(|i| blocks[i].iter().copied()).collect())
    }
}
