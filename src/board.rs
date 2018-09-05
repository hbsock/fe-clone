use tile::*;

pub struct Board {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Board {
    pub fn new(w: usize, h: usize) -> Board {
        Board {
            width: w,
            height: h,
            tiles: vec![
                Tile::new(TileType::Empty); 
                w * h
            ],
        }
    }
}
