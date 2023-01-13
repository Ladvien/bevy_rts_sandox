use bevy::prelude::*;

use crate::{
    components::mechanics::{Destination, MovementSpeed},
    constants::{
        constants::GROUND_LEVEL,
        plane::GAME_BOUNDS,
        units::{ARRIVAL_TOLERANCE, SOCIAL_DISTANCE},
    },
    util::{are_positions_near, keep_in_bounds},
    Game,
};

pub fn adjust_still_units_system(
    mut units: Query<
        (Entity, &mut Transform, &MovementSpeed),
        (With<MovementSpeed>, Without<Destination>),
    >,
    mut game: ResMut<Game>,
    time: Res<Time>,
) {
    if game.mechanics.move_cooldown.tick(time.delta()).finished() {
        let all_units_positions: Vec<(Entity, Vec3)> = units
            .into_iter()
            .map(|t| return (t.0, t.1.translation))
            .collect();

        for (entity, mut transform, speed) in &mut units {
            if game.mechanics.move_cooldown.tick(time.delta()).finished() {
                let new_destination = adjust_movement_for_neighbors(
                    &entity,
                    &transform.translation,
                    transform.translation,
                    &all_units_positions,
                );

                transform.translation = move_unit(
                    &transform.translation,
                    new_destination,
                    &speed,
                    time.delta_seconds(),
                )
            }
        }
    }
}

pub fn movement_system(
    mut commands: Commands,
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut units: Query<
        (Entity, &mut Transform, &mut Destination, &MovementSpeed),
        (With<Destination>, With<MovementSpeed>),
    >,
) {
    let units_positions: Vec<(Entity, Vec3)> = units
        .into_iter()
        .map(|t| return (t.0, t.1.translation))
        .collect();
    for (entity, mut transform, destination, speed) in &mut units {
        if game.mechanics.move_cooldown.tick(time.delta()).finished() {
            let new_destination = adjust_movement_for_neighbors(
                &entity,
                &transform.translation,
                destination.0,
                &units_positions,
            );
            transform.translation = move_unit(
                &transform.translation,
                new_destination,
                &speed,
                time.delta_seconds(),
            );
            stop_at_destination(
                &mut commands,
                entity,
                transform.translation,
                destination.0,
                ARRIVAL_TOLERANCE,
            );
        }
    }
}

fn stop_at_destination(
    commands: &mut Commands,
    unit: Entity,
    unit_position: Vec3,
    destination: Vec3,
    arrival_tolerance: f32,
) -> () {
    if are_positions_near(&destination, &unit_position, arrival_tolerance) {
        commands.entity(unit).remove::<Destination>();
    }
}

fn move_unit(
    unit_position: &Vec3,
    new_destination: Vec3,
    unit_speed: &MovementSpeed,
    delta_seconds: f32,
) -> Vec3 {
    let mut new_unit_position =
        unit_position.lerp(new_destination, unit_speed.value * delta_seconds);
    new_unit_position = keep_in_bounds(GAME_BOUNDS, new_unit_position, 2.);
    new_unit_position.y = GROUND_LEVEL;
    new_unit_position
}

fn adjust_movement_for_neighbors(
    unit: &Entity,
    unit_position: &Vec3,
    unit_destination: Vec3,
    all_units_positions: &Vec<(Entity, Vec3)>,
) -> Vec3 {
    let mut new_destination = Vec3::from(unit_destination);
    for (other_entity, other_position) in all_units_positions {
        // Skip comparing to self.
        if *unit == *other_entity {
            continue;
        }

        if are_positions_near(&unit_position, &other_position, SOCIAL_DISTANCE) {
            // println!("These guys are buddies!");
            let difference = *unit_position - *other_position;
            let minimum_distance = SOCIAL_DISTANCE + SOCIAL_DISTANCE; // Replace with unit specific social distances
            new_destination +=
                difference.normalize() * (minimum_distance - difference) / minimum_distance;
        }
    }
    new_destination
}
