use std::collections::HashSet;

use log::*;

use stdweb::js;

pub mod logging;
use screepsctl as ctl;

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


    trace!("running spawns");
    for room in screeps::game::rooms::values() {
        let ctl = ctl::roomctl::RoomCtl::new(&room, ctl::roomctl::SpawnStrategy::CtrlrUpgrade);
        ctl.manage_spawns();
    }

    trace!("running creeps");
    for creep in screeps::game::creeps::values() {
        if !creep.memory().bool("ignore") {
            ctl::harvester::run(creep);
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
