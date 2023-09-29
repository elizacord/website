FROM rust:1.71-slim-buster AS builder

WORKDIR /tmp/eliza-website

RUN cargo init

COPY Cargo.lock Cargo.toml ./

RUN cargo build --release
RUN rm -f target/release/deps/eliza_website*

COPY ./static ./static
COPY ./templates ./templates
COPY ./src ./src

RUN cargo build --release

FROM debian:buster-slim

COPY --from=builder /etc/ssl/certs /etc/ssl/certs/
COPY --from=builder /usr/share/ca-certificates /usr/share/ca-certificates/
COPY --from=builder /tmp/eliza-website/target/release/eliza-website .
COPY --from=builder /tmp/eliza-website/static ./static
COPY cert.pem key.pem ./

EXPOSE 443
CMD ./eliza-website
