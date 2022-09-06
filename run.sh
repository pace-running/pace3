#!/bin/bash

###############################################################################
# Shell script
###############################################################################
# Shell script options
set -e

###############################################################################
# Execution environment
###############################################################################

# shellcheck disable=SC2034
PROJECT_DIR="$(
  cd "$(dirname "$0")" >/dev/null 2>&1
  pwd -P
)"

###############################################################################
# Setup
###############################################################################

##DOC install - Install application dependencies
task_install() {
  cargo build
}

##DOC update - Update application dependencies
task_update() {
  cargo update
}

##DOC migrate - Run database migration
task_migrate() {
  cargo install diesel_cli --force --no-default-features --features postgres
  diesel setup
  diesel migration run
  cargo fmt
}

###############################################################################
# Start
###############################################################################

##DOC start - Start the application
task_start() {
  cargo run
}

###############################################################################
# Test
###############################################################################

##DOC fmt_check - Run Formatting check by fmt
task_fmt_check() {
  cargo fmt --all -- --check
}

##DOC test - Run all tests
task_test() {
  docker-compose up --force-recreate -d
  sleep 5
  cargo install diesel_cli --force --no-default-features --features postgres
  diesel setup
  diesel migration run
  cargo fmt
  cargo test
  docker-compose down
}

##DOC quality_check - Static check (lint) and testing
task_quality_check() {
  task_fmt_check
  task_test
}

###############################################################################
# Helper functions
###############################################################################

##DOC help - Print cli usage message
help() {
  echo ""
  echo "Commands:"
  grep -e "^##DOC" < "$(basename "$0")" | sed "s/^##DOC \(.*\)/  \1/"

  exit 1
}

## cli main function
main() {
  CMD=${1:-}
  shift || true

  pushd "$DIR" > /dev/null

  if type "task_${CMD}" &> /dev/null; then
    "task_${CMD}" "$@"
  else
    help
  fi

  popd > /dev/null
}

main "$@"
