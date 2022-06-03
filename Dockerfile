FROM rust:latest

RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
RUN cargo install wasm-bindgen-cli

WORKDIR /usr/src/zeldar
COPY . .

# RUN cargo install --path .

# RUN npm install

EXPOSE 3000

CMD ["trunk", "serve"]