use bevy::prelude::*;

#[derive(Message, Debug, Clone)]
pub struct GaugeKaprowHitMassage;

#[derive(Message, Debug, Clone)]
pub struct GaugeEggHitMassage;

#[derive(Message, Debug, Clone)]
pub struct GaugeSpawnMassage {
    pub target_pan: Option<Entity>,
}
