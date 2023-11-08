use std::ops::{Range, RangeInclusive};

#[cfg(feature = "rng_rand")]
use rand::distributions::uniform::{SampleRange, SampleUniform};

pub trait AntiNomRng {
    fn gen_bool(&mut self) -> bool;

    fn gen_range<T, R>(&mut self, range: R) -> T
    where
        T: AntiNomGenRangeSupport,
        R: AntiNomGenRangeRangeSupport<T>;

    fn anarchy_level(&self) -> AnarchyLevel;

    fn anarchy(&mut self) -> bool {
        match self.anarchy_level() {
            AnarchyLevel::LawAndOrder => false,
            AnarchyLevel::ALittleAnarchy => {
                let a = self.gen_range(0..=u8::MAX);
                a <= 2
            }
            AnarchyLevel::CivilWar => {
                let a = self.gen_range(0..=u8::MAX);
                a <= u8::MAX / 2
            }
        }
    }
}

#[cfg(all(feature = "rng_rand", not(feature = "rng_arbitrary")))]
pub trait AntiNomGenRangeSupport: SampleUniform {}

#[cfg(all(not(feature = "rng_rand"), feature = "rng_arbitrary"))]
pub trait AntiNomGenRangeSupport: arbitrary::unstructured::Int {}

#[cfg(all(feature = "rng_rand", feature = "rng_arbitrary"))]
pub trait AntiNomGenRangeSupport: SampleUniform + arbitrary::unstructured::Int {}

#[cfg(feature = "rng_rand")]
pub use rng_rand::new_default_rng;

impl AntiNomGenRangeSupport for u8 {}
impl AntiNomGenRangeSupport for i32 {}
impl AntiNomGenRangeSupport for usize {}

#[cfg(all(feature = "rng_rand", not(feature = "rng_arbitrary")))]
pub trait AntiNomGenRangeRangeSupport<T>: SampleRange<T> {
    fn into_range_inclusive(self) -> RangeInclusive<T>;
}

#[cfg(all(not(feature = "rng_rand"), feature = "rng_arbitrary"))]
pub trait AntiNomGenRangeRangeSupport<T> {
    fn into_range_inclusive(self) -> RangeInclusive<T>;
}

#[cfg(all(feature = "rng_rand", feature = "rng_arbitrary"))]
pub trait AntiNomGenRangeRangeSupport<T>: SampleRange<T> {
    fn into_range_inclusive(self) -> RangeInclusive<T>;
}

impl AntiNomGenRangeRangeSupport<u8> for Range<u8> {
    fn into_range_inclusive(self) -> RangeInclusive<u8> {
        self.start..=(self.end - 1)
    }
}
impl AntiNomGenRangeRangeSupport<i32> for Range<i32> {
    fn into_range_inclusive(self) -> RangeInclusive<i32> {
        self.start..=(self.end - 1)
    }
}
impl AntiNomGenRangeRangeSupport<usize> for Range<usize> {
    fn into_range_inclusive(self) -> RangeInclusive<usize> {
        self.start..=(self.end - 1)
    }
}
impl AntiNomGenRangeRangeSupport<u8> for RangeInclusive<u8> {
    fn into_range_inclusive(self) -> RangeInclusive<u8> {
        self
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AnarchyLevel {
    LawAndOrder,
    ALittleAnarchy,
    CivilWar,
}

#[cfg(feature = "rng_rand")]
pub use rng_rand::AntiNomRandRng;

#[cfg(feature = "rng_rand")]
pub mod rng_rand {
    use rand::{Rng, SeedableRng};

    use super::{AnarchyLevel, AntiNomGenRangeRangeSupport, AntiNomGenRangeSupport, AntiNomRng};

    pub struct AntiNomRandRng<Rand>
    where
        Rand: Rng,
    {
        pub rng: Rand,
        pub anarchy_level: AnarchyLevel,
    }

    impl<Rand> AntiNomRng for AntiNomRandRng<Rand>
    where
        Rand: Rng,
    {
        fn gen_bool(&mut self) -> bool {
            self.rng.gen()
        }

        fn gen_range<T, R>(&mut self, range: R) -> T
        where
            T: AntiNomGenRangeSupport,
            R: AntiNomGenRangeRangeSupport<T>,
        {
            self.rng.gen_range(range)
        }

        fn anarchy_level(&self) -> AnarchyLevel {
            self.anarchy_level
        }
    }

    pub fn new_default_rng() -> AntiNomRandRng<rand_chacha::ChaCha8Rng> {
        AntiNomRandRng {
            rng: rand_chacha::ChaCha8Rng::from_entropy(),
            anarchy_level: AnarchyLevel::LawAndOrder,
        }
    }
}

#[cfg(feature = "rng_arbitrary")]
pub use rng_arbitrary::AntiNomArbitraryRng;

#[cfg(feature = "rng_arbitrary")]
pub mod rng_arbitrary {
    use arbitrary::Unstructured;

    use super::{AnarchyLevel, AntiNomGenRangeRangeSupport, AntiNomGenRangeSupport, AntiNomRng};

    pub struct AntiNomArbitraryRng<'a> {
        pub u: Unstructured<'a>,
        pub anarchy_level: AnarchyLevel,
    }

    impl<'a> AntiNomRng for AntiNomArbitraryRng<'a> {
        fn gen_bool(&mut self) -> bool {
            self.u.arbitrary().unwrap()
        }

        fn gen_range<T, R>(&mut self, range: R) -> T
        where
            T: AntiNomGenRangeSupport,
            R: AntiNomGenRangeRangeSupport<T>,
        {
            self.u.int_in_range(range.into_range_inclusive()).unwrap()
        }

        fn anarchy_level(&self) -> AnarchyLevel {
            self.anarchy_level
        }
    }
}
