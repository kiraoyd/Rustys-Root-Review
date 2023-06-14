# Rustys Root Review
Kira Klingenberg

A web microservice built in Rust for the Animal Crossing New Horizons Turnip Trading site: Tuber Trader. 
Tuber Trader is a site for player of Animal Crossing New Horizons. 
In this game, every player owns an island they build up.
On each island you can buy turnips for a certain price each week, and resell for fluctuating prices throughout the week.
It's a bit of light gambling, with vegetables.
Players can visit each others islands to resell turnips, if their friends have better selling prices.
This site faciliates that process: users can create profiles for their islands and update current selling prices and other turnip related info.

This version of rustys-root-review is a standalone service that will create a simple mock postgres database representative of Tuber Traders 'tuber' database, and run the service against that mock db.
The seeded data for this mock database is very simple. 

The crate ```create_mock_db``` is responsible for generating the mock, pre-seeded, database tables.

The crate ```root_review``` is the actual service: complete with axum server, toxio runtime, CLA processing, and HTTP routes.

This microservice will query the database table for "Profile" and run simple analysis on the turnips held by each island. 

The fully integrated code for this microservice can be found in the repo for tuberTrader:
https://github.com/kiraoyd/tuberTrader

For information about what worked, what didn't, and the challenges overcame in this project, see the JOURNAL.md file.

Instructions on how to run this standalone project are below:

# Instructions

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

TEST ROUTES to run from the Postman application...

>...to get a JSON reply that shows the profile who spent the most on turnips, make a GET request to: **localhost:3333/spender**
>
>...to get a JSON reply that shows the max profits possible for island #6 at a selling price of 262, make a GET request to: **localhost:3333/profits/262/6**
>
>...to get a JSON reply that shows the most profitable island profile if the selling price is 110, make a GET request to: **localhost:3333/profits/110**




