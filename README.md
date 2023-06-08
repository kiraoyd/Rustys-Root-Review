# Rustys Root Review
Kira Klingenberg

A web microservice built in Rust for the in-development Animal Crossing New Horizons Turnip Trading site: Tuber Trader. 

https://github.com/kiraoyd/tuberTrader

This microservice will query Tuber Trader's Database table for "Profiles" and run simple analysis on the turnips held by each island. 

More will be added as development proceeds. Repo is currently Private and will be made public eventually.

# As of 6/7/2023...

### ----- Please ignore the Docker files for now ----

## Locally Restoring and Connecting to the database: 'tuber'

### NOTE: If this is your very first time restoring the tuber database on a machine from the tuber_dump.sql provided here....
 
....you will need to create the tuber database on your machine, to do so:
> Access the psql command line: ```sudo -u postgres psql```
> 
> Run: ```\l``` to see a list of all existing databases
> 
> If 'tuber' does not alredy exist, run: ```CREATE DATABASE tuber```
> 
 ...you will need to create the postgres user called 'tuber', to do so:

> Access the psql command line: ```sudo -u postgres psql```
> 
> Check existing users: ```\du```
> 
> If no user 'tuber' exists, create one: ```CREATE USER tuber WITH PASSWORD 'tuber';``` Upon success you will see the "CREATE ROLE" message
> 
> Grant privilages to the tuber role: ```GRANT ALL PRIVILEGES ON DATABASE tuber to tuber;```

1. Copy the current version of the tuber_dump.sql file onto your local machine, note the file path to this copy (if you have a copy of an earlier version of this file, please delete it)
2. To restore the database in it's pre-seeded form, from the linux command line run: ```psql -h localhost -U <your_postgres_user_name> -W -d tuber -f <file/path/to/copy_of_tuber_dump.sql>```
3. To connect to tuber from the psql command line: ```\c tuber```
4. To view all existing tables in the tuber schema run: ```\dt```

    Here is a list of public tables owned by tuber:
    
    >iphistory
    > 
    >mikro_orm_migrations
    > 
    >profile
    > 
    >selling_price_history
    > 
    >transactions
    > 
    >users

5. Once connected, run the program and hit the routes via postman, instructions below

### To run (from the top level in rustys-root-review)

```agsl
cd root_review
cargo run
```

https://www.postman.com/
Once connected, from Postman...

>...to get a JSON reply that shows the profile who spent the most on turnips, make a GET request to: localhost:3333/spender
>
>...to get a JSON reply that shows the max profits possible for island #6 at a selling price of 262, make a GET request to: localhost:3333/profits/262/6
>
>...to get a JSON reply that shows the most profitable island profile if the selling price is 110, make a GET request to: localhost:3333/profits/110

Database information for "tuber" pulled from the .env for Tuber Trader:
```
DB_HOST=localhost
DB_PORT=5432
DB_NAME=tuber
DB_USER=tuber
DB_PASS=tuber
```
