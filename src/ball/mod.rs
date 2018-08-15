pub mod prefab;

use amethyst::prelude::*;
use amethyst::renderer::{Event, VirtualKeyCode, WindowEvent, MouseButton, ElementState};
use amethyst::input::{is_key_down, is_close_requested, get_key};
use amethyst::assets::{PrefabLoader, RonFormat};
use amethyst::ecs::prelude::Entity;
use amethyst::core::Transform;
use amethyst::core::cgmath::{Vector3, Quaternion, Deg, Rotation, MetricSpace, Rotation3, InnerSpace};
use myPrefabData;

pub struct Ball {
    is_rotating: bool,
    pub sphere: Option<Entity>,
    previous_location: Option<Vector3<f32>>,
    rotation: [f32; 4]
}

impl Default for Ball {
    fn default() -> Self {
        Ball {
            is_rotating: false,
            sphere: None,
            previous_location: None,
            rotation: [0.0, 0.0, 0.0, 0.0]
        }
    }
}

impl<'a, 'b> State<GameData<'a, 'b>> for Ball {
    fn on_start(&mut self, data: StateData<GameData>) {
        let StateData { world, .. } = data;

        // Initialize the scene with an object, a light, and a camera
        let prefab = world.exec(|loader: PrefabLoader<myPrefabData>| {
            loader.load("ball/prefab/sphere.ron", RonFormat, (), ())
        });

        self.sphere = Some(world.create_entity().with(prefab).build());
    }

    fn handle_event(&mut self, data: StateData<GameData>, event: Event) -> Trans<GameData<'a, 'b>> {
        let StateData { world, .. } = data;
        if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
            return Trans::Quit;
        }

        let mouse_down = left_mouse_state_change(&event);
        match mouse_down {
            Some(ElementState::Pressed) => {
                self.is_rotating = true
            },
            Some(ElementState::Released) => {
                self.is_rotating = false
            },
            _ => {}
        };

        if self.is_rotating {
            let new_pos = get_new_cursor_position(&event);
            let mut prev_pos = Vector3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            };

            let maybe_rotate = match new_pos {
                Some(pos) => {
                    prev_pos = match self.previous_location {
                        Some(T) => T,
                        None => prev_pos
                    };
                    let dir_vec = create_directional_vector(pos, prev_pos);
                    Some(Vector3 { x: dir_vec.y, y: -1.0 * dir_vec.x, z: dir_vec.z })
                },
                None => None
            };

            match maybe_rotate {
                Some(T) => {
                    let rel_axis_normalized = Quaternion::from(self.rotation).rotate_vector(T).normalize();
                    let distance = Quaternion::from_sv(1.0, prev_pos)
                        .distance(Quaternion::from_sv(1.0, new_pos.unwrap()));
                    let new_rot = Quaternion::from_axis_angle(
                        rel_axis_normalized,
                        Deg(distance * 1.0)
                    );

                    world.write_storage::<Transform>().get_mut(self.sphere.unwrap()).unwrap()
                        .rotation = new_rot;
                    self.rotation = new_rot.into();
                    self.previous_location = new_pos;
                },
                None => {}
            };

        }

        Trans::None
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(&data.world);
        Trans::None
    }
}

fn left_mouse_state_change(event: &Event) -> Option<ElementState> {
    match *event {
        Event::WindowEvent { ref event, .. } => match *event {
            WindowEvent::MouseInput {
                state,
                button: MouseButton::Left,
                ..
            } => Some(state),
            _ => None
        },
        _ => None
    }
}

fn get_new_cursor_position(event: &Event) -> Option<Vector3<f32>> {
    match *event {
        Event::WindowEvent { ref event, .. } => match *event {
            WindowEvent::CursorMoved {
                position,
                ..
            } => Some(Vector3{ x: position.0 as f32, y: 0.0 as f32, z: position.1 as f32 }),
            _ => None
        },
        _ => None
    }
}

fn create_directional_vector(vector1: Vector3<f32>, vector2: Vector3<f32>) -> Vector3<f32> {
    Vector3 {
        x: vector1[0] - vector2[0],
        y: vector1[1] - vector2[1],
        z: vector1[2] - vector2[2]
    }
}
