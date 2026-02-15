use bevy::prelude::*;

#[derive(Message, Debug, Clone)]
pub struct GaugeKaphraoHitMassage;

#[derive(Message, Debug, Clone)]
pub struct GaugeEggHitMassage;

#[derive(Message, Debug, Clone)]
pub struct GaugeMissMassage;

#[derive(Message, Debug, Clone)]
pub struct GaugeSpawnMassage {
    pub target_pan: Option<Entity>,
}
