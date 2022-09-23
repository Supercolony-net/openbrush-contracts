FROM rust

RUN apt-get update && \
    apt-get install libclang-dev -y && \
    apt-get install nodejs -y && \
    apt-get install npm -y && \
    apt-get install binaryen -y

RUN apt install -y libprotobuf-dev protobuf-compiler cmake

RUN npm install -g n && \
    npm install -g yarn && \
    n 15.8.0

RUN curl -sSf https://sh.rustup.rs/ | sh -s -- --default-toolchain nightly -y

RUN rustup component add rust-src --toolchain nightly-x86_64-unknown-linux-gnu
RUN rustup target add wasm32-unknown-unknown

RUN cargo install cargo-dylint dylint-link

RUN git clone https://github.com/Supercolony-net/substrate-contracts-node --branch feature/pallet-assets-chain-extension \
    cd substrate-contracts-node \
    WASM_BUILD_WORKSPACE_HINT=$PWD cargo install --path node --force --locked

# While Redspot didn't merge `--skip-linting` https://github.com/patractlabs/redspot/pull/181
# we will use our version of `cargo-contract` without linting
# RUN cargo install --force cargo-contract && \
RUN cargo install cargo-contract --git https://github.com/Supercolony-net/cargo-contract --force
