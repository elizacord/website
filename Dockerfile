FROM rust:1.64
WORKDIR /usr/src/eliza.gg
COPY . .
RUN cargo build --release
EXPOSE 443
CMD ./target/release/eliza-website