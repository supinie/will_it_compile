use std::collections::HashMap;
use std::any::TypeId;
use rand::Rng;
use crate::{
    primates::{Barrel, Gorilla, Monke, GoCrazy},
    funky::{do_something, ifff, macros_r_fun},
    unsafe_printing::{unsafe_print_bool_vec, unsafe_print_int_vec},
};

mod funky;
mod maps;
mod primates;
mod unsafe_printing;

fn main() {
    let mut barrel = Barrel {
        raw: HashMap::new(),
    };

    barrel.insert(random_primate!(Gorilla));

    let inserted_monke = barrel.insert(random_primate!(Monke));

    match inserted_monke {
        Some(_) => println!("OOH oOH AHahAH OOoh AH"),
        None => println!("*sad monke noises*"),
    }

    let mut gorilla_genome: Vec<i16> = Vec::new();
    let mut monke_genome: Vec<i16> = Vec::new();

    if let Some(primate) = barrel.raw.get(&TypeId::of::<Gorilla>()) {
        let gorilla: &Gorilla = primate.downcast_ref::<Gorilla>().unwrap();
        gorilla_genome = gorilla.go_bonkers();
    }

    if let Some(primate) = barrel.raw.get(&TypeId::of::<Monke>()) {
        let monke: &Monke = primate.downcast_ref::<Monke>().unwrap();
        monke_genome = monke.go_bonkers();
    }

    macros_r_fun();

    let sequenced_gorilla_genome: Vec<i16> = gorilla_genome
        .chunks(2)
        .map(|chunk| {
            match chunk {
                [x, y] => {
                    do_something(*x, *y)
                },
                _ => panic!("weird gorilla..."),
            }
        })
        .collect();

    let sequenced_monke_genome: Vec<bool> = monke_genome
        .chunks(4)
        .map(|chunk| {
            match chunk {
                [x, y, z, q] => ifff(*x, *y, *z, *q),
                _ => panic!("that's no moon - i mean monke"),
            }
        })
        .collect();

    unsafe {
        unsafe_print_bool_vec(sequenced_monke_genome);
    }
    unsafe_print_int_vec(sequenced_gorilla_genome);
}

