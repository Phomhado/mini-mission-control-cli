pub enum Role {
    Pilot,
    Engineer,
    Medic,
    Scientist,
    Commander,
}

pub enum DutyStatus {
    OnDuty,
    OffDuty,
    EVA,
    Resting,
}

pub struct CrewMember {
    pub id: i64,
    pub name: String,
    pub role: Role,
    pub duty_status: DutyStatus,
}

pub enum SystemName {
    LifeSupport,
    Propulsion,
    Comms,
    Power,
    Navigation
}

pub enum SystemStatus {
    Nominal,
    Warning,
    Critical,
    Offline
}

pub struct ShipSystem {
    pub id: i64,
    pub name: SystemName,
    pub status: SystemStatus,
    pub power_draw_kw: f64,
    pub last_check: String,
}

pub enum EventTag {
    Burn,
    Checklist,
    Telemetry,
    Bug,
    CriticalBug,
    Routine,
    Improvement
}
pub struct MissionEvent {
    pub t_plus_seconds: f64,
    pub title: String,
    pub description: String,
    pub tags: Vec<EventTag>,
}

pub struct MissionClock {
    pub time_start: String,
    pub time_end: String,
}
