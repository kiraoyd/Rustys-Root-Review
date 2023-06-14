//! Rusty's Root Review: create_mock_db crate (builds a postgres database locally that is a mockup of the real 'tuber' database)
//! main.rs - create the mock database and seed it
//! Author: Kira Klingenberg
//! Written for: Bart Massey's Programming in Rust, PSU Spring 2023
//! Last update: 6/13/2023

pub mod db;
pub mod models;

use db::create_db;
use std::env;

fn main() {
    let mut info = Vec::new();

    //process CLA
    for arg in env::args().skip(1) {
        info.push(arg.to_string());
    }
    if info.is_empty() {
        eprintln!("No args provided, please provide the name of your database, your postgres username (usually 'postgres') and your password for that user.");
        std::process::exit(1);
    }

    //CLA order:
    //1 - database name
    //2 - postgres username
    //3 - password for the postgres user

    create_db(info).unwrap();
}
