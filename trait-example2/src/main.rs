use std::ops::Add;

#[derive(Debug)]
struct Complex {
    real:f64,
    imagine: f64,
}

impl Complex {
    fn new(real: f64, imagine: f64) -> Self {
        Self {
            real,
            imagine,
        }
    }
}

impl Add for &Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        let real = self.real + rhs.real;
        let imagine = self.imagine + rhs.imagine;
        Complex::new(real, imagine)
    }
}

impl Add<f64> for &Complex {
    type Output = Complex;

    fn add(self, rhs: f64) -> Self::Output {
        let real = self.real + rhs;
        let imagine = self.imagine;
        Complex::new(real, imagine)
    }
}



fn main() {
    let c1 = Complex::new(1.0, 2.0);
    let c2 = Complex::new(3.0, 4.0);
    // println!("{:?}", c1 + c2);
    println!("{:?}", &c1 + &c2);
    println!("{:?}", &c1 + &c2);
    println!("{:?}", &c1 + 100.0);
}
