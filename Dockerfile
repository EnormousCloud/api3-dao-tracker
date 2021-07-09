FROM ekidd/rust-musl-builder:stable as builder
RUN USER=root cargo new --bin api3tracker
# COPY ./client/Cargo.lock /home/rust/src/client/Cargo.lock
COPY ./client/Cargo.toml /home/rust/src/client/Cargo.toml
COPY ./client/src /home/rust/src/client/src
WORKDIR /home/rust/src/api3tracker
COPY ./server/Cargo.lock ./Cargo.lock
COPY ./server/Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
ADD ./server/src ./src/
RUN rm ./target/x86_64-unknown-linux-musl/release/deps/api3tracker*
RUN cargo build --release

FROM alpine:latest
EXPOSE 8000
ENV TZ=Etc/UTC \
    APP_USER=appuser \
    LOG_LEVEL=api3tracker=debug,info
RUN addgroup -S $APP_USER && adduser -S -g $APP_USER $APP_USER
COPY --from=builder /home/rust/src/api3tracker/target/x86_64-unknown-linux-musl/release/api3tracker /usr/src/app/api3tracker
# COPY ./client/dist /usr/src/app/dist
RUN chown -R $APP_USER:$APP_USER /usr/src/app
USER $APP_USER
WORKDIR /usr/src/app
ENTRYPOINT ["/usr/src/app/api3tracker", "-w"]

