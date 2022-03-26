# Build API
FROM rust:1.59 as builder
WORKDIR /usr/src/backpack
# Copy only the files needed to build the Rust project
COPY src ./src
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# Runtime container
FROM node:16

WORKDIR /usr/src
COPY client ./client
COPY migrations ./migrations
COPY docker/* ./

# Build frontend
WORKDIR /usr/src/client
RUN npm install && npm run build
WORKDIR /usr/src

# Copy backend binary from builder
COPY --from=builder /usr/src/backpack/target/release/backpack .

# Run script to start both processes
CMD "./start.sh"