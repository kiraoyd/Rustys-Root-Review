# Rustys Root Review
Kira Klingenberg

A web microservice built in Rust for the in-development Animal Crossing New Horizons Turnip Trading site: Tuber Trader. 

https://github.com/kiraoyd/tuberTrader

This microservice will query Tuber Trader's Database table for "Profiles" and run simple analysis on the turnips held by each island. 

More will be added as development proceeds. Repo is currently Private and will be made public eventually.

# Current State of the Program...

## Please ignore the Docker files for now

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
