use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

use board::{
    Cursor, 
    BOARD_WIDTH, 
    BOARD_HEIGHT,
    TILE_SPRITE_WIDTH,
    TILE_SPRITE_HEIGHT,
};

pub struct CursorSystem;

impl<'s> System<'s> for CursorSystem {
  type SystemData = (
    WriteStorage<'s, Transform>,
    ReadStorage<'s, Cursor>,
    Read<'s, InputHandler<String, String>>,
  );

  fn run(&mut self, (mut transforms, cursors, input): Self::SystemData) {

    for (cursor, transform) in (&cursors, &mut transforms).join() {
        if let Some(x_move) = input.axis_value("cursor_horizontal") {
            transform.translation[0] = (transform.translation[0] + (x_move as f32 * TILE_SPRITE_WIDTH))
                .min(BOARD_WIDTH - TILE_SPRITE_WIDTH * 0.5)
                .max(TILE_SPRITE_WIDTH * 0.5);

        }

        if let Some(y_move) = input.axis_value("cursor_vertical") {
            transform.translation[1] = (transform.translation[1] + (y_move as f32 * TILE_SPRITE_HEIGHT))
                .min(BOARD_HEIGHT - TILE_SPRITE_HEIGHT * 0.5)
                .max(TILE_SPRITE_HEIGHT * 0.5);

        }
    }
  }
}

