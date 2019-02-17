use serde_derive::{Deserialize, Serialize};

pub type Position = i32;

#[derive(Debug, PartialEq, Copy, Clone, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Deserialize)]
pub enum DeathReason {
    CollisionWithWall,
    CollisionWithObstacle,
    CollisionWithSnake,
    CollisionWithSelf,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GameMode {
    Training,
    Tournament,
    Highscore,
    Arena,
}

#[derive(Debug, PartialEq, Deserialize)]
pub enum PlayerNameInvalidReason {
    Taken,
    Empty,
    InvalidCharacter,
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SnakeInfo {
    pub id: String,
    pub name: String,
    pub points: i32,
    pub positions: Vec<Position>,
    pub tail_protected_for_game_ticks: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameResult {
    pub name: String,
    pub player_id: String,
    pub points: i32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerRank {
    pub player_name: String,
    pub player_id: String,
    pub rank: i32,
    pub points: i32,
    pub alive: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub world_tick: u32,
    pub snake_infos: Vec<SnakeInfo>,
    pub food_positions: Vec<Position>,
    pub obstacle_positions: Vec<Position>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameSettings {
    pub max_noof_players: u32,
    pub start_snake_length: u32,
    pub time_in_ms_per_tick: u32,
    pub obstacles_enabled: bool,
    pub food_enabled: bool,
    pub head_to_tail_consumes: bool,
    pub tail_consume_grows: bool,
    pub add_food_likelihood: u32,
    pub remove_food_likelihood: u32,
    pub spontaneous_growth_every_n_world_tick: u32,
    pub training_game: bool,
    pub points_per_length: u32,
    pub points_per_food: u32,
    pub points_per_caused_death: u32,
    pub points_per_nibble: u32,
    pub noof_rounds_tail_protected_after_nibble: u32,
}

impl Default for GameSettings {
    fn default() -> GameSettings {
        GameSettings {
            max_noof_players: 5,
            start_snake_length: 1,
            time_in_ms_per_tick: 250,
            obstacles_enabled: true,
            food_enabled: true,
            head_to_tail_consumes: true,
            tail_consume_grows: false,
            add_food_likelihood: 15,
            remove_food_likelihood: 5,
            spontaneous_growth_every_n_world_tick: 3,
            training_game: false,
            points_per_length: 1,
            points_per_food: 2,
            points_per_caused_death: 5,
            points_per_nibble: 10,
            noof_rounds_tail_protected_after_nibble: 3,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum InboundMessage {
    #[serde(rename = "se.cygni.snake.api.event.GameEndedEvent", rename_all = "camelCase")]
    GameEnded {
        receiving_player_id: String,
        player_winner_id: String,
        player_winner_name: String,
        game_id: String,
        game_tick: u32,
        map: Map,
    },

    #[serde(rename = "se.cygni.snake.api.event.GameLinkEvent", rename_all = "camelCase")]
    GameLink { receiving_player_id: String, game_id: String, url: String },

    #[serde(rename = "se.cygni.snake.api.event.GameResultEvent", rename_all = "camelCase")]
    GameResult { receiving_player_id: String, game_id: String, player_ranks: Vec<PlayerRank> },

    #[serde(rename = "se.cygni.snake.api.event.GameStartingEvent", rename_all = "camelCase")]
    GameStarting {
        receiving_player_id: String,
        game_id: String,
        noof_players: u32,
        width: u32,
        height: u32,
        game_settings: GameSettings,
    },

    #[serde(rename = "se.cygni.snake.api.response.HeartBeatResponse", rename_all = "camelCase")]
    HeartBeatResponse { receiving_player_id: String },

    #[serde(rename = "se.cygni.snake.api.exception.InvalidPlayerName", rename_all = "camelCase")]
    InvalidPlayerName {
        receiving_player_id: String,
        #[serde(rename = "PlayerNameInvalidReason")]
        reason: PlayerNameInvalidReason,
    },

    #[serde(rename = "se.cygni.snake.api.event.MapUpdateEvent", rename_all = "camelCase")]
    MapUpdate { receiving_player_id: String, game_id: String, game_tick: u32, map: Map },

    #[serde(rename = "se.cygni.snake.api.response.PlayerRegistered", rename_all = "camelCase")]
    PlayerRegistered {
        receiving_player_id: String,
        name: String,
        game_id: String,
        game_mode: GameMode,
        game_settings: GameSettings,
    },

    #[serde(rename = "se.cygni.snake.api.event.SnakeDeadEvent", rename_all = "camelCase")]
    SnakeDead {
        receiving_player_id: String,
        game_id: String,
        game_tick: u32,
        player_id: String,
        x: u32,
        y: u32,
        death_reason: DeathReason,
    },

    #[serde(rename = "se.cygni.snake.api.event.TournamentEndedEvent", rename_all = "camelCase")]
    TournamentEnded {
        receiving_player_id: String,
        player_winner_id: String,
        game_id: String,
        game_result: Vec<GameResult>,
        tournament_id: String,
        tournament_name: String,
    },
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum OutboundMessage<'a> {
    #[serde(rename = "se.cygni.snake.api.request.ClientInfo", rename_all = "camelCase")]
    ClientInfo {
        language: &'a str,
        language_version: &'a str,
        operating_system: &'a str,
        operating_system_version: &'a str,
        client_version: &'a str,
    },

    #[serde(rename = "se.cygni.snake.api.request.HeartBeatRequest", rename_all = "camelCase")]
    HeartBeatRequest { receiving_player_id: &'a str },

    #[serde(rename = "se.cygni.snake.api.request.RegisterMove", rename_all = "camelCase")]
    RegisterMove { direction: Direction, game_tick: u32, game_id: &'a str, receiving_player_id: &'a str },

    #[serde(rename = "se.cygni.snake.api.request.RegisterPlayer", rename_all = "camelCase")]
    RegisterPlayer { player_name: &'a str, game_settings: GameSettings },

    #[serde(rename = "se.cygni.snake.api.request.StartGame", rename_all = "camelCase")]
    StartGame,
}
