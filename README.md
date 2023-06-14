# Rustys Root Review
Kira Klingenberg

A web microservice built in Rust for the in-development Animal Crossing New Horizons Turnip Trading site: Tuber Trader. 

https://github.com/kiraoyd/tuberTrader

This microservice will query Tuber Trader's Database table for "Profiles" and run simple analysis on the turnips held by each island. 

More will be added as development proceeds. Repo is currently Private and will be made public eventually.


# As of 6/13/2023

## Step 1: Create a postgres database locally called 'mocktuber'

When logged on as your postgres super user (usually named 'postgres'), create the mocktuber database on your machine, to do so:

 Access the psql command line: ```sudo -u postgres psql```

 Run: ```\l``` to see a list of all existing databases

 If 'mocktuber' does not alredy exist, run: ```CREATE DATABASE mocktuber;```
 

## Step 2: Run the create_mock_db crate:

From the root directory 'rustys-root-review':

 ```cd create_mock_db```

Run the crate and include command line arguments in this order: mocktuber, <your_postgres_user>, <your_postgres_users_password>:

```cargo run mocktuber <your_postgres_user> <your_postgres_users_password>```

```<your_postgres_user>``` needs to be your postgres superuser(with all privilages granted), usually named 'postgres'.

This should populate the mocktuber database locally with seeded tables.

To verify, from your psql command line run: 

>```\l``` to see a list of databases
> 
> ```\c mocktuber``` to connect to mocktuber as your postgres superuser
> 
>```\dt``` to view a list of tables in mock tuber
> 
> ```SELECT * FROM <tablename>``` to see all seeded data in a table
> 

### PLEASE NOTE: if you run the create_mock_db crate repeatedly with cargo run, the tables will remain unique, but the seed values will be added in duplicate
If you want to start over fresh, on the psql command line run:

```DROP DATABASE mocktuber```

Then rerun the ```CREATE DATABASE mocktuber;``` command to start a fresh empty mocktuber. 

Now you can run ```cargo run``` on the create_mock_db crate and the seeds and tables will all be unique.


## Step 3: Run the root_review crate:

From the root directory 'rustys-root-review':

```cd root_review```

 Run the crate and include command line arguments in this order: mocktuber, <your_postgres_user>, <your_postgres_users_password>:

 ```cargo run mocktuber, <your_postgres_user>, <your_postgres_users_password> ```

```<your_postgres_user>``` needs to be your postgres superuser(with all privilages granted), usually named 'postgres'.


The server should now be listening and ready...

## STEP 4: Hit the routes via postman

https://www.postman.com/
From the Postman application...

>...to get a JSON reply that shows the profile who spent the most on turnips, make a GET request to: **localhost:3333/spender**
>
>...to get a JSON reply that shows the max profits possible for island #6 at a selling price of 262, make a GET request to: **localhost:3333/profits/262/6**
>
>...to get a JSON reply that shows the most profitable island profile if the selling price is 110, make a GET request to: **localhost:3333/profits/110**


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
