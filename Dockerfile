FROM rust:1.60 as build

# Create a new empty shell project
RUN USER=root cargo new --bin backpack
WORKDIR /backpack

# # copy over your manifests
# COPY Cargo.toml Cargo.lock ./
# COPY migration/Cargo.toml ./migration/Cargo.toml

# # Cache dependencies
# RUN cargo build --release
# RUN rm src/*.rs

# # Copy source
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src
COPY ./migration ./migration
COPY ./.git ./.git

# Build for release
# RUN rm ./target/release/deps/backpack*
RUN cargo build --release

# Running stage
FROM gcr.io/distroless/cc

# Copy the build artifact from the build stage
COPY --from=build /backpack/target/release/backpack .

CMD ["./backpack"]