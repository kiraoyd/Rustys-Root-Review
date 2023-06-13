use postgres::{Client, NoTls, Error};
//use crate::models::tuber_tables::IPHistory;
use crate::models::tuber_tables::Profile;
use crate::models::tuber_tables::User;
use root_review::EnvOptions;

//postgres crate v0.19.5: https://docs.rs/postgres/latest/postgres/
//postgres create db tutorial: https://rust-lang-nursery.github.io/rust-cookbook/database/postgres.html
//Assumes an existing database called 'tuber'
//Username: postgres, password: postgres

//Messed around with trying to use the establish_connection method here, but couldn't get it working right
pub fn create_mock_db() -> Result<(), Error> {
    //panic if any of these err
    create_tables().unwrap();
    seed_user_table().unwrap();
    seed_profile_table().unwrap();

    Ok(()) //indicate success
}


pub fn create_tables() -> Result<(), Error> {
    let env_opts = EnvOptions::new();
    //connect to the db we want to run the SQL on
    let mut client = Client::connect(&env_opts.mock_tuber_db_url, NoTls)?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS user (
            id      SERIAL PRIMARY KEY,
            name      VARCHAR NOT NULL,
            email   VARCHAR NOT NULL,
            password    VARCHAR NOT NULL,
            role        TEXT,
            )
    ")?;

    client.batch_execute("
        CREATE TABLE IF NOT EXISTS iphistory (
            id      SERIAL PRIMARY KEY,
            ip      VARCHAR,
            )
    ")?;
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS profile (
            id      SERIAL PRIMARY KEY,
            island_name      VARCHAR,
            picture         VARCHAR,
            owner_id   INTEGER NOT NULL REFERENCES user,
            turnips_held    INTEGER,
            price_paid  INTEGER,
            )
    ")?;

    Ok(())
}


//I already have structs to represent the tables in this db
//the seeder will create a vec of structs to populate the db with for each table type, and insert the values via SQL
pub fn seed_profile_table() -> Result<(), Error> {
    let env_opts = EnvOptions::new();
    //connect to the db we want to run the SQL on
    let mut client = Client::connect(&env_opts.mock_tuber_db_url, NoTls)?;

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
        }
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

pub fn seed_user_table() -> Result<(), Error> {
    let env_opts = EnvOptions::new();
    //connect to the db we want to run the SQL on
    let mut client = Client::connect(&env_opts.mock_tuber_db_url, NoTls)?;

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
    ];

    //iterate through the vec and grab the values from each struct to give to the SQL query to insert
    for user in &users {
        client.execute(
            "INSERT INTO profile (name, picture, turnips_held, price_paid, owner_id) VALUES ($1, $2, $3, $4, $5)",
            &[&user.name, &user.email, &user.password, &user.role],
        )?;
    }

    Ok(())
}