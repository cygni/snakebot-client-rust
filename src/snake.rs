use types::InboundMessage;
use client::Player;
use types::{Direction, Map};
use utils::Coordinate;
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
            map.food_positions
                .iter()
                .map(|pos| Coordinate::from_position(*pos, map.width))
                .collect::<Vec<_>>()
        );

        debug!(
            target: LOG_TARGET,
            "My snake positions are {:?}",
            snake_info
                .positions
                .iter()
                .map(|pos| Coordinate::from_position(*pos, map.width))
                .collect::<Vec<_>>()
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

    fn on_message(&mut self, message: &InboundMessage) {
        match *message {
            InboundMessage::GameStarting { .. } => {
                // Reset snake state here
            }

            _ => {
                // Do nothing
            }
        }
    }
}
