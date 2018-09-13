use tile::*;

use amethyst::input::{is_close_requested, is_key_down};
use amethyst::core::cgmath::{Vector3, Matrix4};
use amethyst::core::transform::{GlobalTransform, Transform};
use amethyst::prelude::*;
use amethyst::renderer::{
    Camera, Event, PngFormat, Projection, Sprite, Texture, TextureHandle,
    VirtualKeyCode, WithSpriteRender,
};


const ARENA_HEIGHT: f32 = 100.0;
const ARENA_WIDTH: f32 = 100.0;
const TILE_SPRITE_HEIGHT: f32 = 19.0;
const TILE_SPRITE_WIDTH: f32 = 16.0;


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




fn initialise_camera(world: &mut World) {
    world.create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            ARENA_HEIGHT,
            0.0,
        )))
        .with(GlobalTransform(
            Matrix4::from_translation(Vector3::new(0.0, 0.0, 1.0)).into()
        ))
        .build();
}

fn initialise_paddles(world: &mut World, spritesheet: TextureHandle) {
    let sprite = Sprite {
        left: 0.0,
        right: TILE_SPRITE_WIDTH,
        top: 0.0,
        bottom: TILE_SPRITE_HEIGHT,
    };
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
    }

}
