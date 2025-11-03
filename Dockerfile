FROM node:25-alpine AS frontend-builder
WORKDIR /frontend
COPY ./frontend /frontend
RUN npm install
RUN npm run build

FROM rust:1.90-slim-bookworm AS backend-builder
WORKDIR /build
COPY . /build
COPY --from=frontend-builder /frontend/dist /build/frontend/dist
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app
COPY --from=frontend-builder /frontend /app/frontend
COPY --from=backend-builder /build/target/release/rust-vue-skeleton rust-vue-skeleton
CMD ["./rust-vue-skeleton"]