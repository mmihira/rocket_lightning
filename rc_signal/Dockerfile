# select build image
FROM rust:1.26 as build

# create a new empty shell project
RUN USER=root cargo new --bin rc_signal
WORKDIR /rc_signal

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./rust-toolchain ./

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*

# copy your source tree
COPY ./src ./src
COPY ./config.json ./
COPY ./diesel.toml ./
COPY ./migrations ./migrations
COPY ./scripts ./scripts

# build for release
RUN cargo build --release

# our final base
FROM rust:1.26-slim

COPY --from=build /rc_signal/target/release/rc_signal .

# set the startup command to run your binary
CMD ["./rc_signal"]
