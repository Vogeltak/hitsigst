FROM rust:1.83-bookworm as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo install --bin hitrelease-server --path hitrelease-server

FROM debian:bookworm
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/hitrelease-server /usr/local/bin/hitrelease-server

EXPOSE 3000/tcp

CMD ["hitrelease-server"]
