FROM rust:1.60 as build

# Create a new empty shell project
RUN USER=root cargo new --bin backpack
WORKDIR /backpack

# copy over your manifests
COPY Cargo.toml Cargo.lock ./

# Cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy source
COPY ./src ./src
COPY ./.git ./.git

# Build for release
RUN rm ./target/release/deps/backpack*
RUN cargo build --release

# Running stage
FROM rust:1.60

# Copy the build artifact from the build stage
COPY --from=build /backpack/target/release/backpack .

# Migrations needed at runtime
COPY ./migrations ./migrations

CMD ["./backpack"]