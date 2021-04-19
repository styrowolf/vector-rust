
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    pub magnitude: f64,
    pub angle: f64,
}

impl std::ops::Add for Vector {
    type Output = Vector;
    fn add(self, rhs: Self) -> Self::Output {

        let x1 = self.magnitude * self.angle.cos();
        let y1 = self.magnitude * self.angle.sin();

        let x2 = rhs.magnitude * rhs.angle.cos();
        let y2 = rhs.magnitude * rhs.angle.sin();

        let x_comp = x1 + x2;
        let y_comp = y1 + y2;

        let magnitude = f64::hypot(x_comp, y_comp);
        let angle = angle(x_comp, y_comp);

        Vector {
            magnitude: magnitude,
            angle: angle,
        }
    }
}

impl std::ops::AddAssign for Vector {
    fn add_assign(&mut self, rhs: Self)  {
        *self = self.clone() + rhs;
    }
}

impl std::ops::Sub for Vector {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self::Output {
        self + rhs.recip()
    }
}

impl std::ops::SubAssign for Vector {
    fn sub_assign(&mut self, rhs: Self) {
        *self = self.clone() - rhs;
    }
}

impl Default for Vector {
    fn default() -> Self {
        Vector::zero()
    }
}

impl Vector {

    pub fn new_with_degrees(m: f64, d: f64) -> Self {
        Vector {
            magnitude: m,
            angle: d.to_radians() % (2.0 * std::f64::consts::PI),
        }
    }

    pub fn new_with_radians(m: f64, r: f64) -> Self {
        Vector {
            magnitude: m,
            angle: r % (2.0 * std::f64::consts::PI),
        }
    }

    pub fn recip(&self) -> Self {
        Vector {
            magnitude: self.magnitude,
            angle: self.angle + std::f64::consts::PI,
        }
    }

    pub fn get_x_component(&self) -> Self {
        let x = self.magnitude * self.angle.cos();
        Vector {
            magnitude: x.abs(),
            angle: match x.is_sign_positive() {
                true => 0.0,
                false => std::f64::consts::PI,
            },
        }
    }

    pub fn get_y_component(&self) -> Self {
        let y = self.magnitude * self.angle.sin();
        Vector {
            magnitude: y.abs(),
            angle: match y.is_sign_positive() {
                true => std::f64::consts::PI / 2.0,
                false => std::f64::consts::PI * 3.0 / 2.0,
            },
        }
    }

    pub fn get_components(&self) -> (Self, Self) {
        (Vector::get_x_component(self), Vector::get_y_component(self))
    }

    pub fn zero() -> Self {
        Vector::new_with_radians(0.0, 0.0)
    }
}

pub fn angle(x: f64, y: f64) -> f64 {
    let is_x_pos = x.is_sign_positive();
    let is_y_pos = y.is_sign_positive();

    match (is_x_pos, is_y_pos) {
        (true, true) => { (y/x).atan() }, 
        (false, true) => { (y/x).atan() + std::f64::consts::PI }, 
        (false, false) => { (y/x).atan() + std::f64::consts::PI }, 
        (true, false) => { (y/x).atan() + 2.0 * std::f64::consts::PI }, 
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
