//!
//! Handles controlling a room and the things within it
//!


use log::*;

use screeps::{prelude::*, find, Room, RoomName, ReturnCode};

use crate::ctl::creep::types::BasicHarvester;
use crate::ctl::creep::types::CreepInfo;

use crate::metrics;


pub struct RoomCtl<'a> {
    name: RoomName,
    room: &'a Room,
    spawn_strategy: SpawnStrategy,
}

impl RoomCtl<'_> {
    pub fn new(room: &Room, strategy: SpawnStrategy) -> RoomCtl {
        RoomCtl {
            name: room.name(),
            room,
            spawn_strategy: strategy
        }
    }

    /// Manage all spawns, running required spawning strategy
    pub fn manage_spawns(&self) {
        for spawn in self.room.find(find::MY_SPAWNS).iter() {
            debug!("running spawn {}", spawn.name());
            
            match self.spawn_strategy {
                SpawnStrategy::CtrlrUpgrade => {

                    // determine if we already have the max supported number of energy harvesters
                    let spots = self.energy_spots(true);
                    if spots as usize > screeps::game::creeps::keys().len() {
                        if spawn.energy() >= BasicHarvester::cost() {
                            // create a unique name, spawn.
                            let name_base = screeps::game::time();
                            let mut additional = 0;

                            // loop until we get a valid name
                            let res = loop {
                                let name = format!("{}-{}", name_base, additional);
                                let res = spawn.spawn_creep(&BasicHarvester::parts(), &name);

                                if res == ReturnCode::NameExists {
                                    additional += 1;
                                } else {
                                    metrics::inc_harvesters(1);
                                    break res;
                                }
                            };

                            if res != ReturnCode::Ok {
                                warn!("couldn't spawn: {:?}", res);
                            }
                        }
                    } else {
                        debug!("Skipping spawn; only {} energy spots", spots);
                    }
                },
                _ => {
                    warn!("Unknown spawn strategy for room {}", self.name)
                }
            }
        }
    }

    /// Determines the number of free spaces for harvesting from energy sources,
    /// then adds a few on top to account for free spaces due to travel time
    pub fn energy_spots(&self, creeps_travel: bool) -> u32 {
        let spots = self.room.find(find::SOURCES).iter().fold(0, |count, source| {
            count + super::source::total_source_spots(source)
        });

        debug!("{} energy spots available in room {}", spots, self.room.name());

        if creeps_travel {
            // increase by a pretty arbitrary number here
            // FIXME: no doubt there's a smarter way to do this, based on average travel distance
            //   and creep types
            spots * 5/3
        } else {
            spots
        }
    }
}


pub enum SpawnStrategy {
    /// Focus exclusively on spawning harvester creeps to upgrade the controller
    CtrlrUpgrade,
    Defence,
}