FROM docker.io/paritytech/ci-unified:latest as builder

WORKDIR /polkadot
COPY . /polkadot

RUN cargo fetch
RUN cargo build --locked --release

FROM docker.io/parity/base-bin:latest

<<<<<<< HEAD
COPY --from=builder /polkadot/target/release/solochain-template-node /usr/local/bin
=======
COPY --from=builder /polkadot/target/release/minimal-template-node /usr/local/bin
>>>>>>> main

USER root
RUN useradd -m -u 1001 -U -s /bin/sh -d /polkadot polkadot && \
	mkdir -p /data /polkadot/.local/share && \
	chown -R polkadot:polkadot /data && \
	ln -s /data /polkadot/.local/share/polkadot && \
# unclutter and minimize the attack surface
	rm -rf /usr/bin /usr/sbin && \
# check if executable works in this container
<<<<<<< HEAD
	/usr/local/bin/solochain-template-node --version
=======
	/usr/local/bin/minimal-inventory-node --version
>>>>>>> main

USER polkadot

EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

<<<<<<< HEAD
ENTRYPOINT ["/usr/local/bin/solochain-template-node"]
=======
ENTRYPOINT ["/usr/local/bin/minimal-template-node"]
>>>>>>> main
