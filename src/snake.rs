use crate::{
    client::Player,
    types::{Direction, InboundMessage, Map},
    utils::Coordinate,
};
use log::debug;

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
            "Food can be found at {:?}",
            map.food_positions.iter().map(|pos| Coordinate::from_position(*pos, map.width)).collect::<Vec<_>>()
        );

        debug!(
            "My snake positions are {:?}",
            snake_info.positions.iter().map(|pos| Coordinate::from_position(*pos, map.width)).collect::<Vec<_>>()
        );

        for &dir in DIRECTIONS.iter() {
            if map.can_snake_move_in_direction(snake_info, dir) {
                debug!("Snake will move in direction {:?}", dir);
                return dir;
            }
        }

        debug!("Snake cannot but will move down.");

        Direction::Down
    }

    fn on_message(&mut self, message: &InboundMessage) {
        if let InboundMessage::GameStarting { .. } = message {
            // Reset snake state here
        }
    }
}
