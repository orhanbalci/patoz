/*!
WebAssembly bindings for patoz. Exposes a single `parse` function to
JavaScript which returns parsed records as plain JS objects.
 */
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct ParseResult<'a> {
    records: &'a [patoz::Record],
}

/// Parses pdb file content. Lines of record types that are not supported
/// yet are returned as `Unknown` records.
#[wasm_bindgen]
pub fn parse(content: &str) -> Result<JsValue, JsError> {
    let (_, pdb_file) = patoz::parse(content).map_err(|e| JsError::new(&format!("{:?}", e)))?;
    let result = ParseResult {
        records: pdb_file.records(),
    };
    result
        .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
        .map_err(|e| JsError::new(&e.to_string()))
}
