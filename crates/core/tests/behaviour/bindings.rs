//! bindings.ts is generated from the command list.

use noobenkyo_lib::bindings::{builder, export};

#[test]
fn exports_bindings() {
    export(&builder()).unwrap();
}
