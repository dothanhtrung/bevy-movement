use bevy::prelude::{
    Component,
    Quat,
    Query,
    Res,
    Time,
    Transform,
    Vec3,
};

use crate::linear::{
    MovementBuilder,
    MovementType,
};

#[cfg_attr(feature = "serialize", derive(serde::Deserialize, serde::Serialize))]
#[derive(Component, Clone)]
#[require(MovementBuilder)]
pub struct CircleMovement {
    pub movement_type: MovementType,
    pub anchor: Vec3,
    pub is_freezed: bool,
    pub axis: Vec3,
}

impl Default for CircleMovement {
    fn default() -> Self {
        Self {
            movement_type: MovementType::Linear(0.),
            anchor: Vec3::ZERO,
            is_freezed: false,
            axis: Vec3::Z,
        }
    }
}

impl CircleMovement {
    pub fn speed(&self) -> f32 {
        match &self.movement_type {
            MovementType::Linear(_speed) => *_speed,
            MovementType::Stepper(step_config) => {
                if !step_config.is_stepping {
                    0.
                } else {
                    step_config.speed
                }
            }
        }
    }
}

pub(crate) fn circle_travel(time: Res<Time>, mut query: Query<(&mut Transform, &CircleMovement)>) {
    for (mut transform, movement) in query.iter_mut() {
        if movement.is_freezed {
            continue;
        }

        let mut offset = transform.translation - movement.anchor;
        let delta_angle = movement.speed() * time.delta_secs();
        let rot = Quat::from_axis_angle(movement.axis, delta_angle);
        offset = rot * offset;

        transform.translation = movement.anchor + offset;
    }
}
