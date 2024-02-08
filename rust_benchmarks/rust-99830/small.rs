extern crate primal_sieve;
use primal_sieve::Primes;
use std::ops::{Add, MulAssign};

fn main() {
    assert_eq!(solve(4), 650);
    assert_eq!(solve(100), 202_476_099);
    assert_eq!(solve(100_000), 403_221_585);
}

fn solve(n: u32) -> u32 {
    let mut product = Mod(1);
    // Changing to a regular `for` loop suppresses the bug.
    // for p in Primes::all().take_while(|&p| p < n as usize) {
    Primes::all().take_while(|&p| p < n as usize).for_each(|p| {
        let p = p as u32;

        let mut k = n / p;
        let mut pp = p;
        while let Some(x) = pp.checked_mul(p) {
            pp = x;
            k += n / pp;
        }

        product *= Mod(p).pow(2 * k) + Mod(1);
    });
    // }
    product.0
}

#[derive(Clone, Copy)]
struct Mod(u32);

impl Mod {
    const MOD: u32 = 10u32.pow(9) + 9;
}

impl Add for Mod {
    type Output = Mod;

    fn add(mut self, rhs: Mod) -> Mod {
        if rhs.0 != 0 {
            let neg_rhs = Self::MOD - rhs.0;
            if self.0 < neg_rhs {
                self.0 += rhs.0;
            } else {
                self.0 -= neg_rhs;
            }
        }
        self
    }
}

impl MulAssign for Mod {
    fn mul_assign(&mut self, rhs: Mod) {
        // Adding this up-casting implementation and the `assert_eq` supresses the bug.
        // let x = (u64::from(self.0) * u64::from(rhs.0) % u64::from(Self::MOD)) as u32;
        unsafe {
            core::arch::asm!(
                "mul edx",
                "div {:e}",
                in(reg) Self::MOD,
                inout("eax") rhs.0 => _,
                inout("edx") self.0,
                options(pure, nomem, nostack),
            );
        }
        // assert_eq!(self.0, x);
    }
}

impl Mod {
    fn pow(self, mut exp: u32) -> Self {
        if exp == 0 {
            return Mod(1);
        }
        if self.0 < 2 {
            return self;
        }

        let mut base = self;
        while exp & 1 == 0 {
            base *= base;
            exp >>= 1;
        }
        let mut acc = base;
        exp >>= 1;
        while exp != 0 {
            base *= base;
            if exp & 1 == 1 {
                acc *= base;
            }
            exp >>= 1;
        }
        acc
    }
}
