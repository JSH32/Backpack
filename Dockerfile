FROM rust:1.54.0 as builder
WORKDIR /usr/src/backpack
# Copy only the files needed to build the Rust project
COPY src ./src
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

FROM node:14
WORKDIR /usr/src
# Copy all files needed at runtime and to build the frontend
COPY client ./client
COPY scripts ./scripts
COPY migrations ./migrations
WORKDIR /usr/src/client
RUN npm install && npm run build
WORKDIR /usr/src
COPY --from=builder /usr/src/backpack/target/release/backpack .

ENTRYPOINT ["/bin/sh", "-c" , "node /usr/src/scripts/check.js && /usr/src/backpack /usr/src/client/build"]