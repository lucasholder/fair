use bencher::*;

fn bench_simulate_games(b: &mut Bencher) {
    let client_seed = "some client seed";
    let server_seed = "some server seed";
    let mut nonce = 0;
    b.iter(|| {
        nonce += 1;
        fair::games::baccarat::simulate(client_seed, server_seed, nonce)
    });
}

benchmark_group!(benches, bench_simulate_games);
benchmark_main!(benches);
