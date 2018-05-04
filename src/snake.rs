use client::Player;
use types::{Direction, Map};
use utils::translate_positions;
use LOG_TARGET;

#[derive(Debug, Clone)]
pub struct Snake;

impl Snake {
    pub fn new() -> Snake {
        Snake
    }
}

const DIRECTIONS: [Direction; 4] = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];

impl Player for Snake {
    fn get_next_move(&mut self, map: &Map, player_id: &str) -> Direction {
        let snake_info = map.get_snake_by_id(&player_id).unwrap();

        debug!(
            target: LOG_TARGET,
            "Food can be found at {:?}",
            translate_positions(&map.food_positions, map.width)
        );
        debug!(
            target: LOG_TARGET,
            "My snake positions are {:?}",
            translate_positions(&snake_info.positions, map.width)
        );

        for &dir in DIRECTIONS.iter() {
            if map.can_snake_move_in_direction(snake_info, dir) {
                debug!(target: LOG_TARGET, "Snake will move in direction {:?}", dir);
                return dir;
            }
        }

        debug!(target: LOG_TARGET, "Snake cannot but will move down.");
        Direction::Down
    }
}
