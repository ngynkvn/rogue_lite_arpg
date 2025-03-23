use bevy::prelude::*;
use bevy_enhanced_input::prelude::{Binding, InputAction};

// Player InputActions
#[derive(Debug, InputAction)]
#[input_action(output = Vec2)]
pub struct Movement;

#[derive(Debug, InputAction)]
#[input_action(output = Vec2)]
pub struct Interact;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
pub struct UseEquip;

#[derive(Debug, InputAction)]
#[input_action(output = bool)]
pub struct PauseRequest;

#[derive(Debug, Event)]
pub enum PauseEvent {
    StatsShop,
    Inventory,
    None,
    Paused,
}

#[derive(Resource)]
struct AppSettings {
    keyboard: KeyboardSettings,
}

struct KeyboardSettings {
    movement: KeyCode,
    use_equip: KeyCode,
    interact: KeyCode,
    #[allow(non_snake_case)]
    pushing_P: KeyCode, // Pause menu
}

use crate::player::Player;
pub fn player_binding(mut trigger: Trigger<Binding<Player>>, settings: Res<AppSettings>) {
    trigger.bind::<Movement>().to(settings.keyboard.movement);
    trigger.bind::<Interact>().to(settings.keyboard.interact);
    trigger.bind::<UseEquip>().to(settings.keyboard.use_equip);
    trigger
        .bind::<PauseRequest>()
        .to(settings.keyboard.use_equip);
}

#[cfg(any())]
mod _old {
    pub fn _player_input(
        mut commands: Commands,
        mut keyboard_input: ResMut<ButtonInput<KeyCode>>, // Access keyboard input
        buttons: Res<ButtonInput<MouseButton>>,
        player_movement_query: Single<Entity, With<Player>>,
    ) {
        let player_entity = player_movement_query.into_inner();

        if keyboard_input.clear_just_pressed(KeyCode::Escape) {
            commands.trigger(PauseInputEvent {
                paused_state: Some(PausedState::MainMenu),
            });
            return;
        }

        if keyboard_input.clear_just_pressed(KeyCode::Space) {
            commands.trigger(PlayerInteractionInput);
            return;
        }

        if buttons.pressed(MouseButton::Left) {
            commands.trigger_targets(
                UseEquipmentInputEvent {
                    slot: EquipmentSlot::Mainhand,
                },
                player_entity,
            );
        }

        if buttons.pressed(MouseButton::Right) {
            commands.trigger_targets(
                UseEquipmentInputEvent {
                    slot: EquipmentSlot::Offhand,
                },
                player_entity,
            );
        }

        let mut direction = Vec2::ZERO;

        // Check input for movement and update direction
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        if direction.length() > 0.0 {
            event_writer.send(PlayerMovementEvent { direction });
        } else {
            commands.trigger(PlayerStoppedEvent);
        }
    }
}
