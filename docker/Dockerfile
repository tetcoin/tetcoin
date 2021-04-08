FROM tetcoin/ci-linux:production as builder
LABEL description="This is the build stage for Tetcoin. Here we create the binary."

ARG PROFILE=release
WORKDIR /tetcoin

COPY . /tetcoin

RUN cargo build --$PROFILE

# ===== SECOND STAGE ======

FROM debian:buster-slim
LABEL description="This is the 2nd stage: a very small image where we copy the Tetcoin binary."
ARG PROFILE=release
COPY --from=builder /tetcoin/target/$PROFILE/tetcoin /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /tetcoin tetcoin && \
	mkdir -p /tetcoin/.local/share && \
	mkdir /data && \
	chown -R tetcoin:tetcoin /data && \
	ln -s /data /tetcoin/.local/share/tetcoin && \
	rm -rf /usr/bin /usr/sbin

USER tetcoin
EXPOSE 30333 9933 9944
VOLUME ["/data"]

CMD ["/usr/local/bin/tetcoin"]
