FROM node:25-alpine AS frontend-builder
COPY ./frontend /frontend
WORKDIR /frontend
RUN npm install
RUN npm run build

FROM rust:1.90
COPY . /app
COPY --from=frontend-builder /frontend /app/frontend
WORKDIR /app
RUN cargo install --path .