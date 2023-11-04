use crate::{AntiNomRng, Buffer, Generator};

pub fn opt<R, B, F>(mut f: F) -> impl Generator<R, B>
where
    R: AntiNomRng,
    B: Buffer,
    F: Generator<R, B>,
{
    move |rng: &mut R, buffer: &mut B| {
        let maybe: bool = rng.gen_bool();
        if maybe {
            f.gen(rng, buffer)
        }
    }
}

pub fn peek<R, B, F>(_f: F) -> impl Generator<R, B>
where
    R: AntiNomRng,
    B: Buffer,
    F: Generator<R, B>,
{
    |_rng: &mut R, _buffer: &mut B| {}
}

pub fn eof<R, B>(_rng: &mut R, _buffer: &mut B)
where
    R: AntiNomRng,
    B: Buffer,
{
}

pub fn recognize<R, B, F>(f: F) -> impl Generator<R, B>
where
    R: AntiNomRng,
    B: Buffer,
    F: Generator<R, B>,
{
    f
}

pub fn cut<R, B, F>(f: F) -> impl Generator<R, B>
where
    R: AntiNomRng,
    B: Buffer,
    F: Generator<R, B>,
{
    f
}
