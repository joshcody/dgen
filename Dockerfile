FROM rustlang/rust:nightly

RUN rustup target add x86_64-unknown-linux-gnu
VOLUME ["/dgen"]
WORKDIR /dgen
RUN echo $PATH
RUN which cargo
RUN "cargo build --release --target=x86_64-unknown-linux-gnu --bin dgen"