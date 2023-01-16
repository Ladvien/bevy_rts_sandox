use bevy::prelude::*;
use bevy_iso3d_rts_cursor_plugin::{Cursor, Pickable};
use bevy_rapier3d::prelude::{Collider, Damping, Dominance, LockedAxes, Restitution, RigidBody};

use crate::{
    components::mechanics::{MovementSpeed, RotationSpeed, Unit},
    constants::{
        constants::GROUND_LEVEL,
        units::{self, SOCIAL_DISTANCE},
    },
    Game,
};

pub fn spawn_unit(
    mut commands: Commands,
    buttons: Res<Input<MouseButton>>,
    cursor: Res<Cursor>,
    asset_server: Res<AssetServer>,
) {
    let number_of_units_to_spawn = 500;

    if buttons.just_pressed(MouseButton::Right) {
        println!("Spawning unit.");
        let scale = 2.;
        spawn_units_in_grid(
            &spawn_tank,
            number_of_units_to_spawn,
            &mut commands,
            &asset_server,
            cursor.location.xyz.x,
            cursor.location.xyz.z,
            scale,
        )
    }
}

fn spawn_tank(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    x: f32,
    z: f32,
    scale: f32,
) -> Entity {
    let entity_id = &commands
        .spawn(SceneBundle {
            transform: Transform {
                translation: Vec3::new(x, GROUND_LEVEL, z),
                scale: Vec3::new(scale, scale, scale),
                ..default()
            },
            scene: asset_server.load("ship.gltf#Scene0"),
            ..default()
        })
        .with_children(|children| {
            children
                .spawn(PointLightBundle {
                    point_light: PointLight {
                        color: Color::rgb(1.0, 1.0, 0.0),
                        intensity: 50.0,
                        range: 45.0,
                        shadows_enabled: false,
                        ..default()
                    },
                    transform: Transform::from_xyz(0.0, 2.0, 0.0),
                    ..default()
                })
                .insert(Name::new("ShipLight"));
        })
        .insert(Unit)
        .insert(Pickable)
        // .insert(RigidBody::Dynamic)
        // .insert(LockedAxes::TRANSLATION_LOCKED_Y | LockedAxes::ROTATION_LOCKED_Y)
        // .insert(Collider::ball(0.5))
        // .insert(Restitution::coefficient(0.01))
        // .insert(Damping {
        //     linear_damping: 15.5,
        //     angular_damping: 1.0,
        // })
        .insert(MovementSpeed { value: 2. })
        .insert(RotationSpeed { value: 150. })
        .id();

    commands
        .entity(*entity_id)
        .insert(Name::new(format!("Ship-{:?}", entity_id)))
        .id()
}

fn spawn_units_in_grid(
    spawn_fn: &dyn Fn(&mut Commands, &Res<AssetServer>, f32, f32, f32) -> Entity,
    units_to_spawn: i32,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    x: f32,
    z: f32,
    scale: f32,
) -> () {
    let num_of_iters = (units_to_spawn as f64).log2().ceil() as i32;
    let group_offset = num_of_iters as f32 * scale - scale;

    for i in 0..num_of_iters {
        for j in 0..num_of_iters {
            let current_unit_index = i * num_of_iters + j;
            if current_unit_index >= units_to_spawn {
                break;
            }
            let adjusted_x = x + i as f32 + SOCIAL_DISTANCE + i as f32 * scale - group_offset;
            let adjusted_z = z + j as f32 + SOCIAL_DISTANCE + j as f32 * scale - group_offset;
            // println!("num: {:?}, x: {:?}, z: {:?}", i + i, z as i32, x as i32);
            spawn_fn(commands, &asset_server, adjusted_x, adjusted_z, scale);
        }
    }
}
