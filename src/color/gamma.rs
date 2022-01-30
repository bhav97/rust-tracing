use crate::color::rgb::RgbColor;
use crate::color::filter::Filter;

pub struct GammaCorrection {
    n: f64
}

impl GammaCorrection {
    pub fn new(y: f64) -> Self {
        GammaCorrection {
            n: 1f64 / y as f64
        }
    }

    // calculate nth root of value using newton's method
    fn nth_root_calc(&self, value: f64) -> f64 {
        let Ai = 1f64 - 1f64 / self.n as f64;
        let Bi = value / self.n as f64;
        
        // let x0 = 1;
        let x1 = Ai + Bi; // 1st iteration
        Ai*x1 + Bi*x1.powf(1f64 - self.n) // 2nd iteration
    }
}

impl Filter for GammaCorrection {
    fn apply_filter(&self, render: &mut Vec<RgbColor>) {
        for iter in 0..render.len() {
            render[iter].x = self.nth_root_calc(render[iter].x);
            render[iter].y = self.nth_root_calc(render[iter].y);
            render[iter].z = self.nth_root_calc(render[iter].z);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn name() {
        unimplemented!();
    }
}
