
use rand::Rng;


/**
 * Module to generate numbers of any kind.
 *
 * ### Overview
 *
 * For simple integers, use `int()`. For decimal/floating-point numbers, use `float()`.
 *
 * For numbers not in base-10, you can use `hex()`, `octal()` and `binary()`.
 *
 * ### Related modules
 *
 * - For numeric strings of a given length, use `faker.string.numeric()`.
 * - For credit card numbers, use `faker.finance.creditCardNumber()`.
 */
pub struct FakeNumber {
    rng: rand::rngs::ThreadRng,
}

impl FakeNumber {
    pub fn new() -> Self {
        FakeNumber {
            rng: rand::thread_rng(),
        }
    }


    

    /**
     * Returns a single random nember between the given range.
     *
     *
     * @example
     * faker.number.number(1..100) // 66
     * faker.number.number(0.0..10.0) // 1.0
     * faker.number.number(0.1f64..100.3f64) // 23.5f64
     */
    pub fn number<T, R>(&mut self,
        range: R
    ) -> T
    where
        T: rand::distributions::uniform::SampleUniform,
        R: rand::distributions::uniform::SampleRange<T> {
            self.rng.gen_range(0.1f64..100.3f64);
        self.rng.gen_range(range)
        
    }
}