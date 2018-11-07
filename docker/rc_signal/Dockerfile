FROM rust:1.26 as build

ARG p_dir

# create a new empty shell project
RUN USER=root cargo new --bin rc_signal/ctx
WORKDIR /rc_signal/ctx

# copy over your manifests
COPY ./${p_dir}/Cargo.lock ./Cargo.lock
COPY ./${p_dir}/Cargo.toml ./Cargo.toml
COPY ./${p_dir}/rust-toolchain ./
WORKDIR /rc_signal
COPY ./lib ./lib
WORKDIR /rc_signal/ctx

# cache dependencies
RUN cargo build --release
RUN rm src/*

# copy the source tree
COPY ./${p_dir}/src ./src
COPY ./${p_dir}/config.json ./
COPY ./${p_dir}/diesel.toml ./
COPY ./${p_dir}/migrations ./migrations
COPY ./entrypoint.sh ./

# build for release
RUN cargo build --release

# Final image
FROM postgres:9.6.10
ENV TZ=Australia/Melbourne
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime \
 && echo $TZ > /etc/timezone \
 && dpkg-reconfigure -f noninteractive tzdata

COPY --from=build /rc_signal/ctx/target/release/rc_signal .
COPY --from=build /rc_signal/ctx/config.json .
COPY --from=build /rc_signal/ctx/diesel.toml .
COPY --from=build /rc_signal/ctx/migrations .
COPY --from=build /rc_signal/ctx/entrypoint.sh .

COPY deps.sh /deps.sh
RUN /deps.sh

# set the startup command to run your binary
CMD ["./entrypoint.sh"]
