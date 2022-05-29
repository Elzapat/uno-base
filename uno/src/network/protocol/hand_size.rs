use bevy_ecs::prelude::Component;
use uuid::Uuid;

use naia_shared::{Property, Replicate};

#[derive(Component, Replicate)]
#[protocol_path = "crate::network::protocol::Protocol"]
pub struct HandSize {
    pub size: Property<usize>,
    pub player_id: Property<String>,
}

impl HandSize {
    pub fn new(size: usize, player_id: Uuid) -> Self {
        HandSize::new_complete(size, player_id.to_string())
    }
}
