[![Build](https://github.com/pace-running/pace3/actions/workflows/test.yml/badge.svg)](https://github.com/pace-running/pace3/actions/workflows/test.yml)
[![Dependencies](https://github.com/pace-running/pace3/actions/workflows/dependencies.yml/badge.svg)](https://github.com/pace-running/pace3/actions/workflows/dependencies.yml)
# pace

Pace is a registration and management application for running events and competitions.

## Running Pace

For starters, you can run the application with 

```
cargo run
```

Access the different endpoints using a browser or curl:

`curl localhost:8080` should return "Hello World".

`curl -X POST localhost 8080 -d [BODY]` should return the body passed in the POST request.

`curl localhost:8080/hey` should return "Hey there!".
