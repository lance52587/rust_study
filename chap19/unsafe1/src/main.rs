fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe{
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        // println!("r is: {}", *r);
    }

    unsafe fn dangerous(){}
    
    unsafe {
        dangerous();
    }

    // dangerous();// requires unsafe function or block

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    let address = 0x01234usize;
    let r = address as *mut i32;

    let slice : &[i32] = unsafe{
        slice::from_raw_parts_mut(r, 10000)
    };

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

extern "C"{
    fn abs(input: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn call_from_c(){
    println!("Just called a Rust function from C!");
}

use std::slice;
fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]){
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    // (&mut slice[..mid], &mut slice[mid..])
    unsafe{
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}