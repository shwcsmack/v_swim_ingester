FROM rust:latest as build

# 1. Create a new empty shell project
RUN USER=root cargo new --bin v_swim_ingester
WORKDIR /v_swim_ingester

# 2. Copy our manifests
COPY Cargo.lock ./Cargo.lock
COPY Cargo.toml ./Cargo.toml

# 3. Build only the dependencies to cache them
RUN cargo build --release
RUN rm src/*.rs

# 4. Now that the dependency is built, copy your source code
COPY src ./src
COPY src/vatsim ./src/vatsim

# 5. Build for release.
RUN rm ./target/release/deps/v_swim_ingester*
RUN cargo install --path .

# our final base
FROM rust:slim-buster

# copy the build artifact from the build stage
COPY --from=build /v_swim_ingester/target/release/v_swim_ingester .

# set the startup command to run your binary
CMD ["./v_swim_ingester"]
