use bevy::prelude::*;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Unit;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Destination(pub Vec3);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Direction {
    pub desired: f32,
    pub current: f32,
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct SelectionHighlighter;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Selected;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Target {
    pub speed: f32,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Health {
    pub value: f32,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Lifetime {
    pub timer: Timer,
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct MovementSpeed {
    pub value: f32,
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct RotationSpeed {
    pub value: f32,
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Rotating;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Ground;
