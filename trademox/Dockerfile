FROM rust:1.80 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc
WORKDIR /app
COPY --from=builder /app/target/release/tradebox /usr/local/bin/tradebox
USER 65532:65532
EXPOSE 3013
HEALTHCHECK --interval=30s --timeout=3s CMD ["/usr/local/bin/tradebox", "--healthcheck"]
ENTRYPOINT ["/usr/local/bin/tradebox"]