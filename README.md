<div align="center">

bevy_movement
==============

[![crates.io](https://img.shields.io/crates/v/bevy_movement)](https://crates.io/crates/bevy_movement)
[![docs.rs](https://docs.rs/bevy_movement/badge.svg)](https://docs.rs/bevy_movement)
[![dependency status](https://deps.rs/crate/bevy_movement/latest/status.svg)](https://deps.rs/crate/bevy_movement/latest)
[![pipeline status](https://gitlab.com/245project/bevy-plugin/bevy-movement/badges/master/pipeline.svg)](https://gitlab.com/245project/bevy-plugin/bevy-movement/-/commits/master)

[![Gitlab](https://img.shields.io/badge/gitlab-%23181717.svg?style=for-the-badge&logo=gitlab&logoColor=white)](https://gitlab.com/245project/bevy-plugin/bevy-movement)
[![Github](https://img.shields.io/badge/github-%23121011.svg?style=for-the-badge&logo=github&logoColor=white)](https://github.com/dothanhtrung/bevy-movement)

</div>


Quickstart
----------

For more detail, please see [examples](./examples).

### Basic

Add plugin the app:
```rust
App::new().add_plugins(MovementPluginAnyState::any());
```

Include a `*Movement` component in the entity. For example:
```rust
command.spawn(
    LinearMovement {
        movement_type: MovementType::Linear(50.), // 50. is speed
        ..default()
    });
```

Move the entity to a new destination by event:
```rust
commands.trigger(NextDes::new(entity, Vec3::new(1., 1., 1.)));
```

### Walk from A to B

Use `LinearMovement` with `movement_type` is `Linear`:
```rust
command.spawn(
    LinearMovement {
        movement_type: MovementType::Linear(50.),
        ..default()
    });
```

### Step from A to B

Use `LinearMovement` with `movement_type` is `Stepper`:
```rust
command.spawn(
    LinearMovement {
        movement_type: MovementType::Stepper(StepConfig::new(50., 0.25)),
        ..default()
    });
```

### Walk in circle around A

Use `CircleMovement` with `movement_type` is `Linear`:
```rust
command.spawn(
    CircleMovement {
        movement_type: MovementType::Linear(5.),
        ..default()
    });
```

### Step in circle around A

Use `CircleMovement` with `movement_type` is `Stepper`:
```rust
command.spawn(
    CircleMovement {
        movement_type: MovementType::Stepper(StepConfig {
            speed: 5.,
            step_timer: 0.25,
            still_timer: 0.5,
            ..default()
        }),
        ..default()
    });
```

### Walk controled by Keyboard/Mouse

Include `KbMovementObject` or `MouseMovementObject` along with `LinearMovement` in the entity.
For example:
```rust
command.spawn(
    KbMovementObject::default(),    // Move by keyboard/gamepad input
    MouseMovementObject::default(), // Move by mouse input
    LinearMovement {
        movement_type: MovementType::Linear(50.),
        ..default()
    });
```

License
-------

Please see [LICENSE](./LICENSE).


Compatible Bevy Versions
------------------------

| bevy      | bevy_movement |
|-----------|---------------|
| 0.20      | 0.5-0.6       |
| 0.19      | 0.4           |
| 0.18      | 0.3           |
| 0.17-0.18 | 0.2           |
| 0.16      | 0.1           |

---------

<div align="center">

![git_bevy-movement](https://count.getloli.com/@git_bevy-movement?name=git_bevy-movement&theme=random&padding=10&offset=0&align=top&scale=1&pixelated=1&darkmode=auto)

</div>
