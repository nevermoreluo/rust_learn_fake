use crate::locales;
use std::marker::PhantomData;
use rand::seq::SliceRandom;

pub struct Animal<'a, T: locales::Data, R: rand::Rng>
{
    _marker: PhantomData<&'a T>,
    rng:  R
}


impl<'a, T: locales::Data, R: rand::Rng> Animal<'a, T, R>
{

    pub fn new(rng: R) -> Self {
        Self { _marker: PhantomData, rng: rng }
    }
    

    /// `cat` 随机一种猫
    /// # Example
    /// 
    /// '''
    /// let mut faker_ins = faker::Faker::<locales::EN>::new();
    /// let dog = faker_ins.animal().cat();
    /// assert!(locales::EN::CATS.contains(&dog));
    /// '''
    pub fn cat(&mut self) -> &str {
        T::CATS.choose(&mut self.rng).unwrap()
    }

    pub fn dog(&mut self) -> &str {
        T::DOGS.choose(&mut self.rng).unwrap()
    }

}

