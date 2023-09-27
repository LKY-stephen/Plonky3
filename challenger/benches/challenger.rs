use criterion::{criterion_group, criterion_main, Criterion};
use p3_baby_bear::BabyBear;
use p3_challenger::{CanObserve, CanSample, DuplexChallenger, LinearChallenger};
use p3_field::{Field, TwoAdicField};
use p3_goldilocks::Goldilocks;
use p3_mds::coset_mds::CosetMds;
use p3_mersenne_31::{Mersenne31, Mersenne31Complex};
use p3_poseidon::Poseidon;
use rand::distributions::Standard;
use rand::prelude::Distribution;
use rand::{thread_rng, Rng};

fn challenger_duplexing_test<F>(c: &mut Criterion)
where
    F: Field + TwoAdicField,
    Standard: Distribution<F>,
{
    let inputs = get_random_inputs::<F, 32>();

    let mds = CosetMds::<F, 16>::default();

    let perm = Poseidon::<F, CosetMds<F, 16>, 16, 5>::new_from_rng(4, 22, mds, &mut thread_rng()); // TODO: Use deterministic RNG

    let mut challenger = DuplexChallenger::<F, Poseidon<F, CosetMds<F, 16>, 16, 5>, 16>::new(perm);

    c.bench_function("duplex challenger observe", |b| {
        b.iter(|| challenger.observe_slice(inputs.as_slice()))
    });
    c.bench_function("duplex challenger sample", |b| {
        b.iter(|| challenger.sample())
    });
}
fn challenger_linear_test<F>(c: &mut Criterion)
where
    F: Field,
{
    let inputs = get_random_inputs::<F, 32>();

    let mut challenger = LinearChallenger::<F>::new();

    c.bench_function("linear challenger observe", |b| {
        b.iter(|| challenger.observe_slice(inputs.as_slice()))
    });
    c.bench_function("linear challenger sample", |b| {
        b.iter(|| challenger.sample())
    });
}

fn test_challenger_fields(c: &mut Criterion) {
    challenger_duplexing_test::<BabyBear>(c);
    challenger_linear_test::<BabyBear>(c);
    challenger_duplexing_test::<Goldilocks>(c);
    challenger_linear_test::<Goldilocks>(c);
    challenger_duplexing_test::<Mersenne31Complex<Mersenne31>>(c);
    challenger_linear_test::<Mersenne31Complex<Mersenne31>>(c);
}

fn get_random_inputs<F: Field, const N: usize>() -> [F; N] {
    let rng: &mut rand::rngs::ThreadRng = &mut rand::thread_rng();
    let mut inputs = [F::ZERO; N];
    for input in inputs.iter_mut() {
        *input = F::from_canonical_u32(rng.gen());
    }
    inputs
}

criterion_group!(benches, test_challenger_fields);
criterion_main!(benches);
