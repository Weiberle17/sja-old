FROM rust:1.74 as builder
WORKDIR /usr/src/myapp
COPY . .
RUN cargo clean
RUN cargo install --path .

FROM debian:bullseye-slim
RUN rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/sja-backend /usr/local/bin/sja-backend
EXPOSE 3001
CMD ["sja-backend"]
