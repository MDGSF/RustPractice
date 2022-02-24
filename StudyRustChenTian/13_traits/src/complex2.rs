pub trait MyAdd {
    type Rhs;
    type Output;
    fn add(self, rhs: Self::Rhs) -> Self::Output;
}

#[derive(Debug)]
struct Complex {
    real: f64,
    imagine: f64,
}

impl Complex {
    pub fn new(real: f64, imagine: f64) -> Self {
        Self { real, imagine }
    }
}

impl MyAdd for &Complex {
    type Rhs = Self;
    type Output = Complex;
    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let imagine = self.imagine + rhs.imagine;
        Complex::new(real, imagine)
    }
}

// 会报错，和上面那个冲突了
// impl MyAdd for &Complex {
//     type Rhs = f64;
//     type Output = Complex;
//     fn add(self, rhs: f64) -> Self::Output {
//         let real = self.real + rhs;
//         Complex::new(real, self.imagine)
//     }
// }

fn main() {}
