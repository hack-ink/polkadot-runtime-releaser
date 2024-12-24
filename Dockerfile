FROM ubuntu:25.04

LABEL maintainer="x@acg.box"
LABEL description="An environment for constructing a deterministic polkadot-sdk-based runtime."
LABEL version="0.1.4"

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
	rustup \
	&& rm -rf /var/lib/apt/lists/*

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
ENV CARGO_TERM_COLOR=always
ENV RUST_BACKTRACE=full
ENV WASM_BUILD_WORKSPACE_HINT=/workdir

WORKDIR /workdir

CMD ["/bin/bash"]
