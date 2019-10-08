FROM clux/muslrust:latest as builder

ADD . /volume

RUN cargo build