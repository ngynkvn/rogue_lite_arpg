extern crate baba_yaga_derive;
use baba_yaga::configuration::assets::AssetGroup;
use baba_yaga_derive::AssetGroup;
use bevy::{asset::Handle, image::Image, prelude::Resource};

#[derive(Resource, AssetGroup)]
pub struct GameIcons {
    #[asset(src = "icons/equip_marker.png")]
    equip_icon: Handle<Image>,
    #[asset(src = "icons/potion.png")]
    potion_icon: Handle<Image>,
    #[asset(src = "icons/spell-book.png")]
    spell_book_icon: Handle<Image>,
    #[asset(src = "icons/sword-brandish.png")]
    melee_icon: Handle<Image>,
    #[asset(src = "icons/wizard-staff.png")]
    staff_icon: Handle<Image>,
}

fn main() {
    GameIcons::hello_macro();
    println!("I made a thing: {:?}", 1);
}
