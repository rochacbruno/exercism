const SECONDS_IN_A_EARTH_YEAR: f64 = 31_557_600_f64;

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Self(s as f64)
    }
}

pub trait Planet {
    const FACTOR: f64;
    fn years_during(d: &Duration) -> f64 {
        d.0 / (SECONDS_IN_A_EARTH_YEAR * Self::FACTOR)
    }
}

macro_rules! planet {
    ($name: ident, $factor: expr) => {
        pub struct $name;
        impl Planet for $name {
            const FACTOR: f64 = $factor;
        }
    };
}

planet!(Mercury, 0.240_846_7);
planet!(Venus, 0.615_197_26);
planet!(Earth, 1.0);
planet!(Mars, 1.880_815_8);
planet!(Jupiter, 11.862_615);
planet!(Saturn, 29.447_498);
planet!(Uranus, 84.016_846);
planet!(Neptune, 164.79132);
