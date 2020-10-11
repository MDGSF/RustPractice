// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
  fn from(s: u64) -> Self {
    Duration(s as f64)
  }
}

pub trait Planet {
  fn years_during(d: &Duration) -> f64;
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
  fn years_during(d: &Duration) -> f64 {
    d.0 / ORBITAL_PERIOD_MERCURY_SEC
  }
}
impl Planet for Venus {
  fn years_during(d: &Duration) -> f64 {
    d.0 / ORBITAL_PERIOD_VENUS_SEC
  }
}
impl Planet for Earth {
  fn years_during(d: &Duration) -> f64 {
    d.0 / ORBITAL_PERIOD_EARTH_SEC
  }
}
impl Planet for Mars {
  fn years_during(d: &Duration) -> f64 {
    d.0 / ORBITAL_PERIOD_MARS_SEC
  }
}
impl Planet for Jupiter {
  fn years_during(d: &Duration) -> f64 {
    d.0 / ORBITAL_PERIOD_JUPITER_SEC
  }
}
impl Planet for Saturn {
  fn years_during(d: &Duration) -> f64 {
    d.0 / ORBITAL_PERIOD_SATURN_SEC
  }
}
impl Planet for Uranus {
  fn years_during(d: &Duration) -> f64 {
    d.0 / ORBITAL_PERIOD_URANUS_SEC
  }
}
impl Planet for Neptune {
  fn years_during(d: &Duration) -> f64 {
    d.0 / ORBITAL_PERIOD_NEPTUNE_SEC
  }
}

const ORBITAL_PERIOD_MERCURY_YEAR: f64 = 0.2408467;
const ORBITAL_PERIOD_VENUS_YEAR: f64 = 0.61519726;
const ORBITAL_PERIOD_EARTH_YEAR: f64 = 1.0;
const ORBITAL_PERIOD_MARS_YEAR: f64 = 1.8808158;
const ORBITAL_PERIOD_JUPITER_YEAR: f64 = 11.862615;
const ORBITAL_PERIOD_SATURN_YEAR: f64 = 29.447498;
const ORBITAL_PERIOD_URANUS_YEAR: f64 = 84.016846;
const ORBITAL_PERIOD_NEPTUNE_YEAR: f64 = 164.79132;

const ORBITAL_PERIOD_EARTH_SEC: f64 = 31557600_f64;
const ORBITAL_PERIOD_MERCURY_SEC: f64 =
  ORBITAL_PERIOD_EARTH_SEC * ORBITAL_PERIOD_MERCURY_YEAR / ORBITAL_PERIOD_EARTH_YEAR;
const ORBITAL_PERIOD_VENUS_SEC: f64 =
  ORBITAL_PERIOD_EARTH_SEC * ORBITAL_PERIOD_VENUS_YEAR / ORBITAL_PERIOD_EARTH_YEAR;
const ORBITAL_PERIOD_MARS_SEC: f64 =
  ORBITAL_PERIOD_EARTH_SEC * ORBITAL_PERIOD_MARS_YEAR / ORBITAL_PERIOD_EARTH_YEAR;
const ORBITAL_PERIOD_JUPITER_SEC: f64 =
  ORBITAL_PERIOD_EARTH_SEC * ORBITAL_PERIOD_JUPITER_YEAR / ORBITAL_PERIOD_EARTH_YEAR;
const ORBITAL_PERIOD_SATURN_SEC: f64 =
  ORBITAL_PERIOD_EARTH_SEC * ORBITAL_PERIOD_SATURN_YEAR / ORBITAL_PERIOD_EARTH_YEAR;
const ORBITAL_PERIOD_URANUS_SEC: f64 =
  ORBITAL_PERIOD_EARTH_SEC * ORBITAL_PERIOD_URANUS_YEAR / ORBITAL_PERIOD_EARTH_YEAR;
const ORBITAL_PERIOD_NEPTUNE_SEC: f64 =
  ORBITAL_PERIOD_EARTH_SEC * ORBITAL_PERIOD_NEPTUNE_YEAR / ORBITAL_PERIOD_EARTH_YEAR;
