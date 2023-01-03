FROM rust:1 as builder
WORKDIR /usr/src/app
COPY ./ ./
RUN cargo build --package cuty-server --release
FROM debian:buster-slim as runner
COPY --from=builder /usr/src/app/target/release/cuty-server /usr/local/bin/cuty-server
RUN apt-get update
RUN apt-get install -y libssl-dev sqlite3 openssl ca-certificates
WORKDIR /usr/src/app
COPY diesel.toml Cargo.toml Rocket.toml .env ./
COPY templates ./templates
COPY database ./database
COPY migrations ./migrations
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["cuty-server"]
