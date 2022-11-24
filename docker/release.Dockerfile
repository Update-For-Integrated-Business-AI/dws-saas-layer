FROM rust:1.65-alpine as build

# Required to use the std library
RUN apk add --no-cache musl-dev

# Add musl target to build for scratch
RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /uws_gateway

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# copy your source tree
COPY ./src ./src

# build for release
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,sharing=locked,target=/uws_gateway/target \
    cargo build --release --target=x86_64-unknown-linux-musl --locked && \
    mv /uws_gateway/target/x86_64-unknown-linux-musl/release/uws_gateway /uws_gateway/uws_gateway

FROM scratch

# copy the build artifact from the build stage
COPY --from=build /uws_gateway/uws_gateway .

# set the startup command to run your binary
CMD ["/uws_gateway"]

