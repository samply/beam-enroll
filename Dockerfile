FROM rust:bullseye AS builder
WORKDIR /usr/src/app
COPY . . 
RUN cargo build --release
RUN mkdir /data

FROM gcr.io/distroless/cc
COPY --from=builder /usr/src/app/target/release/beam_enroll /usr/local/bin/
COPY --from=builder /data /data
ENTRYPOINT [ "/usr/local/bin/beam_enroll" ]
