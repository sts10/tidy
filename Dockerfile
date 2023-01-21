FROM rust:alpine as builder
# Workaround "cannot find crti.o"
RUN apk add --no-cache musl-dev
WORKDIR /app
# First, only copy over the Cargo manifests, and do a build with a dummy
# main.rs. This way, Docker can cache all of this, and only actually download
# and build those depepndencies once. This is nice, because otherwise it's
# quite slow.
COPY Cargo.toml Cargo.lock .
RUN mkdir src && echo "fn main() { }" > src/main.rs
RUN cargo build --release
RUN rm -r src/
# The above should get cached; but the below steps will be executed each
# time you change the source code and do 'docker build'.
COPY . .
RUN cargo build --release
RUN strip target/release/tidy

FROM scratch
COPY --from=builder /app/target/release/tidy /bin/tidy
ENTRYPOINT ["/bin/tidy"]

# Add some metadata about the image, for extra neatness.
LABEL org.opencontainers.image.title="tidy" \
      org.opencontainers.image.description="A command-line tool for combining and cleaning large word list files" \
      org.opencontainers.image.url="https://github.com/sts10/tidy" \
      org.opencontainers.image.authors="Sam Schlinkert <sschlinkert@gmail.com" \
      org.opencontainers.image.licenses="MIT"
