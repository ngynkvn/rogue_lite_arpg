use crate::labels::states::AppState;
use bevy::prelude::*;

mod asset_barrier;
pub use asset_barrier::AssetBarrier;
pub mod asset_group;
pub use asset_group::AssetGroup;

pub struct AssetLoadingPlugin;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::AssetLoading), load_assets_system)
            .add_systems(
                Update,
                exit_asset_loading.run_if(
                    in_state(AppState::AssetLoading)
                        .and(AssetBarrier::<()>::assets_ready)
                        .and(run_once),
                ),
            )
            .init_resource::<SpriteAssets>()
            .init_resource::<SpriteSheetLayouts>()
            .init_resource::<GameIcons>();
    }
}
fn exit_asset_loading(mut app_state: ResMut<NextState<AppState>>) {
    app_state.set(AppState::SpawnPlayer);
}

fn load_assets_system(mut commands: Commands, server: Res<AssetServer>) {
    let guard = AssetBarrier::<()>::new();
    commands.insert_resource(guard.clone());

    let game_icons = GameIcons {
        equip_icon: server.load_acquire("icons/equip_marker.png", guard.clone()),
        potion_icon: server.load_acquire("icons/potion.png", guard.clone()),
        spell_book_icon: server.load_acquire("icons/spell-book.png", guard.clone()),
        melee_icon: server.load_acquire("icons/sword-brandish.png", guard.clone()),
        staff_icon: server.load_acquire("icons/wizard-staff.png", guard.clone()),
    };
    commands.insert_resource(game_icons);

    let sprite_assets = SpriteAssets {
        gold_coin: server.load_acquire("coin.png", guard.clone()),
        tome_of_healing: server.load_acquire("items/tome_of_healing.png", guard.clone()),
        sword: server.load_acquire("items/sword.png", guard.clone()),
        axe: server.load_acquire("items/axe.png", guard.clone()),
        fire_staff: server.load_acquire("items/fire_staff.png", guard.clone()),
        ice_staff: server.load_acquire("items/ice_staff.png", guard.clone()),
        health_potion: server.load_acquire("items/health_potion.png", guard.clone()),
        ice_bolt: server.load_acquire("projectiles/IceBolt.png", guard.clone()),
        fire_ball: server.load_acquire("projectiles/fireball.png", guard.clone()),
        exit_door: server.load_acquire("door.png", guard.clone()),
        ground_tiles: server.load_acquire("tilesets/ground_tiles.png", guard.clone()),
        grass_tiles: server.load_acquire("tilesets/grass_tiles.png", guard.clone()),
        water_tiles: server.load_acquire("tilesets/water_tiles.png", guard.clone()),
        wall_tiles: server.load_acquire("tilesets/wall_tiles.png", guard.clone()),
        wood_tiles: server.load_acquire("tilesets/wood_tiles.png", guard.clone()),
        cobblestone_tiles: server.load_acquire("tilesets/cobblestone_tiles.png", guard.clone()),
        run_start_door: server.load_acquire("door.png", guard.clone()),
        chests_sprite_sheet: server.load_acquire("chests.png", guard.clone()),
        tome_of_healing_effect_sprite_sheet: server
            .load_acquire("spells/tome_of_healing_effect.png", guard.clone()),
        player_sprite_sheet: server.load_acquire("player/player_sprite_sheet.png", guard.clone()),
        ice_mage_enemy_sprite_sheet: server
            .load_acquire("enemies/ice_mage_enemy.png", guard.clone()),
        warrior_enemy_sprite_sheet: server.load_acquire("enemies/warrior_enemy.png", guard.clone()),
        fire_mage_enemy_sprite_sheet: server
            .load_acquire("enemies/fire_mage_enemy.png", guard.clone()),
        shop_keeper_sprite_sheet: server.load_acquire("npcs/shop_keeper.png", guard.clone()),
        game_guide_sprite_sheet: server.load_acquire("npcs/game_guide.png", guard.clone()),
        stat_trainer_sprite_sheet: server.load_acquire("npcs/stat_trainer.png", guard.clone()),
    };
    commands.insert_resource(sprite_assets);

    let player_atlas_layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 13, 21, None, None);
    let enemy_atlas_layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 13, 21, None, None);
    let fireball_layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 5, 1, None, None);
    let ice_bolt_layout = TextureAtlasLayout::from_grid(UVec2::splat(64), 5, 1, None, None);
    let bat_enemy_layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 4, 4, None, None);
    let chest_layout = TextureAtlasLayout::from_grid(
        UVec2::splat(48),
        5,
        8,
        Some(UVec2::new(0, 8)),
        Some(UVec2::new(0, 8)),
    );
    let tome_of_healing_effect =
        TextureAtlasLayout::from_grid(UVec2::splat(100), 10, 1, None, None);
    let sprite_sheet_layouts = SpriteSheetLayouts {
        player_atlas_layout: server.add(player_atlas_layout),
        enemy_atlas_layout: server.add(enemy_atlas_layout),
        fireball_layout: server.add(fireball_layout),
        ice_bolt_layout: server.add(ice_bolt_layout),
        bat_enemy_layout: server.add(bat_enemy_layout),
        chest_layout: server.add(chest_layout),
        tome_of_healing_effect: server.add(tome_of_healing_effect),
    };
    commands.insert_resource(sprite_sheet_layouts);
}

#[derive(Resource, Default)]
pub struct SpriteSheetLayouts {
    pub player_atlas_layout: Handle<TextureAtlasLayout>,
    pub enemy_atlas_layout: Handle<TextureAtlasLayout>,
    pub fireball_layout: Handle<TextureAtlasLayout>,
    pub ice_bolt_layout: Handle<TextureAtlasLayout>,
    pub bat_enemy_layout: Handle<TextureAtlasLayout>,
    pub chest_layout: Handle<TextureAtlasLayout>,
    pub tome_of_healing_effect: Handle<TextureAtlasLayout>,
}

#[derive(Resource, Default)]
pub struct GameIcons {
    pub equip_icon: Handle<Image>,
    pub potion_icon: Handle<Image>,
    pub spell_book_icon: Handle<Image>,
    pub melee_icon: Handle<Image>,
    pub staff_icon: Handle<Image>,
}

#[derive(Resource, Default)]
pub struct SpriteAssets {
    pub gold_coin: Handle<Image>,
    pub tome_of_healing: Handle<Image>,
    pub sword: Handle<Image>,
    pub axe: Handle<Image>,
    pub fire_staff: Handle<Image>,
    pub ice_staff: Handle<Image>,
    pub health_potion: Handle<Image>,
    pub ice_bolt: Handle<Image>,
    pub fire_ball: Handle<Image>,
    pub exit_door: Handle<Image>,
    pub ground_tiles: Handle<Image>,
    pub grass_tiles: Handle<Image>,
    pub water_tiles: Handle<Image>,
    pub wall_tiles: Handle<Image>,
    pub wood_tiles: Handle<Image>,
    pub cobblestone_tiles: Handle<Image>,
    pub run_start_door: Handle<Image>,
    pub chests_sprite_sheet: Handle<Image>,
    pub tome_of_healing_effect_sprite_sheet: Handle<Image>,
    pub player_sprite_sheet: Handle<Image>,
    pub ice_mage_enemy_sprite_sheet: Handle<Image>,
    pub warrior_enemy_sprite_sheet: Handle<Image>,
    pub fire_mage_enemy_sprite_sheet: Handle<Image>,
    pub shop_keeper_sprite_sheet: Handle<Image>,
    pub game_guide_sprite_sheet: Handle<Image>,
    pub stat_trainer_sprite_sheet: Handle<Image>,
}
