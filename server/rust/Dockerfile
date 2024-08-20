FROM rust:1-bookworm AS build

COPY . /src
WORKDIR /src

RUN cargo build --release

FROM debian:bookworm

COPY --from=build /src/target/release/rust /app/rust
WORKDIR /app

EXPOSE 8080
VOLUME /app/images

ENTRYPOINT [ "/app/rust" ]