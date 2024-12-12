FROM rust:1.83-bookworm as builder

WORKDIR /usr/src/app
COPY . .

RUN cargo install --bin hitsigst-server --path hitsigst-server

FROM debian:bookworm
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/hitsigst-server /usr/local/bin/hitsigst-server

EXPOSE 3000/tcp

CMD ["hitsigst-server"]
