use tile::*;

use amethyst::ecs::prelude::{Component, DenseVecStorage};
use amethyst::input::{is_close_requested, is_key_down};
use amethyst::core::cgmath::{Vector3, Matrix4, Ortho};
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, Event, Projection, Sprite, TextureHandle,
    VirtualKeyCode, WithSpriteRender,
};

use png_loader;

const BOARD_HEIGHT: f32 = 190.0;
pub const BOARD_WIDTH: f32 = 240.0;
const TILE_SPRITE_HEIGHT: f32 = 19.0;
pub const TILE_SPRITE_WIDTH: f32 = 16.0;

const SPRITESHEET_SIZE: (f32, f32) = (TILE_SPRITE_WIDTH, TILE_SPRITE_HEIGHT);

const TILE_DEPTH: f32 = 0.0;
const CURSOR_DEPTH: f32 = 2.0;

pub struct Cursor {
    pub x: f32,
    pub y: f32,
}

impl Component for Cursor {
    type Storage = DenseVecStorage<Self>;
}

impl Cursor { 
    pub fn new() -> Cursor {
        Cursor { x: 0.0, y: 0.0 }
    }
}

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

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_width(&self) -> usize {
        self.width
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




fn initialise_camera(world: &mut World) {
    world.create_entity()
        .with(Camera::from(Projection::Orthographic(Ortho {
            left: 0.0,
            right: BOARD_WIDTH,
            top: BOARD_HEIGHT,
            bottom: 0.0,
            near: 0.0,
            far: ::std::f32::MAX,
		})))
        .with(GlobalTransform(
            Matrix4::from_translation(Vector3::new(0.0, 0.0, 1.0)).into()
        ))
        .build();
}


fn initialise_board(world: &mut World, spritesheet: TextureHandle, board: &Board) {

    
    let sprite = Sprite {
        left: 0.0,
        right: TILE_SPRITE_WIDTH,
        top: 0.0,
        bottom: TILE_SPRITE_HEIGHT,
    };

    for y in 0..board.get_height() {
        for x in 0..board.get_width() {

            let mut transform = Transform::default();
            transform.translation = Vector3::new(
                TILE_SPRITE_WIDTH * (0.5 + x as f32), 
                TILE_SPRITE_HEIGHT * (0.5 + y as f32), 
                TILE_DEPTH);

            world
                .create_entity()
                .with_sprite(&sprite, spritesheet.clone(), SPRITESHEET_SIZE)
                .expect("Failed to add tile")
                .with(GlobalTransform::default())
                .with(transform)
                .build();
            
        }
    }
}

fn initialise_cursor(world: &mut World, spritesheet: TextureHandle, board: &Board) {

    let sprite = Sprite {
        left: 0.0,
        right: TILE_SPRITE_WIDTH,
        top: 0.0,
        bottom: TILE_SPRITE_HEIGHT,
    };

    let mut transform = Transform::default();
    transform.translation = Vector3::new(
        TILE_SPRITE_WIDTH * 0.5, 
        TILE_SPRITE_HEIGHT * 0.5, 
        CURSOR_DEPTH);

    let cursor = Cursor::new();

    world
        .create_entity()
        .with_sprite(&sprite, spritesheet.clone(), SPRITESHEET_SIZE)
        .expect("Failed to add cursor")
		.with(cursor)
        .with(GlobalTransform::default())
        .with(transform)
        .build();
}


impl<'a, 'b> State<GameData<'a, 'b>> for Board {


    fn handle_event(&mut self, _: StateData<GameData>, event: Event) -> Trans<GameData<'a, 'b>> {

        if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
            Trans::Quit
        } else {
            Trans::None
        }
    }

    fn update(&mut self, data: StateData<GameData>) -> Trans<GameData<'a, 'b>> {
        data.data.update(&data.world);
        Trans::None
    }

    fn on_start(&mut self, data: StateData<GameData>) {
        let world = data.world;
        world.register::<Cursor>();
        
        let spritesheet = png_loader::load("texture/empty_tile.png", world);
        let cursor_spritesheet = png_loader::load("texture/cursor.png", world);

        initialise_camera(world);
        initialise_board(world, spritesheet, self);
        initialise_cursor(world, cursor_spritesheet, self);
    }

} 


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_board_dimensions() {
        let b = Board::new(5,5);
        assert_eq!(b.get_height(), 5);
        assert_eq!(b.get_width(), 5);
    }

    #[test]
    fn test_init_board_initial_tiles() {
        let b = Board::new(5,5);

        for y in 0..b.get_height() {
            for x in 0..b.get_width() {
                assert_eq!(b.get_tile_at(x, y)
                           .unwrap()
                           .get_type(), 
                           TileType::Empty);
            }
        }
    }
}
