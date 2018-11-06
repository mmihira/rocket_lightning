
# Rocket RC

RSI Calculation
Made 

We use : 

- Rocket (https://rocket.rs/)
- Postgres Adapter Diesel (https://diesel.rs/)
- Graphql - Juniper (https://github.com/graphql-rust/juniper)

## Table of Contents

* [Requiremets](#install)
* [Install](#install)
* [Usage](#usage)
* [Configuration](#config)
* [Test](#test)
* [Changelog](#changelog)

## Requirements

* cargo-make
* cargo
* rust

## Install

cd ./docker/
docker-compose up
cd ../rc_signal
diesel setup

## Usage



#### Run dev environment

Run the docker dev environment in ````./docker/dev/docker-compose.yml````<br>

#### Build production and run production server


## Configuration

## Test

Don't run tests in parallel ! Anything involving the database. DOn't run tests in parallel.
cargo test -- --test-threads=1

## Changelog

