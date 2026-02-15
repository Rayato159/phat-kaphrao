use bevy::prelude::*;

use crate::entities::check_list::{CheckListItem, CheckListPaper};
use crate::entities::ingredient::IngredientType;
use crate::entities::pan::{EggPanStepStateTag, KaprowPanStepStateTag, PanEgg, PanKapaow};
use crate::message::ingredient_message::IngredientDroppedMessage;

use crate::resource::cooking_state::{EggCookingState, KaprowCookingState};
use crate::resource::LastDroppedIngredient;

pub fn update_checklist_on_drop(
    mut commands: Commands,
    mut events: MessageReader<IngredientDroppedMessage>,
    checklist_query: Query<(&Children, &Transform), With<CheckListPaper>>,
    checklist_item_query: Query<&CheckListItem, With<CheckListItem>>,
    mut last_dropped: ResMut<LastDroppedIngredient>,
    asset_server: Res<AssetServer>,
    kaprow_pan_step_tag: Single<&KaprowPanStepStateTag>,
    egg_pan_step_tag: Single<&EggPanStepStateTag>,
    q_kaprow_pan: Query<(), With<PanKapaow>>,
    q_egg_pan: Query<(), With<PanEgg>>,
) {
    let marked_img = asset_server.load("check_list/image/Marked.png");

    for event in events.read() {
        last_dropped.set(event.ingredient_type);

        let Some(target_pan) = event.target_pan else {
            continue;
        };

        let is_kaprow = q_kaprow_pan.contains(target_pan);
        let is_egg = q_egg_pan.contains(target_pan);

        // ถ้าไม่ใช่ทั้งคู่ ก็ไม่ต้อง mark
        if !is_kaprow && !is_egg {
            continue;
        }

        let kaprow_step = kaprow_pan_step_tag.0.clone();
        let egg_step = egg_pan_step_tag.0.clone();

        let allowed = if is_kaprow {
            match event.ingredient_type {
                IngredientType::Oil => kaprow_step == KaprowCookingState::Oil,
                IngredientType::Garlic => kaprow_step == KaprowCookingState::Garlic,
                IngredientType::Chili => kaprow_step == KaprowCookingState::Chili,
                IngredientType::Pork => kaprow_step == KaprowCookingState::Pork,
                IngredientType::OysterSauce => kaprow_step == KaprowCookingState::OysterSauce,
                IngredientType::MSG => kaprow_step == KaprowCookingState::MSG,
                IngredientType::Kaprow => kaprow_step == KaprowCookingState::Kaprow,
                IngredientType::Egg => false,
                _ => true,
            }
        } else {
            // egg pan
            match event.ingredient_type {
                IngredientType::Oil => egg_step == EggCookingState::Oil,
                IngredientType::Egg => egg_step == EggCookingState::Egg,
                _ => false,
            }
        };

        if !allowed {
            continue;
        }

        for (children, paper_tf) in checklist_query.iter() {
            for child in children.iter() {
                if let Ok(item) = checklist_item_query.get(child) {
                    if item.0 == event.ingredient_type {
                        commands.spawn((
                            Sprite {
                                image: marked_img.clone(),
                                ..default()
                            },
                            Transform::from_translation(
                                paper_tf.translation + Vec3::new(0.0, item.1, 100.0),
                            ),
                        ));
                        break;
                    }
                }
            }
        }
    }
}
