use avian2d::prelude::CollidingEntities;

use bevy::prelude::*;

use crate::{
    despawn::components::LiveDuration,
    npc::events::DialogueBegin,
    player::{AttemptInteractionInput, Player},
};

use super::components::NPCInteractionRadius;

// Only query colliding entities with the NPCInteractionRadius component
// When it finds that they are in range, kick off a start dialogue trigger
pub fn handle_dialogue_input(
    _: Trigger<AttemptInteractionInput>,
    mut commands: Commands,
    query: Query<(&Parent, &CollidingEntities), With<NPCInteractionRadius>>,
    player_query: Query<Entity, With<Player>>,
) {
    let player_entity = player_query.single();
    for (parent, colliding_entities) in &query {
        // Check if any of the colliding entities is the player
        if colliding_entities.contains(&player_entity) {
            commands.trigger(DialogueBegin {
                entity: parent.get(),
            });
        }
    }
}

//TODO: Replace all of this with a proper dialogue system
//Temp stuff to test this feature
#[derive(Component)]
#[require(LiveDuration)]
pub struct DialogueBubble {
    initial_alpha: f32,
    owning_entity: Entity,
}

// Triggers once the players presses interact in an NPCs interaction radius
// Insert a child entity with a dialogue box
// above the NPC saying "hello!"
pub fn begin_dialogue(
    dialogue_begin_trigger: Trigger<DialogueBegin>,
    mut commands: Commands,
    query: Query<&Transform>,
    camera_query: Query<(&Camera, &GlobalTransform)>, // Add camera query
) {
    // Get the camera and its transform
    if let Ok((camera, camera_transform)) = camera_query.get_single() {
        // Get the transform component for that entity
        if let Ok(npc_transform) = query.get(dialogue_begin_trigger.entity) {
            // Calculate position above NPC's head in world space
            let y_offset = 110.0;
            let world_pos = npc_transform.translation + Vec3::new(0.0, y_offset, 0.1);

            // Convert world position to screen space
            if let Ok(screen_pos) = camera.world_to_viewport(camera_transform, world_pos) {
                // Spawn the dialogue bubble as a UI element
                commands
                    .spawn((
                        BackgroundColor::from(Color::WHITE),
                        BorderColor::from(Color::BLACK),
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Px(screen_pos.x),
                            top: Val::Px(screen_pos.y),
                            padding: UiRect::all(Val::Px(10.0)),
                            border: UiRect::all(Val::Px(2.0)),
                            ..default()
                        },
                    ))
                    .with_children(|parent| {
                        parent.spawn((
                            Text::new("You wanted something?"),
                            TextFont::default(),
                            TextColor::from(Color::BLACK),
                        ));
                    })
                    .insert(DialogueBubble {
                        initial_alpha: 0.9,
                        owning_entity: dialogue_begin_trigger.entity,
                    });
            }
        }
    }
}

// Update the dialogue bubbles system to handle screen space positioning
pub fn update_dialogue_bubbles(
    mut query: Query<(
        &LiveDuration,
        &DialogueBubble,
        &mut BackgroundColor,
        &mut Node,
    )>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    npc_query: Query<&Transform>, // Query to get NPC positions
) {
    if let Ok((camera, camera_transform)) = camera_query.get_single() {
        for (time_alive, bubble, mut background, mut node) in query.iter_mut() {
            // Calculate fade based on remaining time
            let progress = time_alive.0.fraction();
            let alpha = bubble.initial_alpha * (1.0 - progress);

            if let Ok(npc_transform) = npc_query.get(bubble.owning_entity) {
                let y_offset = 110.0;
                let world_pos = npc_transform.translation + Vec3::new(0.0, y_offset, 0.1);
                if let Ok(screen_pos) = camera.world_to_viewport(camera_transform, world_pos) {
                    node.left = Val::Px(screen_pos.x);
                    node.top = Val::Px(screen_pos.y);
                }
            }
            // Update background transparency
            background.0.set_alpha(alpha);
        }
    }
}
