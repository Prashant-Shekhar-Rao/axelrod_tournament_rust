pub trait Stratergy: FnMut(&[bool], &[bool]) -> bool {}
impl<T> Stratergy for T where T: FnMut(&[bool], &[bool]) -> bool {}
use rand::Rng;

pub struct Tournament {
    players: Vec<fn(&[bool], &[bool]) -> bool>,
    score: Vec<i32>,
    pub both_good: i32,
    pub both_bad: i32,
    pub betray: i32,
}
//I would need to use a macro which constructs the closure like vec! If I want runtime optimizations.
//I can make a function which just runs vec! inside it . Nah that would also need a macro a function can't have variable parameters. This clears problem of different sizes.
impl Tournament {
    pub fn add_player(&mut self, x: fn(&[bool], &[bool]) -> bool) {
        self.players.push(x);
        self.score.push(0);
    }
    pub fn run(&mut self) {
        let both_good = self.both_good;
        let both_bad = self.both_bad;
        let betray = self.betray;
        let mut rng = rand::thread_rng();
        let random_number: u32 = rng.gen_range(200..=230);
        for i in 0..self.players.len() {
            for j in 0..self.players.len() {
                let mut record_i: Vec<bool> = Vec::with_capacity(random_number as usize);
                let mut record_j: Vec<bool> = Vec::with_capacity(random_number as usize);
                for _ in 0..random_number {
                    let a = self.players[i](&record_i, &record_j);
                    let b = self.players[j](&record_j, &record_i);
                    match (a, b) {
                        (true, true) => {
                            self.score[i] = self.score[i] + both_good;
                            self.score[j] = self.score[j] + both_good;
                        }
                        (false, false) => {
                            self.score[i] = self.score[i] + both_bad;
                            self.score[j] = self.score[j] + both_bad;
                        }
                        (false, true) => {
                            self.score[i] = self.score[i] + betray;
                        }
                        (true, false) => {
                            self.score[j] = self.score[j] + betray;
                        }
                    }
                    record_i.push(a);
                    record_j.push(b);
                }
            }
        }
    }
    pub fn new() -> Tournament {
        Tournament {
            players: vec![random_player],
            score: vec![0i32],
            both_good: 5,
            both_bad: 1,
            betray: 10,
        }
    }
}
fn random_player(_: &[bool], _: &[bool]) -> bool {
    let mut rng = rand::thread_rng();
    rng.gen_bool(0.5)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
