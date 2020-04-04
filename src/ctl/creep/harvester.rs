//!
//! Controls harvester creeps
//!

use log::*;
use std::str::FromStr;

use screeps::prelude::*;
use screeps::{find};
use screeps::{Creep, ObjectId, ResourceType, ReturnCode, Source};

use crate::util;
use crate::metrics;
use crate::source;


/// runs a harvester
pub fn run(creep: Creep) {
    let name = creep.name();
    debug!("running creep {}", name);

    // don't tell the creep what to do if it's still spawning
    if creep.spawning() {
        return;
    }

    // handle change in creep storage space
    if creep.memory().bool("harvesting") {
        if creep.store_free_capacity(Some(ResourceType::Energy)) == 0 {
            creep.memory().set("harvesting", false);
        }
    } else {
        if creep.store_used_capacity(None) == 0 {
            creep.memory().set("harvesting", true);
            creep.memory().set("moveToController", false);
            creep.memory().set("moveToSpawn", false);

            // choose the source with the most available spots
            let mut sources = creep.room().find(find::SOURCES);
            sources.sort_by_key(|s| source::free_source_spots(s));
            let id = sources.last().unwrap().id();
            creep.memory().set("destId", id.to_string());
            creep.say(format!("Harvesting energy").as_str(), false);
        }
    }

    // if the creep is currently harvesting, go to the selected source and harvest
    if creep.memory().bool("harvesting") {
        if let Some(source) = creep.memory().string("destId")
            .ok()
            .flatten()
            .map(|raw_id| ObjectId::<Source>::from_str(raw_id.as_str()).ok())
            .flatten()
            .map(|id| id.resolve())
            .flatten()
        {
            if creep.pos().is_near_to(&source) {
                let r = creep.harvest(&source);
                if r != ReturnCode::Ok {
                    warn!("couldn't harvest: {:?}", r);
                }
            } else {
                creep.move_to(&source);
            }
        } else {
            warn!("Couldn't unpack creep destId to Source");
            // unset harvesting, force value reset
            creep.memory().set("harvesting", false);
        }
    // if the creep isn't harvesting, handle transferring the energy
    } else {
        // give the creep directions
        if !creep.memory().bool("moveToSpawn") && !creep.memory().bool("moveToController") {
            // give the creep directions
            // store energy in the spawn 25% of the time
            // TODO: adjust this number based on whether or not we already have enough harvesters?
            if (util::random() % 3) == 0 {
                let spawn = &creep.room().find(find::MY_SPAWNS)[0];

                if spawn.store_free_capacity(Some(ResourceType::Energy)) > 0 {
                    // FIXME: move to a random or nearest spawn
                    creep.move_to(spawn);
                    creep.memory().set("moveToSpawn", true);
                    creep.say("Transferring to spawn", false);
                } else {
                    warn!("Can't set creep to transfer to spawn, spawn has no free capacity");
                }
            }

            // if the above was not successful for any reason, try to upgrade controller
            if !creep.memory().bool("moveToSpawn") {
                if let Some(c) = creep.room().controller() {
                    creep.move_to(&c);
                    creep.memory().set("moveToController", true);
                    creep.say("Upgrading controller", false);
                } else {
                    warn!("creep room has no controller!");
                }
            }
        }

        // follow directions to move and transfer energy to spawn
        if creep.memory().bool("moveToSpawn") {
            let spawn = &creep.room().find(find::MY_SPAWNS)[0];
            let stored = creep.store_used_capacity(Some(ResourceType::Energy));
            let r = creep.transfer_all(spawn, ResourceType::Energy);
            if r == ReturnCode::NotInRange {
                creep.move_to(spawn);
            } else if r == ReturnCode::Full {
                // if the spawn is full, just upgrade the controller instead
                creep.memory().set("moveToSpawn", false);
                creep.memory().set("moveToController", true);
                creep.say("Changing to upgrade controller", false);
            } else if r != ReturnCode::Ok {
                warn!("Creep {} energy transfer to spawn {} failed", creep.name(), spawn.name());
            } else {
                metrics::inc_energy(stored);
            }
        }

        // follow directions to move to and upgrade controller
        if creep.memory().bool("moveToController") {
            if let Some(c) = creep.room().controller() {
                let r = creep.upgrade_controller(&c);
                if r == ReturnCode::NotInRange {
                    creep.move_to(&c);
                } else if r != ReturnCode::Ok {
                    warn!("Creep {} couldn't upgrade: {:?}", creep.name(), r);
                }
            } else {
                creep.memory().set("moveToController", false);
                warn!("Room doesn't have a controller");
            }
        }
    }
}
