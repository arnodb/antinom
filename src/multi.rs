use crate::{AntiNomRng, Buffer, Generator};

pub fn many0<R, B, F>(mut f: F, max_items: u8) -> impl Generator<R, B>
where
    R: AntiNomRng,
    B: Buffer,
    F: Generator<R, B>,
{
    move |rng: &mut R, buffer: &mut B| {
        let n = rng.gen_range(0..=max_items);
        for _ in 0..=n {
            f.gen(rng, buffer);
        }
    }
}

pub fn many1<R, B, F>(mut f: F, max_items: u8) -> impl Generator<R, B>
where
    R: AntiNomRng,
    B: Buffer,
    F: Generator<R, B>,
{
    move |rng: &mut R, buffer: &mut B| {
        let n = rng.gen_range(1..=max_items);
        for _ in 1..=n {
            f.gen(rng, buffer);
        }
    }
}

pub fn many_till<R, B, F, G>(mut f: F, mut g: G, max_items: u8) -> impl Generator<R, B>
where
    R: AntiNomRng,
    B: Buffer,
    F: Generator<R, B>,
    G: Generator<R, B>,
{
    move |rng: &mut R, buffer: &mut B| {
        let n = rng.gen_range(0..=max_items);
        for _ in 0..=n {
            f.gen(rng, buffer);
        }
        g.gen(rng, buffer);
    }
}

pub fn separated_list0<R, B, F, G>(mut sep: G, mut f: F, max_items: u8) -> impl Generator<R, B>
where
    R: AntiNomRng,
    B: Buffer,
    F: Generator<R, B>,
    G: Generator<R, B>,
{
    move |rng: &mut R, buffer: &mut B| {
        let n = rng.gen_range(0..=max_items);
        if n > 0 {
            f.gen(rng, buffer);
        }
        for _ in 1..=n {
            sep.gen(rng, buffer);
            f.gen(rng, buffer);
        }
    }
}

pub fn separated_list1<R, B, F, G>(mut sep: G, mut f: F, max_items: u8) -> impl Generator<R, B>
where
    R: AntiNomRng,
    B: Buffer,
    F: Generator<R, B>,
    G: Generator<R, B>,
{
    move |rng: &mut R, buffer: &mut B| {
        let n = rng.gen_range(1..=max_items);
        f.gen(rng, buffer);
        for _ in 1..=n {
            sep.gen(rng, buffer);
            f.gen(rng, buffer);
        }
    }
}

pub fn many0_count<R, B, F>(f: F, max_items: u8) -> impl Generator<R, B>
where
    R: AntiNomRng,
    B: Buffer,
    F: Generator<R, B>,
{
    many0(f, max_items)
}

pub fn many1_count<R, B, F>(f: F, max_items: u8) -> impl Generator<R, B>
where
    R: AntiNomRng,
    B: Buffer,
    F: Generator<R, B>,
{
    many1(f, max_items)
}
