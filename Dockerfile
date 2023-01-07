FROM rust:latest

ENV ROCKET_ADDRESS="0.0.0.0"

WORKDIR /usr/src/backend
COPY ./backend/ .
COPY ./frontend/build ../frontend/build/

RUN rustup default nightly
RUN cargo fetch
CMD cargo run