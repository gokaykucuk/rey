FROM --platform=arm64 rust
EXPOSE 5800
WORKDIR /app
COPY . .
RUN cargo build --release
ENTRYPOINT ["./target/release/rey"]