use bevy::prelude::*;

use crate::entities::check_list::{CheckListItem, CheckListMark, CheckListPaper};
use crate::entities::ingredient::IngredientType;
use crate::entities::pan::{PanEgg, PanKaphrao};
use crate::message::ingredient_message::IngredientDroppedMessage;

use crate::resource::{LastDroppedIngredient, game_state::GameState};

pub fn update_checklist_on_drop(
    mut commands: Commands,
    mut events: MessageReader<IngredientDroppedMessage>,
    checklist_query: Query<(Entity, &Children), With<CheckListPaper>>,
    checklist_item_query: Query<&CheckListItem, With<CheckListItem>>,
    mut last_dropped: ResMut<LastDroppedIngredient>,
    asset_server: Res<AssetServer>,
    existing_marks: Query<&CheckListMark, With<CheckListMark>>,
    game_stats: Res<GameState>,
    q_kaphrao_pan: Query<(), With<PanKaphrao>>,
    q_egg_pan: Query<(), With<PanEgg>>,
) {
    let marked_img = asset_server.load("check_list/image/Marked.png");

    for event in events.read() {
        last_dropped.set(event.ingredient_type);

        let Some(target_pan) = event.target_pan else {
            continue;
        };

        let is_kaphrao = q_kaphrao_pan.contains(target_pan);
        let is_egg = q_egg_pan.contains(target_pan);

        if !is_kaphrao && !is_egg {
            continue;
        }

        // For Oil, only mark if both pans have oil
        if event.ingredient_type == IngredientType::Oil {
            if !game_stats.kaphrao_has_oil || !game_stats.egg_has_oil {
                continue;
            }
        }

        for (paper_entity, children) in checklist_query.iter() {
            for child in children.iter() {
                if let Ok(item) = checklist_item_query.get(child) {
                    if item.0 == event.ingredient_type {
                        // Check if this item is already marked on this paper
                        let already_marked = existing_marks
                            .iter()
                            .any(|mark| mark.0 == event.ingredient_type && mark.1 == paper_entity);

                        if !already_marked {
                            commands.entity(paper_entity).with_children(|parent| {
                                parent.spawn((
                                    CheckListMark(event.ingredient_type, paper_entity),
                                    Sprite {
                                        image: marked_img.clone(),
                                        ..default()
                                    },
                                    Transform::from_translation(Vec3::new(0.0, item.1, 100.0)),
                                ));
                            });
                        }
                        break;
                    }
                }
            }
        }
    }
}
