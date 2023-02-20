FROM rust:1.67-slim

RUN apt update && apt install -y --no-install-recommends git

WORKDIR /test

RUN cargo init --bin --edition 2021 --name identity

COPY Cargo.toml ./
COPY Cargo.lock ./

RUN cargo build

COPY ./src/ ./src/
RUN cargo install --path .

COPY ./test/ .

ENTRYPOINT [ "./entry.sh" ]
