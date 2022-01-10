pub struct Person {
    name: String,
}

trait Repository<T> {
    fn get() -> Result<T, Box<dyn std::error::Error>>;
}

struct FavoriteCategoryRepo {}

impl Repository<Person> for FavoriteCategoryRepo {
    fn get() -> Result<Person, Box<dyn std::error::Error>> {
        let p = Person { name: String::from("Budi") };
        Ok(p)
    }
}

pub fn generic1() {
    let result = FavoriteCategoryRepo::get().unwrap();
    println!("Result : {}", result.name);
}