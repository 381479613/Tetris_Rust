//常用参数与公式

//total grid size in whole game board
pub const GRID_SIZE: (i32,i32) = (50,80);
//cell size per grid
pub const CELL_SIZE_PER_GRID: (i32,i32) = (32,32);

//decided screen size
pub const SCREEN_SIZE: (f32,f32) = (
    GRID_SIZE.0 as f32 * CELL_SIZE_PER_GRID.0 as f32,
    GRID_SIZE.1 as f32 * CELL_SIZE_PER_GRID.1 as f32,
);

pub const DESIRED_FPS: u32 = 8;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct GridPosition {
    x: i32,
    y: i32,
}

impl GridPosition {
    pub fn new(x: i32, y: i32) -> Self {
        GridPosition {x,y}
    }

    
}