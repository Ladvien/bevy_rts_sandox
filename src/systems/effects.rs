use bevy::prelude::*;

use crate::{components::effects::Blinker, util::map_value_to_range, Game};

pub fn blink_system(
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut blinkers: Query<(Entity, &mut Handle<StandardMaterial>, &mut Blinker), With<Blinker>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    time: Res<Time>,
) {
    if game.mechanics.move_cooldown.tick(time.delta()).finished() {
        for (entity, material, mut blinker) in &mut blinkers {
            if let Some(material) = materials.get_mut(&material) {
                blinker.timer.tick(time.delta());
                if blinker.timer.just_finished() {
                    if blinker.number_of_blinks <= 0 {
                        commands.entity(entity).despawn_recursive();
                        continue;
                    }

                    if blinker.duration < 0. {
                        blinker.duration = 0.;
                        blinker.direction = 1.;
                        blinker.number_of_blinks -= 1;
                    }

                    if blinker.duration > blinker.duration_const {
                        blinker.duration = blinker.duration_const;
                        blinker.direction = -1.;
                    }

                    blinker.duration += (blinker.speed + time.delta_seconds()) * blinker.direction;

                    let alpha =
                        map_value_to_range(blinker.duration, 0., blinker.duration_const, 0., 1.);

                    material.base_color.set_a(alpha);
                    material.emissive.set_a(alpha);
                }
            }
        }
    }
}
