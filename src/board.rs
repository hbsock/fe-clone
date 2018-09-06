use std::fmt; 
use tile::*;

pub struct Board {
    width: usize,
    height: usize,
    pub tiles: Vec<Tile>,
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

impl fmt::Display for Board {

    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        
        for i in 0..self.height {
            for j in 0..self.width {
                let tile = &self.tiles[i * j];
                let t = match tile.get_type() {
                    &TileType::Empty => 'o',
                    &TileType::Plains => '_',
                };
                write!(f, "{}", t);
            }
            write!(f, "\n");
        }

        /*
        for tile in &self.tiles {
            let t = match tile.get_type() {
                &TileType::Empty => 'o',
                &TileType::Plains => '_',
            };
            write!(f, "{}", t);
        }
        */
        
        Ok(())
    }
}
