use rand::Rng;

use crate::{Buffer, Generator};

pub fn char<R, B>(c: B::Char) -> impl Generator<R, B>
where
    R: Rng,
    B: Buffer,
    B::Char: Clone,
{
    move |_rng: &mut R, buffer: &mut B| buffer.push(c.clone())
}

const ALPHA: &str = concat!("ABCDEFGHIJKLMNOPQRSTUVWXYZ", "abcdefghijklmnopqrstuvwxyz");
const ALPHANUMERIC: &str = concat!(
    "0123456789",
    "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
    "abcdefghijklmnopqrstuvwxyz"
);
const SPACE: &str = " \t\r\n";

pub fn alpha1<R, B>(max_length: u8) -> impl Generator<R, B>
where
    R: Rng,
    B: Buffer,
    B::Char: From<u8>,
{
    move |rng: &mut R, buffer: &mut B| {
        for _ in 0..rng.gen_range(1..=max_length) {
            let i = rng.gen_range(0..ALPHA.len());
            buffer.push(ALPHA.as_bytes()[i].into());
        }
    }
}

pub fn alphanumeric1<R, B>(max_length: u8) -> impl Generator<R, B>
where
    R: Rng,
    B: Buffer,
    B::Char: From<u8>,
{
    move |rng: &mut R, buffer: &mut B| {
        for _ in 0..rng.gen_range(1..=max_length) {
            let i = rng.gen_range(0..ALPHANUMERIC.len());
            buffer.push(ALPHANUMERIC.as_bytes()[i].into());
        }
    }
}

pub fn multispace0<R, B>(max_length: u8) -> impl Generator<R, B>
where
    R: Rng,
    B: Buffer,
    B::Char: From<u8>,
{
    move |rng: &mut R, buffer: &mut B| {
        for _ in 0..rng.gen_range(0..=max_length) {
            let i = rng.gen_range(0..SPACE.len());
            buffer.push(SPACE.as_bytes()[i].into());
        }
    }
}

pub fn multispace1<R, B>(max_length: u8) -> impl Generator<R, B>
where
    R: Rng,
    B: Buffer,
    B::Char: From<u8>,
{
    move |rng: &mut R, buffer: &mut B| {
        for _ in 0..rng.gen_range(1..=max_length) {
            let i = rng.gen_range(0..SPACE.len());
            buffer.push(SPACE.as_bytes()[i].into());
        }
    }
}
