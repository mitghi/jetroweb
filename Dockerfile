FROM rust:latest
WORKDIR /usr/src/jetroweb

COPY . .
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
RUN cargo install --locked wasm-bindgen-cli

#RUN wget -qO- https://github.com/thedodd/trunk/releases/download/v0.16.0/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf-

RUN trunk build
RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/jetroweb"]
