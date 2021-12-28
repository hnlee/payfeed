FROM rust:1.57

WORKDIR /app

COPY Cargo.toml .
COPY Cargo.lock .
COPY src* ./src

CMD ["cargo", "run"]
