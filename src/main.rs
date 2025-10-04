mod models;

use models::*;

struct SimState {
    t_plus_s: f64,
    ticks_done: u64,
    mission_over: bool,
    crew: Vec<CrewMember>,
    systems: Vec<ShipSystem>,
    tel: Telemetry,
}

fn tick(sim: &mut SimState) {
    sim.t_plus_s += 1.0;
    sim.ticks_done += 1;
}

fn main() {
    let mission_crew = vec![CrewMember  {
            id: 1,
            name: "Pedro Oliveira".to_string(),
            role: Role::Engineer,
            duty_status: DutyStatus::OnDuty
        }
    ];

    let mission_system = vec![ShipSystem {
            id: 1,
            name: SystemName::LifeSupport,
            status: SystemStatus::Nominal,
            power_draw_kw: 1.0,
            last_check: "Today".to_string(),
        }
    ];

    let tel = Telemetry {
        fuel_pct: 100.0,
        o2_pct: 100.0,
        cabin_temp_c: 22.0,
        battery_pct: 50.0,
        generator_kw: 0.0
    };

    let mut sim = SimState {
        t_plus_s: 0.0,
        ticks_done: 0,
        mission_over: false,
        crew: mission_crew,
        systems: mission_system,
        tel,
    };

    tick(&mut sim);
    tick(&mut sim);
    tick(&mut sim);
    println!("t + {}, ticks = {}", sim.t_plus_s, sim.ticks_done);

}
