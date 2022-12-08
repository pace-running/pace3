[![BuildTestDeploy](https://github.com/pace-running/pace3/actions/workflows/build-test-deploy.yml/badge.svg)](https://github.com/pace-running/pace3/actions/workflows/build-test-deploy.yml)
[![Dependencies](https://github.com/pace-running/pace3/actions/workflows/dependencies.yml/badge.svg)](https://github.com/pace-running/pace3/actions/workflows/dependencies.yml)

# pace

Pace is a registration and management application for running events and competitions.


## Running Pace
to start both frontend and backend applications in corresponding folders (`/frontend/pace-ui` and `/backend`)
```shell
./run start
```
to restart the backend application after code change, exit with `ctrl + c` and start again with 
`cargo run` so existing docker can be reused

### Test
to test admin functions with localhost `_: Identity` needs to be removed from argument in this file `backend/src/handlers/admin.rs`

### Access the different endpoints using a browser or curl:

`curl localhost:8080` should return "Hello World".

`curl -X POST localhost 8080 -d [BODY]` should return the body passed in the POST request.

`curl localhost:8080/hey` should return "Hey there!".

## Run script in backend

This script helps to run specific commands for your application. You can run the script as follows:

```
./run command

Commands:
  install           - Install application dependencies
  update            - Update application dependencies
  migrate           - Run database migration
  start             - Start the application
  fmt_check         - Run Formatting check by fmt
  test_unit         - Run all unit tests
  test_integration  - Run all integration tests
  lint_check        - Run linting check by clippy
  quality_check     - Static check (lint) and testing
  help              - Print cli usage message
```

## Run script in frontend
```
Commands:
install - Install application dependencies
build - Build application to deploy
fmt - Run formatting
start - Start the application
lint - Run lint
tsc - Run Type check by tsc
quality_check - Static check (lint) and testing
help - Print cli usage message
```
