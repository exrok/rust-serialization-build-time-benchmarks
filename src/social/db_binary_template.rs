use crate::models::Database;
use crate::compat;

pub fn load_db(path: &str) -> Database {
    let data = std::fs::read(path).unwrap();
    compat::from_binary(&data)
}

pub fn save_db(path: &str, db: &Database) {
    let data = compat::to_binary(db);
    std::fs::write(path, data).unwrap();
}
