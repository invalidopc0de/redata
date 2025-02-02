FROM rust:1.84 AS server-build
WORKDIR /src

# Copy the Cargo.toml and Cargo.lock files
COPY ./src/redata-server/Cargo.toml ./
COPY ./src/redata-server/Cargo.lock ./

# Create an empty src directory to trick Cargo into thinking it's a valid Rust project
RUN mkdir src && echo "fn main() { println!(\"Dummy\"); }" > ./src/main.rs

# Build the dependencies without the actual source code to cache dependencies separately
RUN cargo build --release

# Now copy the source code
COPY ./src/redata-server/ /src/

# Build your application
RUN cargo build --release && objcopy --compress-debug-sections target/release/redata-server ./main

FROM node:23 AS frontend-build

WORKDIR /src

COPY ./src/redata-frontend/package.json ./
COPY ./src/redata-frontend/yarn.lock ./

RUN yarn install

ENV PATH=/src/node_modules/.bin:$PATH

COPY ./src/redata-frontend /src

RUN yarn run build

FROM debian:bookworm-slim
LABEL authors="Mark Saunders"

WORKDIR /app

COPY --from=server-build /src/main /app
COPY --from=frontend-build /src/dist /www/static

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
ENV ROCKET_LOG_LEVEL=debug

ENTRYPOINT ["/app/main"]
