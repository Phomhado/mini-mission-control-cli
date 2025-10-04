enum Role {
    Pilot,
    Engineer,
    Medic,
    Scientist,
    Commander,
}

enum DutyStatus {
    OnDuty,
    OffDuty,
    EVA,
    Resting,
}

struct CrewMember {
    id: i64,
    name: String,
    role: Role,
    duty_status: DutyStatus,
}

enum SystemName {
    LifeSupport,
    Propulsion,
    Comms,
    Power,
    Navigation
}

enum SystemStatus {
    Nominal,
    Warning,
    Critical,
    Offline
}

struct ShipSystem {
    id: i64,
    name: SystemName,
    status: SystemStatus,
    power_draw_kw: f64,
    last_check: String,
}

enum EventTag {
    Burn,
    Checklist,
    Telemetry,
    Bug,
    CriticalBug,
    Routine,
    Improvement
}
struct MissionEvent {
    t_plus_seconds: f64,
    title: String,
    description: String,
    tags: Vec<EventTag>,
}

struct MissionClock {
    time_start: String,
    time_end: String,
}

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
