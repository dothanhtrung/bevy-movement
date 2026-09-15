//! Move object by mouse click with path finding provided by bevy_northstar.
//! You will need to set up your grid with bevy_northstar first.


use bevy::camera_controller::free_camera::{
    FreeCamera,
    FreeCameraPlugin,
};
use bevy::prelude::*;
use bevy_movement::linear::{
    GridInfo,
    LinearMovement,
};
use bevy_movement::mouse_control::{
    ClickCatcher,
    MouseMovementObject,
};
use bevy_movement::MovementPluginAnyState;
use bevy_northstar::components::AgentPos;
use bevy_northstar::plugin::NorthstarPlugin;
use bevy_northstar::prelude::{
    AgentOfGrid,
    GridSettingsBuilder,
    Nav,
    NorthstarDebugPlugin,
    OrdinalNeighborhood3d,
};
use bevy_northstar::OrdinalGrid3d;

fn main() {
    let mut app = App::new();

    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                name: Some(String::from("bevy")),
                ..default()
            }),
            ..default()
        }),
        FreeCameraPlugin,
    ));

    // NorthstarPlugin for path finding
    app.add_plugins((
        NorthstarPlugin::<OrdinalNeighborhood3d>::default(),
        NorthstarDebugPlugin::<OrdinalNeighborhood3d>::default(),
    ));

    app.add_plugins(MovementPluginAnyState::any()) // This plugin
        .add_systems(Startup, setup);

    app.run();
}

fn setup(
    mut commands: Commands,
    mut grid_info: ResMut<GridInfo>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let default_mat = materials.add(StandardMaterial::default());

    // Let this plugin know about grid offset and tile size.
    // This offset and tile size should be same with the value you set for Northstar
    let tile_size = Vec3::splat(1.);
    let grid_size = UVec3::new(20, 4, 20);
    grid_info.tile_size = tile_size;
    grid_info.grid_offset = Vec3::new(-10. + tile_size.x * 0.5, 0., -10. + tile_size.z * 0.5);

    // Obstacle
    for i in 0..grid_size.x {
        for j in 0..grid_size.z {
            if i % 2 == 0 && j % 2 == 0 {
                let pos = tile_to_world(UVec3::new(i, 0, j), &grid_info);
                commands.spawn((
                    Mesh3d(meshes.add(Cuboid::new(1., 1., 1.))),
                    MeshMaterial3d(default_mat.clone()),
                    Transform::from_xyz(pos.x, 0.5, pos.z),
                ));
            }
        }
    }

    // Set up Northstar grid. Only y = 1 is walkable so the agent stays on the floor
    // instead of pathing under/over the cubes.
    let grid_settings = GridSettingsBuilder::new_3d(grid_size.x, grid_size.y, grid_size.z)
        .chunk_size(4)
        .chunk_depth(4)
        .enable_diagonal_connections()
        .default_impassable()
        .build();
    let mut grid = OrdinalGrid3d::new(&grid_settings);
    for x in 0..grid.width() {
        for z in 0..grid.depth() {
            if x % 2 == 0 && z % 2 == 0 {
                continue;
            }
            grid.set_nav(UVec3::new(x, 1, z), Nav::Passable(1));
        }
    }
    grid.build();
    let grid_id = commands
        .spawn(grid)
        .id();

    // Movement object
    let start_tile = UVec3::new(1, 1, 1);
    commands.spawn((
        AgentPos(start_tile),
        AgentOfGrid(grid_id),
        Transform::from_translation(tile_to_world(start_tile, &grid_info)),
        MouseMovementObject::default(), // Move by mouse input
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(default_mat.clone()),
        LinearMovement {
            speed: 10.,
            ..default()
        },
    ));

    // Ground
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(20., 1., 20.))),
        Transform::from_xyz(0.0, -0.5, 0.0),
        MeshMaterial3d(default_mat.clone()),
    ));

    // Click plane at the ground's top face. Offset lifts the hit onto the y = 1 walkable layer.
    // The plane must sit on the visible surface; otherwise an angled camera maps XZ incorrectly.
    commands.spawn((
        ClickCatcher {
            offset: Vec3::new(0., 1., 0.),
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    commands.spawn((
        PointLight {
            shadow_maps_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0),
    ));

    commands.spawn((
        Camera3d::default(),
        FreeCamera::default(),
        Transform::from_xyz(0.0, 7., 14.0).looking_at(Vec3::new(0., 0., -2.), Vec3::Y),
    ));
}

fn tile_to_world(tile: UVec3, grid_info: &GridInfo) -> Vec3 {
    tile.as_vec3() * grid_info.tile_size + grid_info.grid_offset
}
