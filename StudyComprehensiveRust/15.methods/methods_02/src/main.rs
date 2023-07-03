#[derive(Debug)]
struct Race {
    name: String,
    laps: Vec<i32>,
}

impl Race {
    fn new(name: &str) -> Race {
        Race {
            name: String::from(name),
            laps: Vec::new(),
        }
    }

    fn add_lap(&mut self, lap: i32) {
        self.laps.push(lap);
    }

    fn print_lap(&self) {
        println!("Recorded {} laps for {}:", self.laps.len(), self.name);
        for (idx, lap) in self.laps.iter().enumerate() {
            println!("lap {idx}: {lap} sec");
        }
    }

    fn finish(self) {
        let total = self.laps.iter().sum::<i32>();
        println!("Race {} is finished, total lap time {}", self.name, total);
    }
}

fn main() {
    let mut race = Race::new("Monaco Grand Prix");
    race.add_lap(70);
    race.add_lap(80);
    race.print_lap();
    race.add_lap(71);
    race.print_lap();
    race.finish();
}
