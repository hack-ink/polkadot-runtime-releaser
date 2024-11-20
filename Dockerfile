FROM ubuntu:25.04

LABEL maintainer="x@acg.box"
LABEL description="An environment for constructing a deterministic polkadot-sdk-based runtime."
LABEL version="0.1.2"

RUN apt-get update && \
	DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
	build-essential \
	ca-certificates \
	clang \
	cmake \
	curl \
	git \
	libssl-dev \
	pkg-config \
	protobuf-compiler \
	&& rm -rf /var/lib/apt/lists/*

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --no-modify-path --default-toolchain none && \
	mv $HOME/.rustup /usr/local/rustup && \
	mv $HOME/.cargo /usr/local/cargo && \
	chmod -R a+w /usr/local/rustup /usr/local/cargo

ENV CARGO_HOME=/usr/local/cargo
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
ENV CARGO_TERM_COLOR=always
ENV PATH=/usr/local/cargo/bin:$PATH
ENV RUST_BACKTRACE=full
ENV RUSTUP_HOME=/usr/local/rustup
ENV WASM_BUILD_WORKSPACE_HINT=/workdir

WORKDIR /workdir

CMD ["/bin/bash"]
