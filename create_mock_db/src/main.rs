
pub mod db;
pub mod models;

use std::env;
use db::create_db;

fn main() {

    let mut info = Vec::new();

    //process CLA
    for arg in env::args().skip(1) {
        info.push(arg.to_string());
    }

    if info.is_empty(){
        eprintln!("No args provided, please provide the name of your database, your postgres username (usually 'postgres') and your password for that user.");
        std::process::exit(1);
    }

    //1 - database name
    //2 - postgres username
    //3 - password for the postgres user in #2

    create_db(info).unwrap();
}
