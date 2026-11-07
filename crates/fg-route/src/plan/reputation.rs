//! Selected faction totals are required outcomes; direct quest rewards are the supported source.

use super::{Planner, Stop};

impl Planner<'_> {
    pub(crate) fn has_hard_goals(&self) -> bool {
        !self.model.bis.required_roots().is_empty() || !self.params.reputation_goals.is_empty()
    }

    pub(super) fn preserves_reputation(&self, before: &[f64], after: &[f64]) -> bool {
        self.params.reputation_goals.iter().enumerate().all(|(i, goal)| {
            before.get(i).copied().unwrap_or(0.0) < goal.target as f64
                || after.get(i).copied().unwrap_or(0.0) >= goal.target as f64
        })
    }

    pub(super) fn preserves_reputation_routes(&self, before: &[Stop], after: &[Stop]) -> bool {
        self.params.reputation_goals.is_empty()
            || self.preserves_reputation(&self.reputation_reached(before), &self.reputation_reached(after))
    }

    /// Continue construction from actual state, accepting costly useful work and its ancestors.
    /// Every insertion is replayed with the original route, preserving previously satisfied goals.
    pub fn repair_required_reputation(&self, route: Vec<Stop>) -> Vec<Stop> {
        if self.params.reputation_goals.is_empty() || self.aborted() {
            return route;
        }
        let Some((_, before)) = self.simulate_state(&route, None) else {
            return route;
        };
        if self.sim().reputation_goals_unmet(&before) == 0 {
            return route;
        }
        let candidate = self.greedy_from(route.clone()).0;
        if self.simulate_state(&candidate, None).is_some_and(|(_, after)| {
            self.preserves_goals(&before, &after) && self.hard_goals(&after) < self.hard_goals(&before)
        }) {
            candidate
        } else {
            route
        }
    }

    pub(super) fn validate_reputation(&self, reached: &[f64]) -> anyhow::Result<()> {
        let missing: Vec<_> = self
            .params
            .reputation_goals
            .iter()
            .enumerate()
            .filter_map(|(i, goal)| {
                let have = reached.get(i).copied().unwrap_or(0.0);
                (have < goal.target as f64).then(|| {
                    let name = self
                        .model
                        .faction_names
                        .get(&goal.faction)
                        .cloned()
                        .unwrap_or_else(|| format!("Faction {}", goal.faction));
                    let start = self.model.initial.reputation.get(i).copied().unwrap_or(0.0);
                    let provenance = if self.model.initial.reputation_recorded.contains(&goal.faction) {
                        "recorded standing"
                    } else {
                        "estimated race/class baseline"
                    };
                    format!(
                        "{name} ({}): start {start} ({provenance}), target {}, actual {have}, deficit {}",
                        goal.faction,
                        goal.target,
                        goal.target as f64 - have
                    )
                })
            })
            .collect();
        anyhow::ensure!(
            missing.is_empty(),
            "no satisfying route was found with the supported direct single-completion quest data and settings: {}",
            missing.join("; ")
        );
        Ok(())
    }
}
