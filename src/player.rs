extern crate rltk;
use rltk::{Rltk, VirtualKeyCode, Point};
extern crate specs;
use super::{CombatStats, Map, Player, Position, RunState, State, Viewshed, WantsToMelee};
use specs::prelude::*;
use std::cmp::{min, max};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let entities = ecs.entities();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();
    let map = ecs.fetch::<Map>();

    for (_player, pos, viewshed, entity) in (&mut players, &mut positions, &mut viewsheds, &entities).join() {
        let destination_idx = map.xy_idx(pos.x + delta_x, pos.y + delta_y);

        for potential_target in map.tile_content[destination_idx].iter() {
            let target = combat_stats.get(*potential_target);
            match target {
                None => {}
                Some(_t) => {
                    wants_to_melee.insert(entity, WantsToMelee{ target: *potential_target }).expect("Add target failed");
                    return;
                }
            }
        }
        if !map.blocked[destination_idx] {
            pos.x = min(79 , max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));

            viewshed.dirty = true;
            let mut ppos = ecs.write_resource::<Point>();
            ppos.x = pos.x;
            ppos.y = pos.y;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    // Player movement
    match ctx.key {
        None => return RunState::Paused,
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::Numpad4 | VirtualKeyCode::A => {
                try_move_player(-1, 0, &mut gs.ecs)
            }

            VirtualKeyCode::Right | VirtualKeyCode::Numpad6 | VirtualKeyCode::D => {
                try_move_player(1, 0, &mut gs.ecs)
            }

            VirtualKeyCode::Up | VirtualKeyCode::Numpad8 | VirtualKeyCode::W => {
                try_move_player(0, -1, &mut gs.ecs)
            }

            VirtualKeyCode::Down | VirtualKeyCode::Numpad2 | VirtualKeyCode::S => {
                try_move_player(0, 1, &mut gs.ecs)
            }

            VirtualKeyCode::Numpad9 | VirtualKeyCode::E => try_move_player(1, -1, &mut gs.ecs),

            VirtualKeyCode::Numpad7 | VirtualKeyCode::Q => try_move_player(-1, -1, &mut gs.ecs),

            VirtualKeyCode::Numpad3 | VirtualKeyCode::C => try_move_player(1, 1, &mut gs.ecs),

            VirtualKeyCode::Numpad1 | VirtualKeyCode::Z => try_move_player(-1, 1, &mut gs.ecs),

            _ => return RunState::Paused,
        },
    }
    RunState::Running
}
