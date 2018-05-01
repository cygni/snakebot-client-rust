use maputil::Direction;
use rustc_version::version;
use serde_json::{from_str, from_value, Error, Map, Value};
use std::iter::FromIterator;
use structs;
use structs::GameSettings;
use target_info::Target;

#[derive(Serialize, Deserialize, Debug)]
pub enum Inbound {
    GameEnded(structs::GameEnded),
    TournamentEnded(structs::TournamentEnded),
    MapUpdate(structs::MapUpdate),
    SnakeDead(structs::SnakeDead),
    GameStarting(structs::GameStarting),
    PlayerRegistered(structs::PlayerRegistered),
    InvalidPlayerName(structs::InvalidPlayerName),
    HeartBeatResponse(structs::HeartBeatResponse),
    GameLink(structs::GameLink),
    GameResult(structs::GameResult),
    UnrecognizedMessage,
}

/// We turn the string into `Inbound` by converting the string into a
/// JSON object, extracting the type-field from the object, and using the
/// last part of the type-field to get the correct constructor in `Inbound`.
/// Then we let Serde do its magic and deserialize a constructed JSON object
/// with the constructor name as type. If the type has a Event suffix it is
/// removed since almost all `Inbound` messages are events.
///
/// Example:
/// { type: "foo.bar.baz.GameResult", <Some JSON data> }
///                      ---------- This is the part we extract and hand to serde.
/// Like this: {GameResult: <Some JSON data>}
///
pub fn handle_inbound_msg(s: &str) -> Result<Inbound, Error> {
    let mut json_value =
        from_str::<Value>(s).expect(&format!("Couldn't parse string into JSON: {:?}", s));
    let mut map = json_value
        .as_object_mut()
        .expect(&format!("Couldn't parse string into JSON object: {:?}", s));
    let type_value = map.remove("type")
        .expect(&format!("Couldn't find key `type` in: {:?}", &map));
    let type_str = type_value
        .as_str()
        .expect(&format!("Couldn't turn JSON Value into string: {:?}", &map));
    let typ = type_str
        .rsplit('.')
        .next()
        .expect(&format!(
            "The type parser needs a dot-separated string, this string lacks dots: {:?}",
            type_str
        ))
        .replace("Event", "");
    from_value(Value::Object(Map::from_iter(vec![
        (typ, Value::Object(map.clone())),
    ])))
}

pub enum Outbound {
    RegisterPlayer {
        playerName: String,
        gameSettings: GameSettings,
    },
    StartGame,
    RegisterMove {
        direction: Direction,
        gameTick: u32,
        receivingPlayerId: String,
        gameId: String,
    },
    HeartBeat {
        receivingPlayerId: String,
    },
    ClientInfo,
}

pub fn render_outbound_message(msg: Outbound) -> String {
    (match msg {
        Outbound::RegisterPlayer {
            playerName,
            gameSettings,
        } => json!({
            "type": "se.cygni.snake.api.request.RegisterPlayer",
            "playerName": playerName,
            "gameSettings": gameSettings
        }),
        Outbound::StartGame => json!({
            "type": "se.cygni.snake.api.request.StartGame",
        }),
        Outbound::RegisterMove {
            direction,
            gameTick,
            receivingPlayerId,
            gameId,
        } => json!({
            "type": "se.cygni.snake.api.request.RegisterMove",
            "direction": direction,
            "gameTick": gameTick,
            "receivingPlayerId": receivingPlayerId,
            "gameId": gameId,
        }),
        Outbound::HeartBeat { receivingPlayerId } => json!({
            "type": "se.cygni.snake.api.request.HeartBeatRequest",
            "receivingPlayerId": receivingPlayerId,
        }),
        Outbound::ClientInfo => json!({
            "type": "se.cygni.snake.api.request.ClientInfo",
            "language": "Rust",
            "languageVersion": version().unwrap().to_string(),
            "operatingSystem": Target::os(),
            "operatingSystemVersion": "???",
            "clientVersion": option_env!("CARGO_PKG_VERSION").unwrap_or("0.1337"),
        }),
    }).to_string()
}
