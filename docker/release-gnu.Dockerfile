FROM rust:1.65 as build


WORKDIR /uws_gateway

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# copy your source tree
COPY ./src ./src

# build for release
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,sharing=locked,target=/uws_gateway/target \
    cargo build --release --locked && \
    mv /uws_gateway/target/release/uws_gateway /uws_gateway/uws_gateway

FROM debian:buster-slim

# copy the build artifact from the build stage
COPY --from=build /uws_gateway/uws_gateway .

# set the startup command to run your binary
CMD ["/uws_gateway"]

