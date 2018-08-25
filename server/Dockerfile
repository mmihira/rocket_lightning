FROM rustlang/rust:nightly

ENV TZ=Australia/Melbourne
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime \
 && echo $TZ > /etc/timezone \
 && dpkg-reconfigure -f noninteractive tzdata

COPY ./src /server/src/
COPY ./Cargo.lock /server/
COPY ./Cargo.toml /server/
COPY ./rust-toolchain /server/
COPY ./Rocket.toml /server/
WORKDIR /server/
RUN cargo build --release

CMD ["cargo", "run"]
