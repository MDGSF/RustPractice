use std::fmt;

struct ColoredString {
    input: String,
    fgcolor: String,
    bgcolor: String,
}

impl Default for ColoredString {
    fn default() -> Self {
        ColoredString {
            input: String::default(),
            fgcolor: String::default(),
            bgcolor: String::default(),
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

impl ColoredString {
    fn compute_style(&self) -> String {
        let mut res = String::from("\x1B[");
        let mut has_wrote = false;
        if !self.bgcolor.is_empty() {
            res.push_str(&self.bgcolor);
            has_wrote = true;
        }
        if !self.fgcolor.is_empty() {
            if has_wrote {
                res.push(';');
            }
            res.push_str(&self.fgcolor);
        }
        res.push('m');
        res
    }
}

trait Colorize {
    fn red(self) -> ColoredString;
    fn on_yellow(self) -> ColoredString;
}

impl Colorize for ColoredString {
    fn red(self) -> ColoredString {
        ColoredString {
            fgcolor: String::from("31"),
            ..self
        }
    }

    fn on_yellow(self) -> ColoredString {
        ColoredString {
            bgcolor: String::from("43"),
            ..self
        }
    }
}

impl<'a> Colorize for &'a str {
    fn red(self) -> ColoredString {
        ColoredString {
            fgcolor: String::from("31"),
            input: String::from(self),
            ..ColoredString::default()
        }
    }

    fn on_yellow(self) -> ColoredString {
        ColoredString {
            bgcolor: String::from("43"),
            input: String::from(self),
            ..ColoredString::default()
        }
    }
}

fn main() {
    let hi = "Hello".red().on_yellow();
    println!("{}", hi);
    let hi = "Hello".red();
    println!("{}", hi);
    let hi = "Hello".on_yellow();
    println!("{}", hi);
    let hi = "Hello".on_yellow().red();
    println!("{}", hi);
}
