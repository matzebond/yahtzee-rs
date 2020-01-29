use dice::{Dice, DICE_NUM}; use types::{Category, Entry};
use result::Result;
use player::Player;
use rand::Rng;
use rand::FromEntropy;
use rand::rngs::SmallRng;

const TOSS_NUM: u8 = 3;

pub struct GameState {
    pub result: Result,
    pub history: Vec<(Dice, Entry)>,
}

type HistoryEntry = (Dice, Entry);

impl GameState {
    pub fn new() -> GameState {
        GameState { result: Result::new(), history: Vec::new() }
    }

    fn write_entry(&mut self, dice: Dice, entry: Entry) {
        self.result.add(entry);
        self.history.push((dice, entry));
    }
}

pub struct Game<P: Player, R: Rng> {
    player_states: Vec<(P, GameState)>,
    rng: R,
}

impl<P: Player, R: Rng> Game<P, R> {
    pub fn new(players: Vec<P>, rng: R) -> Game<P, R> {
        let mut player_states = Vec::new();
        for p in players {
            let state = GameState::new();
            player_states.push((p, state));
        }
        Game { player_states, rng }
    }

    pub fn run(&mut self) {
        println!("Start Game");
        for i in 0..Category::into_iter().count() {
            println!("Turn {}", i);
            for (p, state) in self.player_states.iter_mut() {
                println!("Player {}", p.name());
                Game::<P, R>::turn(p, state);
            }
        }
        self.finish();
    }

    fn turn(player: &P, state: &mut GameState) {
        let mut dice = Game::<P, R>::toss(&Dice::new());
        println!("Toss 1: {:?}", dice);
        for i in 1..TOSS_NUM {
            let keep = player.decide_keep(&state.result, TOSS_NUM-i, &dice);
            println!("keep {:?}", keep);
            dice = Game::<P, R>::toss(&keep);
            println!("Toss {}: {:?}", i+1, dice);
        }
        let entry = player.decide_entry(&state.result, &dice);
        println!("Write {} to {:?}", entry.1, entry.0);
        state.write_entry(dice, entry);
    }

    fn toss(keep: &Dice) -> Dice {
        let mut rng = SmallRng::from_entropy();
        let mut new_dice = keep.clone();
        for i in 0..(DICE_NUM - keep.len() as u8) {
            let r = rng.gen_range(1,7);
            new_dice.push(r);
        }
        new_dice.sort_unstable();
        new_dice
    }

    fn finish(&self) {
        for (p, state) in self.player_states.iter() {
            println!("Score of Player {}: {}", p.name(), state.result.value());
        }
    }

}
