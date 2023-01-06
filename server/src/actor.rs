use actix::{Actor, Context, Handler, Message};
use rand::distributions::Alphanumeric;
use rand::Rng;
use uuid::Uuid;

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
    wordList: Option<Vec<String>>,
    timeToGuessInMinutes: Option<u8>,
    numberOfRounds: Option<u8>,
}

struct OngoingGame {
    currentWord: String,
    currentTurnIndex: i32,
}

enum Status {
    Lobby(GameSettings),
    Playing(OngoingGame),
}

struct Game {
    id: Uuid,
    status: Status,
    roomCode: String,
    players: Vec<Player>,
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
            status: Status::Lobby(GameSettings {
                wordList: None,
                timeToGuessInMinutes: None,
                numberOfRounds: None,
            }),
            roomCode: room_code,
            players: Vec::new(),
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
            let player_role: PlayerRole = match self.status {
                Status::Lobby(_) => PlayerRole::Member,
                Status::Playing(_) => PlayerRole::Spectator,
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
