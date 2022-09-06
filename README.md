[![BuildTestDeploy](https://github.com/pace-running/pace3/actions/workflows/build-test-deploy.yml/badge.svg)](https://github.com/pace-running/pace3/actions/workflows/build-test-deploy.yml)
[![Dependencies](https://github.com/pace-running/pace3/actions/workflows/dependencies.yml/badge.svg)](https://github.com/pace-running/pace3/actions/workflows/dependencies.yml)
# pace

Pace is a registration and management application for running events and competitions.

## Database initialization

To start a postgres container: 

```
docker-compose up --force-recreate -d
```

To shut it down:
```
docker-compose down  
```

## Run script

This script helps to run specific commands for your application. You can run the script as follows:

```shell
./run.sh command 

Commands:
  install         - Install application dependencies
  update          - Update application dependencies
  migrate         - Run database migration
  start           - Start the application
  fmt_check       - Run Formatting check by fmt
  test            - Run all tests
  quality_check   - Static check (lint) and testing
  help            - Print cli usage message
```

## Running Pace

For starters, you can run the application with 

```
cargo run
```

Access the different endpoints using a browser or curl:

`curl localhost:8080` should return "Hello World".

`curl -X POST localhost 8080 -d [BODY]` should return the body passed in the POST request.

`curl localhost:8080/hey` should return "Hey there!".
