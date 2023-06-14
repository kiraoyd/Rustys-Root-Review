//! Rusty's Root Review: create_mock_db crate (builds a postgres database locally that is a mockup of the real 'tuber' database)
//! db.rs - runs the SQL to create tables and seed the, with fake data
//! Author: Kira Klingenberg
//! Written for: Bart Massey's Programming in Rust, PSU Spring 2023
//! Last update: 6/13/2023
//! Citations:
//! postgres crate v0.19.5: https://docs.rs/postgres/latest/postgres/
//! postgres create db tutorial: https://rust-lang-nursery.github.io/rust-cookbook/database/postgres.html

use postgres::{Client, Error, NoTls};
use crate::models::tuber_tables::IPHistory;
use crate::models::tuber_tables::Profile;
use crate::models::tuber_tables::User;

//TODO add seeder for ip history and bulk up other seeders

///Wrapper function that calls methods to create all the mock db tables, and seed them with fake data
//Messed around with trying to use the establish_connection method here, but couldn't get it working right
pub fn create_db(info: Vec<String>) -> Result<(), Error> {
    //panic if any of these err
    create_tables(info.clone()).unwrap();
    seed_user_table(info.clone()).unwrap();
    seed_profile_table(info.clone()).unwrap();
    seed_iph_table(info).unwrap();

    Ok(()) //indicate success
}

///Generates SQL to create the users, profile, and iphistory tables
pub fn create_tables(info: Vec<String>) -> Result<(), Error> {
    //connect to the db we want to run the SQL on
    let database_name = info[0].as_str();
    let postgres_username = info[1].as_str();
    let user_password = info[2].as_str();
    let url = format!(
        "postgresql://{}:{}@localhost/{}",
        postgres_username, user_password, database_name
    );

    //let mut client = Client::connect("postgresql://postgres:snaxkYs23@localhost/mocktuber", NoTls)?;
    let mut client = Client::connect(url.as_str(), NoTls)?;

    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS users (
            id              SERIAL PRIMARY KEY,
            name            VARCHAR,
            email           VARCHAR,
            password        VARCHAR,
            role            VARCHAR
            )
    ",
    )?;

    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS iphistory (
            id              SERIAL PRIMARY KEY,
            ip              VARCHAR
            )
    ",
    )?;

    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS profile (
            id              SERIAL PRIMARY KEY,
            island_name     VARCHAR,
            picture         VARCHAR,
            owner_id        INTEGER NOT NULL REFERENCES users,
            turnips_held    INTEGER,
            price_paid      INTEGER
            )
    ",
    )?;

    Ok(())
}

///Seeds the profile table
//I already have structs to represent the tables in this db
//the seeder will create a vec of structs to populate the db with for each table type, and insert the values via SQL
pub fn seed_profile_table(info: Vec<String>) -> Result<(), Error> {
    //connect to the db we want to run the SQL on
    let database_name = info[0].as_str();
    let postgres_username = info[1].as_str();
    let user_password = info[2].as_str();
    let url = format!(
        "postgresql://{}:{}@localhost/{}",
        postgres_username, user_password, database_name
    );
    let mut client = Client::connect(url.as_str(), NoTls)?;

    let profiles = vec![
        Profile {
            id: 1,
            island_name: "orjeene".to_string(),
            picture: "http://placeholder.com/mypic.jpeg".to_string(), //otherwise &str type for literals
            turnips_held: 1100,
            price_paid: 93,
            owner_id: 2,
        },
        Profile {
            id: 2,
            island_name: "popcorn".to_string(),
            picture: "http://placeholder.com/mypic.jpeg".to_string(),
            turnips_held: 100,
            price_paid: 100,
            owner_id: 1,
        },
        Profile {
            id: 3,
            island_name: "squirtle".to_string(),
            picture: "http://placeholder.com/mypic.jpeg".to_string(),
            turnips_held: 2000,
            price_paid: 110,
            owner_id: 3,
        },
        Profile {
            id: 4,
            island_name: "pear".to_string(),
            picture: "http://placeholder.com/mypic.jpeg".to_string(),
            turnips_held: 4050,
            price_paid: 106,
            owner_id: 1,
        },
        Profile {
            id: 5,
            island_name: "fakeorjeene".to_string(),
            picture: "http://placeholder.com/mypic.jpeg".to_string(),
            turnips_held: 500,
            price_paid: 100,
            owner_id: 3,
        },
        Profile {
            id: 6,
            island_name: "melon".to_string(),
            picture: "http://placeholder.com/mypic.jpeg".to_string(),
            turnips_held: 3045,
            price_paid: 109,
            owner_id: 4,
        },
        Profile {
            id: 7,
            island_name: "makeItRain".to_string(),
            picture: "http://placeholder.com/mypic.jpeg".to_string(),
            turnips_held: 6000,
            price_paid: 93,
            owner_id: 5,
        },
        Profile {
            id: 7,
            island_name: "bigSpender".to_string(),
            picture: "http://placeholder.com/mypic.jpeg".to_string(),
            turnips_held: 6000,
            price_paid: 120,
            owner_id: 1,
        },
    ];

    //iterate through the vec and grab the values from each struct to give to the SQL query to insert
    //NOTE: id being a SERIAL PRIMARY KEY will be auto generated, we don't need to specify here
    for profile in &profiles {
        client.execute(
            "INSERT INTO profile (island_name, picture, turnips_held, price_paid, owner_id) VALUES ($1, $2, $3, $4, $5)",
            &[&profile.island_name, &profile.picture, &profile.turnips_held, &profile.price_paid, &profile.owner_id],
        )?;
    }

    Ok(())
}

///Seeds the users table
pub fn seed_user_table(info: Vec<String>) -> Result<(), Error> {
    //connect to the db we want to run the SQL on
    let database_name = info[0].as_str();
    let postgres_username = info[1].as_str();
    let user_password = info[2].as_str();
    let url = format!(
        "postgresql://{}:{}@localhost/{}",
        postgres_username, user_password, database_name
    );
    let mut client = Client::connect(url.as_str(), NoTls)?;

    let users = vec![
        User {
            id: 1,
            name: "kirak".to_string(),
            email: "email@email.com".to_string(),
            password: "password".to_string(),
            role: "Admin".to_string(),
        },
        User {
            id: 2,
            name: "otherGuy".to_string(),
            email: "email@email.com".to_string(),
            password: "password".to_string(),
            role: "User".to_string(),
        },
        User {
            id: 3,
            name: "olimo".to_string(),
            email: "email@email.com".to_string(),
            password: "password".to_string(),
            role: "User".to_string(),
        },
        User {
            id: 4,
            name: "bender".to_string(),
            email: "email@email.com".to_string(),
            password: "password".to_string(),
            role: "User".to_string(),
        },
        User {
            id: 5,
            name: "vimes".to_string(),
            email: "cranky@email.com".to_string(),
            password: "password".to_string(),
            role: "User".to_string(),
        },
    ];

    //iterate through the vec and grab the values from each struct to give to the SQL query to insert
    for user in &users {
        client.execute(
            "INSERT INTO users (name, email, password, role) VALUES ($1, $2, $3, $4)",
            &[&user.name, &user.email, &user.password, &user.role],
        )?;
    }

    Ok(())
}

///Seeds the iphistory table (needed only for the test routes, not actually used by the microservice)
pub fn seed_iph_table(info: Vec<String>) -> Result<(), Error> {
    //connect to the db we want to run the SQL on
    let database_name = info[0].as_str();
    let postgres_username = info[1].as_str();
    let user_password = info[2].as_str();
    let url = format!(
        "postgresql://{}:{}@localhost/{}",
        postgres_username, user_password, database_name
    );
    let mut client = Client::connect(url.as_str(), NoTls)?;

    let ips = vec![
        IPHistory {
            ip: "227.247.179.85".to_string(),
        },
        IPHistory {
            ip: "29.17.217.77".to_string(),
        },
        IPHistory {
            ip: "43.171.185.204".to_string(),
        },
        IPHistory {
            ip: "106.2.44.76".to_string(),
        },
        IPHistory {
            ip: "122.138.62.45".to_string(),
        },
        IPHistory {
            ip: "166.189.162.62".to_string(),
        },
        IPHistory {
            ip: "166.165.70.44".to_string(),
        },
        IPHistory {
            ip: "95.161.112.143".to_string(),
        },
    ];

    //iterate through the vec and grab the values from each struct to give to the SQL query to insert
    for ip in &ips {
        client.execute("INSERT INTO iphistory(ip) VALUES ($1)", &[&ip.ip])?;
    }

    Ok(())
}
