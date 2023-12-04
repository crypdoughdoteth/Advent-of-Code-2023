use anyhow::{bail, Result};
use std::{fs, str::Chars};

#[derive(Debug)]
struct GameInstance {
    id: u32,
    game: Vec<CubeGame>,
}
#[derive(Debug, Copy, Clone)]
struct CubeGame {
    blue: Option<u32>,
    green: Option<u32>,
    red: Option<u32>,
}

impl GameInstance {
    pub fn new() -> Self {
        Self {
            id: 0,
            game: vec![],
        }
    }
    pub fn validate(&self) -> bool {
        let mut success = true;
        self.game.iter().for_each(|game| {
            if game.validate() == false {
                success = false;
            }
        });
        return success;
    }

    pub fn minimum_set(&self) -> CubeGame {
        self.game.iter().fold(CubeGame::new(), |acc, x| {
            let mut minset: CubeGame = CubeGame {
                blue: Some(0),
                green: Some(0),
                red: Some(0),
            };
            if acc.blue.is_none() {
                minset.blue = Some(x.blue.unwrap_or(0));
            } else {
                minset.blue = Some(std::cmp::max(x.blue.unwrap_or(0), acc.blue.unwrap_or(0)));
            }
            if acc.red.is_none() {
                minset.red = Some(x.red.unwrap_or(0));
            } else {
                minset.red = Some(std::cmp::max(x.red.unwrap_or(0), acc.red.unwrap_or(0)));
            }
            if acc.green.is_none() {
                minset.green = Some(x.green.unwrap_or(0));
            } else {
                minset.green = Some(std::cmp::max(x.green.unwrap_or(0), acc.green.unwrap_or(0)));
            }
            minset
        })
    }

    pub fn get_power(&self) -> u32 {
        let minset = self.minimum_set();
        minset.blue.unwrap() * minset.green.unwrap() * minset.red.unwrap()
    }
}

const REF_GAME: CubeGame = CubeGame {
    blue: Some(14),
    green: Some(13),
    red: Some(12),
};

impl CubeGame {
    pub fn new() -> Self {
        Self {
            blue: None,
            green: None,
            red: None,
        }
    }
    pub fn validate(&self) -> bool {
        if let Some(num) = self.red {
            if num > REF_GAME.red.unwrap() {
                return false;
            }
        }
        if let Some(num) = self.green {
            if num > REF_GAME.green.unwrap() {
                return false;
            }
        }

        if let Some(num) = self.blue {
            if num > REF_GAME.blue.unwrap() {
                return false;
            }
        }
        true
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
enum Token {
    Game,
    Id(u32),
    Red(u32),
    Green(u32),
    Blue(u32),
    Semicolon,
}

fn main() -> Result<()> {
    let input = fs::read_to_string("day-2/input.txt")?;
    let lines = input.lines().collect::<Vec<&str>>();

    let preproccess: Vec<_> = lines
        .iter()
        .map(|l| l.split(" ").collect::<Vec<&str>>())
        .collect();

    let mut token_vector: Vec<Token> = vec![];
    let mut cached_val: u32 = 0;
    preproccess.iter().for_each(|t| {
        t.iter().for_each(|tk| match tk {
            &"Game" => token_vector.push(Token::Game),
            &"red" => token_vector.push(Token::Red(cached_val)),
            &"green" => token_vector.push(Token::Green(cached_val)),
            &"blue" => token_vector.push(Token::Blue(cached_val)),
            &"red;" => {
                token_vector.push(Token::Red(cached_val));
                token_vector.push(Token::Semicolon);
            }
            &"green;" => {
                token_vector.push(Token::Green(cached_val));
                token_vector.push(Token::Semicolon);
            }
            &"blue;" => {
                token_vector.push(Token::Blue(cached_val));
                token_vector.push(Token::Semicolon);
            }
            &"red," => {
                token_vector.push(Token::Red(cached_val));
            }
            &"green," => {
                token_vector.push(Token::Green(cached_val));
            }
            &"blue," => {
                token_vector.push(Token::Blue(cached_val));
            }
            _ => match tk.ends_with(':') {
                true => token_vector.push(Token::Id(
                    tk.strip_suffix(':').unwrap().parse::<u32>().unwrap(),
                )),
                false => {
                    cached_val = tk.parse::<u32>().unwrap();
                }
            },
        })
    });

    // loop through token_vector

    //  println!("{token_vector:?}");

    // New game always begins with Game token
    // New pulls seperated by Token::Semicolon

    // group tokens together by Game

    let mut all_instances: Vec<GameInstance> = vec![];

    token_vector.iter().for_each(|t| {
        //     println!("{t:?}");
        match t {
            Token::Game => {
                all_instances.push(GameInstance {
                    id: 0,
                    game: vec![],
                });
            }
            Token::Red(n) => {
                let idx = all_instances.len() - 1usize;
                if all_instances[idx].game.is_empty() {
                    all_instances[idx].game.push(CubeGame::new());
                    all_instances[idx].game[0].red = Some(*n);
                } else {
                    let game_idx = all_instances[idx].game.len() - 1usize;
                    all_instances[idx].game[game_idx].red = Some(*n);
                }
            }
            Token::Green(n) => {
                let idx = all_instances.len() - 1usize;
                if all_instances[idx].game.is_empty() {
                    all_instances[idx].game.push(CubeGame::new());
                    all_instances[idx].game[0].green = Some(*n);
                } else {
                    let game_idx = all_instances[idx].game.len() - 1usize;
                    all_instances[idx].game[game_idx].green = Some(*n);
                }
            }
            Token::Blue(n) => {
                let idx = all_instances.len() - 1usize;
                if all_instances[idx].game.is_empty() {
                    all_instances[idx].game.push(CubeGame::new());
                    all_instances[idx].game[0].blue = Some(*n);
                } else {
                    let game_idx = all_instances[idx].game.len() - 1usize;
                    all_instances[idx].game[game_idx].blue = Some(*n);
                }
            }
            Token::Id(n) => {
                let len = all_instances.len();
                all_instances[len - 1usize].id = *n;
            }
            Token::Semicolon => {
                let idx = all_instances.len() - 1usize;
                all_instances[idx].game.push(CubeGame::new());
            }
        }
    });

    println!("{:#?}", &all_instances);

    let sum: u32 = all_instances
        .iter()
        .map(|i| if i.validate() == true { i.id } else { 0 })
        .sum();

    println!("{sum}");

    let power_sum: u32 = all_instances.iter().fold(0, |acc, x| x.get_power() + acc);
    println!("{power_sum}");
    Ok(())
}
