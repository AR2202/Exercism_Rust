#[derive(Debug)]
pub struct Duration {
    s: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { s: s as f64 }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        d.s / 31557600.0
    }
}

macro_rules! planet {
    ($t:ident) => {
        impl Planet for $t {
            fn years_during(d: &Duration) -> f64 {
                d.s / (31557600.0 * $t::YEAR)
            }
        }
    };
}

pub struct Mercury;
impl Mercury {
    const YEAR: f64 = 0.2408467;
}
planet!(Mercury);

pub struct Venus;
impl Venus {
    const YEAR: f64 = 0.61519726;
}
planet!(Venus);

pub struct Earth;
impl Earth {
    const YEAR: f64 = 1.0;
}
planet!(Earth);

pub struct Mars;
impl Mars {
    const YEAR: f64 = 1.8808158;
}
planet!(Mars);

pub struct Jupiter;
impl Jupiter {
    const YEAR: f64 = 11.862615;
}

planet!(Jupiter);

pub struct Saturn;
impl Saturn {
    const YEAR: f64 = 29.447498;
}
planet!(Saturn);

pub struct Uranus;
impl Uranus {
    const YEAR: f64 = 84.016846;
}
planet!(Uranus);

pub struct Neptune;
impl Neptune {
    const YEAR: f64 = 164.79132;
}
planet!(Neptune);
