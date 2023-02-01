## Group Member Approval Smart Contract
This smart contract provides a way for group members to assert, on chain, their intention to become a member of a
[Provenance Blockchain Group](https://docs.cosmos.network/main/modules/group).

## Development Setup
This assumes the user is running Mac OSX.  

- To start developing with Rust, follow the standard [guide](https://www.rust-lang.org/tools/install).
- The contract uses `wasm-pack` with its `make build` command.  Use this [installer command](https://rustwasm.github.io/wasm-pack/installer/) to install it.
- To build the contract locally with its `make optimize`, a [Docker Environment](https://www.docker.com/products/docker-desktop/) is also required.
