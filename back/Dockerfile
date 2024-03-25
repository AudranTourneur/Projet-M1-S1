FROM ubuntu:22.04

RUN apt-get update -y

RUN apt-get install cargo libssl-dev pkg-config gcc g++ libpq-dev -y

WORKDIR /opt/app

COPY dummy.rs .
COPY Cargo.toml .

RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml
RUN cargo build --release
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml

COPY . .

RUN cargo build

CMD ["cargo", "run"]
