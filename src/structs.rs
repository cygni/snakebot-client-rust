#![allow(non_snake_case)]

#[derive(Serialize, Deserialize, Debug)]
pub struct GameSettings {
    pub maxNoofPlayers: u32,
    pub startSnakeLength: u32,
    pub timeInMsPerTick: u32,
    pub obstaclesEnabled: bool,
    pub foodEnabled: bool,
    pub headToTailConsumes: bool,
    pub tailConsumeGrows: bool,
    pub addFoodLikelihood: u32,
    pub removeFoodLikelihood: u32,
    pub spontaneousGrowthEveryNWorldTick: u32,
    pub trainingGame: bool,
    pub pointsPerLength: u32,
    pub pointsPerFood: u32,
    pub pointsPerCausedDeath: u32,
    pub pointsPerNibble: u32,
    pub noofRoundsTailProtectedAfterNibble: u32,
}

impl Default for GameSettings {
    fn default() -> GameSettings {
        GameSettings {
            maxNoofPlayers: 5,
            startSnakeLength: 1,
            timeInMsPerTick: 250,
            obstaclesEnabled: true,
            foodEnabled: true,
            headToTailConsumes: true,
            tailConsumeGrows: false,
            addFoodLikelihood: 15,
            removeFoodLikelihood: 5,
            spontaneousGrowthEveryNWorldTick: 3,
            trainingGame: false,
            pointsPerLength: 1,
            pointsPerFood: 2,
            pointsPerCausedDeath: 5,
            pointsPerNibble: 10,
            noofRoundsTailProtectedAfterNibble: 3,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerRegistered {
    pub gameId: String,
    pub gameMode: String,
    pub receivingPlayerId: String,
    pub name: String,
    pub gameSettings: GameSettings
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MapUpdate {
    pub receivingPlayerId: String,
    pub gameId: String,
    pub gameTick: u32,
    pub map: Map,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InvalidPlayerName {
    pub reasonCode: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameEnded {
    pub receivingPlayerId: String,
    pub playerWinnerId: String,
    pub gameId: String,
    pub gameTick: u32,
    pub map: Map,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SnakeDead {
    pub playerId: String,
    pub x: u32,
    pub y: u32,
    pub gameId: String,
    pub gameTick: u32,
    pub deathReason: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameStarting {
    pub receivingPlayerId: String,
    pub gameId: String,
    pub noofPlayers: u32,
    pub width: u32,
    pub height: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HeartBeatResponse {
    pub receivingPlayerId: String
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct GameLink {
    pub receivingPlayerId: String,
    pub gameId: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TournamentEnded {
    pub receivingPlayerId: String,
    pub tournamentId: String,
    pub tournamentName: String,
    pub gameResult: Vec<GameResult>,
    pub gameId: String,
    pub playerWinnerId: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GameResult {
    pub points: i32,
    pub playerId: String,
    pub name: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Map {
    pub width: i32,
    pub height: i32,
    pub worldTick: u32,
    pub snakeInfos: Vec<SnakeInfo>,
    pub foodPositions: Vec<i32>,
    pub obstaclePositions: Vec<i32>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SnakeInfo {
    pub name: String,
    pub points: i32,
    pub positions: Vec<i32>,
    pub tailProtectedForGameTicks: u32,
    pub id: String
}
