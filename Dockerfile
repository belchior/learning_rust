FROM rust:1.54-alpine as builder
WORKDIR /builder
COPY . .
RUN cargo build --release --bin rust_calc
RUN rm src/*.rs

FROM alpine
WORKDIR /app
COPY --from=builder /builder/target/release/rust_calc ./
ENTRYPOINT ["./rust_calc"]

# Build
# docker build -t rust_calc .

# Run
# docker run -it --rm rust_calc '1+2-3*4/5'
