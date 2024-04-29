# Plain Api

## What is this?

It's a basic http server written in 🦀. I'm making this to
practice and expand my knowledge of this fascinating language.
It is my way to get familiar with other programming paradigms beside OOP.
Finally, this project should serve a base for any and all
side-projects I may want to have. One thing that is on my mind
currently is building a simple but secure and reliable web-chat.
We'll see what else I'd want to experiment with.

## Project structure

* /src
    * main.rs - init for utilities and start the server
    * api.rs - routing setup
    * config.rs - read env variables
    * server.rs - basic http server based on async TcpListener 
    * /api - routing and handlers for the incoming requests
        * response.rs - static http responses
        * /router - routing based on http headers
            * email.rs
        * /handler - handlers, processing the requests
            * email.rs
            * chat.rs
    * /infra - utilities
        * logger.rs - logger initialization
        * email.rs - sends email to and from emails specified in config
* /.github
    * deploy.yml - github action - deployment automation script, relies on the presence of the Dockerfile