use std::fmt; 
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

    pub fn get_tile_at(&self, x: usize, y: usize) -> Option<&Tile> {
        if x >= self.width || y >= self.height {
            ()
        }
        
        Some(&self.tiles[x * y])
    }

    /*
    pub fn set_tile_at(&mut self, x: usize, y: usize, new_tile: Tile) {
        
    }
    */
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
                let tile = self.get_tile_at(i, j).unwrap();
                
                let t = match tile.get_type() {
                    &TileType::Empty => 'o',
                    //&TileType::Plains => '_',
                };
                write!(f, "{}", t).expect("Could not write tile");
            }
            write!(f, "\n").expect("Could not write new line");
        }
        
        Ok(())
    }
}
