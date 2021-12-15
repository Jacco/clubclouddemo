FROM docker.io/rust:latest

WORKDIR /var/app

RUN mkdir -p src/ && echo "fn main() {}" > src/main.rs && uname -a

COPY Cargo.toml Cargo.lock .

RUN cargo build --release --target aarch64-unknown-linux-gnu

COPY src src

RUN cargo build --release --target aarch64-unknown-linux-gnu

CMD cat target/aarch64-unknown-linux-gnu/release/graphql-server
