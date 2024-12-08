FROM lukemathwalker/cargo-chef:0.1.62-rust-bookworm as chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
ARG DATABASE_URL
ARG PORT
ARG JWT_SECRET
ARG JWT_TTL_IN_MINUTES

ENV DATABASE_URL=${DATABASE_URL}
ENV PORT=${PORT}
ENV JWT_SECRET=${JWT_SECRET}
ENV JWT_TTL_IN_MINUTES=${JWT_TTL_IN_MINUTES}
ENV SQLX_OFFLINE=true

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
COPY ./.sqlx /app/.sqlx

RUN cargo install sqlx-cli --no-default-features --locked --features postgres
RUN cargo build --release --bin server

FROM debian:bookworm-slim AS runtime

WORKDIR /app

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/server server
COPY --from=builder /app/.env ./.env
COPY --from=builder /app/.sqlx ./.sqlx

ARG PORT=8002
ENV PORT=${PORT}
EXPOSE ${PORT}

ENTRYPOINT ["./server"]