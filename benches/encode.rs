#![feature(test)]
extern crate test;
use test::Bencher;

extern crate labrador_ldpc;
use labrador_ldpc::LDPCCode;

#[bench]
fn bench_encode_fast(b: &mut Bencher) {
    let code = LDPCCode::TM8192;
    let txdata: Vec<u8> = (0..code.k()/8).map(|i| !(i as u8)).collect();
    let mut txcode = vec![0u8; code.n()/8];
    let mut g = vec![0u32; code.generator_len()];
    code.init_generator(&mut g);

    b.iter(|| code.encode_fast(&g, &txdata, &mut txcode) );
}

#[bench]
fn bench_encode_xfast(b: &mut Bencher) {
    let code = LDPCCode::TM8192;
    let txdata: Vec<u8> = (0..code.k()/8).map(|i| !(i as u8)).collect();
    let mut txcode = vec![0u8; code.n()/8];
    let mut g = vec![0u32; code.generator_len()];
    code.init_generator(&mut g);

    b.iter(|| code.encode_xfast(&g, &txdata, &mut txcode) );
}
