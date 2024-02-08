fn mul_mod(a: u32, b: u32, m: u32) -> u32 {
    ((a as u64 * b as u64) % (m as u64)) as u32
}

fn pow_mod(a: u32, n: u32, p: u32) -> u32 {
    if n == 0 {
        return 1;
    }
    if n % 2 == 0 {
        pow_mod(mul_mod(a, a, p), n / 2, p)
    } else {
        mul_mod(a, pow_mod(a, n - 1, p), p)
    }
}

fn jacobi(mut a: u32, mut m: u32) -> i32 {
    let mut t = 1;
    a %= m;
    while a != 0 {
        while (a & 1) == 0 {
            a >>= 1;
            if (m & 7) == 3 || (m & 7) == 5 {
                t = -t;
            }
        }
        let aux = a;
        a = m;
        m = aux;
        if (a & 3) == 3 && (m & 3) == 3 {
            t = -t;
        }
        a %= m;
    }
    if m == 1 { t } else { 0 }
}

fn residue(prime: u32, arg: u32) -> u32 {
    if jacobi(arg, prime) == -1 {
        return u32::MAX;
    }

    let mut y = 2;
    while jacobi(y, prime) != -1 {
        y += 1;
    }

    let mut r: u32 = 0;
    let mut q: u32 = prime - 1;
    while q % 2 == 0 {
        r += 1;
        q /= 2;
    }

    let mut result = prime - 1;
    result >>= r;
    y = pow_mod(y, result, prime);
    result >>= 1;
    let mut b = pow_mod(arg, result, prime);
    result = mul_mod(arg, b, prime);
    b *= result;
    b = b % prime;

    while b != 1 {
        let mut t = mul_mod(b, b, prime);
        let mut m: u32 = 1;
        while t != 1 {
            t *= t;
            t = t % prime;
            m += 1;
        }
        t = 0;
        t |= 1 << (r - m - 1);
        t = pow_mod(y, t, prime);
        y = mul_mod(t, t, prime);
        r = m;
        result = mul_mod(result, t, prime);
        b = mul_mod(b, y, prime);
    }
    result
}


fn main() {
    assert_eq!(residue(5, 4), 3);
}
