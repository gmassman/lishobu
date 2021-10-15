FROM rust:1.55-slim-bullseye AS base

ENV USER=root
#ENV ROCKET_ENV=development

WORKDIR /code

RUN cargo init
COPY ./.cargo .cargo
COPY ./vendor vendor
COPY Cargo.toml Cargo.lock ./

FROM base AS development

EXPOSE 8080

CMD [ "cargo", "run" ]

FROM base AS builder

RUN cargo build --release

FROM debian:bullseye-slim

#ENV ROCKET_ENV=production

EXPOSE 8080

COPY --from=builder /code/target/release/lishobu /lishobu

CMD [ "/lishobu" ]
