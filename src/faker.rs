use crate::locales;
use std::marker::PhantomData;
use crate::animal::Animal;
use crate::fake_string::FakeString;
use crate::fake_number::FakeNumber;

pub struct Faker<'a, T: locales::Data> {
    _marker: PhantomData<&'a T>,
    pub rng: rand::rngs::ThreadRng,
    pub animal: Animal<'a, T, rand::rngs::ThreadRng>,
    pub string: FakeString,
    pub number: FakeNumber
}


impl<'a, T: locales::Data> Faker<'a, T> {
    pub fn new() -> Self {
        Self { 
            _marker: PhantomData, 
            rng: rand::thread_rng(), 
            animal: Animal::new(rand::thread_rng()), 
            string: FakeString::new(),
            number: FakeNumber::new()
        }
    }

}

type DefaultLang = locales::EN;

impl<'a> Faker<'a, DefaultLang> {
    pub fn default() -> Self {
        Self::new()
    }
}


pub struct Fake<'a> {
    pub rng: rand::rngs::ThreadRng,
    pub animal: Animal<'a, DefaultLang, rand::rngs::ThreadRng>,
    pub string: FakeString,
    pub number: FakeNumber
}


impl<'a> Fake<'a> {
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
            animal: Animal::new(rand::thread_rng()),
            string: FakeString::new(),
            number: FakeNumber::new()
        }
    }
}
