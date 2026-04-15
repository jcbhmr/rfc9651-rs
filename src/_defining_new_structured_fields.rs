//!
//!
//!
//!
//! ```
//! use jcbhmr_rfc9651::Item;
//!
//! pub struct FooExample {
//!     value: FooExampleValue,
//!     foo_url: Url,
//! }
//!
//! // 0-10 inclusive range-checked type
//! pub struct FooExampleValue(u8);
//! // impl TryFrom<...> for FooExampleValue { ... }
//!
//! impl FromStr for FooExample {
//!    type Err = ();
//!    fn from_str(s: &str) -> Result<Self, Self::Err> {
//!        let item: Item = s.parse().map_err(|_| ())?;
//!        let value = match *item {
//!            Item::Integer(i) => FooExampleValue::try_from(i).map_err(|_| ())?,
//!            _ => return Err(()),
//!        };
//!        let foo_url = match item.parameters().get("foo").ok_or(())? {
//!            ParameterValue::String(s) => Url::parse(s).map_err(|_| ())?,
//!            _ => return Err(()),
//!        };
//!        Ok(FooExample { value, foo_url })
//!    }
//! }
//! ```
