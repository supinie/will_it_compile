fn do_something(x: i16, y: i16) -> i16 {
    fn something(_x: ()) {}
    something(return x + y);
}

fn ifff(a: i16, b: i16, c: i16, d: i16) -> bool {
    if if if a == b {
        b == c
    } else {
        a == c
    } {
        a == d
    } else {
        c == d
    } {
        true
    } else {
        false
    }
}

fn macros_r_fun() -> impl std::fmt::Debug {
    loop {
        if! {
            match! (
                break! {
                    return! {
                        42
                    }
                }
            )
            {}
        }
        {return! 69}
    }
}
