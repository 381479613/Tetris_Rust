//方块的生成与组合逻辑
use rand::seq::SliceRandom;
use ggez::{graphics, Context, GameError};

pub struct Block {
    image_blue_block: graphics::Image,
    image_green_block: graphics::Image,
    image_purple_block: graphics::Image,
    image_red_block: graphics::Image,
    image_yellow_block: graphics::Image,
}

impl Block {
    pub fn new(ctx: &mut Context) -> Result<Self, GameError>{
        let blue = graphics::Image::from_path(ctx,"/assets/pic/blue_block.png")?;
        let green = graphics::Image::from_path(ctx, "/assets/pic/green_block.png")?;
        let purple = graphics::Image::from_path(ctx, "/assets/pic/purple_block.png")?;
        let red = graphics::Image::from_path(ctx, "/assets/pic/red_block.png")?;
        let yellow = graphics::Image::from_path(ctx, "/assets/pic/yellow_block.png")?;

        Ok(Self{image_blue_block: blue,
            image_green_block: green,
            image_purple_block: purple,
            image_red_block: red,
            image_yellow_block: yellow,
        })
    }

    pub fn get_rand_pic(&self) -> &graphics::Image {
        let mut rng = rand::thread_rng();
        let images = [
            &self.image_blue_block,
            &self.image_green_block,
            &self.image_purple_block,
            &self.image_red_block,
            &self.image_yellow_block,
        ];
        let random_image = *images.choose(&mut rng).unwrap();
        random_image
    }



}
