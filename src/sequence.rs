use rand::Rng;

use crate::{macros::succ, Buffer, Generator};

pub fn pair<R, B, F, G>(mut f: F, mut g: G) -> impl Generator<R, B>
where
    R: Rng,
    B: Buffer,
    F: Generator<R, B>,
    G: Generator<R, B>,
{
    move |rng: &mut R, buffer: &mut B| {
        f.gen(rng, buffer);
        g.gen(rng, buffer);
    }
}

pub fn preceded<R, B, F, G>(mut first: F, mut second: G) -> impl Generator<R, B>
where
    R: Rng,
    B: Buffer,
    F: Generator<R, B>,
    G: Generator<R, B>,
{
    move |rng: &mut R, buffer: &mut B| {
        first.gen(rng, buffer);
        second.gen(rng, buffer);
    }
}

pub fn terminated<R, B, F, G>(mut first: F, mut second: G) -> impl Generator<R, B>
where
    R: Rng,
    B: Buffer,
    F: Generator<R, B>,
    G: Generator<R, B>,
{
    move |rng: &mut R, buffer: &mut B| {
        first.gen(rng, buffer);
        second.gen(rng, buffer);
    }
}

pub fn delimited<R, B, F, G, H>(mut first: F, mut second: G, mut third: H) -> impl Generator<R, B>
where
    R: Rng,
    B: Buffer,
    F: Generator<R, B>,
    G: Generator<R, B>,
    H: Generator<R, B>,
{
    move |rng: &mut R, buffer: &mut B| {
        first.gen(rng, buffer);
        second.gen(rng, buffer);
        third.gen(rng, buffer);
    }
}

pub trait Tuple<R, B> {
    fn gen(&mut self, rng: &mut R, buffer: &mut B);
}

macro_rules! tuple_trait(
    ($first:ident $second:ident $($id: ident)+) => (
        tuple_trait!(__impl 2; $first $second; $($id)+);
    );
    (__impl $len:expr; $($current:ident)*; $head:ident $($id: ident)+) => (
        tuple_trait_impl!($len; $($current)*);
        tuple_trait!(__impl ($len + 1); $($current)* $head; $($id)+);
    );
    (__impl $len:expr; $($current:ident)*; $head:ident) => (
        tuple_trait_impl!($len; $($current)*);
        tuple_trait_impl!($len; $($current)* $head);
    );
    );

macro_rules! tuple_trait_impl(
    ($len:expr; $($id:ident)+) => (
        impl<R, B, $($id),+>  Tuple<R, B> for ( $($id),+ ) where R: Rng, B: Buffer, $($id: Generator<R, B>),* {
            fn gen(&mut self, rng: &mut R, buffer: &mut B) {
                tuple_item!(0, self, rng, buffer; $($id)*);
            }
        }
    );
);

macro_rules! tuple_item (
    ($it:tt, $self:ident, $rng:ident, $buffer:ident; $first:ident $($id: ident)*) => (
        $self.$it.gen($rng, $buffer);
        succ!($it, tuple_item!($self, $rng, $buffer; $($id)*));
    );
    ($it:tt, $self:ident, $rng:ident, $buffer:ident;) => ();
);

tuple_trait!(A C D E F G H I J K L M N O P Q S T U V W);

pub fn tuple<R, B, List>(mut l: List) -> impl Generator<R, B>
where
    R: Rng,
    B: Buffer,
    List: Tuple<R, B>,
{
    move |rng: &mut R, buffer: &mut B| l.gen(rng, buffer)
}
