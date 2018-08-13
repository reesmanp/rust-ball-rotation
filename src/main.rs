extern crate amethyst;
#[macro_use]
extern crate serde;

mod ball;

use amethyst::prelude::*;
use amethyst::animation::AnimationBundle;
use amethyst::renderer::{PosNormTex, RenderBundle, DrawShaded};
use amethyst::assets::PrefabLoaderSystem;
use amethyst::core::transform::{TransformBundle, Transform};
use amethyst::controls::FreeRotationSystem;
use amethyst::input::{InputBundle, InputHandler};
use ball::prefab::myPrefabData;
use ball::Ball;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let display_config_path = "./config/display_config.ron";
    let resources = format!("{}/src/", env!("CARGO_MANIFEST_DIR"));

    let game_data = GameDataBuilder::default()
        .with(PrefabLoaderSystem::<myPrefabData>::default(), "ball_prefab_loader", &[])
        //.with_bundle(InputBundle::<String, String>::new())?
        .with_bundle(TransformBundle::new())?
        .with_basic_renderer(display_config_path, DrawShaded::<PosNormTex>::new(), false)?;

    let mut game = Application::new(resources, Ball::default(), game_data)?;

    game.run();
    Ok(())
}
