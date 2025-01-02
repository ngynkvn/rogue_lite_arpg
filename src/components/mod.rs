// src/components/mod.rs

pub mod animation_indices;
pub mod burning_effect;
pub mod damage_effect;
pub mod effects;
pub mod enemy;
pub mod fireball;
pub mod freezing_effect;
pub mod health;
pub mod health_bar;
pub mod icebolt;
pub mod player; // Declare the enemy module
pub mod projectile;
pub mod speed;
pub mod warp_zone;

pub use burning_effect::BurningEffect;
pub use damage_effect::DamageEffect;
pub use effects::*;
pub use health::Health;
pub use health_bar::HealthBar;
pub use projectile::Projectile;
pub use speed::Speed;
pub use warp_zone::Level;
pub use warp_zone::WarpZone;
// Re-export the components so they can be easily accessed
pub use animation_indices::*;
pub use enemy::*;
pub use fireball::*;
pub use freezing_effect::*;
pub use icebolt::*;
pub use player::*;
