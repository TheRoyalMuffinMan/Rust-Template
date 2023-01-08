FROM rust:latest

# ENV ROCKET_ADDRESS="0.0.0.0"
ENV MONGOURI="mongodb+srv://root:rootpassword@mongo:27017/golangDB?retryWrites=true&w=majority"

WORKDIR /usr/src/backend
# COPY ./backend/ .
# COPY ./frontend/build ../frontend/build/

RUN rustup default nightly
RUN cargo fetch
CMD cargo run