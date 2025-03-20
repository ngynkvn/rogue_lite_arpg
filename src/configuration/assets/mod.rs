use crate::labels::states::AppState;
use bevy::{asset::LoadState, prelude::*, utils::dbg};

pub mod asset_group;
pub use asset_group::AssetGroup;

pub struct AssetLoadingPlugin;

impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::AssetLoading), load_assets_system)
            .add_systems(
                Update,
                poll_handle_state.run_if(resource_exists::<LoadingAssets>),
            )
            .init_resource::<SpriteAssets>()
            .init_resource::<SpriteSheetLayouts>()
            .init_resource::<GameIcons>();
    }
}

#[derive(Resource, Clone, Debug, Deref, DerefMut, Default)]
pub struct LoadingAssets(Vec<UntypedHandle>);

fn poll_handle_state(
    mut commands: Commands,
    mut state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
    handles: Res<LoadingAssets>,
) {
    let all_loaded = handles
        .iter()
        .flat_map(|h| asset_server.get_load_state(h))
        .all(|s| s.is_loaded());
    if all_loaded {
        state.set(AppState::SpawnPlayer);
        commands.remove_resource::<LoadingAssets>();
    }
}

fn load_assets_system(mut commands: Commands, server: Res<AssetServer>) {
    let game_icons = GameIcons {
        equip_icon: server.load("icons/equip_marker.png"),
        potion_icon: server.load("icons/potion.png"),
        spell_book_icon: server.load("icons/spell-book.png"),
        melee_icon: server.load("icons/sword-brandish.png"),
        staff_icon: server.load("icons/wizard-staff.png"),
    };
    let mut ids: LoadingAssets = LoadingAssets::default();
    ids.push(game_icons.equip_icon.clone().untyped());
    ids.push(game_icons.potion_icon.clone().untyped());
    ids.push(game_icons.spell_book_icon.clone().untyped());
    ids.push(game_icons.melee_icon.clone().untyped());
    ids.push(game_icons.staff_icon.clone().untyped());
    commands.insert_resource(game_icons);

    let sprite_assets = SpriteAssets {
        gold_coin: server.load("coin.png"),
        tome_of_healing: server.load("items/tome_of_healing.png"),
        sword: server.load("items/sword.png"),
        axe: server.load("items/axe.png"),
        fire_staff: server.load("items/fire_staff.png"),
        ice_staff: server.load("items/ice_staff.png"),
        health_potion: server.load("items/health_potion.png"),
        ice_bolt: server.load("projectiles/IceBolt.png"),
        fire_ball: server.load("projectiles/fireball.png"),
        exit_door: server.load("door.png"),
        ground_tiles: server.load("tilesets/ground_tiles.png"),
        grass_tiles: server.load("tilesets/grass_tiles.png"),
        water_tiles: server.load("tilesets/water_tiles.png"),
        wall_tiles: server.load("tilesets/wall_tiles.png"),
        wood_tiles: server.load("tilesets/wood_tiles.png"),
        cobblestone_tiles: server.load("tilesets/cobblestone_tiles.png"),
        run_start_door: server.load("door.png"),
        chests_sprite_sheet: server.load("chests.png"),
        tome_of_healing_effect_sprite_sheet: server.load("spells/tome_of_healing_effect.png"),
        player_sprite_sheet: server.load("player/player_sprite_sheet.png"),
        ice_mage_enemy_sprite_sheet: server.load("enemies/ice_mage_enemy.png"),
        warrior_enemy_sprite_sheet: server.load("enemies/warrior_enemy.png"),
        fire_mage_enemy_sprite_sheet: server.load("enemies/fire_mage_enemy.png"),
        shop_keeper_sprite_sheet: server.load("npcs/shop_keeper.png"),
        game_guide_sprite_sheet: server.load("npcs/game_guide.png"),
        stat_trainer_sprite_sheet: server.load("npcs/stat_trainer.png"),
    };
    ids.push(sprite_assets.gold_coin.clone().untyped());
    ids.push(sprite_assets.tome_of_healing.clone().untyped());
    ids.push(sprite_assets.sword.clone().untyped());
    ids.push(sprite_assets.axe.clone().untyped());
    ids.push(sprite_assets.fire_staff.clone().untyped());
    ids.push(sprite_assets.ice_staff.clone().untyped());
    ids.push(sprite_assets.health_potion.clone().untyped());
    ids.push(sprite_assets.ice_bolt.clone().untyped());
    ids.push(sprite_assets.fire_ball.clone().untyped());
    ids.push(sprite_assets.exit_door.clone().untyped());
    ids.push(sprite_assets.ground_tiles.clone().untyped());
    ids.push(sprite_assets.grass_tiles.clone().untyped());
    ids.push(sprite_assets.water_tiles.clone().untyped());
    ids.push(sprite_assets.wall_tiles.clone().untyped());
    ids.push(sprite_assets.wood_tiles.clone().untyped());
    ids.push(sprite_assets.cobblestone_tiles.clone().untyped());
    ids.push(sprite_assets.run_start_door.clone().untyped());
    ids.push(sprite_assets.chests_sprite_sheet.clone().untyped());
    ids.push(
        sprite_assets
            .tome_of_healing_effect_sprite_sheet
            .clone()
            .untyped(),
    );
    ids.push(sprite_assets.player_sprite_sheet.clone().untyped());
    ids.push(sprite_assets.ice_mage_enemy_sprite_sheet.clone().untyped());
    ids.push(sprite_assets.warrior_enemy_sprite_sheet.clone().untyped());
    ids.push(sprite_assets.fire_mage_enemy_sprite_sheet.clone().untyped());
    ids.push(sprite_assets.shop_keeper_sprite_sheet.clone().untyped());
    ids.push(sprite_assets.game_guide_sprite_sheet.clone().untyped());
    ids.push(sprite_assets.stat_trainer_sprite_sheet.clone().untyped());
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
    commands.insert_resource(ids);
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
