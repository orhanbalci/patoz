/*!
WebAssembly bindings for patoz. Exposes a single `parse` function to
JavaScript which returns parsed records as plain JS objects.
 */
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ParseResult<'a> {
    records: &'a [patoz::Record],
    /// 1-based line number where parsing stopped, `None` if whole input
    /// was consumed
    first_unparsed_line: Option<usize>,
}

/// Parses pdb file content. Parsing stops at the first unsupported record,
/// `firstUnparsedLine` tells where that happened.
#[wasm_bindgen]
pub fn parse(content: &str) -> Result<JsValue, JsError> {
    let (remaining, pdb_file) =
        patoz::parse(content).map_err(|e| JsError::new(&format!("{:?}", e)))?;
    let result = ParseResult {
        records: pdb_file.records(),
        first_unparsed_line: first_unparsed_line(content, remaining),
    };
    result
        .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
        .map_err(|e| JsError::new(&e.to_string()))
}

fn first_unparsed_line(content: &str, remaining: &[u8]) -> Option<usize> {
    if remaining.iter().all(u8::is_ascii_whitespace) {
        return None;
    }
    let consumed = &content.as_bytes()[..content.len() - remaining.len()];
    Some(consumed.iter().filter(|&&b| b == b'\n').count() + 1)
}

#[cfg(test)]
mod test {
    use super::first_unparsed_line;

    #[test]
    fn unparsed_line_number() {
        let content = "A\nB\nC\n";
        assert_eq!(
            first_unparsed_line(content, &content.as_bytes()[4..]),
            Some(3)
        );
        assert_eq!(first_unparsed_line(content, b""), None);
        assert_eq!(first_unparsed_line(content, b"\n"), None);
    }
}
