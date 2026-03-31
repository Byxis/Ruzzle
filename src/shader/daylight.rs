use raylib::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

/// Lighting state for a specific time of day
///
/// # Examples
///
/// ```
/// use ruzzle::shader::daylight::LightingState;
///
/// let state = LightingState {
///     color: Color::WHITE,
///     ambient: Color::BLACK,
///     bg_top: Color::WHITE,
///     bg_bottom: Color::BLACK,
/// };
/// ```
struct LightingState {
    color: Color,
    ambient: Color,
    bg_top: Color,
    bg_bottom: Color,
}

/// Manages the day cycle and lighting state based on real or test time.
///
/// # Examples
///
/// ```
/// use ruzzle::shader::daylight::DayCycleManager;
///
/// let mut manager = DayCycleManager::new();
/// manager.set_test_hour(Some(12.0));
///
/// let state = manager.get_lighting_state();
/// ```
pub struct DayCycleManager {
    test_hour: Option<f32>,
}

impl DayCycleManager {
    pub fn new() -> Self {
        DayCycleManager { test_hour: None }
    }

    /// Set a test hour (0-24) for debugging. Set to None to use real time.
    pub fn set_test_hour(&mut self, hour: Option<f32>) {
        self.test_hour = hour;
    }

    /// Get current hour (0-24) - uses test_hour if set, otherwise system time
    fn get_current_hour(&self) -> f32 {
        if let Some(test_hour) = self.test_hour {
            return test_hour;
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let seconds_per_day = 86400;
        let seconds_today = now % seconds_per_day;
        let hours = seconds_today as f32 / 3600.0;
        hours
    }

    /// Convert hour (0-24) to normalized cycle position (0-1)
    fn hour_to_cycle_position(&self, hour: f32) -> f32 {
        hour / 24.0
    }

    /// Get lighting state based on real time of day
    ///
    /// Night (0-5)
    /// Dawn (5-7) - 2 hour transition
    /// Early morning (7-9) - 2 hour transition
    /// Peak daylight (LONG stable)
    /// Dusk (17-19) - 2 hour transition
    /// Evening (19-22) - 2 hour transition
    /// Night (22-24)
    fn get_lighting_state_at(&self, hour: f32) -> LightingState {
        if hour < 5.0 {
            Self::midnight_lighting()
        } else if hour < 7.0 {
            let local_t = (hour - 5.0) / 2.0;
            Self::lerp_lighting(
                Self::midnight_lighting(),
                Self::early_morning_lighting(),
                local_t,
            )
        } else if hour < 9.0 {
            let local_t = (hour - 7.0) / 2.0;
            Self::lerp_lighting(
                Self::early_morning_lighting(),
                Self::noon_lighting(),
                local_t,
            )
        } else if hour < 17.0 {
            Self::noon_lighting()
        } else if hour < 19.0 {
            let local_t = (hour - 17.0) / 2.0;
            Self::lerp_lighting(Self::noon_lighting(), Self::dusk_lighting(), local_t)
        } else if hour < 22.0 {
            Self::dusk_lighting()
        } else {
            let local_t = (hour - 22.0) / 2.0;
            Self::lerp_lighting(Self::dusk_lighting(), Self::midnight_lighting(), local_t)
        }
    }

    /// Midnight lighting (0-5) - 2 hour transition
    fn midnight_lighting() -> LightingState {
        LightingState {
            color: Color::new(51, 51, 77, 255), // Cool blue light (moonlight)
            ambient: Color::new(13, 13, 26, 255), // Very dim
            bg_top: Color::new(38, 38, 64, 255), // Dark blue
            bg_bottom: Color::new(26, 26, 51, 255), // Darker blue
        }
    }

    /// Early morning lighting (7-9) - 2 hour transition
    fn early_morning_lighting() -> LightingState {
        LightingState {
            color: Color::new(255, 179, 102, 255),    // Warm orange
            ambient: Color::new(77, 51, 38, 255),     // Warm dim
            bg_top: Color::new(255, 153, 77, 255),    // Orange sky
            bg_bottom: Color::new(204, 102, 51, 255), // Orange-red
        }
    }

    /// Noon lighting (9-17) - 8 hour stable period
    fn noon_lighting() -> LightingState {
        LightingState {
            color: Color::new(255, 230, 191, 255),     // Warm sunlight
            ambient: Color::new(89, 77, 64, 255),      // Bright warm
            bg_top: Color::new(173, 217, 250, 255),    // Pastel blue
            bg_bottom: Color::new(217, 191, 242, 255), // Pastel purple
        }
    }

    /// Dusk lighting (17-19) - 2 hour transition
    fn dusk_lighting() -> LightingState {
        LightingState {
            color: Color::new(255, 153, 128, 255),   // Red-orange
            ambient: Color::new(64, 38, 51, 255),    // Warm dim
            bg_top: Color::new(230, 102, 77, 255),   // Red-orange sky
            bg_bottom: Color::new(77, 51, 102, 255), // Purple
        }
    }

    /// Smoothly interpolate between two lighting states
    fn lerp_lighting(from: LightingState, to: LightingState, t: f32) -> LightingState {
        LightingState {
            color: from.color.lerp(to.color, t),
            ambient: from.ambient.lerp(to.ambient, t),
            bg_top: from.bg_top.lerp(to.bg_top, t),
            bg_bottom: from.bg_bottom.lerp(to.bg_bottom, t),
        }
    }

    /// Get the current light color based on the current hour
    pub fn get_light_color(&self) -> Color {
        let hour = self.get_current_hour();
        self.get_lighting_state_at(hour).color
    }

    /// Get the current ambient color based on the current hour
    pub fn get_ambient_color(&self) -> Color {
        let hour = self.get_current_hour();
        self.get_lighting_state_at(hour).ambient
    }

    /// Get the current background top color based on the current hour
    pub fn get_background_top(&self) -> Color {
        let hour = self.get_current_hour();
        self.get_lighting_state_at(hour).bg_top
    }

    /// Get the current background bottom color based on the current hour
    pub fn get_background_bottom(&self) -> Color {
        let hour = self.get_current_hour();
        self.get_lighting_state_at(hour).bg_bottom
    }

    /// Get the current time string based on the current hour
    pub fn get_current_time_string(&self) -> String {
        let hour = self.get_current_hour() as u32;
        let minute = ((self.get_current_hour() % 1.0) * 60.0) as u32;
        format!("{:02}:{:02}", hour, minute)
    }
}
