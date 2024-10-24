FROM lukemathwalker/cargo-chef:0.1.62-rust-bookworm as chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
ENV SQLX_OFFLINE true

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
COPY ./.sqlx /app/.sqlx

RUN cargo install sqlx-cli --no-default-features --features postgres

RUN cargo build --release --bin server

FROM debian:bookworm-slim AS runtime

WORKDIR /app

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/server server
COPY --from=builder /app/.sqlx ./.sqlx

ENV PORT 8002
EXPOSE 8002

ENTRYPOINT ["./server"]