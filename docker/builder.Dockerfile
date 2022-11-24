FROM rust:1.65 as build


ARG BUILD_ENV

# create a new empty shell project
RUN USER=root cargo new --bin uws_gateway
WORKDIR /uws_gateway

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN if [ "$BUILD_ENV" = "release"  ]; then cargo build --release; else cargo build ; fi
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/${BUILD_ENV}/deps/uws_gateway*
RUN if [ "$BUILD_ENV" = "release"  ]; then cargo build --release; else cargo build ; fi