extern crate chrono;
extern crate rand;

use rand::distributions::Standard;
use rand::prelude::*;
use std::error::Error;
use std::fmt;

// i32 instead of usize to allow for negatives
#[derive(Debug, Clone)]
enum SimError {
    ImpossibleMove {
        curr_x: i32,
        curr_y: i32,
        tried_x: i32,
        tried_y: i32,
    },
}

impl SimError {
    fn new_impossible_move(curr_x: i32, curr_y: i32, tried_x: i32, tried_y: i32) -> SimError {
        SimError::ImpossibleMove {
            curr_x,
            curr_y,
            tried_x,
            tried_y,
        }
    }
}

impl fmt::Display for SimError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SimError::ImpossibleMove {
                curr_x,
                curr_y,
                tried_x,
                tried_y,
            } => write!(
                f,
                "Move not possible: ({}, {}) -> ({}, {})",
                curr_x, curr_y, tried_x, tried_y
            ),
        }
    }
}

impl Error for SimError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[derive(Debug, Clone)]
enum Node {
    Empty,
    Particle,
}

#[derive(Debug, Clone)]
struct Board {
    empty_x: usize,
    empty_y: usize,
    nodes: Vec<Vec<Node>>,
}

// Represents the board with the empty spot as the focal point.
// While we are seeing the behavior of the particles as one is randomly chosen to move in,
// we can reverse the move of the empty spot to determine which particle was chosen and vice versa.
impl Board {
    fn new(x: usize, y: usize) -> Board {
        // 2 x 3 board for the assignment, unneccessary otherwise
        // TODO: add fields for tracking bounds and adjust move functions
        if x > 3 || y > 2 {
            panic!();
        }
        let mut vec = vec![
            vec![Node::Particle, Node::Particle, Node::Particle],
            vec![Node::Particle, Node::Particle, Node::Particle],
        ];
        vec[y][x] = Node::Empty;
        Board {
            empty_x: x,
            empty_y: y,
            nodes: vec,
        }
    }

    fn move_up(&mut self) -> Result<(), SimError> {
        if self.empty_y == 0 {
            return Err(SimError::new_impossible_move(
                self.empty_x as i32,
                self.empty_y as i32,
                self.empty_x as i32,
                self.empty_y as i32 - 1,
            ));
        }
        self.nodes.swap(0, 1);
        self.empty_y = 0;
        Ok(())
    }

    // while move_up and move_down are basically the same,
    // done for consistency and expressibility.
    fn move_down(&mut self) -> Result<(), SimError> {
        if self.empty_y == 1 {
            return Err(SimError::new_impossible_move(
                self.empty_x as i32,
                self.empty_y as i32,
                self.empty_x as i32,
                self.empty_y as i32 + 1,
            ));
        }
        self.nodes.swap(0, 1);
        self.empty_y = 1;
        Ok(())
    }

    fn move_left(&mut self) -> Result<(), SimError> {
        if self.empty_x == 0 {
            return Err(SimError::new_impossible_move(
                self.empty_x as i32,
                self.empty_y as i32,
                self.empty_x as i32 - 1,
                self.empty_y as i32,
            ));
        }
        self.nodes[self.empty_y].swap(self.empty_x, self.empty_x - 1);
        self.empty_x -= 1;
        Ok(())
    }

    fn move_right(&mut self) -> Result<(), SimError> {
        if self.empty_x == 2 {
            return Err(SimError::new_impossible_move(
                self.empty_x as i32,
                self.empty_y as i32,
                self.empty_x as i32 + 1,
                self.empty_y as i32,
            ));
        }
        self.nodes[self.empty_y].swap(self.empty_x, self.empty_x + 1);
        self.empty_x += 1;
        Ok(())
    }

    fn current_position(&self) -> (usize, usize) {
        (self.empty_x, self.empty_y)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut board = String::new();
        for row in &self.nodes {
            for column in row {
                match column {
                    Node::Empty => board.push_str("[ ]"),
                    Node::Particle => board.push_str("[.]"),
                }
            }
            board.push_str("\n");
        }
        board.push_str(&format!(
            "Empty spot position: ({}, {})",
            self.empty_x, self.empty_y
        ));
        write!(f, "{}", board)
    }
}

//[0][1][2]
//[3][4][5]
#[derive(Default, Debug)]
struct Counter {
    iterations: usize,
    zero: usize,
    one: usize,
    two: usize,
    three: usize,
    four: usize,
    five: usize,
}

#[derive(Debug)]
struct CounterPercentages {
    zero: f64,
    one: f64,
    two: f64,
    three: f64,
    four: f64,
    five: f64,
}

impl CounterPercentages {
    fn new(prior: Counter) -> CounterPercentages {
        let percentage_zero = prior.zero as f64 / prior.iterations as f64;
        let percentage_one = prior.one as f64 / prior.iterations as f64;
        let percentage_two = prior.two as f64 / prior.iterations as f64;
        let percentage_three = prior.three as f64 / prior.iterations as f64;
        let percentage_four = prior.four as f64 / prior.iterations as f64;
        let percentage_five = prior.five as f64 / prior.iterations as f64;
        CounterPercentages {
            zero: percentage_zero,
            one: percentage_one,
            two: percentage_two,
            three: percentage_three,
            four: percentage_four,
            five: percentage_five,
        }
    }
}

impl fmt::Display for CounterPercentages {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "In zero: {}\nIn one: {}\nIn two: {}\nIn three:{}\nIn four: {}\nIn five: {}",
            self.zero, self.one, self.two, self.three, self.four, self.five
        )
    }
}

fn simulate(initial_x: usize, initial_y: usize, n: usize) {
    let mut counter: Counter = Default::default();
    counter.iterations = n;
    let mut board = Board::new(initial_x, initial_y);
    let mut rng = StdRng::from_entropy();
    for _i in 0..n {
        // println!("Iteration: {}", i);
        let mut success = false;
        while !success {
            let val: f64 = rng.sample(Standard);
            if val < 1.0 / 4.0 {
                match board.move_up() {
                    Ok(_) => success = true,
                    Err(_) => (),
                }
            } else if val < 2.0 / 4.0 {
                match board.move_down() {
                    Ok(_) => success = true,
                    Err(_) => (),
                }
            } else if val < 3.0 / 4.0 {
                match board.move_left() {
                    Ok(_) => success = true,
                    Err(_) => (),
                }
            } else if val < 1.0 {
                match board.move_right() {
                    Ok(_) => success = true,
                    Err(_) => (),
                }
            }
        }
        let current = board.current_position();
        // Only 6 possible spots
        match current {
            (0, 0) => counter.zero += 1,
            (1, 0) => counter.one += 1,
            (2, 0) => counter.two += 1,
            (0, 1) => counter.three += 1,
            (1, 1) => counter.four += 1,
            (2, 1) => counter.five += 1,
            (_, _) => unreachable!(),
        }
        // println!("{}", board);
    }
    let percentages = CounterPercentages::new(counter);
    println!("{}", percentages);
}

fn main() {
    let duration = chrono::Duration::span(|| simulate(0, 0, 100_000_000));
    println!("Executed in {}", duration);
}
