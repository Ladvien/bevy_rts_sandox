use crate::{
    components::mechanics::{Destination, Rotating, RotationSpeed},
    Game,
};
use bevy::prelude::*;

pub fn rotate_system(
    time: Res<Time>,
    mut commands: Commands,
    mut game: ResMut<Game>,
    mut transforms: Query<
        (Entity, &mut Transform, &RotationSpeed, &Destination),
        (With<RotationSpeed>, With<Destination>, With<Rotating>),
    >,
) {
    for (entity, mut transform, rotation_speed, destination) in &mut transforms {
        if game.mechanics.rotate_cooldown.tick(time.delta()).finished() {
            let difference = transform.translation - destination.0;
            let mut angle_between_pts = difference.z.atan2(difference.x).to_degrees();

            if angle_between_pts < 0.0 {
                angle_between_pts = 360. + angle_between_pts;
            }

            let mut new_angle = transform.rotation.to_axis_angle().1.to_degrees();
            match get_heading(angle_between_pts, new_angle) {
                RotationDirection::CW => new_angle -= rotation_speed.value * time.delta_seconds(),
                RotationDirection::CCW => new_angle += rotation_speed.value * time.delta_seconds(),
            }

            if new_angle > 360.0 {
                new_angle = 0.0;
            }

            if new_angle < 0.0 {
                new_angle = 360.0;
            }

            // If we are approximately in alignment with the destination, remove the
            // Rotating component.
            if new_angle.round() == angle_between_pts.round() {
                commands.entity(entity).remove::<Rotating>();
            }

            transform.rotation = Quat::from_rotation_y(new_angle.to_radians()).inverse();
        }
    }
}

#[derive(Debug)]
enum RotationDirection {
    CW,
    CCW,
}

fn get_heading(desired: f32, current: f32) -> RotationDirection {
    if (desired < current) && (current - desired < 180.0) {
        return RotationDirection::CW;
    }
    if (current < desired) && (desired - current < 180.0) {
        return RotationDirection::CCW;
    }
    if (desired < current) && (current - desired > 180.0) {
        return RotationDirection::CCW;
    }
    if (current < desired) && (desired - current > 180.0) {
        return RotationDirection::CW;
    }
    return RotationDirection::CW;
}

#[cfg(test)]
mod tests {
    use super::get_heading;
    use super::RotationDirection;

    #[test]
    fn rotate_cw_current_180_desired_0() {
        let desired = 0.0;
        let current = 180.0;

        //               D
        //               N
        //        E               W
        //               S
        //               C

        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CW))
    }

    #[test]
    fn rotate_ccw_current_270_desired_0() {
        let desired = 0.0;
        let current = 270.0;

        //               D
        //               N
        //        E               W C
        //               S
        //

        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CCW))
    }

    #[test]
    fn rotate_cw_current_90_desired_0() {
        let desired = 0.0;
        let current = 90.0;

        //               D
        //               N
        //      C E               W
        //               S
        //

        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CW))
    }

    #[test]
    fn rotate_cw_current_90_desired_270() {
        let desired = 270.0;
        let current = 90.0;

        //
        //               N
        //      C E               W D
        //               S
        //

        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CW))
    }

    #[test]
    fn rotate_ccw_current_0_desired_270() {
        let desired = 270.0;
        let current = 0.0;

        //               C
        //               N
        //        E               W D
        //               S
        //

        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CW))
    }

    #[test]
    fn rotate_ccw_current_90_desired_270() {
        let desired = 90.0;
        let current = 270.0;

        //
        //               N
        //       D E               W C
        //               S
        //

        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CW))
    }

    #[test]
    fn rotate_ccw_current_270_desired_180() {
        let desired = 270.0;
        let current = 180.0;

        //
        //               N
        //         E               W D
        //               S
        //               C

        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CCW))
    }

    #[test]
    fn rotate_ccw_current_230_desired_140() {
        let desired = 230.0;
        let current = 140.0;

        //
        //               N
        //        E               W
        //               S
        //          C        D

        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CCW))
    }

    #[test]
    fn rotate_ccw_current_140_desired_50() {
        let desired = 50.0;
        let current = 140.0;

        //          D
        //               N
        //        E               W
        //               S
        //         C

        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CW))
    }

    #[test]
    fn rotate_cw_current_24_desired_25() {
        let desired = 25.0;
        let current = 24.0;
        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CCW))
    }

    #[test]
    fn rotate_cw_current_105_desired_162() {
        let desired = 162.0;
        let current = 105.0;
        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CCW))
    }

    #[test]
    fn rotate_cw_current_59_desired_264() {
        let desired = 264.0;
        let current = 59.0;
        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CW))
    }

    #[test]
    fn rotate_cw_current_194_desired_336() {
        let desired = 336.0;
        let current = 194.0;
        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CCW))
    }

    #[test]
    fn rotate_cw_current_132_desired_203() {
        let desired = 203.0;
        let current = 132.0;
        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CCW))
    }

    #[test]
    fn rotate_cw_current_230_desired_255() {
        let desired = 255.0;
        let current = 230.0;
        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CCW))
    }

    #[test]
    fn rotate_cw_current_288_desired_321() {
        let desired = 321.0;
        let current = 288.0;
        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CCW))
    }

    #[test]
    fn rotate_cw_current_326_desired_174() {
        let desired = 174.0;
        let current = 326.0;
        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CW))
    }

    #[test]
    fn rotate_cw_current_34_desired_168() {
        let desired = 168.0;
        let current = 34.0;
        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CCW))
    }

    #[test]
    fn rotate_cw_current_1_desired_114() {
        let desired = 114.0;
        let current = 1.0;
        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CCW))
    }

    #[test]
    fn rotate_cw_current_90_desired_335() {
        let desired = 335.0;
        let current = 90.0;
        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CW))
    }

    #[test]
    fn rotate_cw_current_238_desired_52() {
        let desired = 52.0;
        let current = 238.0;
        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CCW))
    }

    #[test]
    fn rotate_cw_current_35_desired_4() {
        let desired = 4.0;
        let current = 35.0;
        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CW))
    }

    #[test]
    fn rotate_cw_current_248_desired_66() {
        let desired = 66.0;
        let current = 248.0;
        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CCW))
    }

    #[test]
    fn rotate_cw_current_82_desired_263() {
        let desired = 263.0;
        let current = 82.0;
        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CW))
    }

    #[test]
    fn rotate_cw_current_257_desired_302() {
        let desired = 302.0;
        let current = 257.0;
        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CCW))
    }

    #[test]
    fn rotate_cw_current_234_desired_182() {
        let desired = 182.0;
        let current = 234.0;
        //
        //               N
        //        E               W
        //               S
        //                 D      C
        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CW))
    }

    #[test]
    fn rotate_cw_current_75_desired_79() {
        let desired = 79.0;
        let current = 75.0;
        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CCW))
    }

    #[test]
    fn rotate_cw_current_240_desired_309() {
        let desired = 309.0;
        let current = 240.0;
        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CCW))
    }

    #[test]
    fn rotate_cw_current_243_desired_66() {
        let desired = 66.0;
        let current = 243.0;
        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CW))
    }

    #[test]
    fn rotate_cw_current_315_desired_305() {
        let desired = 305.0;
        let current = 315.0;
        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CW))
    }

    #[test]
    fn rotate_cw_current_337_desired_148() {
        let desired = 148.0;
        let current = 337.0;
        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CCW))
    }

    #[test]
    fn rotate_cw_current_332_desired_353() {
        let desired = 353.0;
        let current = 332.0;
        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CCW))
    }

    #[test]
    fn rotate_cw_current_311_desired_358() {
        let desired = 358.0;
        let current = 311.0;
        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CCW))
    }

    #[test]
    fn rotate_cw_current_238_desired_89() {
        let desired = 89.0;
        let current = 238.0;
        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CW))
    }

    #[test]
    fn rotate_cw_current_35_desired_51() {
        let desired = 51.0;
        let current = 35.0;
        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CCW))
    }

    #[test]
    fn rotate_cw_current_319_desired_215() {
        let desired = 215.0;
        let current = 319.0;
        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CW))
    }

    #[test]
    fn rotate_cw_current_148_desired_156() {
        let desired = 156.0;
        let current = 148.0;
        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CCW))
    }

    #[test]
    fn rotate_cw_current_157_desired_51() {
        //          D
        //               N
        //        E               W
        //               S
        //          C

        let desired = 51.0;
        let current = 157.0;
        let rotation_direction = get_heading(desired, current);
        assert!(matches!(rotation_direction, RotationDirection::CW))
    }
}
