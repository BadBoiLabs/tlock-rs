use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};

#[cfg(unix)]
use pprof::criterion::{Output, PProfProfiler};

fn criterion_benchmark(c: &mut Criterion) {
    let pk_bytes = hex::decode("8200fc249deb0148eb918d6e213980c5d01acd7fc251900d9260136da3b54836ce125172399ddc69c4e3e11429b62c11").unwrap();
    let signature = hex::decode("a4721e6c3eafcd823f138cd29c6c82e8c5149101d0bb4bafddbac1c2d1fe3738895e4e21dd4b8b41bf007046440220910bb1cdb91f50a84a0d7f33ff2e8577aa62ac64b35a291a728a9db5ac91e06d1312b48a376138d77b4d6ad27c24221afe").unwrap();

    c.bench_function("lock with TLE", |b| {
        b.iter_batched_ref(
            || {
                let mut msg = vec![0u8; 16];
                msg.fill_with(rand::random);
                msg
            },
            |msg| {
                let mut encrypted = vec![];
                tlock::encrypt(
                    black_box(&mut encrypted),
                    black_box(msg.as_slice()),
                    black_box(&pk_bytes),
                    black_box(1000),
                )
                .unwrap();
                assert_ne!(encrypted.len(), 0);
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function("unlock with TLE", |b| {
        b.iter_batched_ref(
            || {
                let mut msg = vec![0u8; 16];
                msg.fill_with(rand::random);

                let mut encrypted = vec![];
                tlock::encrypt(
                    black_box(&mut encrypted),
                    black_box(msg.as_slice()),
                    black_box(&pk_bytes),
                    black_box(1000),
                )
                .unwrap();
                (msg, encrypted)
            },
            |(msg, encrypted)| {
                let mut decrypted = vec![];
                tlock::decrypt(
                    black_box(&mut decrypted),
                    black_box(encrypted.as_slice()),
                    black_box(&signature),
                )
                .unwrap();
                decrypted.resize(msg.len(), 0);
                assert_eq!(msg.as_slice(), decrypted);
            },
            BatchSize::SmallInput,
        )
    });

    c.bench_function("lock unlock with TLE", |b| {
        b.iter_batched_ref(
            || {
                let mut msg = vec![0u8; 16];
                msg.fill_with(rand::random);
                msg
            },
            |msg| {
                let mut encrypted = vec![];
                tlock::encrypt(
                    black_box(&mut encrypted),
                    black_box(msg.as_slice()),
                    black_box(&pk_bytes),
                    black_box(1000),
                )
                .unwrap();
                let mut decrypted = vec![];
                tlock::decrypt(
                    black_box(&mut decrypted),
                    black_box(encrypted.as_slice()),
                    black_box(&signature),
                )
                .unwrap();
                decrypted.resize(msg.len(), 0);
                assert_eq!(msg.as_slice(), decrypted);
            },
            BatchSize::SmallInput,
        )
    });
}

#[cfg(unix)]
criterion_group!(
    name = benches;
    config = Criterion::default()
        .with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = criterion_benchmark
);
#[cfg(not(unix))]
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
