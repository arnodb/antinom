use rand::Rng;

use crate::{macros::succ, Buffer, Generator};

pub trait Alt<R, B>
where
    R: Rng,
    B: Buffer,
{
    fn choice(&mut self, rng: &mut R, buffer: &mut B);
}

macro_rules! alt_trait(
    ($first:ident $second:ident $($id: ident)+) => (
        alt_trait!(__impl 2; $first $second; $($id)+);
    );
    (__impl $len:expr; $($current:ident)*; $head:ident $($id: ident)+) => (
        alt_trait_impl!($len; $($current)*);
        alt_trait!(__impl ($len + 1); $($current)* $head; $($id)+);
    );
    (__impl $len:expr; $($current:ident)*; $head:ident) => (
        alt_trait_impl!($len; $($current)*);
        alt_trait_impl!($len; $($current)* $head);
    );
);

macro_rules! alt_trait_impl(
    ($len:expr; $($id:ident)+) => (
        impl<R, B, $($id),+>  Alt<R, B> for ( $($id),+ ) where R: Rng, B: Buffer, $($id: Generator<R, B>),* {
            fn choice(&mut self, rng: &mut R, buffer: &mut B) {
                let i = rng.gen_range(0..$len);
                alt_match_arm!(0, self, rng, buffer, i; $($id)*);
            }
        }
    );
);

macro_rules! alt_match_arm (
    ($it:tt, $self:ident, $rng:ident, $buffer:ident, $i:ident; $first:ident $($id: ident)*) => (
        if $i == $it {
            return $self.$it.gen($rng, $buffer);
        }
        succ!($it, alt_match_arm!($self, $rng, $buffer, $i; $($id)*));
    );
    ($it:tt, $self:ident, $rng:ident, $buffer:ident, $i:ident;) => (
        unreachable!();
    );
);

alt_trait!(A C D E F G H I J K L M N O P Q S T U V W);

pub fn alt<R, B, List>(mut l: List) -> impl Generator<R, B>
where
    R: Rng,
    B: Buffer,
    List: Alt<R, B>,
{
    move |rng: &mut R, buffer: &mut B| l.choice(rng, buffer)
}
