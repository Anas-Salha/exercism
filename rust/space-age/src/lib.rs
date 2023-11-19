// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

const EARTH_YEAR_IN_SECONDS: f64 = 31_557_600.0;

const MERCURY_ORBITAL_PERIOD_RATIO: f64 = 0.2408467;
const VENUS_ORBITAL_PERIOD_RATIO: f64 = 0.61519726;
const MARS_ORBITAL_PERIOD_RATIO: f64 = 1.8808158;
const EARTH_ORBITAL_PERIOD_RATIO: f64 = 1.0;
const JUPITER_ORBITAL_PERIOD_RATIO: f64 = 11.862615;
const SATURN_ORBITAL_PERIOD_RATIO: f64 = 29.447498;
const URANUS_ORBITAL_PERIOD_RATIO: f64 = 84.016846;
const NEPTUNE_ORBITAL_PERIOD_RATIO: f64 = 164.79132;

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s as f64 / EARTH_YEAR_IN_SECONDS)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

macro_rules! planet {
    ($n:ident, $r:expr) => {
        pub struct $n;
        impl Planet for $n {
            fn years_during(d: &Duration) -> f64 {
                d.0 / $r
            }
        }
    };
}

planet!(Mercury, MERCURY_ORBITAL_PERIOD_RATIO);
planet!(Venus, VENUS_ORBITAL_PERIOD_RATIO);
planet!(Mars, MARS_ORBITAL_PERIOD_RATIO);
planet!(Earth, EARTH_ORBITAL_PERIOD_RATIO);
planet!(Jupiter, JUPITER_ORBITAL_PERIOD_RATIO);
planet!(Saturn, SATURN_ORBITAL_PERIOD_RATIO);
planet!(Uranus, URANUS_ORBITAL_PERIOD_RATIO);
planet!(Neptune, NEPTUNE_ORBITAL_PERIOD_RATIO);
