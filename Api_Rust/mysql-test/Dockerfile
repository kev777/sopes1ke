FROM rust:1.55

WORKDIR /app

COPY ./ ./

RUN cargo build --release

COPY . .

EXPOSE 3030

CMD cargo run