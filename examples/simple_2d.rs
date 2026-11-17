use bevy::color::palettes::basic::WHITE;
use bevy::prelude::*;
use bevy_movement::linear::{
    LinearMovement,
    MovementType,
    StepConfig,
};
use bevy_movement::{
    Arrived,
    Destination,
    MovementPluginAnyState,
    NextDes,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins,))
        .add_plugins(MovementPluginAnyState::any())
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    let linear_id = commands
        .spawn((
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            Sprite {
                color: WHITE.into(),
                custom_size: Some(Vec2::new(64., 64.)),
                ..default()
            },
            LinearMovement {
                movement_type: MovementType::Linear(50.),
                ..default()
            },
        ))
        .observe(arrived)
        .id();
    let stepper_id = commands
        .spawn((
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            Sprite {
                color: WHITE.into(),
                custom_size: Some(Vec2::new(64., 64.)),
                ..default()
            },
            LinearMovement {
                movement_type: MovementType::Stepper(StepConfig::new(50., 0.25)),
                ..default()
            },
        ))
        .observe(arrived)
        .id();

    commands.spawn(Camera2d);

    // Just for example. You should not trigger this event manually.
    // Trigger function 'arrived' below to add new destination
    commands.trigger(Arrived {
        entity: linear_id,
        pos: Vec3::ZERO,
    });
    commands.trigger(Arrived {
        entity: stepper_id,
        pos: Vec3::ZERO,
    });
}

fn arrived(trigger: On<Arrived>, mut commands: Commands) {
    let next_pos = Vec3::new(fastrand::i32(-640..640) as f32, fastrand::i32(-360..360) as f32, 0.);

    // New destination
    commands.trigger(NextDes {
        entity: trigger.entity,
        des: Destination::from_pos(next_pos),
        is_chain: false,
    });
}
