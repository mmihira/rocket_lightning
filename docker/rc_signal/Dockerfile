FROM ekidd/rust-musl-builder AS builder
ARG p_dir

# create a new empty shell project
RUN USER=root cargo new --bin /home/rust/rc_signal/ctx
WORKDIR /home/rust/rc_signal/ctx
# copy over your manifests
COPY ./${p_dir}/Cargo.lock ./Cargo.lock
COPY ./${p_dir}/Cargo.toml ./Cargo.toml
COPY ./${p_dir}/rust-toolchain ./
COPY ./lib ./../lib
RUN sudo chown -R rust:rust /home/rust/rc_signal

# cache dependencies
RUN cargo build --release
RUN rm src/*

# copy the source tree
COPY ./${p_dir}/src ./src
COPY ./${p_dir}/config.json ./
COPY ./${p_dir}/diesel.toml ./
COPY ./entrypoint.sh ./

# build for release
RUN cargo build --release

# Final image
FROM alpine:3.8
RUN apk --no-cache add ca-certificates
RUN apk --no-cache add libpq
RUN apk --no-cache add curl

COPY --from=builder /home/rust/rc_signal/ctx/target/x86_64-unknown-linux-musl/release/rc_signal .
COPY --from=builder /home/rust/rc_signal/ctx/config.json .
COPY --from=builder /home/rust/rc_signal/ctx/diesel.toml .
COPY --from=builder /home/rust/rc_signal/ctx/entrypoint.sh .

# set the startup command to run your binary
CMD "./entrypoint.sh"
