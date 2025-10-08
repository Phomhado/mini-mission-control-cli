mod models;
use models::*;

// ===== constants & helpers =====
const FUEL_DRAIN_PER_SEC: f64 = 0.20;
const O2_DRAIN_PER_SEC: f64   = 0.10;
const TEMP_RISE_PER_SEC: f64  = 0.05;

const BATTERY_KWH: f64       = 20.0;

const SIM_TIME_ACCEL: f64    = 30.0;

fn clamp(x: f64, lo: f64, hi: f64) -> f64 {
    if x < lo { lo } else if x > hi { hi } else { x }
}

struct SimState {
    t_plus_s: f64,
    ticks_done: u64,
    mission_over: bool,
    crew: Vec<CrewMember>,
    systems: Vec<ShipSystem>,
    tel: Telemetry,
}

impl SimState {
    fn tick(&mut self) {
        self.t_plus_s += 1.0 * SIM_TIME_ACCEL;
        self.ticks_done += 1;

        self.update_readings();
        self.update_system_statuses();
        self.check_alerts();

        self.mission_over = self.is_mission_over();
    }

    fn total_draw_kw(&self) -> f64 {
        self.systems
            .iter()
            .filter(|s| s.name != SystemName::Power)
            .map(|s| s.power_draw_kw)
            .sum()
    }

    fn update_readings(&mut self) {
        //Basic consumption
        self.tel.fuel_pct     -= FUEL_DRAIN_PER_SEC * SIM_TIME_ACCEL;
        self.tel.o2_pct       -= O2_DRAIN_PER_SEC   * SIM_TIME_ACCEL;
        self.tel.cabin_temp_c += TEMP_RISE_PER_SEC  * SIM_TIME_ACCEL;

        // Generator placeholder
        if self.tel.generator_kw == 0.0 {
            self.tel.generator_kw = 2.0;
        }


        let draw_kw = self.total_draw_kw();
        let net_kw = draw_kw - self.tel.generator_kw; // >0 drena; <0 carrega
        // Δ% = (P[kW] * Δt[s]) / (3600 * Cap[kWh]) * 100
        let delta_batt_pct = (net_kw * (1.0 * SIM_TIME_ACCEL)) / (3600.0 * BATTERY_KWH) * 100.0;
        self.tel.battery_pct -= delta_batt_pct;

        // Clamps
        self.tel.fuel_pct    = clamp(self.tel.fuel_pct, 0.0, 100.0);
        self.tel.o2_pct      = clamp(self.tel.o2_pct, 0.0, 100.0);
        self.tel.battery_pct = clamp(self.tel.battery_pct, 0.0, 100.0);
    }

    fn update_system_statuses(&mut self) {
        for system in &mut self.systems {
            system.status = match system.name {
                SystemName::LifeSupport => {
                    if self.tel.o2_pct < 10.0 { SystemStatus::Critical }
                    else if self.tel.o2_pct < 30.0 { SystemStatus::Warning }
                    else { SystemStatus::Nominal }
                }
                SystemName::Propulsion => {
                    if self.tel.fuel_pct < 10.0 { SystemStatus::Critical }
                    else if self.tel.fuel_pct < 30.0 { SystemStatus::Warning }
                    else { SystemStatus::Nominal }
                }
                SystemName::Power => {
                    if self.tel.battery_pct < 10.0 { SystemStatus::Critical }
                    else if self.tel.battery_pct < 30.0 { SystemStatus::Warning }
                    else { SystemStatus::Nominal }
                }
                SystemName::Comms | SystemName::Navigation => {
                    if self.tel.battery_pct < 15.0 { SystemStatus::Warning }
                    else { SystemStatus::Nominal }
                }
            };
        }
    }

    fn check_alerts(&self) {
        for system in &self.systems {
            match system.status {
                SystemStatus::Warning => {
                    println!("Warning: {:?} returning irregular values", system.name);
                }
                SystemStatus::Critical => {
                    println!("CRITICAL: {:?} failed/collapsed!", system.name);
                }
                _ => {}
            }
        }
    }

    fn hud(&self) {
        println!(
            "T+{:.0}s | Fuel: {:>5.1}% | O2: {:>5.1}% | Temp: {:>5.1}°C | Batt: {:>5.1}% | Gen: {:>4.1}kW | Draw: {:>4.1}kW",
            self.t_plus_s, self.tel.fuel_pct, self.tel.o2_pct, self.tel.cabin_temp_c, self.tel.battery_pct,
            self.tel.generator_kw, self.total_draw_kw()
        );
    }

    fn is_mission_over(&self) -> bool {
        self.tel.fuel_pct <= 0.0 || self.tel.o2_pct <= 0.0 || self.tel.battery_pct <= 0.0
    }
}

fn main() {

    let mission_crew = vec![
        CrewMember { id: 1, name: "Pedro Oliveira".to_string(), role: Role::Engineer, duty_status: DutyStatus::OnDuty },
        CrewMember { id: 2, name: "Aiko Tanaka".to_string(), role: Role::Pilot, duty_status: DutyStatus::Resting },
        CrewMember { id: 3, name: "Dr. Miller".to_string(), role: Role::Medic, duty_status: DutyStatus::OnDuty },
    ];

    let mission_systems = vec![
        ShipSystem { id: 1, name: SystemName::LifeSupport, status: SystemStatus::Nominal, power_draw_kw: 1.2, last_check: "2025-10-07T00:00Z".to_string() },
        ShipSystem { id: 2, name: SystemName::Propulsion, status: SystemStatus::Nominal, power_draw_kw: 4.8, last_check: "2025-10-07T00:00Z".to_string() },
        ShipSystem { id: 3, name: SystemName::Power,      status: SystemStatus::Nominal, power_draw_kw: 0.0, last_check: "2025-10-07T00:00Z".to_string() },
        ShipSystem { id: 4, name: SystemName::Navigation, status: SystemStatus::Nominal, power_draw_kw: 0.3, last_check: "2025-10-07T00:00Z".to_string() },
        ShipSystem { id: 5, name: SystemName::Comms,      status: SystemStatus::Nominal, power_draw_kw: 0.2, last_check: "2025-10-07T00:00Z".to_string() },
    ];

    let tel = Telemetry {
        fuel_pct: 100.0,
        o2_pct: 100.0,
        cabin_temp_c: 22.0,
        battery_pct: 50.0,
        generator_kw: 0.0,
    };

    let mut sim = SimState {
        t_plus_s: 0.0,
        ticks_done: 0,
        mission_over: false,
        crew: mission_crew,
        systems: mission_systems,
        tel,
    };

    sim.hud();

    for _ in 0..10 {
        sim.tick();
        sim.hud();
        if sim.mission_over {
            println!("Mission terminated: Critical resource is over");
            break;
        }
    }
}
