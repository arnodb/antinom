use rng::AntiNomRng;

pub mod branch;
pub mod bytes;
pub mod character;
pub mod combinator;
mod macros;
pub mod multi;
pub mod rng;
pub mod sequence;

pub trait Buffer {
    type Char;
    type Slice: ?Sized;

    fn push(&mut self, c: Self::Char);

    fn push_slice(&mut self, slice: &Self::Slice);
}

impl Buffer for Vec<u8> {
    type Char = u8;
    type Slice = [u8];

    fn push(&mut self, c: Self::Char) {
        self.push(c);
    }

    fn push_slice(&mut self, slice: &Self::Slice) {
        self.extend(slice);
    }
}

impl Buffer for String {
    type Char = char;
    type Slice = str;

    fn push(&mut self, c: Self::Char) {
        self.push(c);
    }

    fn push_slice(&mut self, slice: &Self::Slice) {
        self.push_str(slice);
    }
}

pub trait Generator<R, B>
where
    R: AntiNomRng,
    B: Buffer,
{
    fn gen(&mut self, rng: &mut R, buffer: &mut B);
}

impl<R, B, F> Generator<R, B> for F
where
    R: AntiNomRng,
    B: Buffer,
    F: FnMut(&mut R, &mut B),
{
    fn gen(&mut self, rng: &mut R, buffer: &mut B) {
        self(rng, buffer);
    }
}
