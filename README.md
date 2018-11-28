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

You will need a local installation of openssl, libssl-dev, and libpq5-dev
For Debian
````
  sudo apt update
  sudo apt install openssl libssl-dev libpq5-dev
````

## Usage

Instructions are given for running locally as a docker environment

Build rc_signal                 ` cargo make docker_build_rc_signal ` <br>
Build the server                ` cargo make docker_build_rc_server ` <br>
Build build the app             ` cargo make docker_build_app ` <br>
Build nginx app server          ` cargo make docker_build_nginx ` <br>

Start up postgres docker        ` cargo make docker_reset_db ` <br>
Run the migrations for prod     ` cargo make database_reset_prod ` <br>
Run the migrations for test     ` cargo make database_reset_test ` <br>
Run rc_signal                   ` cargo make run_rc_signal_docker ` <br>
Run rc_server                   ` cargo make run_rc_server_docker ` <br>

### Build Locally

For development it's also possible to run ```` cargo  build ```` inside the
```` /rc_signal ```` or  ```` /server ```` to build those modules and
run them locally.

## Configuration

## Test

Don't run tests in parallel ! Anything involving the database. Don't run tests inparallel.

```` cargo test -- --test-threads=1 ````

## Changelog

