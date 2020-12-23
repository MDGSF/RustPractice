#[derive(Debug)]
struct City {
  name: String,
  years: Vec<u32>,
  populations: Vec<u32>,
}

impl City {
  fn new(name: &str, years: Vec<u32>, populations: Vec<u32>) -> Self {
    Self {
      name: name.to_string(),
      years,
      populations,
    }
  }

  fn city_data<F>(&mut self, mut f: F)
  where
    F: FnMut(&mut Vec<u32>, &mut Vec<u32>),
  {
    f(&mut self.years, &mut self.populations);
  }
}

fn main() {
  let years = vec![
    1372, 1834, 1851, 1881, 1897, 1925, 1959, 1989, 2000, 2005, 2010, 2020,
  ];
  let populations = vec![
    3_250, 15_300, 24_000, 45_900, 58_800, 119_800, 283_071, 478_974, 400_378, 401_694, 406_703,
    437_619,
  ];

  let mut tallinn = City::new("Tallinn", years, populations);

  tallinn.city_data(|city_years, city_populations| {
    let new_vec = city_years
      .into_iter()
      .zip(city_populations.into_iter())
      .take(5)
      .collect::<Vec<(_, _)>>();
    println!("{:?}", new_vec);
  });

  tallinn.city_data(|x, y| {
    x.push(2030);
    y.push(500_000);
  });

  tallinn.city_data(|x, y| {
    let position_option = x.iter().position(|x| *x == 1834);
    if let Some(position) = position_option {
      println!(
        "Going to delete {} at position {:?} now.",
        x[position], position
      );
      x.remove(position);
      y.remove(position);
    }
  });

  println!("Years left are {:?}", tallinn.years);
  println!("Populations left are {:?}", tallinn.populations);
}
