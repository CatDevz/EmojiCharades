use actix::{Actor, Context, Handler, Message};
use itertools::Itertools;
use once_cell::sync::Lazy;
use rand::distributions::Alphanumeric;
use rand::Rng;
use uuid::Uuid;

static DEFAULT_PROMPT_LIST: Lazy<Vec<String>> = Lazy::new(|| {
    include_str!("assets/prompts.txt")
        .split('\n')
        .map_into()
        .collect::<Vec<_>>()
});

#[derive(Debug, Clone, Eq, PartialEq)]
enum PlayerRole {
    Admin,
    Member,
    Spectator,
}

struct Player {
    id: Uuid,
    nickname: String,
    score: u8,
    role: PlayerRole,
}

struct GameSettings {
    prompt_list: Vec<String>,
    time_to_guess_in_seconds: u8,
    number_of_rounds: u8,
    allow_specators: bool,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            prompt_list: DEFAULT_PROMPT_LIST.clone(),
            time_to_guess_in_seconds: 30,
            number_of_rounds: 10,
            allow_specators: true,
        }
    }
}

struct OngoingGame {
    current_prompt: String,
    current_turn_index: i32, // index: not including spectators
    current_round: i32,
}

enum State {
    Lobby,
    Playing(OngoingGame),
}

struct Game {
    id: Uuid,
    state: State,
    room_code: String,
    players: Vec<Player>,
    settings: GameSettings,
}

impl Game {
    fn new() -> Game {
        let room_code: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        Game {
            id: Uuid::new_v4(),
            state: State::Lobby,
            players: Vec::new(),
            settings: GameSettings::default(),
            room_code,
        }
    }
}

impl Actor for Game {
    type Context = Context<Self>;
}

enum PlayerJoinResponse {
    Success,
    NicknameIsTaken,
}

enum StartGameResponse {
    Started,
    GameAlreadyStarted,
    Unauthorized,
}

#[derive(Message)]
#[rtype(result = "Result<PlayerJoinResponse, std::io::Error>")]
struct PlayerJoined {
    nickname: String,
}

impl Handler<PlayerJoined> for Game {
    type Result = Result<PlayerJoinResponse, std::io::Error>;

    fn handle(&mut self, msg: PlayerJoined, _: &mut Context<Self>) -> Self::Result {
        if self
            .players
            .iter()
            .any(|player| player.nickname.eq(&msg.nickname))
        {
            Ok(PlayerJoinResponse::NicknameIsTaken)
        } else {
            let player_role: PlayerRole = match self.state {
                State::Lobby => PlayerRole::Member,
                State::Playing(_) => PlayerRole::Spectator,
            };

            self.players.push(Player {
                id: Uuid::new_v4(),
                nickname: msg.nickname,
                score: 0,
                role: player_role,
            });

            Ok(PlayerJoinResponse::Success)
        }
    }
}

#[derive(Message)]
#[rtype(result = "Result<StartGameResponse, std::io::Error>")]
struct StartGame {
    initiator: Uuid,
}

impl Handler<StartGame> for Game {
    type Result = Result<StartGameResponse, std::io::Error>;

    fn handle(&mut self, msg: StartGame, _: &mut Context<Self>) -> Self::Result {
        if self
            .players
            .iter()
            .any(|it| it.id == msg.initiator && it.role == PlayerRole::Admin)
        {
            return match self.state {
                State::Playing(_) => Ok(StartGameResponse::GameAlreadyStarted),
                State::Lobby => {
                    self.state = State::Playing(OngoingGame {
                        current_prompt: self.settings.prompt_list.first().unwrap().clone(),
                        current_turn_index: 0,
                        current_round: 0,
                    });

                    Ok(StartGameResponse::Started)
                }
            };
        }

        Ok(StartGameResponse::Unauthorized)
    }
}
