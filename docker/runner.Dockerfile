FROM debian:buster-slim

RUN apt update && apt install -y entr && rm -rf /var/lib/apt/lists/*