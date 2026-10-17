use chrono::NaiveDate;

use super::Workspace;
use crate::{ReviewMode, WorkspaceSettings, Zone, ZoneStatus};

impl Workspace {
    pub fn status(&self, zone: Zone, settings: &WorkspaceSettings, today: NaiveDate) -> ZoneStatus {
        let items = self.state.count(zone);
        let mode = self.state.review_mode;
        let capacity = match (zone, mode) {
            (Zone::Learning, _) => settings.learning_capacity,
            (Zone::Review, ReviewMode::Endless) => settings.review_capacity,
            (Zone::Review, ReviewMode::FixedGroup) => settings.fixed_group_size,
        };
        ZoneStatus {
            zone,
            items,
            capacity,
            new_today: self.state.new_on(today),
            daily_new_limit: settings.daily_new_limit,
            review_mode: mode,
            group_finished: zone == Zone::Review && mode == ReviewMode::FixedGroup && items == 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::WorkspaceState;
    use crate::test_support::today;

    #[test]
    fn reports_capacity_by_mode() {
        let settings = WorkspaceSettings::default();
        let endless = Workspace::new(WorkspaceState::default());
        assert_eq!(
            endless.status(Zone::Review, &settings, today()).capacity,
            20
        );
        let fixed = Workspace::new(WorkspaceState {
            review_mode: ReviewMode::FixedGroup,
            ..WorkspaceState::default()
        });
        let status = fixed.status(Zone::Review, &settings, today());
        assert_eq!(status.capacity, 30);
        assert!(status.group_finished);
    }
}
