use std::fmt::Debug;

#[derive(Copy, Clone, Debug)]
pub struct MagneticMoment {
    pub x_vec: f64,
    pub y_vec: f64,
    pub z_vec: f64,
}

impl MagneticMoment {
    pub fn new(x_vec: f64, y_vec: f64, z_vec: f64) -> MagneticMoment {
        MagneticMoment {x_vec, y_vec, z_vec}
    }

    pub fn init() -> MagneticMoment {
        MagneticMoment {x_vec: 0f64, y_vec: 0f64, z_vec: 1f64}
    }

    pub fn length(self) -> f64 {
        f64::sqrt(
            self.x_vec*self.x_vec +
            self.y_vec * self.y_vec + 
            self.z_vec * self.z_vec 
        )
    }
}

#[derive(Debug)]
pub struct Node {
    pub moment: MagneticMoment,
}

impl Node {
    pub fn new() -> Node {
        let moment = MagneticMoment::init();
        Node{moment}
    }

}

impl Clone for Node {
    fn clone(&self) -> Self {
        let moment = MagneticMoment {
            x_vec: self.moment.x_vec,
            y_vec: self.moment.y_vec,
            z_vec: self.moment.y_vec
        };
        Node { moment }
    }
}
