FROM rust:1.67

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

EXPOSE 3000

CMD ["app"]
