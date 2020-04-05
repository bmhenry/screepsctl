//!
//! Controls builder creeps
//!


use log::*;


use screeps::prelude::*;
use screeps::{find};
use screeps::{ConstructionSite, Creep, HasStore, ResourceType, ReturnCode, SpawnOptions, StructureSpawn};
use screeps::memory;

use crate::util;
use crate::metrics;

use super::types::{BasicBuilder, CreepInfo};



/// tries to spawn a basic builder
pub fn spawn_basic_builder(spawn: &StructureSpawn) -> Result<(), String> {
    if spawn.energy() >= BasicBuilder::cost() {
        // create a unique name, spawn.
        let name_base = screeps::game::time();
        let mut additional = 0;

        // set the role of the creep on spawn
        let mem = memory::MemoryReference::new();
        mem.set("role", BasicBuilder::role());
        let opts = SpawnOptions::new().memory(mem);

        // loop until we get a valid name
        let res = loop {
            let name = format!("{}-{}", name_base, additional);
            let res = spawn.spawn_creep_with_options(&BasicBuilder::parts(), &name, &opts);

            if res == ReturnCode::NameExists {
                additional += 1;
            } else {
                metrics::inc_builders(1);
                break res;
            }
        };

        if res != ReturnCode::Ok {
            warn!("couldn't spawn: {:?}", res);
        }

        Ok(())
    } else {
        Err("Failed to ".to_string())
    }
}

/// runs the basic builder
pub fn run_basic_builder(creep: Creep) {
    let name = creep.name();
    trace!("running basic builder {}", name);

    // if the creep isn't building, and has full energy, go build
    if creep.store_free_capacity(None) == 0 {
        creep.memory().set("building", true);
    }

    if creep.memory().bool("building") {
        let target_opt = util::obj_from_mem_id::<ConstructionSite>(creep.memory(), "destId");

        // if the creep has no target, give it one
        let target = match target_opt {
            None => {
                let sites = creep.room().find(find::CONSTRUCTION_SITES);
                // TODO: find a better way to do this
                let site = sites[0].clone();
                creep.memory().set("destId", site.id().to_string());
                creep.say("🏗️ Build", false);
                site
            },
            _ => target_opt.unwrap()
        };

        // try building
        let r = creep.build(&target);
        // if we can't build, site is finished
        // TODO: may also be because a creep is on top of the site, need workaround
        if r == ReturnCode::InvalidTarget {
            creep.memory().set("building", false);
            creep.memory().del("destId");
        // if the site is out of range, move to the target
        } else if r == ReturnCode::NotInRange {
            creep.move_to(&target);
        } else if r == ReturnCode::NotEnough {
            creep.say("📦 Collect", false);
            creep.memory().set("building", false);
        }
    } else {
        // TODO: should be a container > extension > spawn
        let target = &creep.room().find(find::MY_SPAWNS)[0];

        // try to collect energy from the spawn
        if target.store_free_capacity(Some(ResourceType::Energy)) < 50 {
            match creep.withdraw_all(target, ResourceType::Energy) {
                ReturnCode::Ok => {
                    creep.memory().set("building", true);
                },
                ReturnCode::NotInRange => {
                    creep.move_to(target);
                },
                _ => ()
            }
        }
    }
}