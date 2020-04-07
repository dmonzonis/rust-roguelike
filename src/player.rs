use crate::components::*;
use crate::{Map, TurnState};
use bracket_lib::prelude::*;
use specs::prelude::*;

/// Try moving players by the given diff and return the new turn state (Running if the
/// player turn was successful or Paused if no turn was taken)
pub fn try_move_player(dx: i32, dy: i32, ecs: &mut World) -> TurnState {
    let mut moved = false;
    let map = ecs.fetch::<Map>();
    let mut positions = ecs.write_storage::<Position>();
    let fighters = ecs.read_storage::<Fighter>();
    let players = ecs.read_storage::<Player>();
    let mut player_pos_res = ecs.write_resource::<Position>();
    for (pos, _player) in (&mut positions, &players).join() {
        let new_pos = *pos + Position::new(dx, dy);
        let new_pos_idx = map.xy_idx(new_pos.x, new_pos.y);
        if map.in_bounds(Point::new(new_pos.x, new_pos.y)) {
            for ent in map.tile_entities[new_pos_idx].iter() {
                if let Some(target) = fighters.get(*ent) {
                    // TODO: Attack!
                    let mut name = String::from("enemy");
                    if let Some(s) = ecs.read_storage::<Name>().get(*ent) {
                        name = s.name.clone();
                    }
                    console::log(format!("You slap {} in the face", name));
                    return TurnState::Running;
                }
            }

            // Move player
            if !map.blocked[new_pos_idx] {
                *pos = new_pos;
                // Also update the resource
                *player_pos_res = new_pos;
                moved = true;
            }
        }
    }

    // If the player moved, we need to recompute FOV
    if moved {
        let mut visions = ecs.write_storage::<Vision>();
        for (vision, _player) in (&mut visions, &players).join() {
            vision.recompute = true;
        }
        return TurnState::Running;
    }
    TurnState::Paused
}

/// Take player input and return the new turn state
pub fn player_input(ecs: &mut World, ctx: &mut BTerm) -> TurnState {
    // Player movement
    match ctx.key {
        None => TurnState::Paused, // No key is being pressed
        Some(key) => match key {
            // Orthogonal movement
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::H => {
                try_move_player(-1, 0, ecs)
            }
            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::L => {
                try_move_player(1, 0, ecs)
            }
            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::J => {
                try_move_player(0, 1, ecs)
            }
            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::K => {
                try_move_player(0, -1, ecs)
            }
            // Diagonal movement
            VirtualKeyCode::Numpad1 | VirtualKeyCode::B => try_move_player(-1, 1, ecs),
            VirtualKeyCode::Numpad3 | VirtualKeyCode::N => try_move_player(1, 1, ecs),
            VirtualKeyCode::Numpad7 | VirtualKeyCode::Y => try_move_player(-1, -1, ecs),
            VirtualKeyCode::Numpad9 | VirtualKeyCode::U => try_move_player(1, -1, ecs),
            _ => TurnState::Paused,
        },
    }
}
