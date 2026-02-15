use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub fn start_music(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio
        .play(asset_server.load("audio/Rayato159 - Coffee Break.mp3"))
        // Fade-in with a dynamic easing
        .fade_in(AudioTween::new(
            Duration::from_secs(0),
            AudioEasing::OutPowi(0),
        ))
        // Play at lower volume (-10dB)
        .with_volume(-10.)
        .looped();
}
