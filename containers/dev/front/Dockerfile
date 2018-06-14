FROM rustlang/rust:nightly

RUN mkdir -p /usr/src/app
WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y git \
     build-essential \
     libfreetype6 libfreetype6-dev \
     libfontconfig1 libfontconfig1-dev \
     --no-install-recommends && \
     rm -rf /var/lib/apt/lists/*

RUN mkdir -p /usr/local/src/ && \
    git clone https://github.com/sass/sassc.git /usr/local/src/sassc && \
    cd /usr/local/src && \
    . sassc/script/bootstrap && \
    make -C sassc -j4 && \
    mkdir -p /opt/sassc && \
    PREFIX="/opt/sassc" make -C sassc install

ENV PATH=$PATH:/opt/sassc/bin    

RUN cargo install cargo-web

ENV HISTFILE=/usr/src/app/.bash_history
ENV LANG=ja_JP.UTF-8
