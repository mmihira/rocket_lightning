# Rocket RC
RSI Calculation. Full stack including calculation service, server, and UI App.

We use :
- Rocket (https://rocket.rs/)
- Postgres Adapter Diesel (https://diesel.rs/)
- Graphql - Juniper (https://github.com/graphql-rust/juniper)

## Table of Contents

* [Requiremets](#install)
* [Usage](#usage)
* [Test](#test)
* [Changelog](#changelog)

## Requirements

You will need rust. Install if for your system at : ` https://www.rust-lang.org/en-US/install.html `

You will need the diesel-cli and cargo make
````
  cargo install diesel_cli --no-default-features --features postgres
  cargo install --force cargo-make
````

## Usage

Instructions are given for running locally as a docker environment
First find your host IP given by your router.
Make sure to set the ip address in rc_signal/config.json

First start up postgres docker  ` cargo make start_dev_pg ` <br>
Run the migrations for prod     ` cargo make database_reset_prod ` <br>
Run the migrations for test     ` cargo make database_reset_test ` <br>
Build rc_signal                 ` cargo make docker_build_rc_signal ` <br>
Build the server
Build the app


Start the server, rc_signal, and app
```` cargo run rc_signal ````

### Build Locally

For development it's also possible to run ```` cargo  buil ```` inside the
```` /rc_singnal ```` or  ```` /server ```` to build those modules and
run them locally.

## Configuration

## Test

Don't run tests in parallel ! Anything involving the database. Don't run tests inparallel.

```` cargo test -- --test-threads=1 ````

## Changelog

