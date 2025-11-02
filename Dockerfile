FROM rust:1.90
WORKDIR /code
COPY . /code/app/
WORKDIR /code/app
RUN cargo install --path .