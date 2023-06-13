


pub mod db;
pub mod models;


use db::create_db;

fn main() {
    create_db().unwrap();
}
