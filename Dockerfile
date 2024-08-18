
FROM rust:1.80-bookworm as build

# create a new empty shell project
RUN USER=root cargo new --bin rust-axum-cassandra
WORKDIR /rust-axum-cassandra

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

COPY ./src ./src

RUN cargo build --release

# our final base
FROM debian:bookworm-slim AS runtime

# copy the build artifact from the build stage
COPY --from=build /rust-axum-cassandra/target/release/rust-axum-cassandra .

EXPOSE 3000

# set the startup command to run your binary
CMD ["./rust-axum-cassandra"]