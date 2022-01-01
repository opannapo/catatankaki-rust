use crate::entity::user::User as user_entity;

pub fn structs1(data: user_entity) {
    println!("\n\nExample print struct parameter\n");
    println!("Name:{} , Age:{}", data.name, data.age);
}