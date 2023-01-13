use super::CurrentAnimation;
use bevy::prelude::*;
use rand::Rng;

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Skelly {
    pub current_animation: usize,
}

impl CurrentAnimation for Skelly {
    fn walk(&mut self) {
        self.current_animation = 1;
    }

    fn run(&mut self) {
        self.current_animation = 4;
    }

    fn idle(&mut self) {
        self.current_animation = 0;
    }

    fn attack(&mut self) {
        self.current_animation = rand::thread_rng().gen_range(5..7);
    }

    fn spawn(&mut self) {
        self.current_animation = 2;
    }

    fn alerted(&mut self) {
        self.current_animation = 3;
    }
}
