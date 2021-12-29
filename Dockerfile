FROM rust:1.57

WORKDIR /app

COPY Cargo.toml .
COPY Cargo.lock .
COPY set-up-connector.sh .

COPY migrations* ./migrations
RUN cargo install sqlx-cli --no-default-features --features postgres

COPY src* ./src
RUN cargo build

CMD ./set-up-connector.sh && sqlx migrate run && cargo run
