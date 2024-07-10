use colorous::Gradient;
use colorous::PLASMA;
use core::fmt::Write;

// A linear scale
// TODO histogram uses should I simplify here?
//
// use charts::ScaleLinear;
#[allow(non_snake_case)]
#[derive(Clone, Debug)]
pub struct ColorMap {
    domain: [f64; 2],
    m: f64,
    C: f64,
    gradient: Gradient,
}

impl Default for ColorMap {
    fn default() -> Self {
        Self {
            domain: [0.into(), 1.into()],
            m: 1.into(),
            C: 0.into(),
            gradient: PLASMA,
        }
    }
}

impl ColorMap {
    pub(crate) fn new(domain: [f64; 2], gradient: Gradient) -> Self {
        let a = Self {
            domain,
            m: f64::default(),
            C: f64::default(),
            gradient,
        };
        a.update()
    }

    pub(crate) const fn domain(self) -> [f64; 2] {
        self.domain
    }

    pub fn domain_set(mut self, domain: [f64; 2]) -> Self {
        self.domain = domain;
        self.update()
    }
}

impl ColorMap {
    pub fn update(mut self) -> Self {
        // let delta_y = self.range[1] - self.range[0];
        let delta_x = self.domain[1] - self.domain[0];

        self.m = 1.0 / delta_x;

        self.C = -self.m * self.domain[0];

        self
    }

    // Returns a string of the form "#RGB"
    #[inline]
    pub fn rgb(&self, x: f64) -> String {
        let y = x * self.m + self.C;
        let mut hex_color = String::new();
        write!(hex_color, "#{:X}", self.gradient.eval_continuous(y))
            .expect("colormap converting failed");
        hex_color
    }
}

// #[cfg(not(tarpaulin_include))]
// mod test {

//     use super::ColorMap;

//     #[test]
//     fn default_unit_scale() {
//         // Unit scaling [0,1] maps to [0, 1]

//         // Value inside the given range, domain
//         let scale = Scale::default();
//         assert_eq!(scale.project(&0.5_f64), 0.5_f64);

//         // negative values.
//         assert_eq!(scale.project(&-100f64), -100_f64);

//         // extends beyound the given range
//         assert_eq!(scale.project(&100f64), 100_f64);
//     }

//     #[test]
//     fn longitude_to_screen() {
//         let inout = [(0_f64, 0_f64), (180_f64, 50_f64)];

//         let domain = [0_f64, 360_f64];
//         let range = [0_f64, 100_f64];
//         let scale = Scale::new(domain, range);

//         // A -> B under projection
//         for (a, b) in inout {
//             assert_eq!(scale.project(&a), b)
//         }
//     }
// }
