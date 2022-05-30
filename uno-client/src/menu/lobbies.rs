use super::{LobbiesList, LobbyState};
use crate::{
    game::{ExtraMessageEvent, StartGameEvent},
    utils::errors::Error,
};
use bevy::prelude::*;
use naia_bevy_client::events::MessageEvent;
use uno::Player;
use uno::{
    network::{Channels, Protocol},
    Lobby,
};
use uuid::Uuid;

pub fn execute_packets(
    mut commands: Commands,
    mut lobby_state: ResMut<State<LobbyState>>,
    mut lobbies: ResMut<LobbiesList>,
    mut start_game_event: EventWriter<StartGameEvent>,
    mut message_events: EventReader<MessageEvent<Protocol, Channels>>,
    mut extra_message_events: EventWriter<ExtraMessageEvent>,
) {
    for MessageEvent(_, protocol) in message_events.iter() {
        match protocol {
            Protocol::StartGame(_) => {
                info!("RECEIVING GAME STARTO");
                if let LobbyState::InLobby(lobby_id) = lobby_state.current() {
                    for lobby in lobbies.iter() {
                        if lobby.id == *lobby_id {
                            start_game_event.send(StartGameEvent(lobby.players.clone()));
                        }
                    }
                }
            }
            Protocol::JoinLobby(lobby) => {
                lobby_state
                    .set(LobbyState::InLobby(*lobby.lobby_id))
                    .unwrap();
            }
            Protocol::PlayerJoinedLobby(joined_lobby) => {
                for lobby in lobbies.iter_mut() {
                    if lobby.id == *joined_lobby.lobby_id {
                        lobby.players.push(Player::new(
                            Uuid::parse_str(&*joined_lobby.player_id).unwrap(),
                            (*joined_lobby.player_name).clone(),
                        ));
                    }
                }
            }
            Protocol::LeaveLobby(_) => {
                lobby_state.set(LobbyState::LobbiesList).unwrap();
            }
            Protocol::PlayerLeftLobby(left_lobby) => {
                for lobby in lobbies.iter_mut() {
                    if lobby.id == *left_lobby.lobby_id {
                        lobby
                            .players
                            .retain(|p| p.id != Uuid::parse_str(&*left_lobby.player_id).unwrap());
                    }
                }
            }
            Protocol::LobbyCreated(lobby) => {
                lobbies.push(Lobby {
                    id: *lobby.lobby_id,
                    players: Vec::new(),
                });
            }
            Protocol::LobbyDestroyed(lobby) => {
                let idx = lobbies.0.iter().position(|l| l.id == *lobby.lobby_id);
                if let Some(idx) = idx {
                    lobbies.0.remove(idx);
                }
            }
            Protocol::LobbyInfo(lobby) => {
                let players = lobby
                    .players
                    .iter()
                    .map(|(id, name)| Player::new(Uuid::parse_str(id).unwrap(), name.clone()))
                    .collect();

                for existing_lobby in lobbies.iter_mut() {
                    if existing_lobby.id == *lobby.lobby_id {
                        existing_lobby.players = players;
                        return;
                    }
                }

                lobbies.push(Lobby {
                    id: *lobby.lobby_id,
                    players,
                });
            }
            Protocol::Error(error) => {
                commands.spawn().insert(Error {
                    message: (*error.error).clone(),
                });
            }
            protocol => {
                info!("woopsies extra messages in lobby");
                extra_message_events.send(ExtraMessageEvent(protocol.clone()));
                return;
            }
        };
    }
}
