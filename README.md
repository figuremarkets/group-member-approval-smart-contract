## Group Member Approval Smart Contract
This smart contract provides a way for group members to assert, on chain, their intention to become a member of a
[Provenance Blockchain Group](https://docs.cosmos.network/main/modules/group).

This solves an issue in the groups module: any admin of any group can add a member to a group without their knowledge.  
In order to create an ecosystem that respects group member involvement, this grants group members the ability to 
explicitly state that they intended to become a member of a group.

## Contract Instantiation

To instantiate a new instance of the contract, the following parameters are required:

* `contract_name`: This name will identify the contract when separate instances are deployed on chain.  It can be found
via the `query_contract_state` query.
* `attribute_name`: The [Provenance Blockchain Name Module](https://docs.provenance.io/modules/name-module) name that 
will be bound to the contract.  When the `approve_group_membership` execution route is invoked, a
[Provenance Blockchain Attribute](https://docs.provenance.io/modules/account) will be bound to the invoking account that
includes the specified group id as its INT value.
* `bind_attribute_name`: If specified as `true`, the value specified in `attribute_name` will be automatically bound 
as a Provenance Blockchain Name to the contract during the instantiation process.  If this is omitted, the same name
specified in `attribute_name` must manually be bound after the contract is instantiated.  This is useful in circumstances
where the name will be bound using a restricted name module namespace.

Example instantiation payload:
```json
{
  "contract_name": "Sample identifying name for auxiliary contract",
  "attribute_name": "somename.sc.pb",
  "bind_attribute_name": true
}
```

## Contract Execution

In order for a member to verify their membership in a group, they must simply invoke the contract's sole execution route
with the following payload:

```json
{
  "approve_group_membership": {
    "group_id": "1"
  }
}
```

## Contract Query

The contract currently provides a single query route for verifying its version and naming conventions. It can be queried
with the following payload:

```json
{
  "query_contract_state": {}
}
```

## Contract Migration

In order to migrate the contract to new versions, run the migrate command with the following payload:

```json
{
  "contract_upgrade": {}
}
```

## Development Setup
This assumes the user is running Mac OSX.  

- To start developing with Rust, follow the standard [guide](https://www.rust-lang.org/tools/install).
- The contract uses `wasm-pack` with its `make build` command.  Use this [installer command](https://rustwasm.github.io/wasm-pack/installer/) to install it.
- To build the contract locally with its `make optimize`, a [Docker Environment](https://www.docker.com/products/docker-desktop/) is also required.
