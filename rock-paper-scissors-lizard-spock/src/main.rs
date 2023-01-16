use rand::Rng;

const PLAYS: [&'static str; 5] = ["rock", "paper", "scissors", "lizard", "spock"];

struct Player {
    name: String,
    play: String,
    wins: u8,
}

impl Player {
    fn new(name: String) -> Player {
        // Generate a random number in the range of PLAYS [0..5]
        // and pick the corresponding play
        let mut rng = rand::thread_rng();
        let random_index: usize = rng.gen_range(0..PLAYS.len());
        let current_play = PLAYS[random_index];
        // println!("{:?}", current_play); // DEBUG

        Player {
            name,
            play: String::from(current_play),
            wins: 0,
        }
    }

    // fn won(mut self) {
    //     self.wins += 1;
    // }
}

// fn winner(p1: Player, p2: Player) -> () {
//     // pick the winner and add +1 point
//     match p1.play {
//         String::from("rock") => println!("p1 wins"), //p1.won(),
//         // "paper" => (),
//         // "scissors" =>  (),
//         // "lizard" => (),
//         // "spock" => (),
//         // (_, _) => p2.won(),
//         _ => println!("Ain't special"),
//     };
// }

fn main() {
    const NAME: &str = env!("CARGO_PKG_NAME");
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    println!("{} version: {}", NAME, VERSION);

    // initialise the players
    let player1 = Player::new("Player 1".to_string());
    let player2 = Player::new("Player 2".to_string());

    // this also works
    // let player1 = Player { name: "Player 1".to_string(), play: "rock".to_string(), wins: 1 };
    // let player2 = Player { name: "Player 2".to_string(), play: "scissors".to_string(), wins: 0 };

    println!("1: {}, {:?}, {}", player1.name, player1.play, player1.wins);
    println!("2: {}, {:?}, {}", player2.name, player2.play, player2.wins);
}