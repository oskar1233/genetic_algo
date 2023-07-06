use rand::Rng;

const POP_SIZE: usize = 5;
const GENOME_LEN: usize = 140;
const MUTATION_RATE: f64 = 0.003;

trait Gene {
    fn new() -> Self;
    fn mutate(&mut self);
}

#[derive(Debug, Copy, Clone)]
struct Unit {
    legacy: bool,
}

impl Gene for Unit {
    fn new() -> Self {
        Self { legacy: false }
    }
    fn mutate(&mut self) {
        self.legacy = !self.legacy;
    }
}

impl Unit {
    fn from_legacy(legacy: bool) -> Self {
        Unit { legacy }
    }
}

pub struct Genetics {
    population: Vec<[Unit; GENOME_LEN]>,
    generation: usize,
}

impl Genetics {
    pub fn start() -> Self {
        let mut population: Vec<[Unit; GENOME_LEN]> = Vec::new();
        let mut rng = rand::thread_rng();

        for _ in 0..POP_SIZE {
            let genome: [Unit; GENOME_LEN] = (0..GENOME_LEN)
                .map(|_| Unit::from_legacy(rng.gen::<bool>()))
                .collect::<Vec<Unit>>()
                .try_into()
                .unwrap();

            population.push(genome);
        }

        Self {
            population,
            generation: 0,
        }
    }

    pub fn evolve(&mut self) -> () {
        // Evaluate fitness of each individual
        let mut fitness_scores: Vec<(usize, f64)> = self
            .population
            .iter()
            .enumerate()
            .map(|(i, genome)| (i, self.fitness(genome)))
            .collect();
        fitness_scores.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

        // Output best individual of this generation
        if self.generation % 10 == 0 {
            println!("Generation {}: {:?}", self.generation, fitness_scores[0]);
        }

        // Select parents and generate offspring
        let mut offspring: Vec<[Unit; GENOME_LEN]> = Vec::new();
        for _ in 0..POP_SIZE {
            // let parent_a = &self.population[self.roulette_wheel_select(&fitness_scores)];
            // let parent_b = &self.population[self.roulette_wheel_select(&fitness_scores)];
            let parent_a = &self.population[fitness_scores[0].0];
            let parent_b = &self.population[fitness_scores[1].0];
            let child = self.crossover(parent_a, parent_b);
            offspring.push(self.mutate(child));
        }

        // Replace old population with new offspring
        self.population = offspring;
        self.generation += 1;
    }

    pub fn get_last_population(&mut self) -> Option<(usize, Vec<bool>)> {
        match self.population.last() {
            Some(population) => Some((self.generation, population.map(|u| u.legacy).to_vec())),
            None => None,
        }
    }

    // Evaluate fitness of genome
    fn fitness(&self, genome: &[Unit; GENOME_LEN]) -> f64 {
        let true_ratio = genome.iter().filter(|&b| b.legacy).count() as f64 / GENOME_LEN as f64;
        // println!("tr: {}", true_ratio);
        // 1.0 - (true_ratio - 0.5).abs()
        true_ratio
    }

    // Select an individual from the population using roulette wheel selection
    fn roulette_wheel_select(&self, fitness_scores: &[(usize, f64)]) -> usize {
        let total_fitness: f64 = fitness_scores.iter().map(|(_, f)| *f).sum();
        let mut r = rand::thread_rng().gen_range(0.0..total_fitness);
        for (i, score) in fitness_scores.iter() {
            if r < *score {
                return *i;
            }
            r -= score;
        }
        fitness_scores[0].0
    }

    // Generate a new genome by crossover
    fn crossover(
        &self,
        parent_a: &[Unit; GENOME_LEN],
        parent_b: &[Unit; GENOME_LEN],
    ) -> [Unit; GENOME_LEN] {
        let crossover_point = rand::thread_rng().gen_range(0..GENOME_LEN);
        let mut child = [Unit::new(); GENOME_LEN];
        for i in 0..GENOME_LEN {
            child[i] = if i < crossover_point {
                parent_a[i]
            } else {
                parent_b[i]
            };
        }
        child
    }

    // Mutate genome by flipping random bits
    fn mutate(&self, genome: [Unit; GENOME_LEN]) -> [Unit; GENOME_LEN] {
        let mut rng = rand::thread_rng();
        let mut mutated_genome = genome;
        for i in 0..GENOME_LEN {
            if rng.gen::<f64>() < MUTATION_RATE {
                mutated_genome[i].mutate();
            }
        }
        mutated_genome
    }
}
