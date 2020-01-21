use std::fmt::{self, Display, Formatter};

struct City {
  name: &'static str,
  // latitude
  lat: f32,
  // Longitude
  lon: f32,
}

impl Display for City {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
    let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
    write!(
      f,
      "{}: {:.3}°{} {:.3}°{}",
      self.name,
      self.lat.abs(),
      lat_c,
      self.lon.abs(),
      lon_c
    )
  }
}

#[derive(Debug)]
struct Color {
  red: u8,
  green: u8,
  blue: u8,
}

impl Display for Color {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "RGB ({}, {}, {}) 0x{:02X}{:02X}{:02X}",
      self.red, self.green, self.blue, self.red, self.green, self.blue
    )
  }
}

fn main() {
  for city in [
    City {
      name: "Dublin",
      lat: 53.347778,
      lon: -6.259722,
    },
    City {
      name: "Oslo",
      lat: 59.95,
      lon: 10.75,
    },
    City {
      name: "Vancouver",
      lat: 49.25,
      lon: -123.1,
    },
  ]
  .iter()
  {
    println!("{}", *city);
  }

  let colorarray = [
    Color {
      red: 128,
      green: 255,
      blue: 90,
    },
    Color {
      red: 0,
      green: 3,
      blue: 254,
    },
    Color {
      red: 0,
      green: 0,
      blue: 0,
    },
  ];
  for color in colorarray.iter() {
    println!("{:?}", *color);
  }
  for color in colorarray.iter() {
    println!("{}", *color);
  }
}
