use test;
use test::Bencher;


#[bench]
fn bench_md5_calc(b: &mut Bencher) {
    Md5Hash::calc();
}
