use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

use board::Cursor;

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
            println!("x_move is: {}", x_move);
        }

        //let y_move = input.axis_value("cursor_vertical");

    }
  }
}

