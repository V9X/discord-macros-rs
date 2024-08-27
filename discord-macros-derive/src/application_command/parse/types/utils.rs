use bitflags::Flags;
use darling::{Error, Result};
use proc_macro2::Span;
use syn::LitStr;

use crate::utils::SpannedValue;

pub fn validate_string(
    string: &str,
    span: &Span,
    max_size: usize,
    mixed_case: bool,
    whitespace: bool,
) -> Result<()> {
    let mut acc = Error::accumulator();
    let mut err = |err: Error| acc.push(err.with_span(span));

    let string_trimmed = string.trim();

    if string_trimmed.is_empty() {
        err(Error::custom("String cannot be empty"));
    }

    if string_trimmed.len() != string.len() {
        err(Error::custom(
            "String cannot start or end with whitespace characters",
        ));
    }

    let first_char_lowercase = string.chars().next().unwrap().is_lowercase();
    let mut is_mixed_case = false;
    let mut illegal_characters = Vec::new();
    let mut char_len = 0;

    // https://discord.com/developers/docs/interactions/application-commands#application-command-object-application-command-naming
    for char in string.chars() {
        let is_alphabetic = char.is_alphabetic();
        if !((whitespace || !char.is_whitespace()) || is_alphabetic || char == '-' || char == '_') {
            illegal_characters.push(char);
        }

        if !mixed_case && is_alphabetic && first_char_lowercase != char.is_lowercase() {
            is_mixed_case = true;
        }

        char_len += 1;
    }

    if is_mixed_case {
        err(Error::custom(
            "String cannot contain both lowercase and uppercase characters",
        ))
    }

    if !illegal_characters.is_empty() {
        err(Error::custom(format_args!(
            "String contains illegal character(s): {illegal_characters:?}",
        )));
    }

    if char_len > max_size {
        err(Error::custom(format_args!(
            "String too long, expected no more than {max_size} characters, got {char_len}"
        )));
    }

    acc.finish()?;

    Ok(())
}

pub fn parse_bitflags<T: Flags>(values: Vec<LitStr>) -> Result<T> {
    const MIN_JARO_CONFIDENCE: f64 = 0.7;

    let mut err_accumulator = Error::accumulator();
    let mut out = T::empty();

    for value in values {
        let value_string = value.value();
        let value_string_uppercase = value_string.to_uppercase();

        if let Some(flag) = T::from_name(&value_string) {
            out.insert(flag);
        } else {
            let mut candidate: Option<(f64, &str)> = None;
            for alternative in T::FLAGS {
                let confidence = strsim::jaro_winkler(&value_string_uppercase, alternative.name());

                if confidence > MIN_JARO_CONFIDENCE
                    && !candidate.is_some_and(|(c, _)| c > confidence)
                {
                    candidate = Some((confidence, alternative.name()));
                }
            }

            let err = if let Some((_, candidate)) = candidate {
                Error::custom(format_args!(
                    "Unknown value: `{value_string}`. Did you mean `{candidate}`?"
                ))
            } else {
                Error::custom(format_args!("Unknown value: `{value_string}`"))
            };

            err_accumulator.push(err.with_span(&value))
        }
    }

    err_accumulator.finish()?;

    Ok(out)
}

pub fn parse_bitflags_spn<T: Flags>(value: SpannedValue<Vec<LitStr>>) -> Result<SpannedValue<T>> {
    Ok(SpannedValue {
        value: parse_bitflags(value.value)?,
        span: value.span,
    })
}
