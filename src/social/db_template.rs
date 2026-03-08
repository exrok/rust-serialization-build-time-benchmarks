use crate::models::Database;
use crate::compat;

pub fn load_db(path: &str) -> Database {
    let data = std::fs::read_to_string(path).unwrap();
    compat::from_json(&data)
}

pub fn save_db(path: &str, db: &Database) {
    let data = compat::to_json(db);
    std::fs::write(path, data).unwrap();
}
