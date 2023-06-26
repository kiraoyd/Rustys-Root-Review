# Rustys Root Review
Kira Klingenberg

2023

A web microservice built in Rust for the Animal Crossing New Horizons Turnip Trading site: Tuber Trader. 

Tuber Trader is a site for player of Animal Crossing New Horizons. 
In this game, every player owns an island they build up.
On each island you can buy turnips for a certain price each week, and resell for fluctuating prices throughout the week.
It's a bit of light gambling, with vegetables.
Players can visit each others islands to resell turnips, if their friends have better selling prices.
This site faciliates that process: users can create profiles for their islands and update current selling prices and other turnip related info.

This version of rustys-root-review is a standalone demonstration of the service.
It is made up of **two Rust crates**, one that will create a simple mock postgres database representative of Tuber Traders 'tuber' database, and one that will run the service against that mock db.
The seeded data for this mock database is very basic for the purpose of demonstration.

This version does NOT connect to the real tuber database for Tuber Trader.

### CRATES

The crate ```create_mock_db``` is responsible for generating the mock, pre-seeded, database tables in a postgres database called 'mocktuber'.
Instructions on how to run this crate are provided below in the "instructions" section of this README.

----------------------------------------------------

The crate ```root_review``` is the actual service: complete with axum server, toxio runtime, CLA processing, and HTTP routes.
Instructions on how to run this crate are provided below in the "instructions" section of this README.

This routes in this crate will query the mocktuber database table for "Profile" and run simple analysis on the turnips held by each island. 

The fully integrated code for the actual root_review microservice can now also be found in the repo for Tuber Trader:
https://github.com/kiraoyd/tuberTrader

----------------------------------------------------


### JOURNAL OF ISSUES

For information about what worked, what didn't, and the challenges overcame in this project, see the JOURNAL.md file.

----------------------------------------------------

### PRESENTATION VIDEO
A demo run of the application is available in the project presentation video recorded for Bart Massey's class.
A link to the zoom recording can be found here: 

https://pdx.zoom.us/rec/share/jjU8UDgne7ZcjDJ7rykIvxbaJqFip-zquT2uvE79mkjHp9DUN27ywTyRQJ1XILY.ye9yeHQlRbtrfVij?startTime=1686797618000

You may need to be signed in to zoom with your pdx.edu account to view it.

Bart, I also emailed you a copy of the downloaded video, just in case! :)

----------------------------------------------------

### CITATIONS

Citations are provided in the files as comments, and other resources are mentioned in the JOUNRAL.md file.

I'd like to double cite Casey Bailey here as well. 
The template for the server, router, and AppError work in the root_review crate came directly from his doggr_w23 repo example: auth_rs.
A copy of this repo can be found here: https://github.com/kiraoyd/doggr_w23/tree/master/auth_rs

### LICENSE

This work is licensed under the 'MIT License'. Please see the file LICENSE.txt in this distribution for license terms.

----------------------------------------------------

# Instructions

## Step 1: Create a postgres database locally called 'mocktuber'

When logged on as your postgres super user (usually named 'postgres'), create the mocktuber database on your machine:

 Access the psql command line from the linux command line: ```sudo -u postgres psql```

 Run: ```\l``` to see a list of all existing databases

 If 'mocktuber' does not alredy exist, run: ```CREATE DATABASE mocktuber;```
 

## Step 2: Run the create_mock_db crate:

On the linux command line, from the root directory 'rustys-root-review':

 ```cd create_mock_db```

Run the crate and include command line arguments in this order:
```cargo run mocktuber <your_postgres_user> <your_postgres_users_password>```

This should populate the mocktuber database locally with seeded tables.

----------------------------------------------------


PLEASE NOTE: ```<your_postgres_user>``` needs to be your postgres superuser(with all privilages granted), usually named 'postgres'.


----------------------------------------------------
**To verify the database has data, from your psql command line run:** 

```\l``` to see a list of databases
 
 ```\c mocktuber``` to connect to mocktuber as your postgres superuser
 
```\dt``` to view a list of tables in mock tuber
 
 ```SELECT * FROM <tablename>;``` to see all seeded data in a table

----------------------------------------------------

#### PLEASE NOTE: if you run the create_mock_db crate repeatedly with cargo run, the tables will remain unique, but the seed values will be added in duplicate


If you want to start over fresh, on the psql command line run: ```DROP DATABASE mocktuber;```

Then rerun the ```CREATE DATABASE mocktuber;``` command to start a fresh empty mocktuber. 

Then from the linux command line you can run ```cargo run mocktuber <your_postgres_user> <your_postgres_users_password>``` and the seeds and tables will all be reproduced uniquely.


## Step 3: Run the root_review crate:

On the linux command line, from the root directory 'rustys-root-review' of this repo:

```cd root_review```

Create an ENV file called ```.env```.

In the ENV create a variable called "MOCK_TUBER_DB_URL", you may leave it empty, the URL string will be auto generated based off the command line args you provide in the next step.

 Run the crate and include command line arguments in this order: 

 ```cargo run mocktuber <your_postgres_user> <your_postgres_users_password> ```

----------------------------------------------------


PLEASE NOTE: ```<your_postgres_user>``` needs to be your postgres superuser(with all privilages granted), it is usually named 'postgres'.

----------------------------------------------------


The server should now be listening and ready...

## STEP 4: Hit the routes via postman

https://www.postman.com/

TEST ROUTES to run from the Postman application...

...to get a JSON reply that shows the profile island that spent the most on turnips, make a **GET** request to: 
### **localhost:3333/spender**

 This route should return this JSON response data:
 ```json
{
"island": "bigSpender",
"turnip_quantity": 6000,
"price_paid": 120,
"total_spent": 720000,
"owner_name": "kirak"
}
```
----------------------------------------------------
...to get a JSON reply that shows the max profits possible for island #6 at a selling price of 262, make a GET request to:
### **localhost:3333/profits/262/6**

This route should return this JSON response data:
```json
{
"island": "melon",
"turnip_quantity": 3045,
"price_paid": 109,
"total_spent": 331905,
"owner_name": "bender",
"potential_profits": 465885,
"selling_price": 262,
"profited": true
}
```
----------------------------------------------------
...to get a JSON reply that shows the most profitable island profile if the selling price is 110, make a GET request to: 
### **localhost:3333/profits/110**

This route should return this JSON response data:

```json
{
"island": "makeItRain",
"turnip_quantity": 6000,
"price_paid": 93,
"total_spent": 558000,
"owner_name": "vimes",
"potential_profits": 102000,
"selling_price": 110,
"profited": true
}
```
----------------------------------------------------

