use std::f32::consts::PI;

use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::window::PresentMode;
use bevy::{ecs::schedule::SystemSet, prelude::*};
use bevy_inspector_egui::WorldInspectorPlugin;

use bevy_mod_raycast::{DefaultPluginState, RaycastMesh, RaycastMethod, RaycastSource};
use bevy_rapier3d::prelude::{NoUserData, RapierPhysicsPlugin};
use bevy_rapier3d::render::RapierDebugRenderPlugin;
use bevy_scene_hook::HookPlugin;
use components::effects::Blinker;
use components::game::GameState;
use components::mechanics::{Direction, Lifetime};

use constants::camera::*;
use constants::constants::*;
use constants::plane::*;

mod components;
mod constants;
mod plugins;
mod systems;
mod units;
mod util;

use plugins::cursor::CursorPlugin;
use plugins::{AnimationControllerPlugin, RayReflector};
use systems::effects::blink_system;
use systems::movement::{adjust_still_units_system, movement_system};
use systems::spawn_plane::{plane_setup, Cell};
use systems::spawn_unit::spawn_unit;
use systems::{mouse::mouse_system, rotation::rotate_system};

use crate::constants::mechanics::{MOVE_COOLDOWN, ROTATION_SPEED};

fn main() {
    App::new()
        .init_resource::<Game>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: SCREEN_WIDTH,
                height: SCREEN_HEIGHT,
                title: GAME_TITLE.to_string(),
                resizable: false,
                present_mode: PresentMode::AutoVsync,
                position: WindowPosition::At(Vec2::new(START_X_POX, START_Y_POX)),
                ..Default::default()
            },
            ..default()
        }))
        .add_plugin(CursorPlugin)
        .add_plugin(HookPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(AnimationControllerPlugin)
        .add_startup_system(setup_cameras)
        // .add_startup_system(plane_setup)
        .add_startup_system(setup)
        .add_plugin(WorldInspectorPlugin::new())
        .add_system(mouse_system)
        .add_state(GameState::Playing)
        .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(plane_setup))
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                // .with_system(move_player)
                .with_system(scoreboard_system)
                // .with_system(update_lights)
                .with_system(rotate_system)
                .with_system(camera_controls)
                .with_system(blink_system)
                .with_system(lifetime_despawn_system)
                .with_system(movement_system)
                .with_system(adjust_still_units_system)
                .with_system(spawn_unit),
        )
        .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(teardown))
        .add_system_set(SystemSet::on_update(GameState::GameOver).with_system(gameover_keyboard))
        .add_system_set(SystemSet::on_exit(GameState::GameOver).with_system(teardown))
        .add_system(bevy::window::close_on_esc)
        .run();
}

#[derive(Resource, Default)]
pub struct Game {
    board: Vec<Vec<Cell>>,
    mechanics: Mechanics,
    score: i32,
}

#[derive(Default)]
pub struct Mechanics {
    pub move_cooldown: Timer,
    pub rotate_cooldown: Timer,
    pub direction: Direction,
}

pub fn lifetime_despawn_system(
    mut commands: Commands,
    mut entities: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in &mut entities {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn setup_cameras(mut commands: Commands) {
    // Set camera
    commands
        .spawn((Camera3dBundle {
            transform: Transform::from_xyz(
                (((BOARD_SIZE_I * BLOCK_SIZE) as f32 / 2.0 as f32) + CAM_ORIGIN_X) as f32,
                ((BOARD_SIZE_J * BLOCK_SIZE) as f32 / 2.0 as f32) + CAM_ORIGIN_Y,
                ((BOARD_SIZE_J * BLOCK_SIZE) as f32 / 2.0 as f32) + CAM_ORIGIN_Z,
            )
            .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },))
        .insert(RaycastSource::<RayReflector>::new()); // Designate the camera as our source;
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut game: ResMut<Game>) {
    // Skelly
    // commands.spawn((
    //     SceneBundle {
    //         // scene: asset_server.load("BACKROOMS_POLY_GAME.glb#Scene0"),
    //         scene: asset_server.load("basic_toon_skeleton/scene.gltf#Scene0"),
    //         transform: Transform {
    //             translation: Vec3 {
    //                 x: 2.,
    //                 y: GROUND_LEVEL,
    //                 z: 2.,
    //             },
    //             scale: Vec3::ONE * SKELLY_SCALE_MODIFIER,
    //             ..default()
    //         },
    //         ..default()
    //     },
    //     Animated {
    //         current_animation: 4,
    //         animations: Animations(Some(vec![
    //             asset_server.load("basic_toon_skeleton/scene.gltf#Animation0"),
    //             asset_server.load("basic_toon_skeleton/scene.gltf#Animation1"),
    //             asset_server.load("basic_toon_skeleton/scene.gltf#Animation2"),
    //             asset_server.load("basic_toon_skeleton/scene.gltf#Animation3"),
    //             asset_server.load("basic_toon_skeleton/scene.gltf#Animation4"),
    //             asset_server.load("basic_toon_skeleton/scene.gltf#Animation5"),
    //             asset_server.load("basic_toon_skeleton/scene.gltf#Animation6"),
    //             asset_server.load("basic_toon_skeleton/scene.gltf#Animation7"),
    //         ])),
    //         animation_library: AnimationLibrary {
    //             walk: 1,
    //             run: 4,
    //             idle: 0,
    //             attack: 7,
    //             spawn: 2,
    //             alerted: 3,
    //         },
    //     },
    //     Pickable,
    //     MovementSpeed { value: 3.0 },
    //     RotationSpeed { value: 0.5 },
    //     Unit,
    // ));

    game.mechanics.move_cooldown = Timer::from_seconds(MOVE_COOLDOWN, TimerMode::Repeating);
    game.mechanics.rotate_cooldown = Timer::from_seconds(ROTATION_SPEED, TimerMode::Repeating);
    game.mechanics.direction.desired = (0.0 as f32).to_degrees();
    game.mechanics.direction.current = game.mechanics.direction.desired;

    // directional 'sun' light
    const HALF_SIZE: f32 = 10.0;
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 5900.0,
            // Configure the projection to better fit the scene
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 70.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 12.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });

    // scoreboard
    commands.spawn(
        TextBundle::from_section(
            "FPS:",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 40.0,
                color: Color::rgb(1., 0., 0.),
            },
        )
        .with_style(Style {
            // align_self: ,
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(10.),
                left: Val::Px(SCREEN_WIDTH - 150.),
                ..default()
            },
            ..default()
        }),
    );
}

// remove all entities that are not a camera
fn teardown(mut commands: Commands, entities: Query<Entity, Without<Camera>>) {
    for entity in &entities {
        commands.entity(entity).despawn_recursive();
    }
}

fn scoreboard_system(game: Res<Game>, mut query: Query<&mut Text>, diagnostics: Res<Diagnostics>) {
    let mut text = query.single_mut();
    let mut fps = 0.0;
    if let Some(fps_diagnostic) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(fps_smoothed) = fps_diagnostic.smoothed() {
            fps = fps_smoothed;

            text.sections[0].value = format!("FPS: {}", fps.round());
        }
    }
}

// restart the game when pressing spacebar
fn gameover_keyboard(mut state: ResMut<State<GameState>>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        state.set(GameState::Playing).unwrap();
    }
}

fn camera_controls(
    keyboard: Res<Input<KeyCode>>,
    mut game: ResMut<Game>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    if game.mechanics.move_cooldown.tick(time.delta()).finished() {
        let mut camera = camera_query.single_mut();

        let mut forward = camera.forward();
        forward.y = 0.0;
        forward = forward.normalize();

        let mut left = camera.left();
        left.y = 0.0;
        left = left.normalize();

        let speed = CAMERA_MOVEMENT_SPEED;
        let rotate_speed = CAMERA_ROTATION_SPEED;

        //Leafwing
        if keyboard.pressed(KeyCode::W) {
            camera.translation += forward * time.delta_seconds() * speed;
        }
        if keyboard.pressed(KeyCode::S) {
            camera.translation -= forward * time.delta_seconds() * speed;
        }
        if keyboard.pressed(KeyCode::A) {
            camera.translation += left * time.delta_seconds() * speed;
        }
        if keyboard.pressed(KeyCode::D) {
            camera.translation -= left * time.delta_seconds() * speed;
        }
        if keyboard.pressed(KeyCode::Q) {
            camera.rotate_axis(Vec3::Y, rotate_speed * time.delta_seconds())
        }
        if keyboard.pressed(KeyCode::E) {
            camera.rotate_axis(Vec3::Y, -rotate_speed * time.delta_seconds())
        }
    }
}
