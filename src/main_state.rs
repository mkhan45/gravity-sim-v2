use legion::prelude::*;

use ggez::{
    event::EventHandler,
    graphics,
    graphics::{Color, DrawMode, DrawParam},
    input,
    input::{
        keyboard::{KeyCode, KeyMods},
        mouse::MouseButton,
    },
    Context, GameResult,
};

use crate::physics::{apply_gravity, calc_collisions, integrate_kinematics, integrate_positions};
use crate::{Draw, Position, Radius};

pub const DT: f32 = 1.0;

pub struct MainState {
    universe: Universe,
    main_world: World,
}

impl MainState {
    pub fn new(universe: Universe, main_world: World) -> Self {
        MainState {
            universe,
            main_world,
        }
    }
}

impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        for _ in 0..(1.0 / DT) as usize {
            calc_collisions(&mut self.main_world);
            integrate_positions(&self.main_world);
            apply_gravity(&self.main_world);
            integrate_kinematics(&self.main_world);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::new(0.0, 0.0, 0.0, 1.0));
        let draw_query = <(Read<Draw>, Read<Position>, Read<Radius>)>::query();
        draw_query
            .iter(&self.main_world)
            .for_each(|(color, pos, rad)| {
                let point: ggez::mint::Point2<f32> = (*pos).into();
                let circle = ggez::graphics::Mesh::new_circle(
                    ctx,
                    DrawMode::fill(),
                    point,
                    rad.0,
                    0.05,
                    color.0,
                )
                .expect("error building mesh");
                ggez::graphics::draw(ctx, &circle, graphics::DrawParam::new())
                    .expect("error drawing mesh");
            });
        ggez::graphics::present(ctx)
    }
}
