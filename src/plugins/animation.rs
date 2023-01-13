use crate::*;

pub struct AnimationControllerPlugin;

impl Plugin for AnimationControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup_animations))
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(animation_control),
            );
    }
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct AnimationLibrary {
    pub walk: usize,
    pub run: usize,
    pub idle: usize,
    pub attack: usize,
    pub spawn: usize,
    pub alerted: usize,
}

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Animations(pub Option<Vec<Handle<AnimationClip>>>);

#[derive(Component, Reflect, Default, Debug)]
#[reflect(Component)]
pub struct Animated {
    pub current_animation: usize,
    pub animations: Animations,
    pub animation_library: AnimationLibrary,
}

// Once the scene is loaded, start the animation
fn setup_animations(
    mut unit_animations: Query<&mut Animated>,
    mut player: Query<&mut AnimationPlayer>,
    mut done: Local<bool>,
) {
    if !*done {
        if let Ok(mut player) = player.get_single_mut() {
            for unit in unit_animations.iter_mut() {
                if let Some(animations) = &unit.animations.0 {
                    player
                        .play(animations[unit.current_animation].clone_weak())
                        .repeat();
                    *done = true;
                }
            }
        }
    }
}

fn animation_control(
    mut animation_player: Query<&mut AnimationPlayer>,
    mut unit_animations: Query<&mut Animated>,
    mut current_animation_frame: Local<usize>,
) {
    *current_animation_frame = 2;
    if let Ok(mut player) = animation_player.get_single_mut() {
        for animation in unit_animations.iter_mut() {
            if let Some(animations) = &animation.animations.0 {
                player
                    .play(animations[animation.current_animation].clone_weak())
                    .repeat();
            }
        }
    }
}
