FROM rustlang/rust:nightly-alpine AS deps

# RUN echo "https://dl-cdn.alpinelinux.org/alpine/edge/testing" >> /etc/apk/repositories
RUN apk update
RUN apk add musl-dev npm openssl-libs-static openssl-dev binaryen zlib-dev zlib-static
RUN npm install -g sass

ENV OPENSSL_STATIC=1 OPENSSL_LIB_DIR=/usr/lib OPENSSL_INCLUDE_DIR=/usr/include/openssl RUST_BACKTRACE=1 ZLIB_USE_STATIC_LIBS=ON

RUN rustup target add wasm32-unknown-unknown

ENV RUSTFLAGS="-C link-args=-Wl,-Bstatic -C link-args=-lc"

RUN cargo install cargo-generate
RUN cargo install --features no_downloads --locked cargo-leptos

ENV RUSTFLAGS=""

WORKDIR /usr/src/codecheck

# Copy utility crates
# COPY Cargo.lock ./
# RUN printf "[workspace]\nmembers=[\"web\"]\nresolver=\"2\"\n[profile.wasm-release]\ninherits = \"release\"\nopt-level = 'z'\nlto = true\ncodegen-units = 1\npanic = \"abort\"" > Cargo.toml

# RUN USER=root cargo new --bin web

# # Build external libraries
# WORKDIR /usr/src/codecheck/web
# RUN touch src/lib.rs
# COPY web/Cargo.toml .
# # Clear all path-based (local) packages
# RUN sed --in-place '/path = "\.\./d' Cargo.toml
# #RUN if [[ $TARGETARCH = "amd64" ]] ; then cargo build --target x86_64-unknown-linux-musl --release ; \
# #    else cargo build --target aarch64-unknown-linux-musl --release ; fi
# RUN --mount=type=cache,target=/usr/local/cargo/registry \
#     --mount=type=cache,target=/usr/src/codecheck/target \
#     cargo build --release

# Copy and build internal libraries
WORKDIR /usr/src/codecheck
COPY db ./db
COPY util ./util
COPY auth ./auth
COPY macros ./macros
COPY analysis ./analysis
COPY web ./web
RUN printf "[workspace]\nmembers=[\"db\",\"auth\",\"util\",\"macros\",\"analysis\",\"web\"]\nresolver=\"2\"\n[profile.wasm-release]\ninherits = \"release\"\nopt-level = 'z'\nlto = true\ncodegen-units = 1\npanic = \"abort\"" > ./Cargo.toml

# Copy env file to all subdirectories (library crates)
COPY .env .
RUN find ./ -type d -path ./web/assets -prune -exec cp .env {} \;

WORKDIR /usr/src/codecheck/web
COPY web/Cargo.toml ./Cargo.toml

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/codecheck/target \
    cargo build --release
RUN rm -r src
# END LIBRARIES

# Build executable
# Copy actual source files
COPY web/ .
# Copy env file to all subdirectories (main crate)
RUN find . -type d -path ./web/assets -prune -exec cp ../.env {} \;

FROM deps as tester

ARG MONGO_URI
ENV MONGO_TEST_DB_URI=$MONGO_URI

# doing this in the root directory OR using cargo leptos test makes the command panic
# i don't know why this is happening, i'll deal with it later
# WORKDIR /usr/src/codecheck/
# RUN --mount=type=cache,target=/usr/local/cargo/registry \
#     --mount=type=cache,target=/usr/src/codecheck/target \
#     cargo leptos test
WORKDIR /usr/src/codecheck/web

FROM tester as builder
ARG AUTH_BACKEND="basic_auth"

RUN --mount=type=cache,target=/usr/src/codecheck/target \
    rm -f /usr/src/codecheck/target/release/deps/web*

RUN mkdir /usr/src/codecheck/out
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/codecheck/target \
    cargo leptos build --features ${AUTH_BACKEND} --release; mv /usr/src/codecheck/target/release/web /usr/src/codecheck/out/app; mv /usr/src/codecheck/target/site /usr/src/codecheck/out/

FROM alpine AS webserver

ARG APP=/usr/src
ENV RUST_BACKTRACE=1

EXPOSE 8080
EXPOSE 27017
ENV LEPTOS_SITE_ADDR="0.0.0.0:8080"
ENV LEPTOS_SITE_ROOT="site"
#
#ENV TZ=Etc/UTC
##    APP_USER=appuser
##
##RUN addgroup -S $APP_USER \
##    && adduser -S -g $APP_USER $APP_USER
#
#RUN apk update \
#    && apk add --no-cache musl-dev openssl-dev musl openssl musl-utils \
#    && rm -rf /var/cache/apk/*
RUN apk update
RUN apk add --no-cache ca-certificates

COPY --from=builder /usr/src/codecheck/out/ ${APP}/tmp

#RUN chown -R $APP_USER:$APP_USER ${APP}

#USER $APP_USER
WORKDIR ${APP}
RUN mv tmp/app .
RUN if [[ -d "tmp/site" ]] ; then cp -r ./tmp/site/ ./site ; fi
RUN rm -r tmp

# Safety precaution
RUN find . -name .env -type f -delete

ENTRYPOINT ["./app"]