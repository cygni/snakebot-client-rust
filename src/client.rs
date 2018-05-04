use rustc_version::version;
use serde_json;
use std::mem;
use target_info::Target;
use types::{Direction, InboundMessage, Map, OutboundMessage};
use ws;

use LOG_TARGET;

const HEARTBEAT_TOKEN: ws::util::Token = ws::util::Token(1337);
const HEARTBEAT_INTERVAL: u64 = 10_000;

pub trait Player: Clone {
  fn get_next_move(&mut self, map: &Map, player_id: &str) -> Direction;
  fn on_message(&mut self, _: &InboundMessage) {}
}

#[derive(Clone, Debug)]
pub struct Config {
  pub host: String,
  pub port: i32,
  pub venue: String,
  pub snake_name: String,
}

pub struct Client<P: Player> {
  player: P,
  config: Config,
  ws: ws::Sender,
  player_id: Option<String>,
  timeout: Option<ws::util::Timeout>,
}

impl<P: Player> Client<P> {
  pub fn connect(player: P, config: Config) -> ws::Result<()> {
    let connection_url = format!("ws://{}:{}/{}", &config.host, &config.port, &config.venue);
    info!(target: LOG_TARGET, "Connecting to {:?}", connection_url);

    ws::connect(connection_url, |ws| Client {
      player: player.clone(),
      config: config.clone(),
      ws,
      player_id: None,
      timeout: None,
    })
  }

  fn send_message(&self, message: OutboundMessage) -> ws::Result<()> {
    info!(target: LOG_TARGET, "Sending message: {:?}", message);
    let json_string = serde_json::to_string(&message).map_err(Box::new)?;
    self.ws.send(json_string)
  }
}

impl<P: Player> ws::Handler for Client<P> {
  fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
    debug!(target: LOG_TARGET, "WebSocket opened");

    self.send_message(OutboundMessage::ClientInfo {
      language: "Rust",
      language_version: &version().unwrap().to_string(),
      operating_system: Target::os(),
      operating_system_version: "???",
      client_version: option_env!("CARGO_PKG_VERSION").unwrap_or("0.1337"),
    })?;

    self.send_message(OutboundMessage::RegisterPlayer {
      player_name: &self.config.snake_name,
      game_settings: Default::default(),
    })?;

    Ok(())
  }

  fn on_timeout(&mut self, token: ws::util::Token) -> ws::Result<()> {
    match token {
      HEARTBEAT_TOKEN => {
        self.ws.timeout(HEARTBEAT_INTERVAL, HEARTBEAT_TOKEN)?;
        if let Some(ref player_id) = self.player_id {
          self.send_message(OutboundMessage::HeartBeatRequest {
            receiving_player_id: player_id,
          })?;
        }
      }
      _ => {}
    }
    Ok(())
  }

  fn on_new_timeout(&mut self, event: ws::util::Token, timeout: ws::util::Timeout) -> ws::Result<()> {
    if event == HEARTBEAT_TOKEN {
      // Replace the current timeout with the new one
      let prev_timeout_option = mem::replace(&mut self.timeout, Some(timeout));

      if let Some(prev_timeout) = prev_timeout_option {
        self.ws.cancel(prev_timeout)?;
      }
    }
    Ok(())
  }

  fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
    debug!(
      target: LOG_TARGET,
      "WebSocket closed with code {:?} and reason: {}",
      code,
      reason
    );

    if let Some(timeout) = self.timeout.take() {
      self.ws.cancel(timeout).unwrap();
    }
  }

  fn on_message(&mut self, message: ws::Message) -> ws::Result<()> {
    let text = message.into_text()?;
    let message = serde_json::from_str::<InboundMessage>(&text).map_err(Box::new)?;
    debug!(target: LOG_TARGET, "Received message: {:?}", message);

    self.player.on_message(&message);

    match message {
      InboundMessage::PlayerRegistered {
        name,
        game_mode,
        receiving_player_id,
        ..
      } => {
        info!(target: LOG_TARGET, "Successfully registered player {}", name);
        if game_mode == "TRAINING" {
          self.send_message(OutboundMessage::StartGame)?;
        }
        self.player_id = Some(receiving_player_id);
        self.ws.timeout(HEARTBEAT_INTERVAL, HEARTBEAT_TOKEN)?;
      }

      InboundMessage::InvalidPlayerName { .. } => {
        debug!(target: LOG_TARGET, "Player name invalid.");
      }

      InboundMessage::GameStarting { .. } => {
        debug!(target: LOG_TARGET, "All snakes are ready to rock. Game is starting.");
      }

      InboundMessage::GameLink { url, .. } => {
        info!(target: LOG_TARGET, "Watch game at: {}", url);
      }

      InboundMessage::MapUpdate {
        map,
        game_id,
        game_tick,
        receiving_player_id,
        ..
      } => {
        debug!(target: LOG_TARGET, "Game map updated, tick: {}", game_tick);

        let direction = self.player.get_next_move(&map, &receiving_player_id);

        self.send_message(OutboundMessage::RegisterMove {
          direction,
          game_tick,
          receiving_player_id: &receiving_player_id,
          game_id: &game_id,
        })?;
      }

      InboundMessage::SnakeDead { death_reason, .. } => {
        debug!(target: LOG_TARGET, "The snake died, the reason was: {}", death_reason);
      }

      InboundMessage::GameEnded { player_winner_id, .. } => {
        debug!(target: LOG_TARGET, "Game ended, the winner is: {}", player_winner_id);
        if self.config.venue == "training" {
          self.ws.close(ws::CloseCode::Normal)?;
        }
      }

      InboundMessage::TournamentEnded { player_winner_id, .. } => {
        debug!(
          target: LOG_TARGET,
          "Tournament ended, the winner is: {}",
          player_winner_id
        );
        self.ws.close(ws::CloseCode::Normal)?;
      }

      _ => {}
    }
    Ok(())
  }
}
