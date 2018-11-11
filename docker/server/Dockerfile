FROM rust:1.26 as build

ARG p_dir

# create a new empty shell project
RUN USER=root cargo new --bin server/ctx
WORKDIR /server/ctx


# copy over your manifests
COPY ./${p_dir}/Cargo.lock ./Cargo.lock
COPY ./${p_dir}/Cargo.toml ./Cargo.toml
COPY ./${p_dir}/rust-toolchain ./
WORKDIR /server
COPY ./lib ./lib
WORKDIR /server/ctx

# cache dependencies
RUN cargo build --release
RUN rm src/*

# copy the source tree
COPY ./${p_dir}/src ./src
COPY ./${p_dir}/config.json ./
COPY ./${p_dir}/Rocket.toml ./
COPY ./entrypoint.sh ./

# build for release
RUN cargo build --release

# Final image
FROM postgres:9.6.10
ENV TZ=Australia/Melbourne
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime \
 && echo $TZ > /etc/timezone \
 && dpkg-reconfigure -f noninteractive tzdata

COPY --from=build /server/ctx/target/release/server .
COPY --from=build /server/ctx/config.json .
COPY --from=build /server/ctx/Rocket.toml .
COPY --from=build /server/ctx/entrypoint.sh .

# set the startup command to run your binary
CMD ["./entrypoint.sh"]
