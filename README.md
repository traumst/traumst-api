# Plain Api

## Project structure

* /src
    * main.rs - init for utilities and start the server
    * config.rs - read env variables
    * server.rs - basic http server based on async TcpListener 
    * /api - routing and handlers for the incoming requests
        * response.rs - static http responses
        * /route - routing based on http headers
            * email.rs
            * user.rs
            * auth.rs - \*in progress\*
        * /process - handlers, processing the requests
            * email.rs
            * user.rs
            * auth.rs - \*in progress\*
    * /infra - utilities
        * logger.rs - logger initialization
        * email.rs - sends email to and from emails specified in config
    * /db
        * pool.rs - defines a Bridge struct that holds db connection pool
        * transaction.rs - \*in progress\* macro to execute db queries in the same transaction
        * /table - schema creating and CRUD operations on the db
            * user.rs
            * auth.rs
        * /model - structs representing rows of appropriate tables
            * user.rs
            * auth.rs
    * /chat - \*TBD\*
* /.github
    * deploy.yml - github action - deployment automation script, relies on the presence of the Dockerfile