#![feature(const_hash)]
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ptr;
static mut H: DefaultHasher = DefaultHasher::new();

fn dump_var<T: Hash, U: Hash, V: Hash, W: Hash>(val0: T, val1: U, val2: V, val3: W) {
    unsafe {
        val0.hash(&mut H);
        val1.hash(&mut H);
        val2.hash(&mut H);
        val3.hash(&mut H);
    }
}
pub fn fn0(mut _2: [isize; 7], mut _3: usize, mut _4: i32, mut _5: u64) -> [isize; 2] {
    let mut _13: ([bool; 1],) = ([false; 1],);
    let mut _14: [isize; 6] = [0; 6];
    let mut _29: [isize; 6] = [0; 6];
    let ret: [isize; 2] = [13; 2];
    let _6 = fn1(_2);
    fn5(ret, _2, _3, _2, _2, _4, _6);
    dump_var(_4, _5, 0, 0);
    dump_var([-112; 6], _13.0, 0, (4, 10));
    _14 = [(-9223372036854775808_isize); 6];
    dump_var((), (), _29, _14);
    return ret;
}

pub fn fn1(mut _10: [isize; 7]) -> *mut isize {
    let arr = [0; 3];
    let mut _21: (u8, u128, [u64; 3], isize, i16) = (0, 0, arr, 0, 0);
    let mut _24: (u8, u128, [u64; 3], isize, i16) = (0, 0, arr, 0, 0);
    dump_var(_10, _10, 0, 0);
    unsafe {
        _24.hash(&mut H);
        _21.hash(&mut H);
    }
    return core::ptr::addr_of_mut!(_21.3);
}
fn fn5(
    mut _2: [isize; 2],
    mut _6: [isize; 7],
    mut _7: usize,
    mut _9: [isize; 7],
    mut _10: [isize; 7],
    mut _12: i32,
    mut _16: *mut isize,
) {
    let mut _23: (
        *const usize,
        (usize, u8),
        (char, i32, (i64,), u64),
        char,
        (
            (f64, i64, i8),
            [f32; 1],
            (usize, u8),
            i8,
            (isize, i64),
            [f32; 1],
        ),
        *const u8,
    ) = (
        ptr::null(),
        (0, 0),
        ('a', 0, (0,), 0),
        'a',
        ((0., 0, 0), [0.; 1], (0, 0), 0, (0, 0), [0.; 1]),
        ptr::null(),
    );
    let mut _27: ((f64, i64, i8),) = Default::default();
    loop {
        _9 = _6;
        let mut _20 = (-9223372036854775808_isize) as i128;
        let mut _21 = (0,);
        loop {
            _23.0 = core::ptr::addr_of!(_23.1 .0);
            _23.4 .0 = (f64::NAN, (-1102345069964335552_i64), 9_i8);
            dump_var(0, 0, _6, _27.0 .1);
            _27.0 = _23.4 .0;
            match _27.0 .2 {
                9 => {
                    dump_var(_9, _21, _20, _12);
                    dump_var(_2, _10, _6, _7);
                    return;
                }
                1 => break,
                _ => unsafe {
                    (*_16) = 88_isize;
                },
            }
        }
    }
}
pub fn main() {
    println!(
        "{:?}",
        fn0(
            [(-56_isize); 7],
            15609822513776909592_usize,
            -652623562_i32,
            18399139786288871729_u64
        )
    );
    unsafe {
        println!("hash: {}", H.finish());
    }
}
