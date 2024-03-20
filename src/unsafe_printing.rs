pub unsafe fn unsafe_print_bool_vec(vec: Vec<bool>) {
    print!("[");
    for (i, elem) in vec.iter().enumerate() {
        if i != 0 {
            print!(", ");
        }
        match elem {
            true => print!("on god"),
            false => print!("kapp"),
        }
    }
    print!("]\n");
}

pub fn unsafe_print_int_vec(mut vec: Vec<i16>) {
    print!("[");
    let length = vec.len();
    for (i, chunk) in vec.chunks_mut(2).enumerate() {
        let r1 = &chunk[0] as * const i16;
        let r2 = &mut chunk[1] as *mut i16;

        unsafe {
            print!("{}, ", *r1);
            print!("{}", *r2);
        }
        if i != (length / 2) - 1 {
            print!(", ");
        }
    }
    print!("]\n");
}

