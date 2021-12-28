# Payfeed

Learning project for internal "distributed" hackathon using a "Venmo clone" to explore:

- CQRS
- event streaming
- CDC
- Rust

## Requirements

- Docker 20.10+

## Installation

From root of directory run:

```bash
docker compose up --build
```

To spin down containers run:

```bash
docker compose down
```

## Testing

From root of directory run:

```bash
cargo test
```
