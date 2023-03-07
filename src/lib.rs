//! Group Member Approval Smart Contract
//!
//! This contract uses [Cosmwasm](https://github.com/CosmWasm/cosmwasm)'s provided architecture in
//! conjunction with [Provwasm](#https://github.com/provenance-io/provwasm) to create a wasm smart
//! contract that can be deployed to and interact with the Provenance Blockchain.
//!
//! This solves an issue in the groups module: any admin of any group can add a member to a group
//! without their knowledge.  In order to create an ecosystem that respects group member involvement,
//! this grants group members the ability to explicitly state that they intended to become a member
//! of a group.

/// The entrypoint for all external commands sent to the compiled wasm.
pub mod contract;
/// All commands that are available when executing this contract.
pub mod execute;
/// Defines the contract instantiation process.
pub mod instantiate;
/// Defines the contract migration process.
pub mod migrate;
/// Defines the contract query process.
pub mod query;
/// Contains all internal storage communication functionality.
pub mod store;
/// Contains all declared structs for internal and external communication.
pub mod types;
/// Contains helper functionality for contract code facilitation.
pub mod util;

#[cfg(test)]
pub mod test;
