#![feature(test)]
extern crate libc;

extern crate test;
extern crate gmp;

mod benchmark {
    use super::gmp::mpz::Mpz;
    use super::test::Bencher;
    use libc::c_ulong;

    const N: i32 = 10000;

    #[bench]
    fn div_safe(b: &mut Bencher) {
        b.iter(|| {
            let mut a: Mpz = From::<u64>::from(2);
            let b: Mpz = From::<u64>::from(3);
            for i in 0..N {
                a += i as u64;
                &a / &b;
            }
        })
    }

    #[bench]
    fn kronecker(b: &mut Bencher) {
        b.iter(|| {
            let p: Mpz = From::from(1031 as c_ulong);
            let mut a: Mpz = From::from(1 as c_ulong);
            while a < p {
                Mpz::kronecker(&a, &p);
                a += 1;
            }
        })
    }
}
