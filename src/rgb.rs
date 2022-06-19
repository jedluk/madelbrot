use std::ops;
use std::fmt;

pub struct RGB {
    r: f32,
    g: f32,
    b: f32,
}

impl RGB {
    pub fn new(r: f32, g: f32, b: f32) -> RGB {
        RGB {
            r,
            g,
            b,
        }
    }
}

impl ops::Sub<RGB> for RGB {
    type Output = RGB;

    fn sub(self, _rhs: RGB) -> RGB {
        RGB {
            r: self.r - _rhs.r,
            g: self.g - _rhs.g,
            b: self.b - _rhs.b,
        }
    }
}

impl fmt::Display for RGB {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "r:{} g: {}, b: {}", self.r, self.g, self.b)
    }
}