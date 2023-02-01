use std::env::current_dir;
use std::fs::create_dir_all;

use cosmwasm_schema::{export_schema, remove_schemas, schema_for};
use group_memeber_approval_smart_contract::types::core::msg::{
    ExecuteMsg, InstantiateMsg, MigrateMsg,
};

fn main() {
    let mut out_dir = current_dir().expect("Could not fetch current directory");
    out_dir.push("schema");
    create_dir_all(&out_dir).expect("Could not create output directory");
    remove_schemas(&out_dir).expect("Could not remove existing schemas in output directory");
    // Top-level Msg values
    export_schema(&schema_for!(ExecuteMsg), &out_dir);
    export_schema(&schema_for!(InstantiateMsg), &out_dir);
    export_schema(&schema_for!(MigrateMsg), &out_dir);
    // Result data
}
