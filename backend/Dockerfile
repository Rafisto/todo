FROM rust:latest

COPY . .

RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/backend"]