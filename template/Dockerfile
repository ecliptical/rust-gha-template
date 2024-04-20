FROM rust:1.77-alpine AS builder
RUN apk add --no-cache musl-dev
WORKDIR /usr/src/{{ project-name }}
COPY . .
RUN cargo install --path .

FROM scratch
COPY --from=builder /usr/local/cargo/bin/{{ project-name }} /usr/local/bin/{{ project-name }}
ENTRYPOINT [ "/usr/local/bin/{{ project-name }}" ]
