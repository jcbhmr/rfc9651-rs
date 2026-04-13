![🚧 Under construction 👷‍♂️](https://i.imgur.com/LEP2R3N.png)

# Structured Field Values for HTTP for Rust

🤓 [@jcbhmr](https://jcbhmr.com)'s Rust implementation of [<cite>Structured Field Values for HTTP</cite>](https://www.rfc-editor.org/rfc/rfc9651)

<table align=center>
<tr><td>

```rust
// Reporting-Endpoints HTTP header
let reporting_endpoints = r#"mycsp="https://example.org/csp""#;
let reporting_endpoints: Dictionary = reporting_endpoints.parse()?;
for (name, url) in reporting_endpoints.iter() {
    let url = url.try_unwrap_string()?;
    println!("Endpoint '{}' is {}", name, url);
}
```

<tr><td>

```rust
// Sec-CH-UA-Mobile HTTP header
let sec_ch_ua_mobile = "?0";
let sec_ch_ua_mobile: Item = sec_ch_ua_mobile.parse()?;
let sec_ch_ua_mobile = sec_ch_ua_mobile.try_unwrap_boolean()?;
println!("Is this a mobile device? {}", if sec_ch_ua_mobile { "Yes" } else { "No" });
```

</table>

## Installation

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=Rust&logoColor=FFFFFF)

You can install this package from [crates.io](https://crates.io/) using [Cargo](https://doc.rust-lang.org/cargo/):

```sh
cargo add jcbhmr-rfc9651
```

### Features

💡 Install this package as `#![no_std]` compatible using `--no-default-features --features alloc` when running `cargo add`.

- **`alloc`** (enabled by default): Enable automatically using the `Global` allocator.
- **`std`** (enabled by default): Enable integration with the Rust standard library.
- **`serde`:** Implement `serde::Serialize` and `serde::Deserialize` for most types.

## Usage

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=Rust&logoColor=FFFFFF)
![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?style=for-the-badge&logo=WebAssembly&logoColor=FFFFFF)

⚛️ This crate works well with WebAssembly!

The most interesting parts of the `jcbhmr_rfc9651` crate are the `Dictionary`, `List`, and `Item` types. Note that `Item` is a `struct` that `Deref`s to `enum BareItem` with an extra associated `Parameters`.

[`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html) implementations are provided for all three of these types, so you can easily parse structured field values from strings.

```rust
let example_header = r#"foo=1, bar=2"#;
let dictionary: Dictionary = example_header.parse()?;
for (name, item) in dictionary.iter() {
    println!("{}: {}", name, item);
}
// Output:
// foo: 1
// bar: 2
```

[`ToString`](https://doc.rust-lang.org/std/string/trait.ToString.html) implementations are also provided (via `Display`), so you can easily serialize these types back into strings.

```rust
let mut dictionary = Dictionary::new();
dictionary.insert("foo", Item::from(1));
dictionary.insert("bar", Item::from(2));
let example_header = dictionary.to_string();
assert_eq!(example_header, "foo=1, bar=2");
```

[📚 Check out the documentation for more details & examples](https://docs.rs/jcbhmr-rfc9651/latest/jcbhmr_rfc9651/)

## Alternatives

- **[sfv](https://crates.io/crates/sfv):** Not `#![no_std]` compatible.
- **[sfparse](https://crates.io/crates/sfparse):** Not `#![no_std]` compatible. Avoids allocation; uses incremental parsing.

## See also

<!-- TODO: Add more packages to this list -->

- **[mimic-rs](https://docs.rs/mimic-rs/latest/mimic_rs/):** Parse `User-Agent` header into a structured user-agent client hints format.

## Development

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=Rust&logoColor=FFFFFF)

This Cargo package is intended to:

- Be `#![no_std]`-compatible. Have an `std` feature.
- Provide an `<impl Allocator>` generic for things that require allocation. Default to `<A = Global>` when the `alloc` feature is enabled.
- Include detailed quotes from the specification where it is helpful.
- Include example code for parsing headers that are defined in terms of Structured Field Values.
- One example should be WASM-related.
- Offer an API to parse a `&str` into a `Dictionary`, `List`, or `Item`.
- Offer an API to serialize a `Dictionary`, `List`, or `Item` into a `String`.
- Offer an API that is more [SAX](https://en.wikipedia.org/wiki/Simple_API_for_XML)-like for parsing `&str` as any of the structured field value types. This would reduce reliance on allocations for extraneous intermediate fields that a consumer may not care about.

### Including RFC document in source control

You can store the whole RFC document in source control.

> **Am I allowed to reproduce whole RFCs?** \
> Yes. Since the beginning of the RFC series, reproduction of whole RFCs
> (including translation into a language other than English) has been allowed
> and encouraged. The IETF Trust and the RFC Editor place no restrictions on
> this. Most RFCs include the standard phrase “Distribution of this memo is
> unlimited” to indicate this.

&mdash; [Reproducing RFCs | Frequently Asked Questions — IETF Trust](https://trustee.ietf.org/about/faq/#reproducing-rfcs)

The current [`rfc9651.pdf`](./rfc9651.pdf) was last updated on 2026-04-07.

Storing the specification that this project implements _within its source control repository_ is a good way to sort of include it as a dependency (if that's the right word?) and make it easy to see when it changed (`git diff`). It also has the added bonus of making it easy to open the document as a separate side-by-side in-IDE tab when working on code and quickly referencing the specification. PDF format seems to be the most self-contained format for this purpose.

### Excerpts from RFC document in comments & documentation

You can include text from RFC documents in other media, but it must have clear attribution. Large excerpts like the ones used in this project also require the copyright statement.

> **Am I allowed to reproduce extracts from RFCs?** \
> It is common to use extracts from RFCs that are in the form of computer code
> by incorporating them in software. This is the only usage formally allowed by
> the current IETF rules (RFC 5378). Generally speaking the IETF Trust will
> tolerate fair use of other extracts, but you **must indicate the source of the
> extract and you must mention the original copyright statement** if present.

&mdash; [Reproducing RFCs | Frequently Asked Questions — IETF Trust](https://trustee.ietf.org/about/faq/#reproducing-rfcs) _(emphasis added)_

> - c. Licenses For Use Outside the IETF Standards Process. In addition to the
>    rights granted with respect to Code Components described in Section 4
>    below, the IETF Trust hereby grants to each person who wishes to exercise
>    such rights, to the greatest extent that it is permitted to do so, a
>    non-exclusive, royalty-free, worldwide right and license under all
>    copyrights and rights of authors:
>    - i. to copy, publish, display and distribute IETF Contributions and IETF
>      Documents in full and without modification,
>    - ii. to translate IETF Contributions and IETF Documents into languages
>      other than English, and to copy, publish, display and distribute such
>      translated IETF Contributions and IETF Documents in full and without
>      modification,
>    - iii. **to copy, publish, display and distribute unmodified portions of IETF
>      Contributions and IETF Documents** and translations thereof, provided
>      that:
>      - (x) each such portion is **clearly attributed to IETF and identifies
>        the RFC** or other IETF Document or IETF Contribution from which it
>        is taken,
>      - (y) all IETF legends, **legal notices and indications of authorship**
>        contained in the original IETF RFC **must also be included** where any
>        substantial portion of the text of an IETF RFC, and in any event
>        where more than one-fifth of such text, is reproduced in a single
>        document or series of related documents.
>
> - d. Licenses that are not Granted. The following licenses are not granted pursuant to
>   these Legal Provisions:
>   - i. any license to modify IETF Contributions or IETF Documents, or portions
>     thereof (other than to make translations or to extract, use and modify Code
>     Components as permitted under the licenses granted under Section 4 of these
>     Legal Provisions) in any context outside the IETF Standards Process, or
>   - ii. any license to **publish, display or distribute** IETF Contributions or IETF
>     Documents, or portions thereof, **without the required legends and notices**
>     described in these Legal Provisions.

&mdash; [Legal Provisions Relating to IETF Documents](https://trustee.ietf.org/wp-content/uploads/Corrected-TLP-5.0-legal-provsions.pdf) _(emphasis added)_

This project uses significant quotes from the specification in documentation for items like `List`, `Dictionary`, and `Item`. These excerpts should are marked with `<blockquote>` tags (`>` in Markdown) and clearly attributed to the RFC (preferrably to a specific section). The copyright statement is included in the crate-level documentation as well as in the README.

## License

Licensed under either of Apache License, Version 2.0 or MIT license at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

This project's documentation and source code contains lots of quotes from [[RFC9651]](https://www.rfc-editor.org/rfc/rfc9651). These quotes are under a separate copyright & license from the rest of the code. The copyright statement for these quotes is as follows:

> Copyright (c) 2024 IETF Trust and the persons identified as the document
> authors. All rights reserved.
> 
> This document is subject to BCP 78 and the IETF Trust's Legal Provisions
> Relating to IETF Documents (https://trustee.ietf.org/license-info) in effect
> on the date of publication of this document. Please review these documents
> carefully, as they describe your rights and restrictions with respect to this
> document. Code Components extracted from this document must include Revised
> BSD License text as described in Section 4.e of the Trust Legal Provisions and
> are provided without warranty as described in the Revised BSD License.

---

<dl id=bibliography>
<dt id=biblio-rfc9651><b>[RFC9651]</b>
<dd>M. Nottingham; P-H. Kamp. <a href="https://www.rfc-editor.org/rfc/rfc9651"><cite>Structured Field Values for HTTP</cite></a>. September 2024. Proposed Standard. URL: <a href="https://www.rfc-editor.org/rfc/rfc9651">https://www.rfc-editor.org/rfc/rfc9651</a>
</dl>