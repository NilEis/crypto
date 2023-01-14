use num::{BigInt, Integer, One, Zero};
use std::mem;

pub fn gcd(a: &BigInt, b: &BigInt) -> BigInt {
    let mut r = a % b;
    let mut a = b.clone();
    let mut b = r;

    while !(b == BigInt::zero()) {
        r = a.clone() % &b;
        mem::swap(&mut a, &mut b);
        b = r;
    }
    return a;
}

pub fn lcm(a: &BigInt, b: &BigInt) -> BigInt {
    return (a * b) / gcd(a, b);
}

pub fn extended_gcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
    let mut x = BigInt::zero();
    let mut y = BigInt::one();
    let mut u = BigInt::one();
    let mut v = BigInt::zero();
    let mut gcd = a.clone();
    let mut b_ = b.clone();
    while !b_.is_zero() {
        let quotient = gcd.div_floor(&b_);
        let quotient_val = quotient.clone();
        let t = b_.clone();
        b_ = gcd.clone() - &quotient_val * &b_;
        gcd = t;

        let t = x.clone();
        x = u.clone() - quotient_val * x;
        u = t;

        let t = y.clone();
        y = v.clone() - quotient.clone() * y;
        v = t;
    }
    (gcd, u, v)
}

#[cfg(test)]
mod tests {
    use num::BigInt;

    use super::extended_gcd;
    use super::gcd;
    use super::lcm;

    #[test]
    fn test_extended_gcd_1() {
        let a = BigInt::from(10);
        let b = BigInt::from(15);
        let res_gcd = BigInt::from(5);
        let (gcd, x, y) = extended_gcd(&a, &b);
        assert_eq!(res_gcd, gcd, "{} == {}", res_gcd, gcd);
        assert_eq!(a.clone() * x.clone() + b.clone() * y.clone(), gcd, "{}*{} + {}*{} = {}", a, x, b, y, gcd);
    }

    #[test]
    fn test_extended_gcd_2() {
        let a = BigInt::from(3_651);
        let b = BigInt::from(4_635);
        let res_gcd = BigInt::from(3);
        let (gcd, x, y) = extended_gcd(&a, &b);
        assert_eq!(res_gcd, gcd, "{} == {}", res_gcd, gcd);
        assert_eq!(a.clone() * x.clone() + b.clone() * y.clone(), gcd, "{}*{} + {}*{} = {}", a, x, b, y, gcd);
    }

    #[test]
    fn test_extended_gcd_3() {
        let a = BigInt::from(56);
        let b = BigInt::from(42);
        let res_gcd = BigInt::from(14);
        let (gcd, x, y) = extended_gcd(&a, &b);
        assert_eq!(res_gcd, gcd, "{} == {}", res_gcd, gcd);
        assert_eq!(a.clone() * x.clone() + b.clone() * y.clone(), gcd, "{}*{} + {}*{} = {}", a, x, b, y, gcd);
    }

    #[test]
    fn test_extended_gcd_4() {
        let a = BigInt::from(24_826_148);
        let b = BigInt::from(45_296_490);
        let res_gcd = BigInt::from(526);
        let (gcd, x, y) = extended_gcd(&a, &b);
        assert_eq!(res_gcd, gcd, "{} == {}", res_gcd, gcd);
        assert_eq!(a.clone() * x.clone() + b.clone() * y.clone(), gcd, "{}*{} + {}*{} = {}", a, x, b, y, gcd);
    }

    #[test]
    fn test_gcd_1() {
        let a = BigInt::from(10);
        let b = BigInt::from(15);
        let res_gcd = BigInt::from(5);
        let gcd = gcd(&a, &b);
        assert_eq!(res_gcd, gcd, "{} == {}", res_gcd, gcd);
    }

    #[test]
    fn test_gcd_2() {
        let a = BigInt::from(3_651);
        let b = BigInt::from(4_635);
        let res_gcd = BigInt::from(3);
        let gcd = gcd(&a, &b);
        assert_eq!(res_gcd, gcd, "{} == {}", res_gcd, gcd);
    }

    #[test]
    fn test_gcd_3() {
        let a = BigInt::from(56);
        let b = BigInt::from(42);
        let res_gcd = BigInt::from(14);
        let gcd = gcd(&a, &b);
        assert_eq!(res_gcd, gcd, "{} == {}", res_gcd, gcd);
    }

    #[test]
    fn test_gcd_4() {
        let a = BigInt::from(24_826_148);
        let b = BigInt::from(45_296_490);
        let res_gcd = BigInt::from(526);
        let gcd = gcd(&a, &b);
        assert_eq!(res_gcd, gcd, "{} == {}", res_gcd, gcd);
    }

    #[test]
    fn test_lcm_1() {
        let a = BigInt::from(10);
        let b = BigInt::from(15);
        let res_lcm = BigInt::from(30);
        let lcm = lcm(&a, &b);
        assert_eq!(res_lcm, lcm, "{} == {}", res_lcm, lcm);
    }

    #[test]
    fn test_lcm_2() {
        let a = BigInt::from(3_651);
        let b = BigInt::from(4_635);
        let res_lcm = BigInt::from(5_640_795);
        let lcm = lcm(&a, &b);
        assert_eq!(res_lcm, lcm, "{} == {}", res_lcm, lcm);
    }

    #[test]
    fn test_lcm_3() {
        let a = BigInt::from(56);
        let b = BigInt::from(42);
        let res_lcm = BigInt::from(168);
        let lcm = lcm(&a, &b);
        assert_eq!(res_lcm, lcm, "{} == {}", res_lcm, lcm);
    }

    #[test]
    fn test_lcm_4() {
        let a = BigInt::from(24_826_148);
        let b = BigInt::from(45_296_490);
        let res_lcm = BigInt::from(2_137_903_735_020u64);
        let lcm = lcm(&a, &b);
        assert_eq!(res_lcm, lcm, "{} == {}", res_lcm, lcm);
    }
}
