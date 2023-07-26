# build stage
FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# create a smaller final image
FROM debian:buster-slim as api
RUN apt-get update && \
    apt-get install -y libssl1.1 && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/plain-api .
CMD ["./plain-api"]

# pull website
FROM alpine/git as clone
WORKDIR /app
RUN git clone https://github.com/traumst/traumst.github.io

# setup nginx to serve website
FROM nginx:latest as website
COPY --from=clone /app/traumst.github.io /usr/share/nginx/html
COPY ./nginx.conf /etc/nginx/conf.d/default.conf