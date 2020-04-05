//!
//! Handles controlling a room and the things within it
//!


use log::*;

use screeps::{find, Room, RoomName};

use crate::ctl::creep::{builder, harvester};


/// Manages a room and its contents, including creeps, spawning, construction, and more
pub struct RoomCtl<'a> {
    name: RoomName,
    room: &'a Room
}

impl RoomCtl<'_> {
    pub fn new(room: &Room) -> RoomCtl {
        RoomCtl {
            name: room.name(),
            room
        }
    }

    /// Manage all spawns, running required spawning strategy
    pub fn manage_spawns(&self, strategy: SpawnStrategy) {
        for spawn in self.room.find(find::MY_SPAWNS).iter() {
            debug!("running spawn {}", spawn.name());
            
            match strategy {
                SpawnStrategy::CtrlrUpgrade => {
                    // // determine if we already have the max supported number of energy harvesters
                    // let spots = self.energy_spots(true);
                    // if spots as usize > screeps::game::creeps::keys().len() {
                    //     if let Err(e) = harvester::spawn_basic_harvester(&spawn) {
                    //         warn!("Failed to create basic harvester: {}", e);
                    //     }
                    // } else {
                    //     debug!("Skipping spawn; only {} energy spots", spots);
                    // }
                    if let Err(e) = harvester::spawn_basic_harvester(&spawn) {
                        warn!("Failed to create basic harvester: {}", e);
                    }
                },
                SpawnStrategy::Builders => {
                    if let Err(e) = builder::spawn_basic_builder(&spawn) {
                        warn!("Failed to create basic builder: {}", e);
                    }
                }
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

    /// Determines how many construction sites are in the room
    pub fn construction_sites(&self) -> u32 {
        self.room.find(find::CONSTRUCTION_SITES).len() as u32
    }
}


pub enum SpawnStrategy {
    /// Create builders
    Builders,
    /// Focus exclusively on spawning harvester creeps to upgrade the controller
    CtrlrUpgrade,
}