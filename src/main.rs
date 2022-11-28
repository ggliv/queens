use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use std::io::Write;

fn main() {
    print!("n?: ");
    std::io::stdout().flush().expect("Failed to flush stdout.");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Could not read from stdin.");
    match input.trim().parse::<usize>() {
        Ok(i) => {
            if i < 4 {
                println!("Input is too small. Must be greater than 3.");
                main();
                return;
            }

            println!("Solving...");
            let solution = solve(i);
            println!(
                "{}Solution found in {} generations.",
                solution.0.to_string(),
                solution.1
            );
        }
        Err(_) => {
            println!("Could not parse your input. Try again.");
            main();
        }
    }
}

fn solve(board_size: usize) -> (Board, usize) {
    let mut population: Vec<Board> = vec![Board::new_random(&board_size); board_size.pow(2) * 2];
    population.fill_with(|| Board::new_random(&board_size));

    let mut counter = 1usize;
    let mut consecutive_best = 0u8;

    let mut best: Board = population[0].clone();

    loop {
        // find best two boards and put them in the front of the population
        for i in 0..population.len() {
            if population[i].count_intersections() < population[0].count_intersections() {
                population.swap(0, 1);
                population.swap(i, 1);
            } else if population[i].count_intersections() < population[1].count_intersections() {
                population.swap(1, i);
            }
        }

        let seeds: (Board, Board) = (population[0].clone(), population[1].clone());

        // increase mutation aggro as population converges on local minimum
        if best == seeds.0 {
            consecutive_best = consecutive_best.saturating_add(1);
        }
        best = seeds.0.clone();

        // if best board is a solution, return it
        if seeds.0.count_intersections() == 0 {
            return (seeds.0, counter);
        }

        // repopulate
        for i in 0..population.len() {
            population[i] = crossover(&seeds).mutate(&consecutive_best);
        }

        counter += 1;
    }
}

fn crossover(seeds: &(Board, Board)) -> Board {
    let size = seeds.0.dimension;
    let mut working_child: Vec<Option<usize>> = vec![None; size];
    let mut unacceptable_values: Vec<usize> = Vec::with_capacity(size);
    for i in 0..size {
        if seeds.0.queens.get(i).unwrap() == seeds.1.queens.get(i).unwrap() {
            working_child[i] = Some(*seeds.0.queens.get(i).unwrap());
            unacceptable_values.push(*seeds.0.queens.get(i).unwrap());
        }
    }

    let mut acceptable_values = (0..size)
        .filter(|e| !unacceptable_values.contains(e))
        .collect::<Vec<usize>>();
    acceptable_values.shuffle(&mut thread_rng());

    let mut child: Vec<usize> = Vec::with_capacity(size);
    for v in working_child {
        if v.is_some() {
            child.push(v.unwrap());
        } else {
            child.push(acceptable_values.pop().unwrap());
        }
    }

    Board {
        dimension: size,
        queens: child,
    }
}

#[derive(Clone, PartialEq, Debug)]
struct Board {
    dimension: usize,
    queens: Vec<usize>,
}

impl Board {
    fn new_random(dimensions: &usize) -> Board {
        let mut working_queens = (0..*dimensions).collect::<Vec<usize>>();
        working_queens.shuffle(&mut thread_rng());

        Board {
            dimension: *dimensions,
            queens: working_queens,
        }
    }

    fn count_intersections(&self) -> usize {
        let mut count = 0;
        for (i, queen) in self.queens.iter().enumerate() {
            for (j, other) in self.queens.iter().enumerate() {
                if i.abs_diff(j) == queen.abs_diff(*other) {
                    count += 1;
                }
            }
            count -= 1;
        }

        count
    }

    fn mutate(mut self, aggro: &u8) -> Self {
        if thread_rng().gen_bool(0.5 + (*aggro as f64) / (2.0 * u8::MAX as f64)) {
            let mut rand_one = thread_rng().gen_range(0..self.dimension);
            let rand_two = thread_rng().gen_range(0..self.dimension);
            while rand_one == rand_two {
                rand_one = thread_rng().gen_range(0..self.dimension);
            }

            self.queens.swap(rand_one, rand_two);
        }

        self
    }

    fn to_string(&self) -> String {
        let mut result = String::new();
        for queen in &self.queens {
            for i in 0..self.dimension {
                if *queen == i {
                    result.push_str("Q ");
                } else {
                    result.push_str("- ");
                }
            }
            result += "\n";
        }

        result
    }
}
