FROM rust:1.68.0 as builder

WORKDIR /usr/src/rusty-finance-api

COPY . .

RUN cargo install --path .

FROM debian:bullseye-slim

COPY --from=builder /usr/local/cargo/bin/rusty-finance-api /usr/local/bin/rusty-finance-api

CMD ["rusty-finance-api"]
