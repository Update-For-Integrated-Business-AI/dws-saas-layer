FROM rust:1.65 as build

# create a new empty shell project
RUN USER=root cargo new --bin uws_gateway
WORKDIR /uws_gateway

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/uws_gateway*
RUN cargo build --release

FROM debian:buster-slim

# copy the build artifact from the build stage
COPY --from=build /uws_gateway/target/release/uws_gateway .

# set the startup command to run your binary
CMD ["./uws_gateway"]