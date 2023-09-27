use core::marker::PhantomData;

use p3_field::{Field, PrimeField64};

use crate::{CanObserve, CanSample, CanSampleBits, FieldChallenger};

#[derive(Clone)]
pub struct LinearChallenger<F> {
    sponge_state: F,
    _phantom_f: PhantomData<F>,
}

impl<F> LinearChallenger<F> {
    pub fn new() -> Self
    where
        F: Default,
    {
        Self {
            sponge_state: F::default(),
            _phantom_f: PhantomData,
        }
    }
}

impl<F> FieldChallenger<F> for LinearChallenger<F> where F: PrimeField64 {}

impl<F> CanObserve<F> for LinearChallenger<F>
where
    F: Copy + Field,
{
    fn observe(&mut self, value: F) {
        self.sponge_state = self.sponge_state + value;
    }
}

impl<F, const N: usize> CanObserve<[F; N]> for LinearChallenger<F>
where
    F: Copy + Field,
{
    fn observe(&mut self, values: [F; N]) {
        for value in values {
            self.observe(value);
        }
    }
}

impl<F> CanSample<F> for LinearChallenger<F>
where
    F: Copy + Field,
{
    fn sample(&mut self) -> F {
        let res = self.sponge_state;
        self.sponge_state = (self.sponge_state + F::ONE).inverse();
        res
    }
}

impl<F> CanSampleBits<usize> for LinearChallenger<F>
where
    F: PrimeField64,
{
    fn sample_bits(&mut self, bits: usize) -> usize {
        debug_assert!(bits < (usize::BITS as usize));
        debug_assert!((1 << bits) < F::ORDER_U64);
        let rand_f: F = self.sample();
        let rand_usize = rand_f.as_canonical_u64() as usize;
        rand_usize & ((1 << bits) - 1)
    }
}

#[cfg(test)]
mod tests {
    use p3_field::AbstractField;
    use p3_goldilocks::Goldilocks;

    use super::*;

    const WIDTH: usize = 32;
    type F = Goldilocks;

    #[test]
    fn test_linear_challenger() {
        let mut linear_challenger = LinearChallenger::new();

        // observe elements before reaching WIDTH
        let mut sum = 0;
        (0..WIDTH - 1).for_each(|element| {
            linear_challenger.observe(F::from_canonical_u8(element as u8));
            sum += element;
            assert_eq!(linear_challenger.sponge_state, F::from_canonical_usize(sum));
        });

        assert_eq!(linear_challenger.sample(), F::from_canonical_usize(sum));
        assert_eq!(
            linear_challenger.sponge_state,
            F::from_canonical_usize(sum).inverse() + F::ONE,
        );
    }
}
