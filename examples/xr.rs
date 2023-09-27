use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::transform::components::Transform;
use bevy_openxr::xr_input::debug_gizmos::OpenXrDebugRenderer;
use bevy_openxr::xr_input::prototype_locomotion::{proto_locomotion, PrototypeLocomotionConfig};
use bevy_openxr::xr_input::trackers::{
    OpenXRController, OpenXRLeftController, OpenXRRightController, OpenXRTracker,
};
use bevy_openxr::DefaultXrPlugins;

#[derive(Component)]
pub struct Cube;

#[derive(Resource)]
struct SpawnMaster {
    pub width: i32,
    pub height: i32,
    pub cube_size: f32,
    pub should_spawn: bool,
    pub material: Option<Handle<StandardMaterial>>,
    pub mesh: Option<Handle<Mesh>>,
}

fn main() {
    color_eyre::install().unwrap();

    info!("Running `openxr-6dof` skill");
    App::new()
        .add_plugins(DefaultXrPlugins)
        .add_plugins(OpenXrDebugRenderer) //new debug renderer adds gizmos to
        // .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, proto_locomotion)
        .add_systems(Startup, spawn_controllers_example)
        .insert_resource(PrototypeLocomotionConfig::default())
        .insert_resource(SpawnMaster {
            should_spawn: true,
            width: 20,
            height: 100,
            cube_size: 0.01,
            mesh: None,
            material: None,
        })
        .add_systems(Update, (modify_spawn_parameters, spawn_boxes_system))
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
        material: materials.add(Color::rgb(0.8, 0.0, 0.0).into()),
        transform: Transform::from_xyz(0.0, 0.5, 1.0),
        ..default()
    });
    // light
    // commands.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 1500.0,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(4.0, 8.0, 4.0),
    //     ..default()
    // });
    // camera
    // commands.spawn((Camera3dBundle {
    //     transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // },));
}

fn spawn_controllers_example(mut commands: Commands) {
    //left hand
    commands.spawn((
        OpenXRLeftController,
        OpenXRController,
        OpenXRTracker,
        SpatialBundle::default(),
    ));
    //right hand
    commands.spawn((
        OpenXRRightController,
        OpenXRController,
        OpenXRTracker,
        SpatialBundle::default(),
    ));
}

fn modify_spawn_parameters(
    mut commands: Commands,
    things_query: Query<Entity, With<Cube>>,
    keys: Res<Input<KeyCode>>,
    mut spawn_master: ResMut<SpawnMaster>,
) {
    if keys.just_released(KeyCode::Up) {
        spawn_master.height += 10;
        println!("Boxes: {}", spawn_master.width * spawn_master.height);
    }

    if keys.just_released(KeyCode::Down) {
        spawn_master.height -= 10;
        println!("Boxes: {}", spawn_master.width * spawn_master.height);
    }

    if keys.just_released(KeyCode::Right) {
        spawn_master.width += 10;
        println!("Boxes: {}", spawn_master.width * spawn_master.height);
    }

    if keys.just_released(KeyCode::Left) {
        spawn_master.width -= 10;
        println!("Boxes: {}", spawn_master.width * spawn_master.height);
    }

    if keys.just_released(KeyCode::R) {
        for e in things_query.iter() {
            commands.entity(e).despawn()
        }
        spawn_master.should_spawn = true;
        println!("Spawning started");
    }
}

fn spawn_boxes_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
    mut spawn_master: ResMut<SpawnMaster>,
) {
    if !spawn_master.should_spawn {
        return;
    }
    let cube_size = spawn_master.cube_size;

    if spawn_master.mesh.is_none() {
        spawn_master.mesh = Some(meshes.add(Mesh::from(shape::Cube { size: cube_size })));
        spawn_master.material = Some(material_assets.add(Color::GOLD.into()));
    }
    println!("We are spawning cubes");
    spawn_master.should_spawn = false;
    let mesh = spawn_master.mesh.as_ref().unwrap();
    let material = spawn_master.material.as_ref().unwrap();
    let cube_on_x = spawn_master.width;
    let cube_on_y = spawn_master.height;
    let margin = cube_size * 0.1 as f32;
    let offset = Vec3::new(-cube_on_x as f32 * 0.5 * cube_size, 0.0, -3.0);

    for x in 0..cube_on_x {
        for z in 0..1 {
            for y in 0..cube_on_y {
                commands.spawn((
                    PbrBundle {
                        mesh: mesh.clone_weak(),
                        material: material.clone_weak(),
                        // mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
                        // material: materials.add(
                        //     Color::rgb(0.8 / (x as f32 / 34.0), 0.7, 0.6 / (z as f32 / 34.0))
                        //         .into(),
                        // ),
                        transform: Transform::from_xyz(
                            x as f32 * (cube_size + margin) + offset.x,
                            y as f32 * (cube_size + margin) + offset.y,
                            z as f32 * (cube_size + margin) + offset.z,
                        ),
                        ..default()
                    },
                    Cube,
                ));
            }
        }
    }
}
