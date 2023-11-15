


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

    fn test_base<F, C>(f: F, test_char: C)
    where
        F: Fn(&faker::Faker<LANG>, usize) -> String,
        C: Fn(char) -> bool
    {
        let faker_ins = faker::Faker::new();
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        for _ in 1..10 {
            let len = rng.gen_range(1..=100);
            let char = f(&faker_ins, len);
            assert_eq!(char.len(), len);
            assert!(char.chars().all(|c| test_char(c)));
        }
    }

    #[test]
    fn test_random_alpha() {
        test_base(
            |faker, length| faker.string.alpha(length),
            |c| c.is_alphabetic()
        ); 
    }

    #[test]
    fn test_random_alphanumeric() {
        test_base(
            |faker, length| faker.string.alphanumeric(length),
            |c| c.is_alphanumeric()
        ); 
    }


    #[test]
    fn test_rnadom_binary() {
        test_base(
            |faker, length| faker.string.binary(length),
            |c| c == '1' || c == '0'
        ); 
    }


    #[test]
    fn test_random_octal() {
        test_base(
            |faker, length| faker.string.octal(length),
            |c| c.is_digit(8)
        ); 
    }

    #[test]
    fn test_random_hex() {
        test_base(
            |faker, length| faker.string.hex(length),
            |c| c.is_digit(16)
        );
    }

    #[test]
    fn test_random_number() {
        test_base(
            |faker, length| faker.string.numeric(length),
            |c| c.is_numeric()
        );
    }


    #[test]
    fn test_random_sample() {
        test_base(
            |faker, length| faker.string.sample(length),
            |c| c.is_ascii_graphic()
        );
    }

    #[test]
    fn test_random_string() {
        test_base(
            |faker, length| faker.string.symbol(length),
            |c| c.is_ascii_punctuation()
        );
    }

}


mod test_number {
    use fake::faker::Faker;
    use fake::locales::DefaultLang;

    #[test]
    fn test_random_number() {
        
        let mut faker_ins: Faker<'_, DefaultLang> = Faker::new();
        let fake_num = faker_ins.number.number(1..i32::MAX);
        assert!(fake_num <= i32::MAX && fake_num >= 1);
        println!("number {}", fake_num);
    }
}
