use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl FromStr for Point {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords = s
            .trim_matches(|p| p == '{' || p == '}')
            .split(",")
            .collect::<Vec<&str>>();
        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;
        Ok(Point {
            x: x_fromstr,
            y: y_fromstr,
        })
    }
}

fn main() {
    let p = Point::from_str("{1,2}");
    assert_eq!(p.unwrap(), Point { x: 1, y: 2 });

    let p = Point::from_str("{3,u}");
    println!("{:?}", p);
}
