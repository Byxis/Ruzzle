use raylib::prelude::*;

/// Represents the different animations available for the crab model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrabAnimation {
    Emote = 0,
    Idle = 1,
    IdleJump = 2,
    LandJump = 3,
    MoveFront = 4,
    // UNUSED: MoveHorizontal = 5,
    PreJump = 6,
}

/// Handles the animation of the crab model.
///
/// # Examples
///
/// ```
/// use raylib::prelude::*;
/// use ruzzle::crab::CrabAnimator;
///
/// let mut rl = RaylibHandle::new();
/// let thread = RaylibThread::new();
/// let mut crab = CrabAnimator::new(&mut rl, &thread, "rsc/crab.glb");
/// crab.handle_animation(&mut rl, &thread);
/// ```
pub struct CrabAnimator {
    pub model: Model,
    pub current: CrabAnimation,
    animations: Vec<ModelAnimation>,
    pub anim_frame_counter: i32,
}

impl CrabAnimator {
    /// Creates a new `CrabAnimator` with the given model path.
    pub fn new(rl: &mut RaylibHandle, thread: &RaylibThread, path: &str) -> Self {
        Self {
            model: rl.load_model(thread, path).expect("Failed to load model"),
            current: CrabAnimation::Idle,
            animations: rl
                .load_model_animations(thread, path)
                .expect("Failed to load anims"),
            anim_frame_counter: 0,
        }
    }

    /// Handles the animation of the crab model (Play in loop or change to a new animation).
    pub fn handle_animation(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        let anim_index = self.current as usize;

        rl.update_model_animation(
            thread,
            &mut self.model,
            &self.animations[anim_index],
            self.anim_frame_counter,
        );

        let frame_count = self.animations[anim_index].frameCount;
        self.anim_frame_counter += 1;

        if self.current == CrabAnimation::PreJump && self.anim_frame_counter >= frame_count {
            self.change_animation(CrabAnimation::IdleJump);
        } else if self.current == CrabAnimation::LandJump && self.anim_frame_counter >= frame_count
        {
            self.change_animation(CrabAnimation::Idle);
        } else if self.current == CrabAnimation::Emote && self.anim_frame_counter >= frame_count {
            self.change_animation(CrabAnimation::Idle);
        } else {
            self.anim_frame_counter %= frame_count;
        }
    }

    /// Plays the jump animation if the current animation is idle or in the pre-jump state.
    pub fn jump(&mut self) {
        if !matches!(
            self.current,
            CrabAnimation::IdleJump | CrabAnimation::PreJump | CrabAnimation::LandJump
        ) {
            self.change_animation(CrabAnimation::PreJump);
        }
    }

    /// Plays the land animation if the current animation is not already the land animation.
    pub fn land(&mut self) {
        if self.current != CrabAnimation::LandJump {
            self.change_animation(CrabAnimation::LandJump);
        }
    }

    /// Changes the current animation to the given animation.
    pub fn change_animation(&mut self, animation: CrabAnimation) {
        if self.current != animation {
            self.current = animation;
            self.anim_frame_counter = 0;
        }
    }
}
