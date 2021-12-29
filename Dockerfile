FROM rust:1.57

WORKDIR /app

COPY Cargo.toml .
COPY Cargo.lock .
COPY migrations* ./migrations
RUN cargo install sqlx-cli --no-default-features --features postgres

COPY src* ./src
RUN cargo build

CMD sqlx migrate run && cargo run
