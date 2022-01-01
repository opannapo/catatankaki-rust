pub struct User {
    pub name: String,
    pub age: i32,
}

impl User {
    //Pakai self pointer supaya reference ke ownership tidak berubah
    pub fn get_first_name(&self) -> String {
        let name = &self.name;
        let s: Vec<&str> = name.split(' ').collect();
        return String::from(s[0]);
    }

    //Pakai self pointer supaya reference ke ownership tidak berubah
    pub fn get_last_name(&self) -> String {
        let name = &self.name;
        let s: Vec<&str> = name.split(' ').collect();
        return String::from(s[s.len() - 1]);
    }
}