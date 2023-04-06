use bevy::{prelude::*, window::PrimaryWindow};
use bevy_ggrs::ggrs::{self, PlayerType};
use bevy_matchbox::prelude::SingleChannel;
use bevy_matchbox::MatchboxSocket;

use crate::{
    constants::{AppState, GgrsConfig, INPUT_LEFT, INPUT_RIGHT},
    resources::LocalPlayerHandle,
};

pub fn start_matchbox_socket(mut commands: Commands) {
    let room_url = "ws://127.0.0.1:3536/extreme_bevy?next=2";
    info!("connecting to matchbox server: {:?}", room_url);
    commands.insert_resource(MatchboxSocket::new_ggrs(room_url));
}

pub fn close_matchbox_socket(mut commands: Commands) {
    commands.remove_resource::<MatchboxSocket<SingleChannel>>();
}

pub fn wait_for_players(
    mut commands: Commands,
    mut socket: ResMut<MatchboxSocket<SingleChannel>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if socket.get_channel(0).is_err() {
        return; // we've already started
    }

    // Check for new connections
    socket.update_peers();
    let players = socket.players();

    let num_players = 2;
    if players.len() < num_players {
        return; // wait for more players
    }

    info!("All peers have joined, going in-game");

    // create a GGRS P2P session
    let mut session_builder = ggrs::SessionBuilder::<GgrsConfig>::new()
        .with_num_players(num_players)
        .with_input_delay(2);

    for (i, player) in players.into_iter().enumerate() {
        if player == PlayerType::Local {
            commands.insert_resource(LocalPlayerHandle(i));
        }

        session_builder = session_builder
            .add_player(player, i)
            .expect("failed to add player");
    }

    // move the channel out of the socket (required because GGRS takes ownership of it)
    let socket = socket.take_channel(0).unwrap();

    // start the GGRS session
    let ggrs_session = session_builder
        .start_p2p_session(socket)
        .expect("failed to start session");

    commands.insert_resource(bevy_ggrs::Session::P2PSession(ggrs_session));
    next_state.set(AppState::InGame);
}

pub fn network_input_system(
    _: In<ggrs::PlayerHandle>,
    keys: Res<Input<KeyCode>>,
    touches: Res<Touches>,
    primary_query: Query<&Window, With<PrimaryWindow>>,
) -> u8 {
    let mut input = 0u8;

    if keys.any_pressed([KeyCode::Left, KeyCode::A]) {
        input |= INPUT_LEFT
    }
    if keys.any_pressed([KeyCode::Right, KeyCode::D]) {
        input |= INPUT_RIGHT;
    }

    let maybe_touch = touches.iter().last();
    if let Ok(window) = primary_query.get_single() {
        let center = window.width() / 2.0;
        if let Some(touch) = maybe_touch {
            if touches.get_pressed(touch.id()).is_some() {
                let tx = touch.position().x;
                if tx > center {
                    input |= INPUT_RIGHT;
                } else if tx < center {
                    input |= INPUT_LEFT;
                }
            }
        }
    }

    input
}
