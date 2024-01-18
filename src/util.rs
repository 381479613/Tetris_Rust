//常用参数与公式
use ggez::glam::Vec2;

//total grid size in whole game board
const GRID_SIZE: (i32,i32) = (10,20);

//the whole window should wilder than the game goard
const WINDOW_GRID: (i32,i32) = (GRID_SIZE.0 + 10, GRID_SIZE.1 + 10);

//cell size per grid
pub const CELL_SIZE_PER_GRID: (i32,i32) = (32,32);
//decided screen size: width,height
pub const SCREEN_SIZE: (f32,f32) = (
    WINDOW_GRID.0 as f32 * CELL_SIZE_PER_GRID.0 as f32,
    WINDOW_GRID.1 as f32 * CELL_SIZE_PER_GRID.1 as f32,
);

pub const MOVE_SEEP: f32 = CELL_SIZE_PER_GRID.0 as f32;
pub const DESIRED_FPS: u32 = 8;


//pic scale number
pub const PIC_SCALE_NUMBER: Vec2 = Vec2::new(0.22, 0.22);


pub const GAME_BOARD_START_POSITION_X: f32 = (WINDOW_GRID.0 - GRID_SIZE.0) as f32 * WINDOW_GRID.0 as f32;
pub const GAME_BOARD_START_POSITION_Y: f32 = (WINDOW_GRID.1 - GRID_SIZE.1) as f32 * WINDOW_GRID.1 as f32;

pub const SCORE_WORD_START_POSITION: Vec2 = Vec2::new(SCREEN_SIZE.0 / 10.0 * 2.0, SCREEN_SIZE.1 /10.0 * 2.0);
