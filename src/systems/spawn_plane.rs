use crate::{
    constants::{constants::GROUND_LEVEL, plane::*},
    Game,
};
use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
};
use bevy_iso3d_rts_cursor_plugin::{CursorReflector, RayReflector};
use bevy_mod_raycast::RaycastMesh;
use bevy_rapier3d::prelude::RigidBody;
use bevy_scene_hook::{HookedSceneBundle, SceneHook};

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Plane {
    pub x_size: usize,
    pub y_size: usize,
}
pub struct Cell {
    pub height: f32,
}

pub fn plane_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut game: ResMut<Game>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // spawn the game board
    let cell_scene = asset_server.load("block_3.gltf#Scene0");
    game.board = (0..BOARD_SIZE_J as usize)
        .map(|j| {
            (0..BOARD_SIZE_I as usize)
                .map(|i| {
                    let height = 0.0;
                    commands
                        .spawn(
                            // HookedSceneBundle {
                            //     scene: SceneBundle {
                            //         transform: Transform::from_xyz(
                            //             // Probably should be half block
                            //             (i as f32 * BLOCK_SIZE / 2.) as f32,
                            //             height,
                            //             (j as f32 * BLOCK_SIZE / 2.) as f32,
                            //         ),
                            //         scene: cell_scene.clone(),
                            //         visibility: Visibility { is_visible: true },
                            //         ..default()
                            //     },
                            //     hook: SceneHook::new(|entity, commands| {
                            //         match entity.get::<Name>().map(|t| t.as_str()) {
                            //             Some("ground") => commands.insert(Ground),
                            //             // Some("GltfNode2") => commands.insert(Ground),
                            //             // Some("Entity (4)") => commands.insert(Ground),
                            //             // Some("Pbr Mesh (11)") => commands.insert(Ground),
                            //             _ => commands.insert(Ground),
                            //         };
                            //     }),
                            // },
                            (
                                SceneBundle {
                                    transform: Transform::from_xyz(
                                        // Probably should be half block
                                        (i as f32 * BLOCK_SIZE / 2.) as f32,
                                        height,
                                        (j as f32 * BLOCK_SIZE / 2.) as f32,
                                    ),
                                    scene: cell_scene.clone(),
                                    visibility: Visibility { is_visible: true },
                                    ..default()
                                },
                                // RayReflector,
                            ),
                        )
                        .insert(RigidBody::Fixed)
                        .insert(Name::new(format!("plane-{}-{}", i, j)))
                        .insert(RaycastMesh::<RayReflector>::default()); // Make this mesh ray cast-able;
                    Cell { height }
                })
                .collect()
        })
        .collect();

    pub const MAP_SIZE: f32 = 100.;
    pub const HALF_MAP_SIZE: f32 = MAP_SIZE / 2.;
    pub const MAP_REFLECTOR_OVERHANG: f32 = 40.;

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: MAP_SIZE })),
            material: materials.add(StandardMaterial {
                alpha_mode: AlphaMode::Blend,
                base_color: Color::Rgba {
                    red: 0.0,
                    green: 0.0,
                    blue: 0.0,
                    alpha: 0.0,
                },
                // emissive: Color::RED,
                unlit: false,
                ..default()
            }),
            transform: Transform {
                translation: Vec3::new(
                    HALF_MAP_SIZE - MAP_REFLECTOR_OVERHANG,
                    GROUND_LEVEL + 0.3,
                    HALF_MAP_SIZE - MAP_REFLECTOR_OVERHANG,
                ),
                ..default()
            },
            visibility: Visibility { is_visible: true },
            ..default()
        })
        .insert(NotShadowReceiver)
        .insert(NotShadowCaster)
        .insert(CursorReflector);
}
