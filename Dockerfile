# build stage
FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# create a smaller final image
FROM debian:buster-slim
RUN apt-get update && \
    apt-get install -y libssl1.1 && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/plain-api .
CMD ["./plain-api"]