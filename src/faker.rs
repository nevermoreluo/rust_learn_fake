use crate::locales;
use std::marker::PhantomData;
use crate::animal::Animal;
use crate::fake_string::FakeString;

pub struct Faker<'a, T: locales::Data> {
    _marker: PhantomData<&'a T>,
    pub rng: rand::rngs::ThreadRng,
    pub animal: Animal<'a, T, rand::rngs::ThreadRng>,
    pub string: FakeString
}


impl<'a, T: locales::Data> Faker<'a, T> {
    pub fn new() -> Self {
        Self { 
            _marker: PhantomData, 
            rng: rand::thread_rng(), 
            animal: Animal::new(rand::thread_rng()), 
            string: FakeString::new() 
        }
    }

}




