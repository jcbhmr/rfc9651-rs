mod common;

use static_regular_grammar::RegularGrammar;
use std::error::Error;

#[derive(RegularGrammar)]
#[grammar(file = "tests/abnf.abnf", entry_point = "sf-list", ascii)]
pub struct List([u8]);

#[derive(RegularGrammar)]
#[grammar(file = "tests/abnf.abnf", entry_point = "sf-dictionary", ascii)]
pub struct Dictionary([u8]);

#[derive(RegularGrammar)]
#[grammar(file = "tests/abnf.abnf", entry_point = "sf-item", ascii)]
pub struct Item([u8]);

#[test]
fn test_list() -> Result<(), Box<dyn Error>> {
    let list = List::new(b"foo, bar, baz").map_err(|_| "Failed to create list")?;
    _ = list;

    common::StringArray

    Ok(())
}
