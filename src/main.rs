#[derive(Clone)]
enum TileType {
    Empty,
}

#[derive(Clone)]
struct Tile {
    tile_type: TileType,
}

struct Board {
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
                Tile{ tile_type: TileType::Empty }; 
                w * h
            ],
        }
    }
}

fn main() {
    println!("Hello, world!");
}
