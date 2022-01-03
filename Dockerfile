ARG FLAVOR=alpine
FROM rust:alpine AS base-alpine

RUN apk add --no-cache musl-dev

FROM rust:slim-bullseye AS base-debian

ARG FLAVOR
FROM base-${FLAVOR} AS base

ENV USER=root

WORKDIR /code
RUN cargo init
COPY Cargo.toml /code/Cargo.toml
COPY Cargo.lock /code/Cargo.lock
RUN --mount=type=cache,target=/root/.cargo/git \
    --mount=type=cache,target=/root/.cargo/registry \
    cargo fetch

COPY src /code/src

RUN --mount=type=cache,target=/root/.cargo/git \
    --mount=type=cache,target=/root/.cargo/registry \
    cargo build --release

FROM debian:bullseye-slim AS debian

COPY --from=base /code/target/release/joule /usr/local/bin/

ENTRYPOINT ["/usr/local/bin/joule"]
CMD ["--help"]

FROM gcr.io/distroless/base-debian11 AS distroless

COPY --from=base /code/target/release/joule /usr/local/bin/

ENTRYPOINT ["/usr/local/bin/joule"]
CMD ["--help"]

FROM scratch AS export

COPY --from=base /code/target/release/joule /joule