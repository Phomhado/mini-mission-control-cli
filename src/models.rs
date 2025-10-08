#[derive(Debug, Clone, Copy)]
pub enum Role {
    Pilot,
    Engineer,
    Medic,
    Scientist,
    Commander,
}

#[derive(Debug, Clone, Copy)]
pub enum DutyStatus {
    OnDuty,
    OffDuty,
    EVA,
    Resting,
}

#[derive(Debug, Clone)]
pub struct CrewMember {
    pub id: i64,
    pub name: String,
    pub role: Role,
    pub duty_status: DutyStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemName {
    LifeSupport,
    Propulsion,
    Comms,
    Power,
    Navigation
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SystemStatus {
    Nominal,
    Warning,
    Critical,
    Offline
}

#[derive(Debug, Clone)]
pub struct ShipSystem {
    pub id: i64,
    pub name: SystemName,
    pub status: SystemStatus,
    pub power_draw_kw: f64,
    pub last_check: String,
}

#[derive(Debug, Clone, Copy)]
pub enum EventTag {
    Burn,
    Checklist,
    Telemetry,
    Bug,
    CriticalBug,
    Routine,
    Improvement
}

#[derive(Debug, Clone)]
pub struct MissionEvent {
    pub t_plus_seconds: f64,
    pub title: String,
    pub description: String,
    pub tags: Vec<EventTag>,
}

#[derive(Debug, Clone)]
pub struct MissionClock {
    pub time_start: String,
    pub time_end: String,
}

#[derive(Debug, Clone)]
pub struct Telemetry {
    pub fuel_pct: f64,
    pub o2_pct: f64,
    pub cabin_temp_c: f64,
    pub battery_pct: f64,
    pub generator_kw: f64
}
