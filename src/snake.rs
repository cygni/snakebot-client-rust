use structs::{ MapUpdate, GameEnded, TournamentEnded, SnakeDead, GameStarting, PlayerRegistered, InvalidPlayerName};
use maputil::{ Direction };
use util::{ translate_positions };

const LOG_TARGET: &'static str = "snake";

pub struct Snake;

impl Snake {
    pub fn get_next_move(&self, msg: &MapUpdate) -> Direction {
        debug!(target: LOG_TARGET, "Game map updated, tick: {}", msg.gameTick);

        let ref map = msg.map;
        let player_id = &msg.receivingPlayerId;
        let snake = map.get_snake_by_id(player_id).unwrap();

        debug!(target: LOG_TARGET, "Food can be found at {:?}", translate_positions(&map.foodPositions, map.width));
        debug!(target: LOG_TARGET, "My snake positions are {:?}", translate_positions(&snake.positions, map.width));

        let direction = if map.can_snake_move_in_direction(snake, Direction::Down) {
            Direction::Down
        } else if map.can_snake_move_in_direction(snake, Direction::Left) {
            Direction::Left
        } else if map.can_snake_move_in_direction(snake, Direction::Right) {
            Direction::Right
        } else if map.can_snake_move_in_direction(snake, Direction::Up) {
            Direction::Up
        } else {
            // this is bad
            Direction::Down
        };

        debug!(target: LOG_TARGET, "Snake will move in direction {:?}", direction);
        direction
    }

    pub fn on_game_ended(&self, msg: &GameEnded) {
        debug!(target: LOG_TARGET, "Game ended, the winner is: {:?}", msg.playerWinnerId);
    }

    pub fn on_tournament_ended(&self, msg: &TournamentEnded) {
        debug!(target: LOG_TARGET, "Game ended, the winner is: {:?}", msg.playerWinnerId);
    }

    pub fn on_snake_dead(&self, msg: &SnakeDead) {
        debug!(target: LOG_TARGET, "The snake died, reason was: {:?}", msg.deathReason);
    }

    pub fn on_game_starting(&self, _: &GameStarting) {

    }

    pub fn on_player_registered(&self, _: &PlayerRegistered) {

    }

    pub fn on_invalid_playername(&self, _: &InvalidPlayerName) {

    }
}
