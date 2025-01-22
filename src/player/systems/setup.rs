use std::time::Duration;

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    combat::{
        attributes::{mana::Mana, Health},
        damage::components::HasIFrames,
        weapon::staffs::spawn_fire_staff,
    },
    configuration::assets::SpriteAssets,
    helpers::labels::GameCollisionLayer,
    items::{spawn_health_potion, spawn_helmet, spawn_shovel, spawn_sword},
    labels::{layer::ZLayer, states::AppState},
    movement::components::SimpleMotion,
    player::{systems::*, Inventory, Player, PlayerEquipmentSlots, PlayerStats},
};

#[derive(Component, Default)]
pub struct AimPosition {
    pub position: Vec2, // position where entitiy is aiming, for player this is the cursor
}

pub fn player_setup(
    mut commands: Commands,
    mut game_state: ResMut<NextState<AppState>>,
    sprites: Res<SpriteAssets>,
) {
    let mut inventory = Inventory::default_inventory();
    let _ = inventory.add_item(spawn_health_potion(&mut commands));
    let _ = inventory.add_item(spawn_sword(&mut commands, &sprites));
    let _ = inventory.add_item(spawn_helmet(&mut commands, &sprites));
    let _ = inventory.add_item(spawn_shovel(&mut commands, &sprites));

    inventory
        .add_item(spawn_fire_staff(&mut commands, &sprites))
        .ok();

    commands
        .spawn((
            Player,
            PlayerStats::default(),
            AimPosition::default(),
            SimpleMotion::new(450.0),
            Health::new(100.0),
            Mana::new(100.0),
            inventory,
            PlayerEquipmentSlots::default(),
            HasIFrames {
                duration: Duration::from_secs(1),
            },
            RigidBody::Dynamic,
            Collider::rectangle(100.0, 100.0),
            CollisionLayers::new(
                GameCollisionLayer::Player,
                [
                    GameCollisionLayer::Npc,
                    GameCollisionLayer::Interaction,
                    GameCollisionLayer::Portal,
                    GameCollisionLayer::Enemy,
                    GameCollisionLayer::Wall,
                    GameCollisionLayer::Water,
                ],
            ),
            LockedAxes::new().lock_rotation(),
            Sprite::from_image(sprites.skeleton_player.clone()),
            Transform::from_xyz(0., 0., ZLayer::Player.z()),
        ))
        .observe(death::on_player_defeated)
        .observe(equip::on_main_hand_activated);
    game_state.set(AppState::Playing);
}
