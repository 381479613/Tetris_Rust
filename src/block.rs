//方块的生成与组合逻辑
use rand::seq::SliceRandom;
use rand::Rng;
use ggez::graphics::{self,DrawParam,Canvas};
use ggez::{Context,GameError};

use crate::util::{self, GridPosition};

const DEFAULT_POSITION: (i32,i32) = (0,0);

#[derive(Clone)]
pub struct Block {
    image: graphics::Image,
    position: GridPosition,
}

impl Block {
    pub fn new(ctx: &mut Context) -> Self{
        let image = Block::get_rand_pic(ctx);
        let position = GridPosition::new(DEFAULT_POSITION.0,DEFAULT_POSITION.1);

        Self{
            image: image,
            position: position,
        }
    }

    pub fn set_block_position(&mut self, pos:(i32,i32)) {
        self.position.set_grid_position(pos);
    }

    pub fn get_rand_pic(ctx: &mut Context) -> graphics::Image {
        let mut rng = rand::thread_rng();
        let random_num = rng.gen_range(1..=5);
        let random_image = {
        match random_num {
            1 => {
                graphics::Image::from_path(ctx,"/assets/pic/blue_block.png")
            }
            2 => {
                graphics::Image::from_path(ctx, "/assets/pic/green_block.png")
            }
            3 => {
                graphics::Image::from_path(ctx, "/assets/pic/purple_block.png")
            }
            4 => {
                graphics::Image::from_path(ctx, "/assets/pic/red_block.png")
            }
            5 => {
                graphics::Image::from_path(ctx, "/assets/pic/yellow_block.png")
            }
            _ => {Err(GameError::GraphicsInitializationError)}
        }
        }.unwrap();
        random_image
    }

    pub fn set_pic(&mut self, pic: &graphics::Image) {
        self.image = pic.clone();
    }

    pub fn get_pic(&self) -> graphics::Image {
        self.image.clone()
    }

    pub fn boundary_check(&self) -> bool {
        let result = self.position.get_grid_position();
        if result.0 <= 0 || result.0 >= util::GRID_SIZE.0 || result.1 <= 0 || result.1 >= util::GRID_SIZE.1 {
            return false;
        }
        return true;
    }

    pub fn move_to_left(&mut self) {
        let grid_pos = self.position.get_grid_position();
        if grid_pos.0 <= 0 {
            return ;
        }
        self.position.move_to_left();
    }

    pub fn can_move_to_left(&self) -> bool{
        let grid_pos = self.position.get_grid_position();
        if grid_pos.0 <= 0 {
            false
        } else {
            true
        }
    }

    pub fn move_to_right(&mut self) {
        let grid_pos = self.position.get_grid_position();
        if grid_pos.0 >= util::GRID_SIZE.0 {
            return ;
        }
        self.position.move_to_right();
    }

    pub fn can_move_to_right(&self) -> bool{
        let grid_pos = self.position.get_grid_position();
        if grid_pos.0 >= util::GRID_SIZE.0 {
            false
        } else {
            true
        }
    }

    pub fn move_to_top(&mut self) {
        let grid_pos = self.position.get_grid_position();
        if grid_pos.1 <= 0 {
            return ;
        }
        self.position.move_to_top();
    }
    pub fn can_move_to_top(&self) -> bool{
        let grid_pos = self.position.get_grid_position();
        if grid_pos.1 <= 0 {
            false
        } else {
            true
        }
    }

    pub fn move_to_bottom(&mut self) {
        let grid_pos = self.position.get_grid_position();
        if grid_pos.1 >= 20 {
            return ;
        }
        self.position.move_to_bottom();
    }

    pub fn can_move_to_bottom(&self) -> bool{
        let grid_pos = self.position.get_grid_position();
        if grid_pos.1 >= util::GRID_SIZE.1 {
            false
        } else {
            true
        }
    }

    pub fn check_collision_down(&self, static_block: &StaticBlockGroup) -> bool {
        let grid_pos = self.position.get_grid_position();
        let try_pos = (grid_pos.0, grid_pos.1 + 1);

        for block in static_block.get_block_vec() {
            if try_pos == block.position.get_grid_position() {
                return true;
            }
        }
        return false;
    }

    pub fn draw(&mut self, canvas: &mut Canvas) {
        canvas.draw(&self.image, DrawParam::new()
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
        let mut block1 = Block::new(ctx);
        let mut block2 = Block::new(ctx);
        let mut block3 = Block::new(ctx);
        let mut block4 = Block::new(ctx);
        
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
        let image = block1.get_pic();
        block1.set_pic(&image);
        block2.set_pic(&image);
        block3.set_pic(&image);
        block4.set_pic(&image);

        BlockGroup {
            block1: block1,
            block2: block2,
            block3: block3,
            block4: block4,
            image: image,
            position: blockgroup_position,
        }

    }

    pub fn collision_detection(&self, static_block: &StaticBlockGroup) -> bool {
        //if blockgroup cant fall, collsion occurred
        if !self.can_move_to_bottom() {
            return true;
        }
        if !self.can_fell(static_block) {
            return true;
        }
        return false;
    }

    pub fn draw(&mut self, canvas: &mut Canvas) {
        self.block1.draw(canvas);
        self.block2.draw(canvas);
        self.block3.draw(canvas);
        self.block4.draw(canvas);
    }
    pub fn can_move_to_left(&self) -> bool {
        if !self.block1.can_move_to_left() {
            return false;
        }
        if !self.block2.can_move_to_left() {
            return false;
        }
        if !self.block3.can_move_to_left() {
            return false;
        }
        if !self.block4.can_move_to_left() {
            return false;
        }
        return true;
    }

    //actually no error detected :)
    pub fn move_to_left(&mut self) -> Result<(), GameError>{
        if self.can_move_to_left() == false {
            return Ok(())
        }
        self.block1.move_to_left();
        self.block2.move_to_left();
        self.block3.move_to_left();
        self.block4.move_to_left();
        Ok(())
    }

    pub fn can_move_to_right(&self) -> bool {
        if !self.block1.can_move_to_right() {
            return false;
        }
        if !self.block2.can_move_to_right() {
            return false;
        }
        if !self.block3.can_move_to_right() {
            return false;
        }
        if !self.block4.can_move_to_right() {
            return false;
        }
        return true;
    }

    pub fn move_to_right(&mut self) -> Result<(), GameError>{
        if self.can_move_to_right() == false {
            return Ok(())
        }
        self.block1.move_to_right();
        self.block2.move_to_right();
        self.block3.move_to_right();
        self.block4.move_to_right();
        Ok(())
    }

    pub fn can_move_to_bottom(&self) -> bool {
        if !self.block1.can_move_to_bottom() {
            return false;
        }
        if !self.block2.can_move_to_bottom() {
            return false;
        }
        if !self.block3.can_move_to_bottom() {
            return false;
        }
        if !self.block4.can_move_to_bottom() {
            return false;
        }
        return true;
    }

    pub fn move_to_bottom(&mut self) -> Result<(), GameError>{
        if self.can_move_to_bottom() == false {
            return Ok(())
        }
        self.block1.move_to_bottom();
        self.block2.move_to_bottom();
        self.block3.move_to_bottom();
        self.block4.move_to_bottom();
        Ok(())
    }

    pub fn can_move_to_top(&self) -> bool {
        if !self.block1.can_move_to_top() {
            return false;
        }
        if !self.block2.can_move_to_top() {
            return false;
        }
        if !self.block3.can_move_to_top() {
            return false;
        }
        if !self.block4.can_move_to_top() {
            return false;
        }
        return true;
    }

    pub fn move_to_top(&mut self) -> Result<(), GameError>{
        if self.can_move_to_top() == false {
            return Ok(())
        }
        self.block1.move_to_top();
        self.block2.move_to_top();
        self.block3.move_to_top();
        self.block4.move_to_top();
        Ok(())
    }

    pub fn can_fell(&self, static_block: &StaticBlockGroup) -> bool {
        if self.block1.check_collision_down(static_block) {
            return false;
        }
        if self.block2.check_collision_down(static_block) {
            return false;
        }
        if self.block3.check_collision_down(static_block) {
            return false;
        }
        if self.block4.check_collision_down(static_block) {
            return false;
        }
        return true;
    }
    
}

pub struct StaticBlockGroup {
    block_vec: Vec<Block>,
}

impl StaticBlockGroup {
    pub fn new() -> Self{
        let block_vec: Vec<Block>= Vec::new();
        StaticBlockGroup{ 
            block_vec, 
        }
    }

    pub fn get_block_vec(&self) -> Vec<Block> {
        self.block_vec.clone()
    }

    pub fn add_group_to_static(&mut self, block_group: &BlockGroup) {
        let _ = &self.block_vec.push(block_group.block1.clone());
        let _ = &self.block_vec.push(block_group.block2.clone());
        let _ = &self.block_vec.push(block_group.block3.clone());
        let _ = &self.block_vec.push(block_group.block4.clone());
    }

    pub fn remove_from_static(&mut self, index: usize) {
        let _ = &self.block_vec.remove(index);
    }

    pub fn draw(&mut self, canvas: &mut Canvas) {
        if self.block_vec.is_empty() {
            return ;
        }
        self.block_vec.iter_mut().for_each(|b|{
            b.draw(canvas);
        });
    }

}