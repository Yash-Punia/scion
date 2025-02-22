mod animations;

use std::collections::{HashMap, HashSet};
use rand::rngs::ThreadRng;
use rand::{Rng, thread_rng};
use scion::core::world::{GameData, World};
use scion::{
    config::{scion_config::ScionConfigBuilder, window_config::WindowConfigBuilder},
    core::{
        components::{
            maths::{coordinates::Coordinates, transform::Transform},
            tiles::{sprite::Sprite, tileset::Tileset},
        },
        scene::Scene,
    },
    utils::file::app_base_path,
    Scion,
};
use scion::core::components::animations::Animations;
use crate::animations::get_case_animation;

#[derive(Debug)]
struct Case(Coordinates);

pub(crate) enum MoveDirection {
    Left,
    Top,
    Right,
    Bottom,
    None,
}

struct Taquin {
    board: [[bool; 4]; 4],
}

impl Taquin {
    fn new(cases: &HashMap<usize, Option<(usize)>>) -> Self {
        let mut board = [[true; 4]; 4];
        for line in 0..4 {
            for column in 0..4 {
                if cases.get(&(line * 4 + column)).unwrap().is_none() {
                    board[column][line] = false;
                }
            }
        }

        Self { board }
    }

    fn try_move(&mut self, column: usize, line: usize) -> MoveDirection {
        self.board[column][line] = false;
        if column > 0 && !self.board[column - 1][line] {
            self.board[column - 1][line] = true;
            MoveDirection::Left
        } else if line > 0 && !self.board[column][line - 1] {
            self.board[column][line - 1] = true;
            MoveDirection::Top
        } else if column < 3 && !self.board[column + 1][line] {
            self.board[column + 1][line] = true;
            MoveDirection::Right
        } else if line < 3 && !self.board[column][line + 1] {
            self.board[column][line + 1] = true;
            MoveDirection::Bottom
        } else {
            self.board[column][line] = true;
            MoveDirection::None
        }
    }
}

fn taquin_system(data: &mut GameData) {
    let (world, resources) = data.split();
    let inputs = resources.inputs();
    let mut taquin = resources.get_resource_mut::<Taquin>().unwrap();

    let mut animation_running = false;
    for (_e, animations) in world.query_mut::<&mut Animations>() {
        if animations.any_animation_running() {
            animation_running = true;
        }
    }

    if animation_running {
        return;
    }

    for (_, (case, animations)) in world.query_mut::<(&mut Case, &mut Animations)>() {
        inputs.on_left_click_pressed(|mouse_x, mouse_y| {
            if mouse_x > (case.0.x() * 192.) as f64
                && mouse_y > (case.0.y() * 192.) as f64
                && mouse_x < (case.0.x() * 192. + 192.) as f64
                && mouse_y < (case.0.y() * 192. + 192.) as f64
            {
                match taquin.try_move(case.0.x() as usize, case.0.y() as usize) {
                    MoveDirection::Left => {
                        case.0.set_x(case.0.x() - 1.);
                        animations.run_animation("LEFT");
                    }
                    MoveDirection::Top => {
                        case.0.set_y(case.0.y() - 1.);
                        animations.run_animation("TOP");
                    }
                    MoveDirection::Right => {
                        case.0.set_x(case.0.x() + 1.);
                        animations.run_animation("RIGHT");
                    }
                    MoveDirection::Bottom => {
                        case.0.set_y(case.0.y() + 1.);
                        animations.run_animation("BOTTOM");
                    }
                    MoveDirection::None => {}
                };
            }
        })
    }
}

#[derive(Default)]
struct MainScene;

impl Scene for MainScene {
    fn on_start(&mut self, data: &mut GameData) {
        let tileset_ref = data.assets_mut().register_tileset(Tileset::new(
            app_base_path().join("examples/taquin/assets/taquin.png").get(),
            4,
            4,
            192,
        ));

        let cases = compute_mixed_cases();

        for line in 0..4 {
            for column in 0..4 {
                let case = cases.get(&(line * 4 + column)).expect("Expect all the case to be in the map");
                if case.is_some() {
                    let square = (
                        Transform::from_xy(column as f32 * 192., line as f32 * 192.),
                        tileset_ref.clone(),
                        Sprite::new(case.unwrap()),
                        Case(Coordinates::new(column as f32, line as f32)),
                        Animations::new(get_case_animation())
                    );
                    data.push(square);
                }
            }
        }
        data.add_default_camera();

        data.insert_resource(Taquin::new(&cases));
    }
}

fn compute_mixed_cases() -> HashMap<usize, Option<(usize)>> {
    let mut cases = HashMap::new();
    // Creating the default board
    for line in 0..4 {
        for column in 0..4 {
            if !(line == 3 && column == 3) {
                cases.insert(line * 4 + column, Some((line * 4 + column)));
            } else {
                cases.insert(line * 4 + column, None);
            }
        }
    }

    // Mixing it with a classic random shuffle algorithm
    let mut rand = ThreadRng::default();
    for _i in 0..300 {
        let a = rand.gen_range(0..cases.len());
        let b = rand.gen_range(0..cases.len());

        let tmp_a = cases.get(&a).unwrap().clone();
        let tmp_b = cases.get(&b).unwrap().clone();

        cases.insert(a, tmp_b);
        cases.insert(b, tmp_a);
    }
    cases
}

fn main() {
    Scion::app_with_config(
        ScionConfigBuilder::new()
            .with_window_config(
                WindowConfigBuilder::new().with_resizable(true).with_dimensions((768, 768)).get(),
            )
            .get(),
    )
        .with_system(taquin_system)
        .with_scene::<MainScene>()
        .run();
}
