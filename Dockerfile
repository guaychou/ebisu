FROM rust:1.55.0-bullseye as build-env
WORKDIR /app
ADD . /app
RUN RUSTFLAGS="-C link-arg=-s" cargo build --release

FROM gcr.io/distroless/cc-debian11
ENV TZ="Asia/Jakarta"
WORKDIR /app
COPY --from=build-env /app/target/release/ebisu /app
CMD ["./ebisu"]