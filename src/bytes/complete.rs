use rand::Rng;

use crate::{Buffer, Generator};

pub fn tag<R, B, T>(tag: T) -> impl Generator<R, B>
where
    R: Rng,
    B: Buffer,
    T: AsRef<B::Slice> + Clone,
{
    move |_rng: &mut R, buffer: &mut B| buffer.push_slice(tag.clone().as_ref())
}

#[test]
fn tag_should_push_to_vec_u8() {
    use rand::SeedableRng;
    let mut rng = rand_chacha::ChaCha8Rng::from_entropy();
    let mut buffer = Vec::<u8>::new();
    let mut t = tag("_");
    t.gen(&mut rng, &mut buffer);
}

#[test]
fn tag_should_push_to_string() {
    use rand::SeedableRng;
    let mut rng = rand_chacha::ChaCha8Rng::from_entropy();
    let mut buffer = String::new();
    let mut t = tag("_");
    t.gen(&mut rng, &mut buffer);
}
