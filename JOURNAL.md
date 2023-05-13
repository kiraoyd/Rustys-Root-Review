# A Journal of my progress building Rusty's Root Review for Bart Masseys Programming in Rust course Spring 2023

Anytime I mirror the code comming out of doggr (which happened a lot when establishing the server itself), I will do the legwork to understand why it works and actually learn something rather than just regurgitate the code.
This journal chronicals that process.

5/11/2023

A late start on this repo, but a start. I'm setting up the files needed for the project now, using doggr_w23 as a reference.
I have been working on updating Tuber Trader so it'd DB can be ready to run with Root Review, but it's still in progress.
I'm not sure how to go about building this here in a seperate repo, and testing it against tubers DB.
I believe I can link to the DB using the postgres URL, and just ensure Tuber Trader is up and running in Docker when I make test requests through this server.


5/12/2023

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
To attach cors and the pool to our router (written in routes.rs), we can use the .layer method provided by tower_http.
Extension() allows to extract and share state from the Pool, to our Routes: https://docs.rs/axum/latest/axum/index.html
I may explore switching to Using the "State" extractor method instead, for practice.

A quick note on the router: https://docs.rs/axum/latest/axum/index.html#routing
We can quickly register routes (written in handlers.rs) here in the router to stack them up for use.
Wrapping the whole establishment of the router in a closure, allows us to return it whenever "routes()" is called.
Once all this is done, 'app' essentially contains our new Router, stacked with middlewares and a DB connection to tuber

Next we need to get our server listening. 
Since we are going to run this safely in Docker, we can set the range of socket addresses to be pretty much all of them.
To do this, we will create a SocketAddr enum containing the IP address 0.0.0.0 and the port of 3333.
Then we can as the axum server to serve our app: https://docs.rs/axum/latest/axum/

Now it's time to call the run() function in main. 
We need main to be async, so we add the [tokio:::main] macro to establish a Tokio runtime. 
I'm not 100% clear on what a Tokio runtime  is exactly, other than what the docs say: that it allows for execution of asynchronous code.
Tokio crate: https://docs.rs/tokio/latest/tokio/

Getting some unexpected errors, that I suspect stem from not yet setting up error type wrapping Casey did. 
When i return to this, i'll go through that code and try to unpack what's going on, write it, and see if that helps some of these type errors.

# Things this project helped me practice

- Rust Web server/router setup
- Crates and modules
- Struct implementation
- -async/await in Rust, plus Futures
- Getting result values correctly