use rand::Rng;

const PLAYS: [&str; 5] = ["rock", "paper", "scissors", "lizard", "spock"];

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

    fn won(mut self) {
        self.wins += 1;
    }
}

fn winner(p1: Player, p2: Player) {
    // draw, no points assigned to either palyer
    if p1.play == p2.play {
        println!("draw");
        return;
    }

    // pick the winner and add +1 point
    match (p1.play.as_str(), p2.play.as_str()) {
        // p1 rock -> crushes lizard, crushes scissors
        ("rock", "lizard") | ("rock", "scissors") => {
            println!("p1 wins");
            p1.won()
        }
        // p1 paper -> covers rock, disproves spock
        ("paper", "rock") | ("paper", "spock") => {
            println!("p1 wins");
            p1.won()
        }
        // p1 scissors -> decapitates lizard, cuts paper
        ("scissors", "lizard") | ("scissors", "paper") => {
            println!("p1 wins");
            p1.won()
        }
        // p1 lizard -> eats paper, poisons spock
        ("lizard", "paper") | ("lizard", "spock") => {
            println!("p1 wins");
            p1.won()
        }
        // p1 spock -> vaporises rock, smashes scissors
        ("spock", "rock") | ("spock", "scissors") => {
            println!("p1 wins");
            p1.won()
        }
        // all the other cases p2 wins
        (_, _) => {
            println!("p2 wins");
            p2.won()
        }
    };
}

fn main() {
    const NAME: &str = env!("CARGO_PKG_NAME");
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    println!("{} version: {}", NAME, VERSION);

    // initialise the players
    let player1 = Player::new("Player 1".to_string());
    let player2 = Player::new("Player 2".to_string());

    println!("1: {}, {}, {}", player1.name, player1.play, player1.wins);
    println!("2: {}, {}, {}", player2.name, player2.play, player2.wins);
    winner(player1, player2);
    // println!("p1: {}, p2: {}", player1.wins, player2.wins);
}
