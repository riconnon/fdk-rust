FROM alpine:3.8

COPY boilerplate /boilerplate

ARG tag=latest
RUN sed -i "s/TAG/${tag}/" /boilerplate/func.init.yaml

WORKDIR /boilerplate
CMD ["tar", "c", "Cargo.toml", "func.init.yaml", "src"]
