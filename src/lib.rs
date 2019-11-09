#![no_std]
use js_ffi::*;
use rand::rngs::StdRng;
use rand::Rng;
use rand::SeedableRng;

pub struct Random {
    rng: StdRng,
}

impl Default for Random {
    fn default() -> Self {
        let random = register("Math.random");
        let rng: StdRng = SeedableRng::from_seed([
            (255.0 * call_0(UNDEFINED, random)) as u8,
            (255.0 * call_0(UNDEFINED, random)) as u8,
            (255.0 * call_0(UNDEFINED, random)) as u8,
            (255.0 * call_0(UNDEFINED, random)) as u8,
            (255.0 * call_0(UNDEFINED, random)) as u8,
            (255.0 * call_0(UNDEFINED, random)) as u8,
            (255.0 * call_0(UNDEFINED, random)) as u8,
            (255.0 * call_0(UNDEFINED, random)) as u8,
            (255.0 * call_0(UNDEFINED, random)) as u8,
            (255.0 * call_0(UNDEFINED, random)) as u8,
            (255.0 * call_0(UNDEFINED, random)) as u8,
            (255.0 * call_0(UNDEFINED, random)) as u8,
            (255.0 * call_0(UNDEFINED, random)) as u8,
            (255.0 * call_0(UNDEFINED, random)) as u8,
            (255.0 * call_0(UNDEFINED, random)) as u8,
            (255.0 * call_0(UNDEFINED, random)) as u8,
            (255.0 * call_0(UNDEFINED, random)) as u8,
            (255.0 * call_0(UNDEFINED, random)) as u8,
            (255.0 * call_0(UNDEFINED, random)) as u8,
            (255.0 * call_0(UNDEFINED, random)) as u8,
            (255.0 * call_0(UNDEFINED, random)) as u8,
            (255.0 * call_0(UNDEFINED, random)) as u8,
            (255.0 * call_0(UNDEFINED, random)) as u8,
            (255.0 * call_0(UNDEFINED, random)) as u8,
            (255.0 * call_0(UNDEFINED, random)) as u8,
            (255.0 * call_0(UNDEFINED, random)) as u8,
            (255.0 * call_0(UNDEFINED, random)) as u8,
            (255.0 * call_0(UNDEFINED, random)) as u8,
            (255.0 * call_0(UNDEFINED, random)) as u8,
            (255.0 * call_0(UNDEFINED, random)) as u8,
            (255.0 * call_0(UNDEFINED, random)) as u8,
            (255.0 * call_0(UNDEFINED, random)) as u8,
        ]);
        Random { rng: rng }
    }
}

impl Random {
    pub fn gen<T>(&mut self) -> T
    where
        rand::distributions::Standard: rand::distributions::Distribution<T>,
    {
        self.rng.gen::<T>()
    }
}
