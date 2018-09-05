#[derive(Clone)]
pub enum TileType {
    Empty,
}

#[derive(Clone)]
pub struct Tile {
    tile_type: TileType,
}

impl Tile {
    pub fn new(tt: TileType) -> Tile {
        Tile {
            tile_type: tt,
        }
    }
}
