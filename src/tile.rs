#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TileType {
    Empty,
    //Plains,
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

    pub fn get_type(&self) -> TileType {
        self.tile_type.clone()
    }
}
