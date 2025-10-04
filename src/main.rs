mod models;
use models::*;

fn main() {
    let _crew_member = CrewMember {
        id: 1,
        name: "Pedro Oliveira".to_string(),
        role: Role::Engineer,
        duty_status: DutyStatus::OnDuty,
    };

    let _ship_system = ShipSystem {
        id: 12,
        name: SystemName::LifeSupport,
        status: SystemStatus::Warning,
        power_draw_kw: 1.0,
        last_check: "10/03/2025".to_string(),
    };

    let _mission_event = MissionEvent {
        t_plus_seconds: 15.2,
        title: "Fix bug on Life Support systems".to_string(),
        description: "There is a bug that is causing the Life Support systems to turn off for around 2 seconds, we need to fix it".to_string(),
        tags: vec![EventTag::CriticalBug, EventTag::Improvement],
    };

    let _mission_clock = MissionClock {
        time_start: "10/03/2025".to_string(),
        time_end: "Still Doing".to_string(),
    };
}
