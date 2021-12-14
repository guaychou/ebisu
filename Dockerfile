FROM rust:1.57.0-bullseye as build-env
WORKDIR /app
ADD . /app
RUN RUSTFLAGS="-C link-arg=-s --cfg unsound_local_offset" cargo build --release

FROM gcr.io/distroless/cc-debian11:nonroot-amd64
ENV TZ="Asia/Jakarta"
WORKDIR /app
COPY --from=build-env /app/target/release/ebisu /app
CMD ["./ebisu"]
