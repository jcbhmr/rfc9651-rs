//! > # Appendix C. ABNF
//! >
//! > This section uses the Augmented Backus-Naur Form (ABNF) notation
//! > [[RFC5234]](https://www.rfc-editor.org/rfc/rfc5234) to illustrate the
//! > expected syntax of Structured Fields. However, it cannot be used to
//! > validate their syntax because it does not capture all requirements.
//!
//! > This section is non-normative. If there is disagreement between the
//! > parsing algorithms and ABNF, the specified algorithms take precedence.
//! >
//! > ```abnf
//! > sf-list       = list-member *( OWS "," OWS list-member )
//! > list-member   = sf-item / inner-list
//! >
//! > inner-list    = "(" *SP [ sf-item *( 1*SP sf-item ) *SP ] ")"
//! >                 parameters
//! >
//! > parameters    = *( ";" *SP parameter )
//! > parameter     = param-key [ "=" param-value ]
//! > param-key     = key
//! > key           = ( lcalpha / "*" )
//! >                 *( lcalpha / DIGIT / "_" / "-" / "." / "*" )
//! > lcalpha       = %x61-7A ; a-z
//! > param-value   = bare-item
//! >
//! > sf-dictionary = dict-member *( OWS "," OWS dict-member )
//! > dict-member   = member-key ( parameters / ( "=" member-value ))
//! > member-key    = key
//! > member-value  = sf-item / inner-list
//! >
//! > sf-item   = bare-item parameters
//! > bare-item = sf-integer / sf-decimal / sf-string / sf-token
//! >             / sf-binary / sf-boolean / sf-date / sf-displaystring
//! >
//! > sf-integer       = ["-"] 1*15DIGIT
//! > sf-decimal       = ["-"] 1*12DIGIT "." 1*3DIGIT
//! > sf-string        = DQUOTE *( unescaped / "%" / bs-escaped ) DQUOTE
//! > sf-token         = ( ALPHA / "*" ) *( tchar / ":" / "/" )
//! > sf-binary        = ":" base64 ":"
//! > sf-boolean       = "?" ( "0" / "1" )
//! > sf-date          = "@" sf-integer
//! > sf-displaystring = "%" DQUOTE *( unescaped / "\" / pct-encoded )
//! >                    DQUOTE
//! >
//! > base64       = *( ALPHA / DIGIT / "+" / "/" ) *"="
//! >
//! > unescaped    = %x20-21 / %x23-24 / %x26-5B / %x5D-7E
//! > bs-escaped   = "\" ( DQUOTE / "\" )
//! >
//! > pct-encoded  = "%" lc-hexdig lc-hexdig
//! > lc-hexdig = DIGIT / %x61-66 ; 0-9, a-f
//! > ```
//!
//! &mdash; [[RFC9651]](https://www.rfc-editor.org/rfc/rfc9651)
//!
//! > # 5.6.2. Tokens
//! >
//! > Tokens are short textual identifiers that do not include whitespace or
//! > delimiters.
//! >
//! >  ```abnf
//! >  token          = 1*tchar
//! >  
//! >  tchar          = "!" / "#" / "$" / "%" / "&" / "'" / "*"
//! >                 / "+" / "-" / "." / "^" / "_" / "`" / "|" / "~"
//! >                 / DIGIT / ALPHA
//! >                 ; any VCHAR, except delimiters
//! >  ```
//! >  
//! >  Many HTTP field values are defined using common syntax components,
//! >  separated by whitespace or specific delimiting characters. Delimiters are
//! >  chosen from the set of US-ASCII visual characters not allowed in a token
//! >  (DQUOTE and "(),/:;<=>?@[\]{}").
//!
//! &mdash; [[RFC9110]](https://www.rfc-editor.org/rfc/rfc9110)
//!
//! > # 5.6.3. Whitespace
//! >
//! > This specification uses three rules to denote the use of linear whitespace: OWS (optional whitespace), RWS (required whitespace), and BWS ("bad" whitespace).
//! >
//! > The OWS rule is used where zero or more linear whitespace octets might appear. For protocol elements where optional whitespace is preferred to improve readability, a sender SHOULD generate the optional whitespace as a single SP; otherwise, a sender SHOULD NOT generate optional whitespace except as needed to overwrite invalid or unwanted protocol elements during in-place message filtering.
//! >
//! > The RWS rule is used when at least one linear whitespace octet is required to separate field tokens. A sender SHOULD generate RWS as a single SP.
//! >
//! > OWS and RWS have the same semantics as a single SP. Any content known to be defined as OWS or RWS MAY be replaced with a single SP before interpreting it or forwarding the message downstream.
//! >
//! > The BWS rule is used where the grammar allows optional whitespace only for historical reasons. A sender MUST NOT generate BWS in messages. A recipient MUST parse for such bad whitespace and remove it before interpreting the protocol element.
//! >
//! > BWS has no semantics. Any content known to be defined as BWS MAY be removed before interpreting it or forwarding the message downstream.
//! >
//! > ```abnf
//! >   OWS            = *( SP / HTAB )
//! >                  ; optional whitespace
//! >   RWS            = 1*( SP / HTAB )
//! >                  ; required whitespace
//! >   BWS            = OWS
//! >                  ; "bad" whitespace
//! > ```
//!
//! &mdash; [[RFC9110]](https://www.rfc-editor.org/rfc/rfc9110)
use alloc::{string::String, vec::Vec};
use chumsky::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct List(Vec<ListMember>);

#[derive(Debug, Clone, PartialEq)]
pub enum ListMember {
    Item(Item),
    InnerList(InnerList),
}

#[derive(Debug, Clone, PartialEq)]
pub struct InnerList {
    items: Vec<Item>,
    parameters: Parameters,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameters(Vec<Parameter>);

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    key: ParameterKey,
    value: Option<ParameterValue>,
}

pub type ParameterKey = String;

pub type ParameterValue = BareItem;

#[derive(Debug, Clone, PartialEq)]
pub struct Dictionary(Vec<DictMember>);

#[derive(Debug, Clone, PartialEq)]
pub struct DictMember {
    key: Key,
    rest: DictMemberRest,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DictMemberRest {
    Parameters(Parameters),
    Value(MemberValue),
}

#[derive(Debug, Clone, PartialEq)]
pub enum MemberValue {
    Item(Item),
    InnerList(InnerList),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Item {
    bare_item: BareItem,
    parameters: Parameters,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BareItem {
    Integer(i64),
    Decimal(f64),
    String(String),
    Token(String),
    Binary(Vec<u8>),
    Boolean(bool),
    Date(i64),
    DisplayString(String),
}

pub type Key = String;

pub fn list<'a>() -> impl Parser<'a, &'a str, List> {
    list_member()
        .then(ows().ignore_then(just(',')).ignore_then(ows()).ignore_then(list_member()).repeated().collect::<Vec<_>>()).map(|(first, rest)| {
            let mut members = Vec::new();
            members.push(first);
            members.extend(rest);
            List(members)
        })
}

pub fn dictionary<'a>() -> impl Parser<'a, &'a str, Dictionary> {

}

pub fn item<'a>() -> impl Parser<'a, &'a str, Item> {

}

fn list_member<'a>() -> impl Parser<'a, &'a str, ListMember> {
    item().map(ListMember::Item).or(inner_list().map(ListMember::InnerList))
}

fn inner_list<'a>() -> impl Parser<'a, &'a str, InnerList> {
    just('(')
        .ignore_then(sp().repeated())
        .ignore_then(
            item().then(
                sp().ignore_then(sp().repeated()).ignore_then(item()).repeated().collect::<Vec<_>>()
            ).then_ignore(sp().repeated())
            .map(|(first, rest)| {
                let mut items = Vec::new();
                items.push(first);
                items.extend(rest);
                items
            })
        )
        .then_ignore(just(')'))
        .then(parameters())
        .map(|(items, parameters)| InnerList { items, parameters })
}

fn parameters<'a>() -> impl Parser<'a, &'a str, Parameters> {
    just(';')
        .ignore_then(sp().repeated())
        .ignore_then(parameter())
        .repeated()
        .collect::<Vec<_>>()
        .map(Parameters)
}

fn parameter<'a>() -> impl Parser<'a, &'a str, Parameter> {
    param_key()
        .then(
            just('=')
                .ignore_then(param_value())
                .or_not()
        )
        .map(|(key, value)| Parameter { key, value })
}

fn param_key<'a>() -> impl Parser<'a, &'a str, ParameterKey> {
    key()
}

fn param_value<'a>() -> impl Parser<'a, &'a str, ParameterValue> {
    bare_item()
}

fn tchar<'a>() -> impl Parser<'a, &'a str, char> {
    one_of("!#$%&'*+-.^_`|~1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz")
}

fn sp<'a>() -> impl Parser<'a, &'a str, char> {
    just(' ')
}

fn ows<'a>() -> impl Parser<'a, &'a str, ()> {
    one_of(" \t").repeated().ignored()
}