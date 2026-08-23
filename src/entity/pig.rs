use crate::entity::Entity;

/// Custom data stored on each pig entity.
/// Drives the simple random-walk AI: the pig picks a direction, walks for
/// `direction_timer` seconds, then picks a new one.
pub struct PigEntityData {
    /// Seconds remaining in the current walk direction
    pub direction_timer: f32,
}

impl PigEntityData {
    pub fn new() -> Self {
        PigEntityData {
            direction_timer: 0.0,
        }
    }

    pub fn get_pig_data(entity: &Entity) -> Option<&Self> {
        let custom_data = entity.custom_data.as_ref()?;
        Some(
            custom_data
                .downcast_ref::<PigEntityData>()
                .expect("Pig entity custom data must be PigEntityData"),
        )
    }

    pub fn get_pig_data_mut(entity: &mut Entity) -> Option<&mut Self> {
        let custom_data = entity.custom_data.as_mut()?;
        Some(
            custom_data
                .downcast_mut::<PigEntityData>()
                .expect("Pig entity custom data must be PigEntityData"),
        )
    }
}
