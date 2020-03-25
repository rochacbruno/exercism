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

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Earth {
    const FACTOR: f64 = 1.0;
}

impl Planet for Mercury {
    const FACTOR: f64 = 0.240_846_7;
}
impl Planet for Venus {
    const FACTOR: f64 = 0.615_197_26;
}

impl Planet for Mars {
    const FACTOR: f64 = 1.880_815_8;
}
impl Planet for Jupiter {
    const FACTOR: f64 = 11.862_615;
}
impl Planet for Saturn {
    const FACTOR: f64 = 29.447_498;
}
impl Planet for Uranus {
    const FACTOR: f64 = 84.016_846;
}
impl Planet for Neptune {
    const FACTOR: f64 = 164.79132;
}
