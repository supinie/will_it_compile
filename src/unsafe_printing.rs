pub unsafe fn unsafe_print_bool_vec(vec: Vec<bool>) {
    for elem in vec {
        match elem {
            true => println!("on god"),
            false => println!("kapp"),
        }
    }
}

pub fn unsafe_print_int_vec(mut vec: Vec<i16>) {
    for chunk in vec.chunks_mut(2) {
        let r1 = &chunk[0] as * const i16;
        let r2 = &mut chunk[1] as *mut i16;

        unsafe {
            println!("{}", *r1);
            println!("{}", *r2);
        }
    }
}

