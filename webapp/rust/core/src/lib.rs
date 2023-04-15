pub mod db;
pub mod models;
pub mod repos;
pub mod services;

pub const ASSIGNMENTS_DIRECTORY: &str = "../assignments/";
pub const SQL_DIRECTORY: &str = "../sql/";
pub const INIT_DATA_DIRECTORY: &str = "../data/";
pub const MYSQL_ERR_NUM_DUPLICATE_ENTRY: u16 = 1062;
