use ggez::event::{self,EventHandler};
use ggez::graphics::{self,Color, DrawParam};
use ggez::{Context,ContextBuilder,GameResult};
use ggez::glam::Vec2;
use std::env;
use std::path::{PathBuf, Path};

use crate::util;
    struct MainState {
        frames:usize,
        image_blue_block: graphics::Image,

    }

    impl MainState {
        fn new(ctx:&mut Context) -> GameResult<MainState> {
            ctx.gfx.add_font(
                "LiberationMono",
                graphics::FontData::from_path(ctx,"/assets/font/LiberationMono-Regular.ttf")?,
            );

            let image_blue_block = graphics::Image::from_path(ctx,"/assets/pic/blue_block.png")?;
            
            let s = MainState {
                frames: 0,
                image_blue_block,
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
            
            let offset = self.frames as f32/10.0;
            let dest_point = Vec2::new(offset,offset);
            

            canvas.draw(&self.image_blue_block, DrawParam::new().dest(dest_point));

            canvas.draw(
                graphics::Text::new("Hello world!")
                    .set_font("LiberationMono")
                    .set_scale(48.),
                dest_point,
            );
            canvas.finish(ctx)?;

            self.frames += 1;
            if (self.frames % 100) == 0 {
                println!("FPS: {}",ctx.time.fps());
            }
            Ok(())
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
