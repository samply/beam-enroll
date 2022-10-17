FROM rust:bullseye AS builder
WORKDIR /usr/src/app
COPY . . 
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=builder /usr/src/app/target/release/beam_enroll /usr/local/bin/
RUN mkdir -p  /data
ENTRYPOINT [ "/usr/local/bin/beam_enroll" ]
