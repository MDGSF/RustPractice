use std::convert::From;
use std::fmt;
use std::str::FromStr;
use std::string::String;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Color {
    Red,
    Yellow,
    Blue,
}

impl Color {
    pub fn to_fg_str(&self) -> &str {
        match *self {
            Color::Red => "31",
            Color::Yellow => "33",
            Color::Blue => "34",
        }
    }
    pub fn to_bg_str(&self) -> &str {
        match *self {
            Color::Red => "41",
            Color::Yellow => "43",
            Color::Blue => "44",
        }
    }
}

impl<'a> From<&'a str> for Color {
    fn from(src: &str) -> Color {
        src.parse().unwrap_or(Color::Red)
    }
}

impl From<String> for Color {
    fn from(src: String) -> Color {
        src.parse().unwrap_or(Color::Red)
    }
}

impl FromStr for Color {
    type Err = ();
    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let src = src.to_lowercase();
        match src.as_ref() {
            "red" => Ok(Color::Red),
            "yellow" => Ok(Color::Yellow),
            "blue" => Ok(Color::Blue),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ColoredString {
    input: String,
    fgcolor: Option<Color>,
    bgcolor: Option<Color>,
}

impl ColoredString {
    fn compute_style(&self) -> String {
        let mut res = String::from("\x1B[");
        let mut has_wrote = false;
        if let Some(ref bgcolor) = self.bgcolor {
            if has_wrote {
                res.push(';');
            }
            res.push_str(bgcolor.to_bg_str());
            has_wrote = true;
        }
        if let Some(ref fgcolor) = self.fgcolor {
            if has_wrote {
                res.push(';');
            }
            res.push_str(fgcolor.to_fg_str());
        }
        res.push('m');
        res
    }
}

impl Default for ColoredString {
    fn default() -> Self {
        ColoredString {
            input: String::default(),
            fgcolor: None,
            bgcolor: None,
        }
    }
}

impl fmt::Display for ColoredString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let input = &self.input.clone();
        f.write_str(&self.compute_style())?;
        f.write_str(input)?;
        f.write_str("\x1B[0m")?;
        Ok(())
    }
}

trait Colorize {
    fn red(self) -> ColoredString;
    fn yellow(self) -> ColoredString;
    fn blue(self) -> ColoredString;
    fn color<S: Into<Color>>(self, color: S) -> ColoredString;
    fn on_red(self) -> ColoredString;
    fn on_yellow(self) -> ColoredString;
    fn on_blue(self) -> ColoredString;
    fn on_color<S: Into<Color>>(self, color: S) -> ColoredString;
}

impl Colorize for ColoredString {
    fn red(self) -> ColoredString {
        self.color(Color::Red)
    }
    fn yellow(self) -> ColoredString {
        self.color(Color::Yellow)
    }
    fn blue(self) -> ColoredString {
        self.color(Color::Blue)
    }
    fn color<S: Into<Color>>(self, color: S) -> ColoredString {
        ColoredString {
            fgcolor: Some(color.into()),
            ..self
        }
    }
    fn on_red(self) -> ColoredString {
        self.on_color(Color::Red)
    }
    fn on_yellow(self) -> ColoredString {
        self.on_color(Color::Yellow)
    }
    fn on_blue(self) -> ColoredString {
        self.on_color(Color::Blue)
    }
    fn on_color<S: Into<Color>>(self, color: S) -> ColoredString {
        ColoredString {
            bgcolor: Some(color.into()),
            ..self
        }
    }
}

impl<'a> Colorize for &'a str {
    fn red(self) -> ColoredString {
        self.color(Color::Red)
    }
    fn yellow(self) -> ColoredString {
        self.color(Color::Yellow)
    }
    fn blue(self) -> ColoredString {
        self.color(Color::Blue)
    }
    fn color<S: Into<Color>>(self, color: S) -> ColoredString {
        ColoredString {
            fgcolor: Some(color.into()),
            input: String::from(self),
            ..ColoredString::default()
        }
    }
    fn on_red(self) -> ColoredString {
        self.on_color(Color::Red)
    }
    fn on_yellow(self) -> ColoredString {
        self.on_color(Color::Yellow)
    }
    fn on_blue(self) -> ColoredString {
        self.on_color(Color::Blue)
    }
    fn on_color<S: Into<Color>>(self, color: S) -> ColoredString {
        ColoredString {
            bgcolor: Some(color.into()),
            input: String::from(self),
            ..ColoredString::default()
        }
    }
}

fn main() {
    let red = "red".red();
    println!("{}", red);

    let yellow = "yellow".yellow().on_blue();
    println!("{}", yellow);

    let blue = "blue".blue();
    println!("{}", blue);

    let red = "red".color("red");
    println!("{}", red);

    let yellow = "yellow".on_color("yellow");
    println!("{}", yellow);
}
