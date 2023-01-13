use bevy::prelude::Vec3;

// pub fn mean(numbers: Vec<f32>) -> f32 {
//     let sum: f32 = numbers.iter().sum();
//     sum as f32 / numbers.len() as f32
// }

pub fn map_value_to_range(value: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    return (value - in_min) * (out_max - out_min) / (in_max - in_min) + out_min;
}

pub fn are_positions_near(v1: &Vec3, v2: &Vec3, sensitivity: f32) -> bool {
    v2.cmpgt(*v1 - sensitivity).all() && v2.cmplt(*v1 + sensitivity).all()
}

pub struct Bounds2D {
    pub min_x: f32,
    pub min_z: f32,
    pub max_x: f32,
    pub max_z: f32,
}

pub fn keep_in_bounds(bounds: Bounds2D, mut pos: Vec3, padding: f32) -> Vec3 {
    if pos.x < bounds.min_x + padding {
        pos.x = bounds.min_x + padding
    };
    if pos.x > bounds.max_x - padding {
        pos.x = bounds.max_x - padding
    };
    if pos.z < bounds.min_z + padding {
        pos.z = bounds.min_z + padding
    };
    if pos.z > bounds.max_z - padding {
        pos.z = bounds.max_z - padding
    };
    pos
}

#[cfg(test)]
mod tests {

    use bevy::prelude::Vec3;

    use super::are_positions_near;
    pub const GROUND_LEVEL: f32 = 8.;

    #[test]
    fn point_v1_is_near_v2_by_1_1_gameplay_units_when_v2_in_quad_1() {
        let v1 = Vec3::new(8., 0., 8.);
        let v2 = Vec3::new(7., 0., 7.);

        assert!(are_positions_near(&v1, &v2, 1.1));
    }

    #[test]
    fn point_v1_is_near_v2_by_1_1_gameplay_units_when_v2_in_quad_2() {
        let v1 = Vec3::new(8., 0., 8.);
        let v2 = Vec3::new(9., 0., 7.);

        assert!(are_positions_near(&v1, &v2, 1.1));
    }

    #[test]
    fn point_v1_is_near_v2_by_1_1_gameplay_units_when_v2_in_quad_3() {
        let v1 = Vec3::new(8., 0., 8.);
        let v2 = Vec3::new(9., 0., 9.);

        assert!(are_positions_near(&v1, &v2, 1.1));
    }

    #[test]
    fn point_v1_is_near_v2_by_1_1_gameplay_units_when_v2_in_quad_4() {
        let v1 = Vec3::new(8., 0., 8.);
        let v2 = Vec3::new(7., 0., 9.);

        assert!(are_positions_near(&v1, &v2, 1.1));
    }

    #[test]
    fn point_v2_is_near_v1_by_1_1_gameplay_units_when_v2_in_quad_1() {
        let v1 = Vec3::new(8., 0., 8.);
        let v2 = Vec3::new(7., 0., 7.);

        assert!(are_positions_near(&v2, &v1, 1.1));
    }

    #[test]
    fn point_v2_is_near_v1_by_1_1_gameplay_units_when_v2_in_quad_2() {
        let v1 = Vec3::new(8., 0., 8.);
        let v2 = Vec3::new(9., 0., 7.);

        assert!(are_positions_near(&v2, &v1, 1.1));
    }

    #[test]
    fn point_v2_is_near_v1_by_1_1_gameplay_units_when_v2_in_quad_3() {
        let v1 = Vec3::new(8., 0., 8.);
        let v2 = Vec3::new(9., 0., 9.);

        assert!(are_positions_near(&v2, &v1, 1.1));
    }

    #[test]
    fn point_v2_is_near_v1_by_1_1_gameplay_units_when_v2_in_quad_4() {
        let v1 = Vec3::new(8., 0., 8.);
        let v2 = Vec3::new(7., 0., 9.);

        assert!(are_positions_near(&v2, &v1, 1.1));
    }

    #[test]
    fn point_v1_not_near_v2_in_quad_1() {
        let v1 = Vec3::new(8., 0., 8.);
        let v2 = Vec3::new(2., 0., 2.);

        assert!(!are_positions_near(&v1, &v2, 1.1));
    }

    #[test]
    fn point_v1_not_near_v2_in_quad_3() {
        let v1 = Vec3::new(8., 0., 8.);
        let v2 = Vec3::new(11., 0., 11.);

        assert!(!are_positions_near(&v1, &v2, 1.1));
    }
}
