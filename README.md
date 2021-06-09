# Tetcoin

Implementation of a https://tetcoin.org node in Rust based on the Tetcore framework.

[tetcore-repo]: https://github.com/tetcoin/tetcore

This repo contains runtimes for the Tetcoin, Metrocoin, and Westend networks. The README provides
information about installing the `tetcoin` binary and developing on the codebase. For more
specific guides, like how to be a validator, see the
[Tetcoin Wiki](https://wiki.tetcoin.network/docs/en/).

## Installation

If you just wish to run a Tetcoin node without compiling it yourself, you may
either run the latest binary from our
[releases](https://github.com/tetcoin/tetcoin/releases) page, or install
Tetcoin from one of our package repositories.

Installation from the debian or rpm repositories will create a `systemd`
service that can be used to run a Tetcoin node. This is disabled by default,
and can be started by running `systemctl start tetcoin` on demand (use
`systemctl enable tetcoin` to make it auto-start after reboot). By default, it
will run as the `tetcoin` user.  Command-line flags passed to the binary can
be customised by editing `/etc/default/tetcoin`. This file will not be
overwritten on updating tetcoin. You may also just run the node directly from
the command-line.

### Debian-based (Debian, Ubuntu)

Currently supports Debian 10 (Buster) and Ubuntu 20.04 (Focal), and
derivatives. Run the following commands as the `root` user.

```
# Import the security@parity.io GPG key
gpg --recv-keys --keyserver hkps://keys.mailvelope.com 9D4B2B6EB8F97156D19669A9FF0812D491B96798
gpg --export 9D4B2B6EB8F97156D19669A9FF0812D491B96798 > /usr/share/keyrings/parity.gpg
# Add the Parity repository and update the package index
echo 'deb [signed-by=/usr/share/keyrings/parity.gpg] https://releases.parity.io/deb release main' > /etc/apt/sources.list.d/parity.list
apt update
# Install the `tetsy-keyring` package - This will ensure the GPG key
# used by APT remains up-to-date
apt install tetsy-keyring
# Install tetcoin
apt install tetcoin

```

### RPM-based (Fedora, CentOS)

Currently supports Fedora 32 and CentOS 8, and derivatives.

```
# Install dnf-plugins-core (This might already be installed)
dnf install dnf-plugins-core
# Add the repository and enable it
dnf config-manager --add-repo https://releases.parity.io/rpm/tetcoin.repo
dnf config-manager --set-enabled tetcoin
# Install tetcoin (You may have to confirm the import of the GPG key, which
# should have the following fingerprint: 9D4B2B6EB8F97156D19669A9FF0812D491B96798)
dnf install tetcoin
```

## Building

### Install via Cargo

If you want to install Tetcoin in your PATH, you can do so with with:

```bash
cargo install --git https://github.com/tetcoin/tetcoin --tag <version> tetcoin --locked
```

### Build from Source

If you'd like to build from source, first install Rust. You may need to add Cargo's bin directory
to your PATH environment variable. Restarting your computer will do this for you automatically.

```bash
curl https://sh.rustup.rs -sSf | sh
```

If you already have Rust installed, make sure you're using the latest version by running:

```bash
rustup update
```

Once done, finish installing the support software:

```bash
sudo apt install build-essential git clang libclang-dev pkg-config libssl-dev
```

Build the client by cloning this repository and running the following commands from the root
directory of the repo:

```bash
git checkout <latest tagged release>
./scripts/init.sh
cargo build --release
```

Note that compilation is a memory intensive process. We recommend having 4 GiB of phyiscal RAM or swap available (keep in mind that if a build hits swap it tends to be very slow).

## Networks

This repo supports runtimes for Tetcoin, Metrocoin, and Westend.

### Connect to Tetcoin Mainnet

Connect to the global Tetcoin Mainnet network by running:

```bash
./target/release/tetcoin --chain=tetcoin
```

You can see your node on [telemetry] (set a custom name with `--name "my custom name"`).

[telemetry]: https://telemetry.tetcoin.org/#list/Tetcoin

### Connect to the "Metrocoin" Canary Network

Connect to the global Metrocoin canary network by running:

```bash
./target/release/tetcoin --chain=metrocoin
```

You can see your node on [telemetry] (set a custom name with `--name "my custom name"`).

[telemetry]: https://telemetry.tetcoin.org/#list/Metrocoin

### Connect to the Westend Testnet

Connect to the global Westend testnet by running:

```bash
./target/release/tetcoin --chain=westend
```

You can see your node on [telemetry] (set a custom name with `--name "my custom name"`).

[telemetry]: https://telemetry.tetcoin.org/#list/Westend

### Obtaining TETs

If you want to do anything on Tetcoin, Metrocoin, or Westend, then you'll need to get an account and
some TET, MTC, or WND tokens, respectively. See the
[claims instructions](https://claims.tetcoin.network/) for Tetcoin if you have TETs to claim. For
Westend's WND tokens, see the faucet
[instructions](https://wiki.tetcoin.network/docs/en/learn-TET#getting-westies) on the Wiki.

## Hacking on Tetcoin

If you'd actually like hack on Tetcoin, you can grab the source code and build it. Ensure you have
Rust and the support software installed. This script will install or update Rust and install the
required dependencies (this may take up to 30 minutes on Mac machines):

```bash
curl https://gettetcore.io -sSf | bash -s -- --fast
```

Then, grab the Tetcoin source code:

```bash
git clone https://github.com/tetcoin/tetcoin.git
cd tetcoin
```

Then build the code. You will need to build in release mode (`--release`) to start a network. Only
use debug mode for development (faster compile times for development and testing).

```bash
./scripts/init.sh   # Install WebAssembly. Update Rust
cargo build # Builds all native code
```

You can run the tests if you like:

```bash
cargo test --all
```

You can start a development chain with:

```bash
cargo run -- --dev
```

Detailed logs may be shown by running the node with the following environment variables set:

```bash
RUST_LOG=debug RUST_BACKTRACE=1 cargo run -- --dev
```

### Development

You can run a simple single-node development "network" on your machine by running:

```bash
tetcoin --dev
```

You can muck around by heading to https://tetcoin.js.org/apps and choose "Local Node" from the
Settings menu.

### Local Two-node Testnet

If you want to see the multi-node consensus algorithm in action locally, then you can create a
local testnet. You'll need two terminals open. In one, run:

```bash
tetcoin --chain=tetcoin-local --alice -d /tmp/alice
```

And in the other, run:

```bash
tetcoin --chain=tetcoin-local --bob -d /tmp/bob --port 30334 --bootnodes '/ip4/127.0.0.1/tcp/30333/p2p/ALICE_BOOTNODE_ID_HERE'
```

Ensure you replace `ALICE_BOOTNODE_ID_HERE` with the node ID from the output of the first terminal.

### Using Docker
[Using Docker](doc/docker.md)

### Shell Completion
[Shell Completion](doc/shell-completion.md)

## Contributing

### Contributing Guidelines

[Contribution Guidelines](CONTRIBUTING.md)

### Contributor Code of Conduct

[Code of Conduct](CODE_OF_CONDUCT.md)

## License

Tetcoin is [GPL 3.0 licensed](LICENSE).

## Important Notice

https://tetcoin.org/testnetdisclaimer
