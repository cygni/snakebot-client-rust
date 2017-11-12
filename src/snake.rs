use structs::{ MapUpdate, GameEnded, TournamentEnded, SnakeDead, GameStarting, PlayerRegistered, InvalidPlayerName };
use maputil::{ Direction };
use util::{ translate_positions };

const LOG_TARGET: &'static str = "snake";

pub struct Snake;

impl Snake {
    pub fn get_next_move(&self, msg: &MapUpdate) -> Direction {
        debug!(target: LOG_TARGET, "Game map updated, tick: {}", msg.gameTick);

        let map = &msg.map;
        let snake = map.get_snake_by_id(&msg.receivingPlayerId).unwrap();

        debug!(target: LOG_TARGET, "Food can be found at {:?}", translate_positions(&map.foodPositions, map.width));
        debug!(target: LOG_TARGET, "My snake positions are {:?}", translate_positions(&snake.positions, map.width));
        for &d in [Direction::Down, Direction::Left, Direction::Right, Direction::Up].into_iter() {
            if map.can_snake_move_in_direction(snake, d) {
                debug!(target: LOG_TARGET, "Snake will move in direction {:?}", d);
                return d;
            }
        }

        debug!(target: LOG_TARGET, "Snake cannot but will move down.");
        return Direction::Down;
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
        debug!(target: LOG_TARGET, "All snakes are ready to rock. Game is starting.");
    }

    pub fn on_player_registered(&self, _: &PlayerRegistered) {
        debug!(target: LOG_TARGET, "Player has been registered.");
    }

    pub fn on_invalid_playername(&self, _: &InvalidPlayerName) {
        debug!(target: LOG_TARGET, "Player name invalid.");
    }
}
