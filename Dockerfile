FROM rust:1.54.0-buster as build-env
WORKDIR /app
ADD . /app
RUN RUSTFLAGS="-C link-arg=-s" cargo build --release

FROM gcr.io/distroless/cc-debian10
ENV TZ="Asia/Jakarta"
WORKDIR /app
COPY --from=build-env /app/target/release/ebisu /app
CMD ["./ebisu"]