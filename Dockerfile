# build stage
FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN rustup default nightly
RUN cargo build --release

# create a smaller final image
FROM debian:bookworm-slim as api
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /data
WORKDIR /app
COPY --from=builder /app/target/release/plain-api .
CMD ["./plain-api"]

# pull website
FROM alpine/git as clone
WORKDIR /app
RUN git clone https://github.com/traumst/traumst.github.io

# setup nginx to serve website
FROM nginx:latest as web
COPY --from=clone /app/traumst.github.io /usr/share/nginx/html