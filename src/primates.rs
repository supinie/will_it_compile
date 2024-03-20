use std::any::TypeId;
use std::collections::HashMap;
use std::any::Any;
use crate::maps::{another_byte_operation, last_one_i_promise};

// Define a trait and some concrete types implementing it
pub trait GoCrazy {
    fn go_bonkers(&self) -> Vec<i16>;
}

pub struct Monke {
    #[warn(non_snake_case)]
    pub DNA: [u8; 24],
}
impl GoCrazy for Monke {
    fn go_bonkers(&self) -> Vec<i16> {
        another_byte_operation(&self.DNA)
    }
}

pub struct Gorilla {
    #[warn(non_snake_case)]
    pub DNA: [u8; 24],
}
impl GoCrazy for Gorilla {
    fn go_bonkers(&self) -> Vec<i16> {
        last_one_i_promise(&self.DNA)
    }
}

#[macro_export]
macro_rules! random_primate {
    ($type:ident) => {{
        let mut rng = rand::thread_rng();
        $type { DNA: rng.gen::<[u8; 24]>() }
    }};
}

// Define a struct that holds a HashMap of type erasure
pub struct Barrel {
    pub raw: HashMap<TypeId, Box<dyn Any>>,
}

impl Barrel {
    // Function to insert a value into the collection
    pub fn insert<T: 'static>(&mut self, value: T) -> Option<T> 
    where
        T: GoCrazy
    {
        self.raw.insert(TypeId::of::<T>(), Box::new(value) as Box<dyn Any>)
            .map(|any| *any.downcast::<T>().unwrap())
    }
}

