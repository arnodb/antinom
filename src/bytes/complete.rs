use crate::{AntiNomRng, Buffer, Generator};

pub fn tag<R, B, T>(tag: T) -> impl Generator<R, B>
where
    R: AntiNomRng,
    B: Buffer,
    T: AsRef<B::Slice> + Clone,
{
    move |_rng: &mut R, buffer: &mut B| buffer.push_slice(tag.clone().as_ref())
}

#[cfg(feature = "rng_rand")]
#[test]
fn tag_should_push_to_vec_u8() {
    use crate::rng::new_default_rng;

    let mut rng = new_default_rng();

    let mut buffer = Vec::<u8>::new();
    let mut t = tag("_");
    t.gen(&mut rng, &mut buffer);
}

#[cfg(feature = "rng_rand")]
#[test]
fn tag_should_push_to_string() {
    use crate::rng::new_default_rng;

    let mut rng = new_default_rng();

    let mut buffer = String::new();
    let mut t = tag("_");
    t.gen(&mut rng, &mut buffer);
}
