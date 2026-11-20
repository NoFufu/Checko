# Checko sync server (used by Render). Builds only the server crate.
FROM rust:1-slim AS build
WORKDIR /app
COPY . .
RUN cargo build --release -p checko-server

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=build /app/target/release/checko-server /usr/local/bin/checko-server
ENV PORT=8080
EXPOSE 8080
CMD ["checko-server"]
