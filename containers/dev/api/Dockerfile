FROM rust:1.26

RUN mkdir -p /usr/src/app
WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y pandoc \
     libfreetype6 libfreetype6-dev \
     libfontconfig1 libfontconfig1-dev \
     --no-install-recommends && \
     rm -rf /var/lib/apt/lists/*

RUN cargo install diesel_cli

ENV HISTFILE=/usr/src/app/.bash_history
ENV LANG=ja_JP.UTF-8
