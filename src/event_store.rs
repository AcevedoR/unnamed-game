use serde::{Deserialize, Serialize};

use crate::embed::get_event_chains;
use crate::event::{Event};
use crate::event_chain::EventChain;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EventStore {
    pub event_chains: Vec<EventChain>,
}

impl EventStore {
    pub fn new() -> EventStore {
        return EventStore {
            event_chains: get_event_chains(),
        };
    }

    pub fn get_event(self, event_name: String) -> Option<Event> {
        for chain in self.event_chains {
            if chain.events.contains_key(&event_name) {
                return Some(chain.events.get(&event_name).unwrap().clone());
            }
        }
        return None
    }

    pub fn get_chain(self, event_name: String) -> Option<String> {
        for chain in self.event_chains {
            if chain.events.contains_key(&event_name) {
                return Some(chain.title)
            }
        }
        return None
    }
}
