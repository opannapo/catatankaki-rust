use crate::entity::user::User as user_entity;

pub fn structs1(data: user_entity) {
    println!("\n\nExample print struct parameter\n");
    println!("Name:{} , Age:{}", data.name, data.age);
}

pub fn structs2(data: user_entity) {
    println!("\n\nExample struct with impl. Get First Name from struct user name\n");

    println!("First Name:{}", data.get_first_name());
    println!("Last Name:{:?}", data.get_last_name());
}