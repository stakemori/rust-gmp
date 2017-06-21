#![feature(test)]

extern crate test;
extern crate gmp;

mod benchmark {
    use super::gmp::mpz::Mpz;
    use super::test::Bencher;

    const N: i32 = 10000;

    #[bench]
    fn div_safe(b: &mut Bencher) {
        b.iter(|| {
            let mut a: Mpz = From::<u64>::from(2);
            let b: Mpz = From::<u64>::from(3);
            for i in 0..N {
                a += i;
                &a / &b;
            }
        })
    }
}

