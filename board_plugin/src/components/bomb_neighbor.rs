use bevy::prelude::Component;

//bomb component
#[derive(Debug, Copy, Clone, Ord, PartialEq, PartialOrd, Eq, Hash, Component)]
pub struct BombNeighbor {
    pub count: u8,
}
