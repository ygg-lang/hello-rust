use super::*;

pub(super) fn parse_cst(input: &str, rule: Json5Rule) -> OutputResult<Json5Rule> {
    state(input, |state| match rule {
        Json5Rule::Value => parse_value(state),
        Json5Rule::Object => parse_object(state),
        Json5Rule::ObjectPair => parse_object_pair(state),
        Json5Rule::ObjectKey => parse_object_key(state),
        Json5Rule::Array => parse_array(state),
        Json5Rule::String => parse_string(state),
        Json5Rule::StringRaw => parse_string_raw(state),
        Json5Rule::StringText => parse_string_text(state),
        Json5Rule::StringEscape => parse_string_escape(state),
        Json5Rule::Number => parse_number(state),
        Json5Rule::Integer => parse_integer(state),
        Json5Rule::Identifier => parse_identifier(state),
        Json5Rule::Boolean => parse_boolean(state),
        Json5Rule::Null => parse_null(state),
        Json5Rule::WhiteSpace => parse_white_space(state),
        Json5Rule::IgnoreText => unreachable!(),
        Json5Rule::IgnoreRegex => unreachable!(),
    })
}
#[inline]
fn parse_value(state: Input) -> Output {
    state.rule(Json5Rule::Value, |s| {
        Err(s)
            .or_else(|s| parse_object(s).and_then(|s| s.tag_node("object")))
            .or_else(|s| parse_array(s).and_then(|s| s.tag_node("array")))
            .or_else(|s| parse_string(s).and_then(|s| s.tag_node("string")))
            .or_else(|s| parse_number(s).and_then(|s| s.tag_node("number")))
            .or_else(|s| parse_boolean(s).and_then(|s| s.tag_node("boolean")))
            .or_else(|s| parse_null(s).and_then(|s| s.tag_node("null")))
    })
}
#[inline]
fn parse_object(state: Input) -> Output {
    state.rule(Json5Rule::Object, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "{", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.optional(|s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| parse_object_pair(s).and_then(|s| s.tag_node("object_pair")))
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| {
                                    s.repeat(0..4294967295, |s| {
                                        s.sequence(|s| {
                                            Ok(s)
                                                .and_then(|s| builtin_text(s, ",", false))
                                                .and_then(|s| builtin_ignore(s))
                                                .and_then(|s| parse_object_pair(s).and_then(|s| s.tag_node("object_pair")))
                                        })
                                    })
                                })
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| s.optional(|s| builtin_text(s, ",", false)))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "}", false))
        })
    })
}
#[inline]
fn parse_object_pair(state: Input) -> Output {
    state.rule(Json5Rule::ObjectPair, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| parse_object_key(s).and_then(|s| s.tag_node("object_key")))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, ":", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| parse_value(s).and_then(|s| s.tag_node("value")))
        })
    })
}
#[inline]
fn parse_object_key(state: Input) -> Output {
    state.rule(Json5Rule::ObjectKey, |s| {
        Err(s)
            .or_else(|s| parse_integer(s).and_then(|s| s.tag_node("integer")))
            .or_else(|s| parse_identifier(s).and_then(|s| s.tag_node("identifier")))
            .or_else(|s| parse_string(s).and_then(|s| s.tag_node("string")))
    })
}
#[inline]
fn parse_array(state: Input) -> Output {
    state.rule(Json5Rule::Array, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "[", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| {
                    s.optional(|s| {
                        s.sequence(|s| {
                            Ok(s)
                                .and_then(|s| parse_value(s).and_then(|s| s.tag_node("value")))
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| {
                                    s.repeat(0..4294967295, |s| {
                                        s.sequence(|s| {
                                            Ok(s)
                                                .and_then(|s| builtin_text(s, ",", false))
                                                .and_then(|s| builtin_ignore(s))
                                                .and_then(|s| parse_value(s).and_then(|s| s.tag_node("value")))
                                        })
                                    })
                                })
                                .and_then(|s| builtin_ignore(s))
                                .and_then(|s| s.optional(|s| builtin_text(s, ",", false)))
                        })
                    })
                })
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_text(s, "]", false))
        })
    })
}
#[inline]
fn parse_string(state: Input) -> Output {
    state.rule(Json5Rule::String, |s| {
        Err(s)
            .or_else(|s| {
                s.sequence(|s| {
                    Ok(s)
                        .and_then(|s| builtin_text(s, "'", false))
                        .and_then(|s| parse_string_raw(s).and_then(|s| s.tag_node("string_raw")))
                        .and_then(|s| builtin_text(s, "'", false))
                })
                .and_then(|s| s.tag_node("string_0"))
            })
            .or_else(|s| {
                s.sequence(|s| {
                    Ok(s)
                        .and_then(|s| builtin_text(s, "'", false))
                        .and_then(|s| s.repeat(0..4294967295, |s| parse_string_text(s).and_then(|s| s.tag_node("string_text"))))
                        .and_then(|s| builtin_text(s, "'", false))
                })
                .and_then(|s| s.tag_node("string_1"))
            })
    })
}
#[inline]
fn parse_string_raw(state: Input) -> Output {
    state.rule(Json5Rule::StringRaw, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^([^\"]*)").unwrap())
        })
    })
}
#[inline]
fn parse_string_text(state: Input) -> Output {
    state.rule(Json5Rule::StringText, |s| {
        Err(s).or_else(|s| parse_string_escape(s).and_then(|s| s.tag_node("string_escape"))).or_else(|s| {
            builtin_regex(s, {
                static REGEX: OnceLock<Regex> = OnceLock::new();
                REGEX.get_or_init(|| Regex::new("^([^\"]+)").unwrap())
            })
            .and_then(|s| s.tag_node("string_text_1"))
        })
    })
}
#[inline]
fn parse_string_escape(state: Input) -> Output {
    state.rule(Json5Rule::StringEscape, |s| {
        s.sequence(|s| {
            Ok(s)
                .and_then(|s| builtin_text(s, "\\", false))
                .and_then(|s| builtin_ignore(s))
                .and_then(|s| builtin_any(s).and_then(|s| s.tag_node("c")))
        })
    })
}
#[inline]
fn parse_number(state: Input) -> Output {
    state.rule(Json5Rule::Number, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^([+-]?(0|[1-9][0-9]*))").unwrap())
        })
    })
}
#[inline]
fn parse_integer(state: Input) -> Output {
    state.rule(Json5Rule::Integer, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^(0|[1-9][0-9]*)").unwrap())
        })
    })
}
#[inline]
fn parse_identifier(state: Input) -> Output {
    state.rule(Json5Rule::Identifier, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^([_\\p{XID_start}][\\p{XID_continue}]*)").unwrap())
        })
    })
}
#[inline]
fn parse_boolean(state: Input) -> Output {
    state.rule(Json5Rule::Boolean, |s| {
        Err(s)
            .or_else(|s| builtin_text(s, "true", false).and_then(|s| s.tag_node("boolean_0")))
            .or_else(|s| builtin_text(s, "false", false).and_then(|s| s.tag_node("boolean_1")))
    })
}
#[inline]
fn parse_null(state: Input) -> Output {
    state.rule(Json5Rule::Null, |s| s.match_string("null", false))
}
#[inline]
fn parse_white_space(state: Input) -> Output {
    state.rule(Json5Rule::WhiteSpace, |s| {
        s.match_regex({
            static REGEX: OnceLock<Regex> = OnceLock::new();
            REGEX.get_or_init(|| Regex::new("^([\\p{WhiteSpace}])").unwrap())
        })
    })
}

/// All rules ignored in ast mode, inline is not recommended
fn builtin_ignore(state: Input) -> Output {
    state.repeat(0..u32::MAX, |s| parse_white_space(s))
}

fn builtin_any(state: Input) -> Output {
    state.rule(Json5Rule::IgnoreText, |s| s.match_char_if(|_| true))
}

fn builtin_text<'i>(state: Input<'i>, text: &'static str, case: bool) -> Output<'i> {
    state.rule(Json5Rule::IgnoreText, |s| s.match_string(text, case))
}

fn builtin_regex<'i, 'r>(state: Input<'i>, regex: &'r Regex) -> Output<'i> {
    state.rule(Json5Rule::IgnoreRegex, |s| s.match_regex(regex))
}
