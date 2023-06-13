# A Journal of my progress building Rusty's Root Review for Bart Masseys Programming in Rust course Spring 2023

Anytime I mirror the code comming out of doggr (which happened a lot when establishing the server itself), I will do the legwork to understand why it works and actually learn something rather than just regurgitate the code.
This journal chronicals that process.

5/11/2023

A late start on this repo, but a start. I'm setting up the files needed for the project now, using doggr_w23 as a reference.
I have been working on updating Tuber Trader so it'd DB can be ready to run with Root Review, but it's still in progress.
I'm not sure how to go about building this here in a seperate repo, and testing it against tubers DB.
I believe I can link to the DB using the postgres URL, and just ensure Tuber Trader is up and running in Docker when I make test requests through this server.


5/12/2023 (these notes added to on 5/19/2023)

Adding in the dockerfiles from doggr reference repo, updating to fit root_review
Won't need the docker-compose yaml once I move this over to tuberTrader, but added it in case.
I'm still learning about how to run the service in it's own docker container here, outside of tuberTrader.

Let's start by writing the method to connect us to our DB.
Looking into sqlx::Pool to learn about it: https://docs.rs/sqlx/latest/sqlx/struct.Pool.html
Also reading on PoolOptions: https://docs.rs/sqlx/latest/sqlx/pool/struct.PoolOptions.html#method.new
Looks like Pool allows us the ability to concurrently make queries against the DB
Pool enforces reuse of connections, and we can set the min or max number of connections in PgPoolOptions.
Postgres limit on connections is 97, but a pool ensures we don't exceed this, placing clients in a fair queue awaiting an available connection.
Connect() will immediately establish one connection to the DB.
PgPoolOptions::new() will return the pool struct containing DB successfully connected to.

I'm mirroring what Casey wrote to get us access to the DATA_BASE url stored in the .env file, since Pool needs access to that url to create the connection.
This is cool: in our lib.rs file, we create a struct that is build to hold whatever we need from the env file.
Then we implement on that struct, what is basically a constructor that populates the feilds with infor directly from the .env.
std::env::var("VAR_NAME") gets us into the .env, and we can slap a custom error message upon panic using .expect().
Then back in main, all we have to do is invoke that new() method to get a struct that contains the contents of our .env file.
And we can simply pass a reference to the data_base url feild of said struct, to the connect method provided by Pool.

Time to do a little reading on Rust's async/await: https://rust-lang.github.io/async-book/03_async_await/01_chapter.html
An async function in Rust returns a value that implements the "Future" trait: https://rust-lang.github.io/async-book/02_execution/02_future.html
A Future is some asynchronous computation that can produce a value or be empty () with just a unit.
A Future upon completion will return a enum type Poll set to Ready(result).
If the Future cannot complete it returns that Poll set to Pending, and waits to be woken later to make more process.
The example given: We read from a socket that may or may not have data ready, if data is ready we get back Poll::Ready(data), otherwise this future is blocked and Pending
So inside the async function establish_connection, we slap an .await on the creation of a new pool. 
The functions called to make the new pool, are the Future, and the .await when called will try to run those functions to completion.
The result once .max_connections and .connect have finished, is stored into 'pool' and will implement the Future trait.
establish_connection can now return the pool of connections to be used by our routes later.

Now it's time to set the server up to connect to the DB, set up middlewares, register our routes, and listen...
To get the connection we can just call our establish connection function. 
To get access to cors, we can use the tower_http::cors module: https://docs.rs/tower-http/latest/tower_http/cors/index.html
We will use the CorsLayer struct, to apply the Cors middleware and add headers for CORS. https://docs.rs/tower-http/latest/tower_http/cors/struct.CorsLayer.html
For now we will set the allow-origin header to "any", to allow any domain to make requests and get responses: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Access-Control-Allow-Origin
To attach cors and the pool to our router (written in routes.rs), we can use the .layer method provided by the tower trait, implemented by tower_http.
https://docs.rs/tower/latest/tower/trait.Layer.html
The layer method allows us to decorate the app with the middlewares and our DB connection
We layer axums Extension struct to insert the DB's state into our router and middlewares, giving them access: https://docs.rs/axum/latest/axum/struct.Extension.html
Extension() allows to extract and share state from the Pool, to our Routes: https://docs.rs/axum/latest/axum/index.html
If we don't have this Extension, then the server will reject a request with a 500 internal server error response.
I may explore switching to Using the "State" extractor method instead, for practice.


A quick note on the router: https://docs.rs/axum/latest/axum/index.html#routing
We can quickly register routes (written in handlers.rs) here in the router to stack them up for use.
Wrapping the whole establishment of the router in a closure, allows us to return it whenever "routes()" is called.

Once all this is done, 'app' essentially contains our new Router, stacked with middlewares and a DB connection to tuber

Next we need to get our server listening. 
Since we are going to run this safely in Docker, we can set the range of socket addresses to be pretty much all of them.
To do this, we will create a SocketAddr enum containing the IP address 0.0.0.0 and the port of 3333. https://doc.rust-lang.org/stable/std/net/enum.SocketAddr.html
It looks like Casey's strategy was to use the SocketAddr's 'from' fucntion to convert a tuple containing the Ip Address and a u16 as the port number, to create a new SocketAddr struct with those values.

Then we can ask the axum server to serve our app: https://docs.rs/axum/latest/axum/, https://docs.rs/axum-server/latest/axum_server/
To do this we can use the 'bind' method: https://docs.rs/axum-server/latest/axum_server/fn.bind.html
The 'bind' method creates a server that is bound to the address provided as its arg, which in our case is the SocketAddr we just created.
We specify that we want it to serve our app (that consists of the routes we write, the cors middlewares, and connetion to our DB).
Setting up the bind has to complete before the run() function can complete, so we await the success of bind.
I was having trouble locating the docs for the into_make_service() method casey is using here, so I asked chatGPT where it comes from to try and get some keywords to refine my google search.
This led me to the MakeService trait that is part of the tower crate: https://docs.rs/tower/0.4.13/tower/trait.MakeService.html
An into_service method is implemented for this trait, but I don't see the into_make_service one specifically. I'll have to do more digging to unravel this one.
For now I'm content with knowing that .serve requires a MakeService instance, and that instance is presumably created from the into_make_service method. https://docs.rs/axum-server/latest/axum_server/struct.Server.html

Now it's time to call the run() function in main. 
We need main to be async, so we add the [tokio:::main] macro to establish a Tokio runtime. 
I'm not 100% clear on what a Tokio runtime  is exactly, other than what the docs say: that it allows for execution of asynchronous code.
Tokio crate: https://docs.rs/tokio/latest/tokio/

Getting some unexpected errors, that I suspect stem from not yet setting up error type wrapping Casey did. 
When i return to this, i'll go through that code and try to unpack what's going on, write it, and see if that helps some of these type errors.

5/19/2023

Alright time to start the next big project workday. I update the previous entry with some new knowledge I got from reading about the axum server setup.
Now I'm going to pick up where I left off and try to figure out the error type wrapping stuff.

We create an error type wratper struct called AppError, that contains an anyhow crate struct called 'Error': https://docs.rs/anyhow/latest/anyhow/index.html#structs
An anyhow::Error is an error type, and when we print it's contents, we get the error message it stores: https://docs.rs/anyhow/latest/anyhow/struct.Error.html

When we implement the IntoResponse trait for our AppError struct, we are allowing AppError the use of a custom into_response function we write.
axums IntoReponse trait allow types that implement it to be returned from handlers. 
since the goal of making our own AppError wrapper is to be able to send errors back as responses, it's important that AppError implements this trait. 
The IntoResponse docs mention it's already implemented for many types, but will need to be implemented for a custom type, like our situation: https://docs.rs/axum/latest/axum/response/trait.IntoResponse.html
From the docs we can see that implement IntoReponse for a custom type, we need to call into_response() on a tuple containing an http StatusCode struct: https://docs.rs/http/latest/http/status/struct.StatusCode.html
and the body of the error message. 
If I'm connecting the dots right, what we are doing is creating Rust's version of sending back "return reply.status(404).send({"Error message text"})" in javascript.


Next up, following Casey's lead in Doggr, we write a function for AppError that allows us to use the ? syntax on functions returning 'Result<_, anyhow::Error>',
and convert that Result into a 'Result<_,AppError>' that uses our custome Error type. I'm really not sure how this works yet. I remember when Casey was
writing this he was very specific that this involved some complex Rust specific stuff that we dind't need to know at the tim ebcasue fullstack wasn't as Rust class.
So since this is now a Rust project, I'm going to try to decode what's going on here to better learn about Rust. :) 
But in the interest of prioritizing I'll save that task for later.

So lets figure out which functions can possible return a Result<_,AppError>, and utilize the new ? feature. 
Run() could error, so could establish_connection, and any of the future routes we write may return an error as well. 

Ok took a break, now it's time to check for compiler errors. 
I had to delete my test route (there were too many rabbit hole to go down to create "dummy" pieces to return a fake route), I'll just write real routes for my project next.
The one compiler error I'm left with says that the trait Clone is not implemented for anyhow::Error, in relation to line 33 in main.rs:     let app = routes().layer(cors).layer(Extension(connection));
Comparing to Doggr, it looks like I forgot to convert the establish_connection Result(_,anyhow::Error) to our custom AppError instead! 
But now I'm wondering what this has to do with the clone trait....Something else I'll have to shelve util after I have my MPV running.

Ok now to hook this up to tuberTrader's DB and try writing a route! 
Looks like I might need to set up a test DB that mirrors tuberTraders, otherwise there will be no way for Bart to run this without having all of Tubers files. 
But for now, I can connect it to my actual database successfully, awesome!

First lets set up the models using serde. Looks like I can use the sqlx crate.
The FromRow trait can be derived on a struct representing the data coming from a row in one of my DB's tables. https://docs.rs/sqlx/latest/sqlx/trait.FromRow.html
This will cause a sequences of calls to Row::try_get using the name from each of the structs feilds. https://docs.rs/sqlx/latest/sqlx/trait.Row.html#method.try_get
Try_get indexes into a DB row and decodes a single value, using a string index (the struct feild) to access a column by name.
Values that are references to other models, will be represented by i32 types, as they reference the ID's to those other tables.
i'll probably have to query both tables in order to get all the data back that I want.

First hangup: I named by DB columns in camelCase, and clippy wants all struct feilds to be in snake_case. 
Luckily, FromRow has a field attribute that allows me to rename a column fetched, back in the Rust model.

Alright, made an attempt to write a get route for all Users, and there are plenty of little things to sort out.
I'll get back to those first thing next work period! Tata for now....

This tutorial may prove useful: https://betterprogramming.pub/how-to-interact-with-postgresql-from-rust-using-sqlx-cfa2a7c758e7

5/22/2023

Working on getting a single route working, running into some hangups.
First thing to note: my tuberTrader DB is off (likely due to me swapping the ORM's recently)
I'm probably going to have to get a test DB up and running for root review asap.

I've got my root review server and router connecting successfully to my tuber trader postgres DB,
but the route to get the id and ip from my IPHistory table is failing to deserialize the JSON body
correctly, claiming a missing feild of "id".  
I worked out some kinks on how query_as was querying, and am going to pare this down to just trying to grab the ip column
from the IPHistory table, and reduce the IPHistory stuct I made as a model to just this feild as the next test.
Aha, I just realised I was bringing in a reference to something I didn't need, and removed it (credentials)
Now it looks like the test route is working!

So breakdown of what's going on when building a route here:
1. Create a public async function that will be called in the router to initiate the route code.
This public funtion needs to have access to the Pool of connections we created, in order to access and query the DB. 
If everything goes to plan, this wrapper function will return the result of a private inner function that makes the query.
The inner function will be responsible for converting it's result, into Json, so that the wrapper recieves it already and packaged to send back.
We can use the overloaded ? now to ensure that if things fail, we send back an AppError type
2. The inner async functions responsibility is to actually make the query to the DB, and convert the result into a Json that can be sent back as a reply.
This inner function needs a references to the Pool to be able to connect, and it needs to return either a Result containing the status code for success and the data, or an Error.
To query the DB, we can use the sqlx query! macro, and it's associated macros like query_as!: https://docs.rs/sqlx/latest/sqlx/macro.query.html
This macro lets us write direct SQL query text, and chain some handy associated methods like fetch_on or fetch_all to control for how many rows we expect.
3. If we want to use fetch_all, we have to make sure the type we catch the result in is a vector.
Which means we better adjust the return types to expect a Json<Vec<whatever>> instead of just a Json<whatever>

So next, I need to work on building out the custom error messaging for if this route fails.

Just met with Nicholas, some things to note: add in doc comments to all structures and methods, unit test the program logic on the routes I build.
Next up: get some data from the DB, and write some rust code to do something interesting with it!

5/30/23

Ok I'm back, that was a long heitus but my son's birthday took over the priorities of life and that's how it should be really. So here I am!
Picking back up writing a route that will pull data from the Profile table.
This first route will just run through all the prices paid and total turnips for each island, and returnt the Profile of the biggest spender.
I feel pretty good about the iterator piece of this, but I need to figure out the best way to copy one struct into another.
Looks like deriving the clone trait might be the way to go: https://levelup.gitconnected.com/rust-cloning-structs-explained-d633713d5de0

Ok had some trouble using query_as!, it was saying the table names I gave in the query did not exist. 
After much troubleshooting, and not much luck, I switched to using query! macro instead. 
The issue here is that appears to return a Record type, when I'd like it to return a Profile type struct.
My first thought is I need to accept the Recrod types, and find a way to convert them into my Profile struct type. 
Nope this was barking up the wrong tree entirely, query_as should work fine.
The issue now is that my raw SQL can't seem to link with the reference column "owner" in my DB.
I've gone down a big rabbit hole chasing this one, and am gonna shelve it and try to grab the data without owner included for now.
This opened a more fun, Rust related issue finally: I can't just clone one of the vectors structs into a waiting variable without working with borrows (via the compilers suggestion).
JUST KIDDING! Why deal with references, when I can clone! I forgot I had the struct derive the Clone trait already, and instead of trying to pass references, I'll just clone the struct.
Ok so now I can get it to query and return the biggest spender Profile (sans owner info for now...).
Things I'd like to try doing: package up the profile info to include just the name, turnip info AND the total spent. Also I'd like to fix the owner issue so I can include the users name in the reply.

https://doc.rust-lang.org/book/ch05-01-defining-structs.html#:~:text=We%20create%20an%20instance%20by,declared%20them%20in%20the%20struct.
Alright! I figured out how to get the reply packaged up to include the spent amount that was calculated. 
I knew I'd need another struct to carry the new info, but trying to make on in the lib.rs proved to be difficult.
It worked logically, but it turns out the axum Json() function was expecting an actual sqlx defined struct.
I moved the definition of the Spender Reply struct, into my tuber_tables module, and derived sqlx::FromRow for it. 
This allowed Json() to work on it, and send back just the data I wanted.
I'm not sure this is the most elegant way, as it doesn't help readability to have a response struct that doesn't represent an actual table in teh DB, floating around in the tuber_tables DB representation, but it'll do for now.
I also had to start thinking closer about the types I gave my feilds. 
At first everything was i32s, but upon further thought, that "spent" variable really needed to be bigger as the multiplication has the potential to grow the i32.
So I went back in to update some of the types to be i64s.
I'm also wondering if I should make all these unsigned instead of signed, as it's not possible in the game for any of these values to be negative.
I supose the advantage here, is that it's possible that my eventual calculation of "profits" could be negative, so by keeping all of these as signed I'll not run into trouble for that route.
But this choice limits how high the maximum positive number that can be referenced in an i32 can get (cuts it in half)....so I need to chew on this decision.

5/31/23

Thanks to some help on Zulip, I figured out the issue with getting the owner id data from my DB. 
I dumped the schema creation SQL to the console from Tuber Trader's backend, and discovered that while the owner colum was a reference to another table (User),
the values (ID numbers) stored there were actually typed as ints. 
So I swapped the typing in my Rust struct to match, and threw out the attempt at renaming the column, now things are working.
So the next step is to use that owner_id for the biggest_spender to query the DB again and get the name of the User that owns the big spending island.
First hangup: how to insert a varaibles value into the SQL text string for query_as!....
Great! found it in the docs, using the bind parameter ($N where N is the 1-based positional argument index) for postgres: https://docs.rs/sqlx/latest/sqlx/macro.query.html

Next route idea:
Take the price paid for an islands turnips, and the quantity, and calculate the profits possible for that island given a certain selling price.
I'm going to try to get the selling_price and island_id in as URL params. 
Looks like the way to do this is with axums Path(): https://docs.rs/axum/latest/axum/extract/struct.Path.html
I will be grabbing a tuple from the URL params, and will need to pass the tuple into the try function that does the actual work.
Thanks to a reminder from this stack overflow: https://stackoverflow.com/questions/39878382/is-it-possible-to-unpack-a-tuple-into-function-arguments
I can use pattern matching to pass the args through correctly.
Here's the new problem...using Path means the args come in as uuids...I either need to figure out how to convert those back to primitives to calculate with OR find out if I can use Path with different arg types.

6/1/2023
Following the examples in the doc for extracting URL params, I initially started by typing those params as uuid's.
This unravelled into a rabbit hole pretty fast, and for the sake of time I'm going to switch gears and work with them as Strings instead.
It looks like in real life, a Uuid I much prefered for its universal uniqueness, but I think in the interest of writing more stuff I'll hold on that for now.
So to bring in the params as strings, I'll need to parse them out to convert to numberic types I can run calculations with.
Luckily we did this when parsing CLA in the homeworks, so I'll be referencing my code there to get that done.
Alright that route works!

Interesting, thanks to clippy I now know that if a type implements the Copy trait, there is no need to use .clone() for it.
Also worth noting, perhaps I already have: looks like we can only use query_as! if we plan to fill all the feilds of the struct we are populating
Otherwise, just stick with the query! macro and use the SQL itself to specify what feilds to grab, and extract from the Record

Next up, lets write a route that reports back the island that has the max profits at some given selling price (which will likely be the one with the most turnips, but we will do this using math anyways), out of ALL islands.
Well that was almost too easy. I just needed to pull elements from the already constructed routes to achieve this goal.
For the next route I should think about something a little more challenging....

Side note: I was having trouble abstracting out my Reply data structs into their own file. 
With some help from Mathew Cooper on zulip, this was the answer:
"A module cannot be declare outside the root/crate module tree (i.e., going up the module tree, a submodule must always have a parent that is declared directly in lib.rs or main.rs, so the first program submodule must always be declared there — a tree data structure if it isn't already obvious enough)."
all new files in src need to be published either in main.rs or lib.rs via: pub mod filename;


6/13/2023

Well, I've been stuck trying to get my sql dumpfile to load on Bart's side. 
After hours of searching and reading online, I still can't find a solution for the issue we are having.
Something to do with the sql file creating public schema by default, and not loading when a restore is attempted.
so I've pivoted, and started building a second crate that will build a mock version of the db on Barts machine when run.
I got the bones of things from this tutorial:https://rust-lang-nursery.github.io/rust-cookbook/database/postgres.html
And have been troubleshooting little erorrs. 
Originally I tried to make this a seperate file inside the root_review crate, but encountered a "cannot establish a runtime within a runtime" error.
I did some digging but decided it was going to take too long to figure it out this close to the deadline.
So I opted to build this seperate crate.

I've been hashing out some SQL query issues using a combo of the postgres docs and some queries to chatGPT.
Clippy was very helpful in sorting out some silly Rust errors I made in the process.
When testing on my local machine after making a new database on my postgres user, called "mocktuber", it is successful!

So now I just need to bulk up the seed data, and test that I can run against this DB in my root_review crate.


# Things this project helped me practice and learn
- Rust Web server/router setup
- Crates and modules
- Struct implementation
- async/await in Rust, plus Futures
- Getting result values correctly
- Reference passing of a DB connection Pool 
- Creating Rust db model structs
- Serde to serialize and deserialize said structs
- Iterators
- Custom Structs
- number typing and size decisions
- References VS deriving the Clone trait
- Extracting URL params and parsing strings
- Building and seeding a DB from sql queries using the postgres crate