


#[cfg(test)]
mod test_animal{
    type LANG = locales::EN;
    // use fake::animal::*;
    use fake::locales;
    use fake::faker;
    use fake::locales::Data;

    #[test]
    fn test_random_dog() {

        let mut faker_ins = faker::Faker::<LANG>::new();
        let dog = faker_ins.animal.dog();
        assert!(LANG::DOGS.contains(&dog));
        
        println!("Random Dog: {}", dog);
    }

    #[test]
    fn test_random_cat() {

        let mut faker_ins = faker::Faker::<LANG>::new();
        let dog = faker_ins.animal.cat();
        assert!(LANG::CATS.contains(&dog));
        
        println!("Random Dog: {}", dog);
    }


}

#[cfg(test)]
mod test_string {
    use rand::Rng;
    use fake::locales;
    use fake::faker;
    type LANG = locales::EN;    // use fake::animal::*;

    #[test]
    fn test_random_alpha() {

        let faker_ins = faker::Faker::<LANG>::new();
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        for _ in 1..10 {
            let len = rng.gen_range(1..=100);
            let char = faker_ins.string.alpha(len);
            assert_eq!(char.len(), len);
            assert!(char.chars().all(|c| c.is_alphabetic()));
        }
        
    }

    #[test]
    fn test_random_alphanumeric() {
        
        let faker_ins = faker::Faker::<LANG>::new();
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        for _ in 1..10 {
            let len = rng.gen_range(1..=100);
            let char = faker_ins.string.alphanumeric(len);
            assert_eq!(char.len(), len);
            assert!(char.chars().all(|c| c.is_alphanumeric()));
        }
    }
}
