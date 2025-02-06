FROM rust:1.84 AS server-build
WORKDIR /app

COPY src/redata-server ./

RUN --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --locked --release
cp ./target/release/redata-server /app/main
EOF


FROM node:23 AS frontend-build

WORKDIR /app

COPY ./src/redata-frontend/package.json ./
COPY ./src/redata-frontend/yarn.lock ./

RUN yarn install

ENV PATH=/app/node_modules/.bin:$PATH

COPY ./src/redata-frontend /app

RUN yarn run build

FROM debian:bookworm-slim
LABEL authors="Mark Saunders"

WORKDIR /app

COPY --from=server-build /app/main /app
COPY --from=frontend-build /app/dist /www/static

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
ENV ROCKET_LOG_LEVEL=debug

ENTRYPOINT ["/app/main"]
