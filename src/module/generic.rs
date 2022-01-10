use crate::entity::user::User;

trait Repository<T> {
    fn get() -> Result<T, Box<dyn std::error::Error>>;
}

struct FavoriteCategoryRepo {}

impl Repository<User> for FavoriteCategoryRepo {
    fn get() -> Result<User, Box<dyn std::error::Error>> {
        let p = User { name: String::from("Opannapo"), age: 33 };
        Ok(p)
    }
}

pub fn generic1() {
    let result = FavoriteCategoryRepo::get().unwrap();
    println!("Result : {}", result.name);
}