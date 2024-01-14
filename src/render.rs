use ggez::event::{self,EventHandler};
use ggez::graphics::{self,Color, DrawParam};
use ggez::{Context,ContextBuilder,GameResult};
use ggez::glam::Vec2;
    pub struct Renderer {
        //TODO:定义渲染器的属性和方法
        blue_block:graphics::Image,
        // green_block:Image,
        // purple_block:Image,
        // red_block:Image,
        // yellow_block:Image,
    }

    impl Renderer {
        //TODO:实现渲染方法
        pub fn init(ctx: &mut Context) ->GameResult<Renderer> {
            let blue_block = graphics::Image::from_path(ctx,"/home/aspasia/aspasia/RustDemo/Tetris_rust/assets/blue_block.png")?;

            Ok(Renderer {blue_block})
        }
    }

    impl event::EventHandler for Renderer {
        //todo:更新逻辑
        fn update(&mut self, _:&mut Context) -> GameResult<()> {
            Ok(())
        }

        fn draw(&mut self, ctx: &mut Context) -> GameResult {
            let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
            let dest_point = Vec2::new(100.0, 100.0);
            
            canvas.draw(&self.blue_block, DrawParam::new().dest(dest_point));

            canvas.finish(ctx)?;
            Ok(())
            
        }

    }

    pub fn run() -> GameResult<()> {
        let (mut ctx, event_loop) = ggez::ContextBuilder::new("Tetris", "aspasia")
            .window_setup(ggez::conf::WindowSetup::default().title("Tetris Window"))
            .window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0))
            //.audio_backend(ggez::conf::AudioBackEnd::Null)
            .build()
            .expect("could not create ggez centext!");

        let game = Renderer::init(&mut ctx).expect("renderer init failed!");

        event::run(ctx, event_loop, game)
    }
