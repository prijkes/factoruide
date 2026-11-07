//! Bounded structural support for gear sources. This does not promise a schedulable route.
use super::{Dungeon, Profile, Quest};
use crate::{
    job::StartState,
    params::{Might, Params},
};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

/// An ordered, model-owned quest action (quest indexes, not game IDs).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BisQuestAction {
    Accept(u32),
    Objective { quest: u32, objective: u8 },
    TurnIn(u32),
}

impl BisQuestAction {
    fn quest(self) -> u32 {
        match self {
            Self::Accept(q) | Self::TurnIn(q) | Self::Objective { quest: q, .. } => q,
        }
    }
}

/// A source and its residual support, in legal dependency order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BisQuestPlan {
    pub root: u32,
    pub quests: Vec<u32>,
    pub actions: Vec<BisQuestAction>,
}

const MAX_WORK: usize = 20_000;
const MAX_ACTIONS: usize = 4096;
const MAX_CHOICES: usize = 128;
const LIMIT: &str = "selection limit reached (support not determined)";

#[derive(Debug, Clone, Default)]
pub(super) struct Closure {
    nodes: BTreeMap<BisQuestAction, u8>,
    edges: BTreeSet<(BisQuestAction, BisQuestAction)>,
    participants: BTreeSet<u32>,
}

pub(super) struct SourceGraph {
    pub(super) actions: Vec<BisQuestAction>,
    pub(super) edges: Vec<(BisQuestAction, BisQuestAction)>,
    pub(super) quests: Vec<u32>,
}

#[derive(Clone)]
enum Task {
    Action(BisQuestAction, u8),
    Choose {
        ids: Vec<i64>,
        need: usize,
        members: u8,
        before: BisQuestAction,
    },
}

/// Evidence is kept in member order; the profile's group union is never substituted for it.
pub(super) struct Selector<'a> {
    quests: &'a [Quest],
    dungeons: &'a [Dungeon],
    skipped: &'a [(i64, String, String)],
    profile: &'a Profile,
    params: &'a Params,
    known: &'a HashSet<i64>,
    starts: &'a [&'a StartState],
    index: HashMap<i64, u32>,
    pub(super) committed: Closure,
}

impl<'a> Selector<'a> {
    #[expect(clippy::too_many_arguments, reason = "the nonmutating model inputs")]
    pub(super) fn new(
        quests: &'a [Quest],
        dungeons: &'a [Dungeon],
        skipped: &'a [(i64, String, String)],
        profile: &'a Profile,
        params: &'a Params,
        known: &'a HashSet<i64>,
        starts: &'a [&'a StartState],
    ) -> Self {
        Self {
            quests,
            dungeons,
            skipped,
            profile,
            params,
            known,
            starts,
            index: quests.iter().enumerate().map(|(i, q)| (q.id, i as u32)).collect(),
            committed: Closure::default(),
        }
    }

    fn completed(&self, id: i64, member: usize) -> bool {
        self.starts.get(member).map_or_else(
            || self.profile.mates.is_empty() && self.profile.completed_quests.contains(&id),
            |s| s.completed.iter().any(|q| q.0 == id) && !s.log.iter().any(|q| q.id.0 == id),
        )
    }

    fn residual(&self, action: BisQuestAction, members: u8) -> u8 {
        let q = &self.quests[action.quest() as usize];
        let mut remaining = members;
        for m in 0..8 {
            if members & (1 << m) == 0 {
                continue;
            }
            let satisfied = self.completed(q.id, m)
                || self.starts.get(m).is_some_and(|s| {
                    s.log.iter().find(|l| l.id.0 == q.id).is_some_and(|log| match action {
                        BisQuestAction::Accept(_) => true,
                        BisQuestAction::Objective { objective, .. } => crate::job::objective_progress(q, log)
                            .iter()
                            .any(|&(k, v)| k == objective && v >= 1.0),
                        BisQuestAction::TurnIn(_) => false,
                    })
                });
            if satisfied {
                remaining &= !(1 << m);
            }
        }
        remaining
    }

    fn completion(
        &self,
        id: i64,
        members: u8,
        before: BisQuestAction,
        state: &mut Closure,
        tasks: &mut Vec<Task>,
    ) -> Result<(), String> {
        let members = (0..8)
            .filter(|m| members & (1 << m) != 0 && !self.completed(id, *m))
            .fold(0, |mask, m| mask | (1 << m));
        if members == 0
            || self.known.contains(&id)
                && self
                    .index
                    .get(&id)
                    .is_none_or(|&i| !self.quests[i as usize].required && !self.quests[i as usize].zone_forced)
                && !self.quests[before.quest() as usize].required
                && !self.quests[before.quest() as usize].zone_forced
        {
            return Ok(());
        }
        let Some(&i) = self.index.get(&id) else {
            let reason = self
                .skipped
                .iter()
                .find(|(q, _, _)| *q == id)
                .map_or("not modeled", |(_, _, why)| why.as_str());
            return Err(format!(
                "prerequisite {id} is {reason} and has no recorded completion for every participating member"
            ));
        };
        self.dependency(BisQuestAction::TurnIn(i), members, before, state, tasks);
        Ok(())
    }

    fn dependency(
        &self,
        action: BisQuestAction,
        members: u8,
        before: BisQuestAction,
        state: &mut Closure,
        tasks: &mut Vec<Task>,
    ) {
        state.participants.insert(action.quest());
        if self.residual(action, members) != 0 {
            state.edges.insert((action, before));
            tasks.push(Task::Action(action, members));
        }
    }

    pub(super) fn finalize(&self) -> Result<SourceGraph, String> {
        Ok(SourceGraph {
            actions: self.order(&self.committed)?,
            edges: self
                .committed
                .edges
                .iter()
                .copied()
                .filter(|(a, b)| self.committed.nodes.contains_key(a) && self.committed.nodes.contains_key(b))
                .collect(),
            quests: self.committed.participants.iter().copied().collect(),
        })
    }

    pub(super) fn select(&self, root: u32) -> Result<(BisQuestPlan, Closure), String> {
        let mut work = 0;
        let count = self.starts.len().max(self.profile.mates.len() + 1).min(8);
        let participants = ((1u16 << count) - 1) as u8;
        let state = self.solve(
            self.committed.clone(),
            vec![Task::Action(
                BisQuestAction::TurnIn(root),
                self.quests[root as usize].members & participants,
            )],
            0,
            &mut work,
        )?;
        let ordered = self.order(&state)?;
        let mut needed = BTreeSet::from([BisQuestAction::TurnIn(root)]);
        loop {
            let old = needed.len();
            for &(a, b) in &state.edges {
                if needed.contains(&b) {
                    needed.insert(a);
                }
            }
            if old == needed.len() {
                break;
            }
        }
        let actions: Vec<_> = ordered.into_iter().filter(|a| needed.contains(a)).collect();
        let mut quests: Vec<_> = actions.iter().map(|a| a.quest()).collect();
        quests.push(root);
        quests.sort_unstable();
        quests.dedup();
        Ok((BisQuestPlan { root, quests, actions }, state))
    }

    fn solve(
        &self,
        mut state: Closure,
        mut tasks: Vec<Task>,
        depth: usize,
        work: &mut usize,
    ) -> Result<Closure, String> {
        if depth > MAX_CHOICES {
            return Err(LIMIT.into());
        }
        while let Some(task) = tasks.pop() {
            *work += 1;
            if *work > MAX_WORK || state.nodes.len() > MAX_ACTIONS {
                return Err(LIMIT.into());
            }
            let (action, members) = match task {
                Task::Action(a, m) => (a, m),
                Task::Choose {
                    mut ids,
                    need,
                    members,
                    before,
                } => {
                    ids.sort_unstable();
                    ids.dedup();
                    if need == 0 {
                        continue;
                    }
                    if ids.len() < need {
                        return Err(format!(
                            "requires {need} distinct alternatives but only {} are available",
                            ids.len()
                        ));
                    }
                    // Already committed books first, then cheap alternatives, then stable game ID.
                    ids.sort_by_key(|id| {
                        (
                            self.index
                                .get(id)
                                .is_none_or(|&i| !state.nodes.contains_key(&TurnIn(i)))
                                && !(0..8).all(|m| members & (1 << m) == 0 || self.completed(*id, m)),
                            self.index
                                .get(id)
                                .map_or(usize::MAX, |&i| self.quests[i as usize].objectives.len()),
                            *id,
                        )
                    });
                    let mut last = "no supported alternative".to_owned();
                    for k in 0..=ids.len() - need {
                        let mut branch = state.clone();
                        let mut pending = tasks.clone();
                        pending.push(Task::Choose {
                            ids: ids[k + 1..].to_vec(),
                            need: need - 1,
                            members,
                            before,
                        });
                        match self
                            .completion(ids[k], members, before, &mut branch, &mut pending)
                            .and_then(|()| self.solve(branch, pending, depth + 1, work))
                        {
                            Ok(s) => return Ok(s),
                            Err(e) if e == LIMIT => return Err(e),
                            Err(e) => last = e,
                        }
                    }
                    return Err(last);
                }
            };
            state.participants.insert(action.quest());
            let members = self.residual(action, members);
            if members == 0 {
                continue;
            }
            let old = state.nodes.get(&action).copied().unwrap_or(0);
            let members = members & !old;
            if members == 0 {
                continue;
            }
            let q = &self.quests[action.quest() as usize];
            if members & !q.members != 0 {
                return Err(format!(
                    "{} ({}) is unavailable to a participating member",
                    q.name, q.id
                ));
            }
            state.nodes.insert(action, old | members);
            self.compatible(&state, action)?;
            use BisQuestAction::{Accept, Objective, TurnIn};
            match action {
                Accept(i) => {
                    for &id in &q.pre_all {
                        self.completion(id, members, action, &mut state, &mut tasks)?;
                    }
                    if !q.pre_any.is_empty() {
                        let mut histories = BTreeMap::<Vec<i64>, u8>::new();
                        for m in 0..8 {
                            if members & (1 << m) != 0 {
                                let history = q.pre_any.iter().copied().filter(|id| self.completed(*id, m)).collect();
                                *histories.entry(history).or_default() |= 1 << m;
                            }
                        }
                        for members in histories.into_values() {
                            tasks.push(Task::Choose {
                                ids: q.pre_any.clone(),
                                need: q.pre_need,
                                members,
                                before: action,
                            });
                        }
                    }
                    for id in &q.pre_held {
                        if (0..8).any(|m| members & (1 << m) != 0 && self.completed(*id, m)) {
                            return Err(format!("held prerequisite {id} was already turned in"));
                        }
                        let &parent = self
                            .index
                            .get(id)
                            .ok_or_else(|| format!("held prerequisite {id} is not modeled"))?;
                        self.dependency(Accept(parent), members, Accept(i), &mut state, &mut tasks);
                        // Even if the parent's turn-in is discovered later, it follows the child's accept.
                        state.edges.insert((Accept(i), TurnIn(parent)));
                    }
                }
                Objective { quest: i, objective: k } => {
                    self.dependency(Accept(i), members, action, &mut state, &mut tasks);
                    let o = &q.objectives[k as usize];
                    for before in 0..q.objectives.len() {
                        if o.after & (1 << before) != 0 {
                            self.dependency(
                                Objective {
                                    quest: i,
                                    objective: before as u8,
                                },
                                members,
                                action,
                                &mut state,
                                &mut tasks,
                            );
                        }
                    }
                    if !o.handed_by.is_empty() {
                        tasks.push(Task::Choose {
                            ids: o.handed_by.clone(),
                            need: 1,
                            members,
                            before: action,
                        });
                    }
                }
                TurnIn(i) => {
                    if let Some(&j) = q.breadcrumb_for.as_ref().and_then(|id| self.index.get(id)) {
                        state.edges.insert((action, Accept(j)));
                    }
                    self.dependency(Accept(i), members, action, &mut state, &mut tasks);
                    for k in 0..q.objectives.len() {
                        self.dependency(
                            Objective {
                                quest: i,
                                objective: k as u8,
                            },
                            members,
                            action,
                            &mut state,
                            &mut tasks,
                        );
                    }
                }
            }
        }
        self.order(&state)?;
        Ok(state)
    }

    fn compatible(&self, state: &Closure, action: BisQuestAction) -> Result<(), String> {
        let q = &self.quests[action.quest() as usize];
        let taken: HashSet<_> = state.nodes.keys().map(|a| a.quest()).collect();
        for (i, other) in self.quests.iter().enumerate() {
            if i == action.quest() as usize {
                continue;
            }
            let exclusive = q.exclusive.contains(&other.id) || other.exclusive.contains(&q.id);
            if exclusive && (taken.contains(&(i as u32)) || other.required) {
                return Err(format!(
                    "{} ({}) conflicts with {} ({})",
                    q.name, q.id, other.name, other.id
                ));
            }
        }
        for m in 0..8 {
            if q.members & (1 << m) == 0 || self.completed(q.id, m) {
                continue;
            }
            if let Some(s) = self.starts.get(m) {
                for id in s.completed.iter().map(|q| q.0).chain(s.log.iter().map(|q| q.id.0)) {
                    if q.exclusive.contains(&id)
                        || q.breadcrumb_for == Some(id)
                        || self
                            .index
                            .get(&id)
                            .is_some_and(|&i| self.quests[i as usize].exclusive.contains(&q.id))
                    {
                        return Err(format!("{} ({}) conflicts with recorded quest {id}", q.name, q.id));
                    }
                }
            }
        }
        for requirement in self.profile.class_requirements.iter().filter(|c| c.selected) {
            let alternatives: Vec<_> = requirement
                .quests
                .iter()
                .filter_map(|id| self.index.get(id))
                .filter(|&&i| self.quests[i as usize].mandatory)
                .collect();
            if !alternatives.is_empty() && alternatives.iter().all(|&&i| self.class_path_conflicts(i, &taken)) {
                return Err(format!(
                    "conflicts with selected class requirement {}",
                    requirement.name
                ));
            }
        }
        Ok(())
    }

    /// A class version includes its unavoidable support, not just the named final quest.
    fn class_path_conflicts(&self, root: u32, taken: &HashSet<u32>) -> bool {
        let mut todo = vec![root];
        let mut seen = HashSet::new();
        while let Some(i) = todo.pop() {
            if !seen.insert(i) {
                continue;
            }
            let q = &self.quests[i as usize];
            if taken.iter().any(|&j| {
                i != j
                    && (q.exclusive.contains(&self.quests[j as usize].id)
                        || self.quests[j as usize].exclusive.contains(&q.id))
            }) {
                return true;
            }
            for id in q
                .pre_all
                .iter()
                .chain(&q.pre_held)
                .chain(q.pre_any.iter().filter(|_| q.pre_any.len() == 1))
            {
                if let Some(&j) = self.index.get(id) {
                    todo.push(j);
                }
            }
        }
        false
    }

    /// Topological scheduling with level intervals catches cycles and elapsed windows together.
    fn order(&self, state: &Closure) -> Result<Vec<BisQuestAction>, String> {
        let mut incoming: BTreeMap<_, usize> = state.nodes.keys().map(|&a| (a, 0)).collect();
        let mut following: BTreeMap<_, Vec<_>> = BTreeMap::new();
        for &(a, b) in &state.edges {
            if state.nodes.contains_key(&a) && state.nodes.contains_key(&b) {
                *incoming.get_mut(&b).expect("present node") += 1;
                following.entry(a).or_default().push(b);
            }
        }
        let mut ready: BTreeSet<_> = incoming.iter().filter_map(|(&a, &n)| (n == 0).then_some(a)).collect();
        let mut floors: BTreeMap<_, i64> = BTreeMap::new();
        let mut ordered = Vec::new();
        while let Some(a) = ready.pop_first() {
            let (min, max) = self.window(a)?;
            let floor = min.max(floors.get(&a).copied().unwrap_or(0));
            if floor > max {
                return Err(format!(
                    "{} ({}) needs level {floor}, outside its supported window ending at {max}",
                    self.quests[a.quest() as usize].name,
                    self.quests[a.quest() as usize].id
                ));
            }
            for &b in following.get(&a).into_iter().flatten() {
                let next = floors.entry(b).or_default();
                *next = (*next).max(floor);
                let count = incoming.get_mut(&b).expect("present node");
                *count -= 1;
                if *count == 0 {
                    ready.insert(b);
                }
            }
            ordered.push(a);
        }
        if ordered.len() != state.nodes.len() {
            return Err("cyclic quest action dependencies".into());
        }
        Ok(ordered)
    }

    fn window(&self, a: BisQuestAction) -> Result<(i64, i64), String> {
        let q = &self.quests[a.quest() as usize];
        let combat = crate::plan::Combat {
            params: self.params,
            dungeons: self.dungeons,
        };
        let mut min = self.profile.from_level;
        let mut max = self.profile.to_level - 1;
        let d = match a {
            BisQuestAction::Accept(_) => {
                min = min
                    .max(q.min_level)
                    .max(combat.guard_level(&q.start_guard, Might::PLAIN));
                q.start_dungeon
            }
            BisQuestAction::Objective { objective, .. } => {
                let o = &q.objectives[objective as usize];
                if o.dungeon_of().is_none() {
                    min = min.max(
                        o.places()
                            .map(|p| combat.level_at(q, p, Might::PLAIN))
                            .min()
                            .unwrap_or(0),
                    );
                }
                o.dungeon_of()
            }
            BisQuestAction::TurnIn(_) => {
                min = min
                    .max(q.completion_window.min_level.unwrap_or(0))
                    .max(combat.guard_level(&q.end_guard, Might::PLAIN));
                max = max.min(q.completion_window.max_level.unwrap_or(max));
                q.ends.iter().find_map(|l| l.inside.map(usize::from))
            }
        };
        if let Some(d) = d {
            let d = self
                .dungeons
                .get(d)
                .ok_or_else(|| "dungeon is not modeled".to_owned())?;
            min = min.max(d.min_level);
            if !q.implicit_window {
                min = min.max(q.completion_window.min_level.unwrap_or(0));
            }
            max = max.min(d.max_level);
        }
        Ok((min, max))
    }
}

#[cfg(test)]
mod tests;

/// Objectives already satisfied by every participating recorded member must survive filtering.
pub(super) fn recorded_objectives(quests: &[Quest], starts: &[&StartState]) -> HashSet<(i64, u8)> {
    let mut out = HashSet::new();
    for q in quests {
        for k in 0..q.objectives.len() {
            if (0..8).filter(|m| q.members & (1 << m) != 0).all(|m| {
                starts.get(m).is_some_and(|s| {
                    s.completed.iter().any(|id| id.0 == q.id)
                        || s.log.iter().find(|l| l.id.0 == q.id).is_some_and(|l| {
                            crate::job::objective_progress(q, l)
                                .iter()
                                .any(|&(j, v)| j == k as u8 && v >= 1.0)
                        })
                })
            }) {
                out.insert((q.id, k as u8));
            }
        }
    }
    out
}
