# Rustys Root Review
Kira Klingenberg

A web microservice built in Rust for the in-development Animal Crossing New Horizons Turnip Trading site: Tuber Trader. 

https://github.com/kiraoyd/tuberTrader

This microservice will query Tuber Trader's Database table for "Profiles" and run simple analysis on the turnips held by each island. 

More will be added as development proceeds. Repo is currently Private and will be made public eventually.

# Current State of the Program...

### To run

```agsl
cargo run
```

Database information for "tuber" pulled from the .env for Tuber Trader:

DB_HOST=localhost
DB_PORT=5432
DB_NAME=tuber
DB_USER=tuber
DB_PASS=tuber

The following steps courtesy of chatGPT:

To restore the DB from the tuber_dump.sql file in linux:

1. Copy tuber_dump.sql to the machine you would like to restore the database too, note the path to this copy (to obtain run: realpath filename.sql)
2. Open the terminal on that machine
2. Navigate to the directory where the PSQL command line tools are located (if unsure, run:  ls user/bin/pg* to see if the pg files exist in user/bin)
3. Run: psql -U <your_psql_username> -d <your_name_for_the_db> -f <path/to/copy_of_tuber_dump.sql>


Once restored, start and connect to tuber in linux:

1. To start: sudo service postgresql start
2. To connect: psql -U <your_psql_username> -d <your_name_for_the_db>
3. To list all tables in the db: \dt
4. To select and display all data from a specific table: SELECT * FROM <table_name>

Trouble shooting Postgres from ubunutu:

1. If the sudo service postgresql start command fails, the postgres server package may not be installed:
- To install the postgreSQL server package: sudo apt-get install postgresql-14
- To upgrade: sudo apt-get upgrade postgresql-14
- To see version: psql --version