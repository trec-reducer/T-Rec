extern crate core;
use core::ptr;

pub fn dump_var(val0: u16) {
    println!("{val0}");
}

pub unsafe fn fn10_rs() -> *const *mut i64 {
    let mut _16: usize = 0;
    let mut _23: i128 = 0;
    let mut isize_min: isize = 0;
    let mut _46: i64 = 0;
    let mut _107: i16 = 0;
    let mut _47: u16 = 0;
    let mut _88: usize = 0;

    let mut _31: *mut u8 = ptr::null_mut();
    let mut _33: *mut *mut i64 = ptr::null_mut();
    let mut _81: *const i128 = ptr::null();
    let mut _90: *mut u8 = ptr::null_mut();
    let mut _177: *mut i64 = ptr::null_mut();

    let mut _44: ((u64, u64, u8), u32, f32) = Default::default();
    let mut _61: ((u64, u64, u8), u32, f32) = Default::default();
    let mut tup: (((u64, u64, u8), u32, f32), bool) = Default::default();
    let mut _95: (((u64, u64, u8), u32, f32), bool) = Default::default();

    'bb2: loop {
        let two: u16 = 2;
        _44.0 .2 = 1;
        _31 = core::ptr::addr_of_mut!(_44.0 .2);
        _23 = 11;
        'bb45: loop {
            (*_31) = 1;
            isize_min = isize::MIN;
            'bb65: loop {
                let tup_ptr = core::ptr::addr_of_mut!(tup);
                _31 = core::ptr::addr_of_mut!((*tup_ptr).0 .0 .2);
                _16 = 18215089233857363959_usize;
                match isize_min {
                    isize::MIN => {
                        _90 = _31.wrapping_offset(isize::MIN);
                        _95.0 = _44;
                        _46 = 42;
                        _81 = core::ptr::addr_of!(_23);
                        _44 = tup.0;
                        _88 = _16;
                        'bb80: loop {
                            _31 = _90.wrapping_offset(isize_min);
                            match *_81 {
                                11 => 'bb88: loop {
                                    let tup_ptr2 = core::ptr::addr_of_mut!(tup);
                                    (*tup_ptr2) = _95;
                                    isize_min = _107 as isize;
                                    _47 = two >> *_31;
                                    (*tup_ptr) = _95;
                                    match _88 {
                                        18215089233857363959 => {
                                            *_31 = _61.0 .2.swap_bytes();
                                            _88 = (*tup_ptr2).0 .2 as usize;
                                            _33 = core::ptr::addr_of_mut!(_177);
                                            match _46 {
                                                42 => {
                                                    (*_33) = core::ptr::addr_of_mut!(_46);
                                                    match *_177 {
                                                        42 => {
                                                            dump_var(_47);
                                                            return core::ptr::addr_of!(_177);
                                                        }
                                                        _ => continue 'bb2,
                                                    }
                                                }
                                                _ => match *_81 {
                                                    11 => continue 'bb88,
                                                    _ => continue 'bb65,
                                                },
                                            }
                                        }
                                        0 => continue 'bb80,
                                        _ => continue 'bb65,
                                    }
                                },
                                _ => continue 'bb65,
                            }
                        }
                    }
                    _ => continue 'bb45,
                }
            }
        }
    }
}

pub fn main() {
    unsafe {
        fn10_rs();
    }
}
