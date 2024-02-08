use std::ptr;
pub fn print_var(v: u8) {
    println!("{v}");
}
pub unsafe fn fn12_rs() -> ([u128; 7], *mut i8, *mut bool) {
    let mut v2: bool = false;
    let mut v8: u64 = 0;
    let mut v9: usize = 0;
    let mut v12: *mut u8 = ptr::null_mut();
    let mut v17: *mut bool = ptr::null_mut();
    let mut v20: [u8; 8] = Default::default();
    let mut v21: [u8; 8] = Default::default();
    let mut v31: (bool, u8, usize, f32) = Default::default();
    let mut v33: ([u128; 7], *mut i8, *mut bool) = ([0; 7], ptr::null_mut(), ptr::null_mut());
    let mut v39: (usize, [u128; 7], ([u32; 6], usize, *mut [u32; 6]), [u32; 2]) =
        (0, [0; 7], ([0; 6], 0, ptr::null_mut()), [0; 2]);
    let mut ret: ([u128; 7], *mut i8, *mut bool) = ([0; 7], ptr::null_mut(), ptr::null_mut());
    ret.2 = core::ptr::addr_of_mut!(v2);
    'l0: loop {
        v12 = core::ptr::addr_of_mut!(v20[v9]);
        v20 = [197_u8; 8];
        v9 = 2_usize;
        'l1: loop {
            match *v12 {
                197 => {
                    // Taken
                    v8 = 13978819448286864680_u64;
                    v33.2 = ret.2;
                    match v39.0 {
                        0 => {
                            // Taken
                            'l2: loop {
                                v20 = [11_u8; 8]; // What LLVM with low mir-opt prints
                                (*v12) = 22; // What Miri prints
                                'l3: loop {
                                    v21 = v20;
                                    match v8 {
                                        13978819448286864680 => {
                                            // Taken
                                            v39.2 .0 = [2262110980_u32; 6];
                                            v8 = !13152832795211590855_u64;
                                            v39.0 = 6;
                                            v17 = v33.2;
                                            v33.2 = core::ptr::addr_of_mut!(v31.0);
                                            v31.1 = *v12;
                                            (*v17) = true;
                                            v20 = v21;
                                            match v39.0 {
                                                6 => {
                                                    // Taken
                                                    print_var(v31.1);
                                                }
                                                0 => continue 'l2,
                                                _ => return ret,
                                            }
                                        }
                                        _ => continue 'l0,
                                    }
                                }
                            }
                        }
                        _ => return ret,
                    }
                }
                4 => {
                    v12 = core::ptr::addr_of_mut!(v20[v9]);
                }
                _ => return ret,
            }
        }
    }
}
pub fn main() {
    unsafe {
        fn12_rs();
    }
}
