FROM rust:1.40 as builder
WORKDIR /usr/src/backpack
COPY . .
RUN cargo build --release

FROM node:14
COPY . .
WORKDIR /usr/src/client
RUN npm install
COPY --from=builder /usr/src/backpack/target/release/backpack /usr/src/