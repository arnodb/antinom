use rand::Rng;

use crate::{Buffer, Generator};

pub fn opt<R, B, F>(mut f: F) -> impl Generator<R, B>
where
    R: Rng,
    B: Buffer,
    F: Generator<R, B>,
{
    move |rng: &mut R, buffer: &mut B| {
        let maybe: bool = rng.gen();
        if maybe {
            f.gen(rng, buffer)
        }
    }
}

pub fn peek<R, B, F>(_f: F) -> impl Generator<R, B>
where
    R: Rng,
    B: Buffer,
    F: Generator<R, B>,
{
    |_rng: &mut R, _buffer: &mut B| {}
}

pub fn eof<R, B>(_rng: &mut R, _buffer: &mut B)
where
    R: Rng,
    B: Buffer,
{
}

pub fn recognize<R, B, F>(f: F) -> impl Generator<R, B>
where
    R: Rng,
    B: Buffer,
    F: Generator<R, B>,
{
    f
}

pub fn cut<R, B, F>(f: F) -> impl Generator<R, B>
where
    R: Rng,
    B: Buffer,
    F: Generator<R, B>,
{
    f
}
