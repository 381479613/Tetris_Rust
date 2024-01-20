use ggez::event;
use ggez::graphics::{self,DrawParam};
use ggez::{Context,GameResult};
use ggez::glam::Vec2;
use ggez::input::keyboard::{KeyCode,KeyInput};
use std::env;
use std::path::PathBuf;

use crate::util;
use crate::block::Block;
    struct MainState {
        frames:usize,
        block:Block,
    }

    impl MainState {
        fn new(ctx:&mut Context) -> GameResult<MainState> {
            ctx.gfx.add_font(
                "LiberationMono",
                graphics::FontData::from_path(ctx,"/assets/font/LiberationMono-Regular.ttf")?,
            );
            let block = Block::new(ctx)?;

            let s = MainState {
                frames: 0,
                block:block,
                };
            Ok(s)
        }
    }

    impl event::EventHandler<ggez::GameError> for MainState {
        fn update(&mut self, _ctx: &mut Context) -> GameResult {
            Ok(())
        }

        fn draw(&mut self, ctx:&mut Context) -> GameResult {
            let mut canvas = 
                graphics::Canvas::from_frame(ctx,graphics::Color::from([0.1,0.2,0.3,1.0]));

            let _ = &self.block.draw(&mut canvas);

            canvas.draw(
                graphics::Text::new("Welcome")
                    .set_font("LiberationMono")
                    .set_scale(48.),
                    util::SCORE_WORD_START_POSITION,
            );
            canvas.finish(ctx)?;

            self.frames += 1;
            if (self.frames % 100) == 0 {
                println!("FPS: {}",ctx.time.fps());
            }
            Ok(())
        }

        fn key_down_event(
                &mut self,
                ctx: &mut Context,
                input: KeyInput,
                _repeated: bool,
            ) -> Result<(), ggez::GameError> {
                match input.keycode {
                    Some(KeyCode::Up) => self.block.move_to_top(),
                    Some(KeyCode::Left) => self.block.move_to_left(),
                    Some(KeyCode::Right) => self.block.move_to_right(),
                    Some(KeyCode::Down) => self.block.move_to_bottom(),
                    _ => Ok(()),
                }
        }
    }

    pub fn run() -> GameResult<()> {
        let resource_path = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
            PathBuf::from(manifest_dir)
        } else {
            PathBuf::from("./")
        };
        let (mut ctx, event_loop) = ggez::ContextBuilder::new("Tetris", "aspasia")
            .add_resource_path(resource_path)
            .window_setup(ggez::conf::WindowSetup::default().title("Tetris Window"))
            .window_mode(ggez::conf::WindowMode::default().dimensions(util::SCREEN_SIZE.0,util::SCREEN_SIZE.1))
            .build()
            .expect("could not create ggez centext!");

        let state = MainState::new(&mut ctx)?;

        event::run(ctx, event_loop, state)
    }
