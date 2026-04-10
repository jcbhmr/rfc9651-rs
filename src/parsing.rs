use crate::ascii_string::{AsciiStr, AsciiString};
use alloc::{borrow::ToOwned, string::String, vec::Vec};

/// > Given an ASCII string as input_string, return a Date. input_string is modified to remove the parsed value.
/// >
/// > 1. If the first character of input_string is not "@", fail parsing.
/// > 2. Discard the first character of input_string.
/// > 3. Let output_date be the result of running Parsing an Integer or Decimal ([Section 4.2.4](https://www.rfc-editor.org/rfc/rfc9651#parse-number)) with input_string.
/// > 4. If output_date is a Decimal, fail parsing.
/// > 5. Return output_date.
///
/// &mdash; [4.2.9. Parsing a Date | RFC 9651](https://www.rfc-editor.org/rfc/rfc9651#name-parsing-a-date)
// Given an ASCII string as input_string, return a Date. input_string is modified to remove the parsed value.
pub fn parsing_a_date(input_string: &mut AsciiString) -> Result<Date, ()> {
    // 1. If the first character of input_string is not "@", fail parsing.
    if !input_string.as_str().starts_with("@") {
        return Err(());
    }

    // 2. Discard the first character of input_string.
    input_string.drain(..1);

    // 3. Let output_date be the result of running Parsing an Integer or Decimal ([Section 4.2.4](https://www.rfc-editor.org/rfc/rfc9651#parse-number)) with input_string.
    let output_date = parsing_an_integer_or_decimal(input_string)?;

    // 4. If output_date is a Decimal, fail parsing.
    let output_date = match output_date {
        IntegerOrDecimal::Integer(i) => i,
        IntegerOrDecimal::Decimal(_) => return Err(()),
    };

    // 5. Return output_date.
    Ok(output_date)
}

/// > Given an ASCII string as input_string, return a sequence of Unicode code points. input_string is modified to remove the parsed value.
/// >
/// > 1. If the first two characters of input_string are not "%" followed by DQUOTE, fail parsing.
/// > 2. Discard the first two characters of input_string.
/// > 3. Let byte_array be an empty byte array.
/// > 4. While input_string is not empty:
/// >     1. Let char be the result of consuming the first character of input_string.
/// >     2. If char is in the range %x00-1f or %x7f-ff (i.e., it is not in VCHAR or SP), fail parsing.
/// >     3. If char is "%":
/// >         1. Let octet_hex be the result of consuming two characters from input_string. If there are not two characters, fail parsing.
/// >         2. If octet_hex contains characters outside the range %x30-39 or %x61-66 (i.e., it is not in 0-9 or lowercase a-f), fail parsing.
/// >         3. Let octet be the result of hex decoding octet_hex ([Section 8](https://rfc-editor.org/rfc/rfc4648#section-8) of [[RFC4648](https://www.rfc-editor.org/rfc/rfc9651#RFC4648)]).
/// >         4. Append octet to byte_array.
/// >     4. If char is DQUOTE:
/// >         1. Let unicode_sequence be the result of decoding byte_array as a UTF-8 string ([Section 3](https://rfc-editor.org/rfc/rfc3629#section-3) of [[UTF8](https://www.rfc-editor.org/rfc/rfc9651#RFC3629)]). Fail parsing if decoding fails.
/// >         2. Return unicode_sequence.
/// >     5. Otherwise, if char is not "%" or DQUOTE:
/// >         1. Let byte be the result of applying ASCII encoding to char.
/// >         2. Append byte to byte_array.
/// > 5. Reached the end of input_string without finding a closing DQUOTE; fail parsing.
///
/// &mdash; [4.2.10. Parsing a Display String | RFC 9651](https://www.rfc-editor.org/rfc/rfc9651#name-parsing-a-display-string)
// Given an ASCII string as input_string, return a sequence of Unicode code points. input_string is modified to remove the parsed value.
pub fn parsing_a_display_string(input_string: &mut AsciiString) -> Result<String, ()> {
    // 1. If the first two characters of input_string are not "%" followed by DQUOTE, fail parsing.
    if !input_string.as_str().starts_with("%\"") {
        return Err(());
    }

    // 2. Discard the first two characters of input_string.
    input_string.drain(..2);

    // 3. Let byte_array be an empty byte array.
    let mut byte_array: Vec<u8> = Vec::new();

    // 4. While input_string is not empty:
    while !input_string.as_str().is_empty() {
        // 1. Let char be the result of consuming the first character of input_string.
        let char = input_string.drain(..1).next().unwrap();

        // 2. If char is in the range %x00-1f or %x7f-ff (i.e., it is not in VCHAR or SP), fail parsing.
        if matches!(char, 0x00..=0x1f | 0x7f..=0xff) {
            return Err(());
        }

        // 3. If char is "%":
        if char == b'%' {
            // 1. Let octet_hex be the result of consuming two characters from input_string. If there are not two characters, fail parsing.
            let octet_hex: Vec<_> = input_string.drain(..2).collect();
            if octet_hex.len() != 2 {
                return Err(());
            }

            // 2. If octet_hex contains characters outside the range %x30-39 or %x61-66 (i.e., it is not in 0-9 or lowercase a-f), fail parsing.
            if octet_hex
                .iter()
                .any(|&b| !matches!(b, 0x30..=0x39 | 0x61..=0x66))
            {
                return Err(());
            }

            // 3. Let octet be the result of hex decoding octet_hex ([Section 8](https://rfc-editor.org/rfc/rfc4648#section-8) of [[RFC4648](https://www.rfc-editor.org/rfc/rfc9651#RFC4648)]).
            let octet_hex_str = unsafe { str::from_utf8_unchecked(&octet_hex) };
            let octet_result = u8::from_str_radix(octet_hex_str, 16);
            let octet = unsafe { octet_result.unwrap_unchecked() };

            // 4. Append octet to byte_array.
            byte_array.push(octet);
        }
        // 4. If char is DQUOTE:
        else if char == b'"' {
            // 1. Let unicode_sequence be the result of decoding byte_array as a UTF-8 string ([Section 3](https://rfc-editor.org/rfc/rfc3629#section-3) of [[UTF8](https://www.rfc-editor.org/rfc/rfc9651#RFC3629)]). Fail parsing if decoding fails.
            let unicode_sequence = str::from_utf8(&byte_array).map_err(|_| ())?;

            // 2. Return unicode_sequence.
            return Ok(unicode_sequence.to_owned());
        }
        // 5. Otherwise, if char is not "%" or DQUOTE:
        else {
            // 1. Let byte be the result of applying ASCII encoding to char.
            let byte = char;

            // 2. Append byte to byte_array.
            byte_array.push(byte);
        }
    }

    // 5. Reached the end of input_string without finding a closing DQUOTE; fail parsing.
    Err(())
}
