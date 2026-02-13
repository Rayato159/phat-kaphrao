//! Gauge Components and Events
//!
//! Contains all component definitions related to the timing gauge mini-game:
//! - TimingGauge: Main timing gauge component for the mini-game
//! - BarGauge: Component for the bar gauge visualization
//! - BallGaugeKapaow: Component for the Kapaow recipe ball gauge
//! - BallGaugeEgg: Component for the Egg recipe ball gauge
//! - GaugeSpawnEvent: Event fired when a timing gauge should be spawned

use bevy::prelude::*;

/// Component for the Timing Gauge mini-game
/// This is displayed when oil is added to the pan and requires
/// the player to hit the spacebar when the indicator is in the green zone
#[derive(Component)]
pub struct TimingGauge {
    /// The target zone where the player should hit (0.0 to 1.0)
    pub target_zone_start: f32,
    pub target_zone_end: f32,
    /// Current position of the gauge indicator (0.0 to 1.0)
    pub indicator_position: f32,
    /// Direction of movement (-1.0 for left, 1.0 for right)
    pub direction: f32,
    /// Speed of the gauge movement
    pub speed: f32,
    /// Whether the player has attempted to hit
    pub has_hit: bool,
    /// Whether the hit was successful (in target zone)
    pub is_success: bool,
}

impl Default for TimingGauge {
    fn default() -> Self {
        Self {
            target_zone_start: 0.4,
            target_zone_end: 0.6,
            indicator_position: 0.0,
            direction: 1.0,
            speed: 1.5,
            has_hit: false,
            is_success: false,
        }
    }
}

/// Component for the Kapaow Ball Gauge
/// Used for the Kapaow recipe timing mini-game
#[derive(Component)]
pub struct BallGauge {
    /// Current position of the ball (0.0 to 1.0)
    pub position: f32,
    /// Direction of movement (-1.0 for left, 1.0 for right)
    pub direction: f32,
    /// Speed of the ball movement
    pub speed: f32,
    pub target_zone_start: f32,
    /// End of the target zone (0.0 to 1.0)
    pub target_zone_end: f32,
}

impl Default for BallGauge {
    fn default() -> Self {
        Self {
            position: 0.0,
            direction: 1.0,
            speed: 1.5,
            target_zone_start: 0.4,
            target_zone_end: 0.6,
        }
    }
}

/// Component for a gauge that follows a specific pan
/// Used to link timing gauges to their respective cooking pans
#[derive(Component)]
pub struct GaugeFollowsPan {
    pub pan_entity: Entity,
}
