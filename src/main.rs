use std::io::{self, Write};
use std::{thread, time};

extern crate rand;
use rand::Rng;

#[derive(Clone,Eq,PartialEq,PartialOrd)]
struct Individual {
    chromosome: String,
    fitness: u32
}

impl Individual {
    // Into<T>: take all arguments that can be converted into T
    fn new<T: Into<String>>(s: T) -> Individual {
        Individual { chromosome: s.into(), fitness: 0 }
    }

    fn new_rand(n: usize) -> Individual {
        let mut rng = rand::thread_rng();
        let c: String = (0..n)
            .map(|_| (rng.gen_range(32, 127) as u8) as char)
            .collect();
        Individual::new(c)
    }

    fn mutate(&mut self) {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0, self.chromosome.len());
        self.chromosome = self.chromosome
            .chars()
            .enumerate()
            .map(|(i, a)| if i == idx { (rng.gen_range(32, 127) as u8) as char } else { a })
            .collect();
    } 
}

struct Population<'a> {
    individuals: Vec<Individual>,
    f: &'a dyn Fn(&Individual) -> u32
}

impl<'a> Population<'a> {
    fn new(n: usize, k: usize, f: &'a dyn Fn(&Individual) -> u32) -> Population<'a> {
        let mut individuals: Vec<Individual> = Vec::with_capacity(n);
        for _ in 0..n {
            individuals.push(Individual::new_rand(k));
        }
        Population { individuals: individuals, f: f }
    }

    fn fitness(&mut self) {
        for individual in &mut self.individuals {
            individual.fitness = (self.f)(&individual);
        }
        self.individuals.sort_by(|a,b| b.fitness.cmp(&a.fitness));
    }

    fn crossover(&self, a: &Individual, b: &Individual) -> (Individual, Individual) {
        let mut desc_a: String = String::with_capacity(a.chromosome.len());
        let mut desc_b: String = String::with_capacity(b.chromosome.len());

        let mut rng = rand::thread_rng();
        let crossover_point = rng.gen_range(1, a.chromosome.len()-1);
        
        for (x,y) in a.chromosome.chars().zip(b.chromosome.chars()).take(crossover_point) {
            desc_a.push(y);
            desc_b.push(x);
        }
        for (x,y) in a.chromosome.chars().zip(b.chromosome.chars()).skip(crossover_point) {
            desc_a.push(x);
            desc_b.push(y);
        }

        (Individual::new(desc_a), Individual::new(desc_b))
    }
}

fn hamming(a: &String, b: &String) -> u32 {
    assert!(a.len() == b.len());
    a.chars()
        .zip(b.chars())
        .map(|(x,y)| if x == y {0} else {1})
        .sum()
}

fn main() {
    let mut rng = rand::thread_rng();

    let solution = String::from("Happy Birthday, Old Fox!");

    let del: String = (0..solution.len()).map(|_| '\x08').collect();

    let n_of_individuals = 100;
    let generations = 500;
    let mutation = 0.18;
    let f = |individual: &Individual| -> u32 {
        (solution.len() as u32) - hamming(&solution, &individual.chromosome)
    };


    let mut population = Population::new(n_of_individuals, solution.len(), &f);
    population.fitness();

    for _i in 0..generations {
        // best at this generation
        let best = &population.individuals[0];
        //println!("Gen. {}: {} {}", i, best.chromosome, best.fitness);
        
        print!("{}{}", best.chromosome, del);
        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(25));

        if best.chromosome == solution {
            println!("{}", best.chromosome);
            break;
        }

        let mut np: Vec<Individual> = Vec::with_capacity(n_of_individuals);
        for _ in 0..n_of_individuals {
            // random selection parents then crossover
            let a = rng.gen_range(0, n_of_individuals);
            let b = rng.gen_range(0, n_of_individuals);
            let (mut d_a, mut d_b) = population.crossover(&population.individuals[a], &population.individuals[b]);
            
            // are these descendants getting mutated?
            if rng.gen::<f32>() >= mutation { d_a.mutate(); }
            if rng.gen::<f32>() >= mutation { d_b.mutate(); }

            // add to population
            np.push(d_a);
            np.push(d_b);
        }
        population.individuals = np.clone();
        population.fitness();
    }
}
