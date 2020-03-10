use std::collections::HashMap;

fn main() {
  let mut player_stats = HashMap::new();

  fn random_stat_buff() -> u8 {
    42
  }

  player_stats.entry("health").or_insert(100);

  player_stats
    .entry("defence")
    .or_insert_with(random_stat_buff);

  let stat = player_stats.entry("attack").or_insert(100);
  *stat += random_stat_buff();

  for (key, val) in &player_stats {
    println!("{}: {}", key, val);
  }
}
