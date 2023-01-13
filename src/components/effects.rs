use bevy::prelude::*;
#[derive(Component, Reflect, Default, Clone)]
#[reflect(Component)]
pub struct Blinker {
    pub timer: Timer,
    pub speed: f32,
    pub duration: f32,
    pub number_of_blinks: usize,
    pub duration_const: f32,
    pub direction: f32,
}

impl Blinker {
    pub fn new(speed: f32, duration: f32, number_of_blinks: usize) -> Self {
        Self {
            timer: Timer::from_seconds(0.01, TimerMode::Repeating),
            speed: speed,
            duration: duration,
            number_of_blinks: number_of_blinks,
            duration_const: duration,
            direction: -1.,
        }
    }
}
