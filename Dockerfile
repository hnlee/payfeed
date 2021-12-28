FROM rust:1.57

WORKDIR /app

COPY Cargo.toml .
COPY Cargo.lock .
COPY src* ./src
COPY migrations* ./migrations
COPY diesel.toml . 

RUN cargo install diesel_cli --no-default-features --features postgres 
RUN cargo build

CMD diesel migration run && cargo run
