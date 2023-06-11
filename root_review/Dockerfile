FROM rust:buster as builder

ENV DATABASE_URL=postgres://tuber:tuber@postgres/tuber


WORKDIR /app/src
RUN USER=root cargo new --bin root_review
COPY Cargo.toml Cargo.lock ./root_review/

WORKDIR /app/src/root_review
RUN cargo build

COPY ./ ./
RUN cargo build

FROM debian:stable-slim
RUN apt update \
    && apt install -y openssl ca-certificates \
    && apt clean \
    && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

WORKDIR /app
ENV DATABASE_URL=postgres://tuber:tuber@postgres/tuber


EXPOSE 3333

COPY --from=builder /app/src/root_review/target/debug/root_review ./

CMD ["/app/root_review"]