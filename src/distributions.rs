use crate::types::float;
use crate::types::PI;
use crate::types::E;
pub trait Distribution {
    fn get(self, x: float) -> float;
}
pub struct NormalDistribution {
    sigma: float,
    mu: float,
    c0: float,
    c1: float,
}
impl NormalDistribution {
    pub fn new(sigma: float, mu: float) -> Self {
        Self {
            sigma : sigma,
            mu : mu,
            c0 : (1.0/(float::sqrt(2.0*PI*sigma*sigma))),
            c1 : 2.0*sigma*sigma,
        }

    }
}
impl Distribution for NormalDistribution {
    fn get(self, x: float) -> float {
        self.c0*float::powf(E, -((x-self.mu)*(x-self.mu))/self.c1)
    }
}
