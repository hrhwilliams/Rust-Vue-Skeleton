FROM node:24-alpine AS frontend-builder
WORKDIR /frontend
COPY ./frontend /frontend
RUN npm install @fullcalendar/core @fullcalendar/vue3 @fullcalendar/daygrid @fullcalendar/interaction
RUN npm install
RUN npm run build

FROM rust:1.91-alpine3.22 AS chef
WORKDIR /build
RUN apk add --no-cache mold musl-dev openssl-dev openssl-libs-static
RUN cargo install cargo-chef

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS backend-builder
WORKDIR /build
RUN cargo install sqlx-cli --no-default-features --features postgres,native-tls --locked
COPY --from=planner /build/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
COPY --from=frontend-builder /frontend/dist /build/frontend/dist
ENV SQLX_OFFLINE=true
RUN cargo build --release

FROM alpine:3.22 AS runtime
WORKDIR /app
RUN apk add curl
COPY --from=backend-builder /build/frontend/dist /app/frontend/dist
COPY --from=backend-builder /build/target/release/rust-vue-skeleton rust-vue-skeleton
COPY --from=backend-builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx
COPY --from=backend-builder /build/migrations /app/migrations
COPY --from=backend-builder /build/scripts/init_db.sh init_db.sh
ENV DB_HOST=postgres
ENTRYPOINT ["./init_db.sh"]
CMD ["./rust-vue-skeleton"]