
//方块的生成与组合逻辑
use rand::seq::SliceRandom;
use rand::Rng;
use ggez::graphics::{self,DrawParam,Canvas};
use ggez::{Context,GameError};
use ggez::glam::Vec2;

use crate::util::{self, GridPosition};


pub struct Block {
    image_blue_block: graphics::Image,
    image_green_block: graphics::Image,
    image_purple_block: graphics::Image,
    image_red_block: graphics::Image,
    image_yellow_block: graphics::Image,
    position: GridPosition,
}

impl Block {
    pub fn new(ctx: &mut Context) -> Result<Self, GameError>{
        let blue = graphics::Image::from_path(ctx,"/assets/pic/blue_block.png")?;
        let green = graphics::Image::from_path(ctx, "/assets/pic/green_block.png")?;
        let purple = graphics::Image::from_path(ctx, "/assets/pic/purple_block.png")?;
        let red = graphics::Image::from_path(ctx, "/assets/pic/red_block.png")?;
        let yellow = graphics::Image::from_path(ctx, "/assets/pic/yellow_block.png")?;

        let position = GridPosition::new(0, 0);
        Ok(Self{image_blue_block: blue,
            image_green_block: green,
            image_purple_block: purple,
            image_red_block: red,
            image_yellow_block: yellow,
            position: position,
        })
    }

    pub fn set_block_position(&mut self, pos:(i32,i32)) {
        self.position.set_grid_position(pos);
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

    pub fn move_to_left(&mut self) {
        let grid_pos = self.position.get_grid_position();
        if grid_pos.0 <= 0 {
            return ;
        }
        self.position.move_to_left();
    }

    pub fn move_to_right(&mut self) {
        let grid_pos = self.position.get_grid_position();
        if grid_pos.0 >= 10 {
            return ;
        }
        self.position.move_to_right();
    }

    pub fn move_to_top(&mut self) {
        let grid_pos = self.position.get_grid_position();
        if grid_pos.1 <= 0 {
            return ;
        }
        self.position.move_to_top();
    }

    pub fn move_to_bottom(&mut self) {
        let grid_pos = self.position.get_grid_position();
        if grid_pos.1 >= 20 {
            return ;
        }
        self.position.move_to_bottom();
    }

    pub fn draw(&mut self, canvas: &mut Canvas, pic: &graphics::Image) {
        canvas.draw(pic, DrawParam::new()
        .dest(self.position.get_actual_position())
        .scale(util::PIC_SCALE_NUMBER));
    }

}


pub struct BlockGroup {
    block1: Block,
    block2: Block,
    block3: Block,
    block4: Block,
    image: graphics::Image,
    position: GridPosition, //绝对坐标，所有里面的block会根据这个坐标进行偏移
}

const BLOCK_SHAPE: [[(i32,i32);4];4] = [
    [ (0,0), (0,1), (1,0), (1,1)], //rect
    [ (0,0), (0,1), (0,2), (0,3)], //bar
    [ (0,0), (0,1), (1,1), (1,2)], //zigzag
    [ (0,1), (1,0), (1,1), (1,2)], //T 
];

impl BlockGroup {
    pub fn random_group_generation(ctx: &mut Context) -> Self{
        //init block
        let mut block1 = Block::new(ctx).unwrap();
        let mut block2 = Block::new(ctx).unwrap();
        let mut block3 = Block::new(ctx).unwrap();
        let mut block4 = Block::new(ctx).unwrap();
        
        //init blockgroup position
        let blockgroup_position = GridPosition::new(0, 0);

        //random type
        let rand_number = rand::thread_rng().gen_range(0..4);
        println!("rand number: {rand_number}");
        
        let block_type = BLOCK_SHAPE[rand_number];
    
        block1.set_block_position(blockgroup_position.add(block_type[0]));
        block2.set_block_position(blockgroup_position.add(block_type[1]));
        block3.set_block_position(blockgroup_position.add(block_type[2]));
        block4.set_block_position(blockgroup_position.add(block_type[3]));

        //random image
        let image = block1.get_rand_pic().clone();

        BlockGroup {
            block1: block1,
            block2: block2,
            block3: block3,
            block4: block4,
            image: image,
            position: blockgroup_position,
        }

    }

    pub fn draw(&mut self, canvas: &mut Canvas) {
        self.block1.draw(canvas, &self.image);
        self.block2.draw(canvas, &self.image);
        self.block3.draw(canvas, &self.image);
        self.block4.draw(canvas, &self.image);
    }

    //actually no error detected :)
    pub fn move_to_left(&mut self) -> Result<(), GameError>{
        //TODO:边界检测
        self.block1.move_to_left();
        self.block2.move_to_left();
        self.block3.move_to_left();
        self.block4.move_to_left();
        Ok(())
    }

    pub fn move_to_right(&mut self) -> Result<(), GameError>{
        //TODO:边界检测
        self.block1.move_to_right();
        self.block2.move_to_right();
        self.block3.move_to_right();
        self.block4.move_to_right();
        Ok(())
    }

    pub fn move_to_bottom(&mut self) -> Result<(), GameError>{
        //Todo:碰撞检测
        self.block1.move_to_bottom();
        self.block2.move_to_bottom();
        self.block3.move_to_bottom();
        self.block4.move_to_bottom();
        Ok(())
    }

    pub fn move_to_top(&mut self) -> Result<(), GameError>{
        //Todo:碰撞检测
        self.block1.move_to_top();
        self.block2.move_to_top();
        self.block3.move_to_top();
        self.block4.move_to_top();
        Ok(())
    }
    
}