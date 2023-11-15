//! fake random things
pub mod animal;
pub mod locales;
pub mod faker;
pub mod fake_string;

#[cfg(doctest)]
/// `cat` random cat
/// # Example
/// 
/// '''
/// let mut faker_ins = faker::Faker::<locales::EN>::new();
/// let dog = faker_ins.animal().cat();
/// assert!(locales::EN::CATS.contains(&dog));
/// '''
fn test_doc_test() {

}

