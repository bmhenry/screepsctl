use std::collections::HashSet;

use log::*;

use stdweb::js;

use screepsctl as ctl;
use ctl::types::*;

pub mod logging;



fn main() {
    // initialize logger
    let _ = logging::setup_logging(logging::Info);
    logging::store_log_level();

    // initialize metrics
    ctl::metrics::init_metrics();

    js! {
        var game_loop = @{game_loop};

        module.exports.loop = function() {
            // Provide actual error traces.
            try {
                game_loop();
            } catch (error) {
                // console_error function provided by 'screeps-game-api'
                console_error("caught exception:", error);
                if (error.stack) {
                    console_error("stack trace:", error.stack);
                }
                console_error("resetting VM next tick.");
                // reset the VM since we don't know if everything was cleaned up and don't
                // want an inconsistent state.
                module.exports.loop = wasm_initialize;
            }
        }
    }
}

fn game_loop() {
    // check log level
    logging::watch_log_level();

    trace!("loop starting! CPU: {}", screeps::game::cpu::get_used());

    // step metrics to next tick iteration
    ctl::metrics::tick_metrics();

    // run creeps first
    // determine their roles, handle tasks
    trace!("running creeps");

    let mut harvesters = 0;
    let mut builders = 0;
    for creep in screeps::game::creeps::values() {
        if !creep.memory().bool("ignore") || creep.ticks_to_live() == 0 {
            if let Ok(Some(role)) = creep.memory().string("role") {
                if role == BasicHarvester::role() {
                    ctl::harvester::run_basic_harvester(creep);
                    harvesters += 1;
                } else if role == BasicBuilder::role() {
                    ctl::builder::run_basic_builder(creep);
                    builders += 1;
                }
            }
        }
    }

    // run spawns next, using any info gathered from number of creps per role
    trace!("running spawns");
    for room in screeps::game::rooms::values() {
        let r = ctl::roomctl::RoomCtl::new(&room);

        // these need to be ranked from highest to lowest priority
        // TODO: manage the strategy in a more cohesive way
        if harvesters < r.energy_spots(true) {
            r.manage_spawns(ctl::roomctl::SpawnStrategy::CtrlrUpgrade);
        } else if builders < (r.construction_sites()/2) {
            r.manage_spawns(ctl::roomctl::SpawnStrategy::Builders);
        }
    }


    let time = screeps::game::time();

    if time % 32 == 3 {
        info!("running memory cleanup");
        cleanup_memory().expect("expected Memory.creeps format to be a regular memory object");
    }

    ctl::metrics::log();
    trace!("done! cpu: {}", screeps::game::cpu::get_used());


    // send out all logs from the tick
    logging::print_logs();
}

fn cleanup_memory() -> Result<(), Box<dyn std::error::Error>> {
    let alive_creeps: HashSet<String> = screeps::game::creeps::keys().into_iter().collect();

    let screeps_memory = match screeps::memory::root().dict("creeps")? {
        Some(v) => v,
        None => {
            warn!("not cleaning game creep memory: no Memory.creeps dict");
            return Ok(());
        }
    };

    for mem_name in screeps_memory.keys() {
        if !alive_creeps.contains(&mem_name) {
            debug!("cleaning up creep memory of dead creep {}", mem_name);
            screeps_memory.del(&mem_name);
        }
    }

    Ok(())
}
