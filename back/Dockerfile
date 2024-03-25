FROM ubuntu:22.04

RUN apt-get update -y

RUN apt-get install cargo libssl-dev pkg-config gcc g++ libpq-dev -y

COPY . /opt/app

WORKDIR /opt/app

RUN cargo build

CMD ["cargo", "run"]
