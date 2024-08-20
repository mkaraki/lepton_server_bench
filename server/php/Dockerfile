
FROM php:8.3 AS build-lepton

RUN apt-get update && \
    apt-get install -y \
    libclang-dev \
    curl \
    git && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y

RUN git clone --depth 1 https://github.com/mkaraki/microsoft_lepton_jpeg_rust_php.git /ms-lepton
WORKDIR /ms-lepton

RUN . "$HOME/.cargo/env"; cargo build --release

FROM php:8.3-apache

RUN mv "$PHP_INI_DIR/php.ini-production" "$PHP_INI_DIR/php.ini"

COPY --from=build-lepton /ms-lepton/target/release/libmicrosoft_lepton_jpeg_rust_php.so /ms-lepton.so
RUN mv /ms-lepton.so $(php-config --extension-dir)/ms-lepton.so
RUN echo extension=ms-lepton.so > $(php-config --ini-dir)/20-ms-lepton.ini

VOLUME /var/www/html/images/

COPY index.php /var/www/html/
