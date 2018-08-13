use amethyst::utils::scene::BasicScenePrefab;
use amethyst::renderer::PosNormTex;
use amethyst::animation::AnimationSetPrefab;
use amethyst::core::Transform;
use std::vec::Vec;

pub type myPrefabData = Option<BasicScenePrefab<Vec<PosNormTex>>>;