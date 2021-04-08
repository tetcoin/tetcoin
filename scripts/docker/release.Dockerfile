FROM debian:buster-slim

# metadata
ARG VCS_REF
ARG BUILD_DATE
ARG TETCOIN_VERSION

LABEL io.parity.image.authors="devops-team@parity.io" \
	io.parity.image.vendor="Parity Technologies" \
	io.parity.image.title="parity/tetcoin" \
	io.parity.image.description="tetcoin: a platform for web3" \
	io.parity.image.source="https://github.com/tetcoin/tetcoin/blob/${VCS_REF}/scripts/docker/Dockerfile" \
	io.parity.image.revision="${VCS_REF}" \
	io.parity.image.created="${BUILD_DATE}" \
	io.parity.image.documentation="https://github.com/tetcoin/tetcoin/"

# show backtraces
ENV RUST_BACKTRACE 1

# install tools and dependencies
RUN apt-get update && \
		DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
			libssl1.1 \
			ca-certificates \
			curl \
			gnupg && \
		useradd -m -u 1000 -U -s /bin/sh -d /tetcoin tetcoin && \
		gpg --recv-keys --keyserver hkps://keys.mailvelope.com 9D4B2B6EB8F97156D19669A9FF0812D491B96798 && \
		gpg --export 9D4B2B6EB8F97156D19669A9FF0812D491B96798 > /usr/share/keyrings/parity.gpg && \
		echo 'deb [signed-by=/usr/share/keyrings/parity.gpg] https://releases.parity.io/deb release main' > /etc/apt/sources.list.d/parity.list && \
		apt-get update && \
		apt-get install -y --no-install-recommends tetcoin=${TETCOIN_VERSION#?} && \
# apt cleanup
		apt-get autoremove -y && \
		apt-get clean && \
		rm -rf /var/lib/apt/lists/*

USER tetcoin

# check if executable works in this container
RUN /usr/bin/tetcoin --version

EXPOSE 30333 9933 9944
VOLUME ["/tetcoin"]

ENTRYPOINT ["/usr/bin/tetcoin"]

