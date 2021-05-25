#[derive(Debug, PartialEq)]
enum ParsedGridInteger {
    LowercaseInteger(i64),
    UppercaseInteger(i64),
}

#[derive(Debug, PartialEq)]
struct ParsedGridIndex {
    line: Option<ParsedGridInteger>,
    field: Option<ParsedGridInteger>,
    character: Option<ParsedGridInteger>,
}

/*
 * lowercase_line = "l" integer
 * uppercase_line = "L" integer
 * lowercase_field = "f" integer
 * uppercase_field = "F" integer
 * lowercase_char = "c" integer
 * uppercase_char = "C" integer
 * line = lowercase_line | uppercase_line
 * field = lowercase_field | uppercase_field
 * char = lowercase_char | uppercase_char
 * grid_index = line [field] [char] | line [char] [field] | field [line] [char] | field [char] [line] | char [line] [field] | char [field] [line]
 * grid_slice = [grid_index] ':' [grid_index] [':' [grid_index]] | grid_index
 */
#[derive(Debug, PartialEq)]
struct ParsedGridSlice {
    from: ParsedGridIndex,
    to: ParsedGridIndex,
    step: ParsedGridIndex,
}

fn parse_integer(it: &mut std::str::Chars) -> Option<i64> {
    let peek = it.clone();
    // take_while() advances iterator past TWO elements after the last one. To work
    // around this issue we will have to advance the iterator manually.
    let integer = peek
        .enumerate()
        .take_while(|(n, c)| c.is_digit(10) || (*n == 0 && *c == '-'))
        .map(|(_, c)| c)
        .collect::<String>();
    match integer.parse().ok() {
        Some(v) => {
            // poor man's advance_by()
            for _ in 0..integer.len() {
                it.next();
            }
            Some(v)
        }
        None => None,
    }
}

fn parse_separator(it: &mut std::str::Chars) -> bool {
    let mut peek = it.clone();
    if peek.next() == Some(':') {
        *it = peek;
        return true;
    }
    false
}

fn parse_char_and_int(it: &mut std::str::Chars, prefix: char) -> Option<ParsedGridInteger> {
    let mut peek = it.clone();
    if peek.next()? == prefix {
        if let Some(v) = parse_integer(&mut peek) {
            *it = peek;
            if prefix.is_lowercase() {
                return Some(ParsedGridInteger::LowercaseInteger(v));
            } else {
                return Some(ParsedGridInteger::UppercaseInteger(v));
            }
        }
    }
    None
}

fn parse_line(it: &mut std::str::Chars) -> Option<ParsedGridInteger> {
    match parse_char_and_int(it, 'l') {
        Some(v) => Some(v),
        None => parse_char_and_int(it, 'L'),
    }
}

fn parse_field(it: &mut std::str::Chars) -> Option<ParsedGridInteger> {
    match parse_char_and_int(it, 'f') {
        Some(v) => Some(v),
        None => parse_char_and_int(it, 'F'),
    }
}

fn parse_char(it: &mut std::str::Chars) -> Option<ParsedGridInteger> {
    match parse_char_and_int(it, 'c') {
        Some(v) => Some(v),
        None => parse_char_and_int(it, 'C'),
    }
}

fn parse_grid_index(it: &mut std::str::Chars) -> Option<ParsedGridIndex> {
    let mut peek = it.clone();
    let gi: Option<ParsedGridIndex> = if let Some(line) = parse_line(&mut peek) {
        if let Some(field) = parse_field(&mut peek) {
            Some(ParsedGridIndex {
                line: Some(line),
                field: Some(field),
                character: parse_char(&mut peek),
            })
        } else if let Some(character) = parse_char(&mut peek) {
            Some(ParsedGridIndex {
                line: Some(line),
                field: parse_field(&mut peek),
                character: Some(character),
            })
        } else {
            Some(ParsedGridIndex {
                line: Some(line),
                field: None,
                character: None,
            })
        }
    } else if let Some(field) = parse_field(&mut peek) {
        if let Some(line) = parse_line(&mut peek) {
            Some(ParsedGridIndex {
                line: Some(line),
                field: Some(field),
                character: parse_char(&mut peek),
            })
        } else if let Some(character) = parse_char(&mut peek) {
            Some(ParsedGridIndex {
                line: parse_line(&mut peek),
                field: Some(field),
                character: Some(character),
            })
        } else {
            Some(ParsedGridIndex {
                line: None,
                field: Some(field),
                character: None,
            })
        }
    } else if let Some(character) = parse_char(&mut peek) {
        if let Some(line) = parse_line(&mut peek) {
            Some(ParsedGridIndex {
                line: Some(line),
                field: parse_field(&mut peek),
                character: Some(character),
            })
        } else if let Some(field) = parse_field(&mut peek) {
            Some(ParsedGridIndex {
                line: parse_line(&mut peek),
                field: Some(field),
                character: Some(character),
            })
        } else {
            Some(ParsedGridIndex {
                line: None,
                field: None,
                character: Some(character),
            })
        }
    } else {
        None
    };
    if gi.is_some() {
        *it = peek;
    }
    gi
}

// grid_slice = [grid_index] ':' [grid_index] [':' [grid_index]] | grid_index
fn parse_grid_slice_impl(it: &mut std::str::Chars) -> Option<ParsedGridSlice> {
    let mut peek = it.clone();
    let mut gs = ParsedGridSlice {
        from: ParsedGridIndex {
            line: None,
            field: None,
            character: None,
        },
        to: ParsedGridIndex {
            line: None,
            field: None,
            character: None,
        },
        step: ParsedGridIndex {
            line: None,
            field: None,
            character: None,
        },
    };
    if let Some(from_grid_index) = parse_grid_index(&mut peek) {
        gs.from = from_grid_index;
    }
    if !parse_separator(&mut peek) {
        if gs.from.line.is_none() && gs.from.field.is_none() {
            return None;
        } else {
            *it = peek;
            return Some(gs);
        }
    }
    if let Some(to_grid_index) = parse_grid_index(&mut peek) {
        gs.to = to_grid_index;
    }
    if !parse_separator(&mut peek) {
        *it = peek;
        return Some(gs);
    }
    if let Some(step_grid_index) = parse_grid_index(&mut peek) {
        gs.step = step_grid_index;
    }
    *it = peek;
    Some(gs)
}

#[derive(Debug)]
pub struct GridSliceRange {
    pub from: i64,
    pub to: i64,
    pub step: i64,
}

#[derive(Debug)]
pub struct GridSliceFilter {
    pub line: GridSliceRange,
    pub field: GridSliceRange,
    pub character: GridSliceRange,
}

fn extract_valid_range(
    from: &Option<ParsedGridInteger>,
    to: &Option<ParsedGridInteger>,
) -> Result<(Option<i64>, Option<i64>), &'static str> {
    let mut from_int: Option<i64> = None;
    let mut to_int: Option<i64> = None;
    if let Some(from) = from {
        match from {
            ParsedGridInteger::LowercaseInteger(i) => {
                from_int = Some(*i);
            }
            ParsedGridInteger::UppercaseInteger(i) => {
                from_int = Some(*i);
                to_int = Some(*i);
            }
        }
    }

    if let Some(to) = to {
        match to {
            ParsedGridInteger::LowercaseInteger(i) => {
                if to_int.is_some() {
                    return Err("Ambiguous range specified. 'L'/'F' was used in conjunction with 'l'/'f' for the same range");
                }
                to_int = Some(*i);
            }
            ParsedGridInteger::UppercaseInteger(i) => {
                if to_int.is_some() || from_int.is_some() {
                    return Err("Ambiguous range specified. 'L'/'F' was used in conjunction with 'l'/'f' for the same range");
                }
                from_int = Some(*i);
                to_int = Some(*i);
            }
        }
    }

    Ok((from_int, to_int))
}

pub fn parse_grid_slice(input: &String) -> Result<GridSliceFilter, &'static str> {
    let mut chars = input.chars().into_iter();
    let pgs = match parse_grid_slice_impl(&mut chars) {
        Some(v) => v,
        None => return Err("Unable to parse the input"),
    };
    if chars.next().is_some() {
        return Err("Unable to fully parse the input");
    }

    let line_range = extract_valid_range(&pgs.from.line, &pgs.to.line)?;
    let field_range = extract_valid_range(&pgs.from.field, &pgs.to.field)?;
    let character_range = extract_valid_range(&pgs.from.character, &pgs.to.character)?;
    Ok(GridSliceFilter {
        line: GridSliceRange {
            from: line_range.0.unwrap_or(0),
            to: line_range.1.unwrap_or(-1),
            step: if let Some(line_step) = pgs.step.line {
                match line_step {
                    ParsedGridInteger::LowercaseInteger(i) => i,
                    ParsedGridInteger::UppercaseInteger(_) => {
                        return Err("Step line cannot be 'L'");
                    }
                }
            } else {
                1
            },
        },
        field: GridSliceRange {
            from: field_range.0.unwrap_or(0),
            to: field_range.1.unwrap_or(-1),
            step: if let Some(field_step) = pgs.step.field {
                match field_step {
                    ParsedGridInteger::LowercaseInteger(i) => i,
                    ParsedGridInteger::UppercaseInteger(_) => {
                        return Err("Step field cannot be 'F'");
                    }
                }
            } else {
                1
            },
        },
        character: GridSliceRange {
            from: character_range.0.unwrap_or(0),
            to: character_range.1.unwrap_or(-1),
            step: if let Some(character_step) = pgs.step.character {
                match character_step {
                    ParsedGridInteger::LowercaseInteger(i) => i,
                    ParsedGridInteger::UppercaseInteger(_) => {
                        return Err("Step field cannot be 'C'");
                    }
                }
            } else {
                1
            },
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_test_helper<E: std::cmp::PartialEq + std::fmt::Debug>(
        f: fn(&mut std::str::Chars) -> E,
        input: &str,
        expects: E,
        next: Option<char>,
    ) {
        let mut it = input.chars();
        assert_eq!(f(&mut it), expects);
        assert_eq!(it.next(), next);
    }

    #[test]
    fn parse_integer_test() {
        parse_test_helper(parse_integer, "42", Some(42), None);
        parse_test_helper(parse_integer, "-42", Some(-42), None);
        parse_test_helper(parse_integer, "abc", None, Some('a'));
        parse_test_helper(parse_integer, "42abc", Some(42), Some('a'));
        parse_test_helper(parse_integer, "42-abc", Some(42), Some('-'));
    }

    #[test]
    fn parse_separator_test() {
        parse_test_helper(parse_separator, ":abc", true, Some('a'));
        parse_test_helper(parse_separator, "a:bc", false, Some('a'));
    }

    #[test]
    fn parse_char_and_int_test() {
        fn parse_char_and_int_helper(
            input: &str,
            prefix: char,
            expects: Option<ParsedGridInteger>,
            next: Option<char>,
        ) {
            let mut it = input.chars();
            assert_eq!(parse_char_and_int(&mut it, prefix), expects);
            assert_eq!(it.next(), next);
        }
        parse_char_and_int_helper(
            "l123",
            'l',
            Some(ParsedGridInteger::LowercaseInteger(123)),
            None,
        );
        parse_char_and_int_helper("l123", 'f', None, Some('l'));
        parse_char_and_int_helper("lf123", 'l', None, Some('l'));
    }

    #[test]
    fn parse_line_test() {
        parse_test_helper(
            parse_line,
            "l123",
            Some(ParsedGridInteger::LowercaseInteger(123)),
            None,
        );
        parse_test_helper(
            parse_line,
            "l-123abc",
            Some(ParsedGridInteger::LowercaseInteger(-123)),
            Some('a'),
        );
        parse_test_helper(parse_line, "f-123abc", None, Some('f'));
        parse_test_helper(
            parse_line,
            "L123",
            Some(ParsedGridInteger::UppercaseInteger(123)),
            None,
        );
        parse_test_helper(
            parse_line,
            "L-123abc",
            Some(ParsedGridInteger::UppercaseInteger(-123)),
            Some('a'),
        );
    }

    #[test]
    fn parse_field_test() {
        parse_test_helper(
            parse_field,
            "f123",
            Some(ParsedGridInteger::LowercaseInteger(123)),
            None,
        );
        parse_test_helper(
            parse_field,
            "f-123abc",
            Some(ParsedGridInteger::LowercaseInteger(-123)),
            Some('a'),
        );
        parse_test_helper(parse_field, "l-123abc", None, Some('l'));
        parse_test_helper(
            parse_field,
            "F123",
            Some(ParsedGridInteger::UppercaseInteger(123)),
            None,
        );
        parse_test_helper(
            parse_field,
            "F-123abc",
            Some(ParsedGridInteger::UppercaseInteger(-123)),
            Some('a'),
        );
    }

    #[test]
    fn parse_char_test() {
        parse_test_helper(
            parse_char,
            "c1",
            Some(ParsedGridInteger::LowercaseInteger(1)),
            None,
        );
        parse_test_helper(
            parse_char,
            "c-2abc",
            Some(ParsedGridInteger::LowercaseInteger(-2)),
            Some('a'),
        );
        parse_test_helper(parse_char, "l-123abc", None, Some('l'));
        parse_test_helper(
            parse_char,
            "C123",
            Some(ParsedGridInteger::UppercaseInteger(123)),
            None,
        );
        parse_test_helper(
            parse_char,
            "C-123abc",
            Some(ParsedGridInteger::UppercaseInteger(-123)),
            Some('a'),
        );
    }

    #[test]
    fn parse_grid_index_test() {
        fn pl(value: i64) -> Option<ParsedGridInteger> {
            Some(ParsedGridInteger::LowercaseInteger(value))
        }
        fn pu(value: i64) -> Option<ParsedGridInteger> {
            Some(ParsedGridInteger::UppercaseInteger(value))
        }
        fn pi(
            line: Option<ParsedGridInteger>,
            field: Option<ParsedGridInteger>,
            character: Option<ParsedGridInteger>,
        ) -> Option<ParsedGridIndex> {
            Some(ParsedGridIndex {
                line: line,
                field: field,
                character: character,
            })
        }
        parse_test_helper(
            parse_grid_index,
            "F-42abc",
            pi(None, pu(-42), None),
            Some('a'),
        );
        parse_test_helper(
            parse_grid_index,
            "l1337F-42abc",
            pi(pl(1337), pu(-42), None),
            Some('a'),
        );
        parse_test_helper(parse_grid_index, "L42_", pi(pu(42), None, None), Some('_'));
        parse_test_helper(
            parse_grid_index,
            "l42L43_",
            pi(pl(42), None, None),
            Some('L'),
        );
        parse_test_helper(parse_grid_index, "r42_", None, Some('r'));
        parse_test_helper(parse_grid_index, "L1F2C3", pi(pu(1), pu(2), pu(3)), None);
        parse_test_helper(parse_grid_index, "l1f2c3", pi(pl(1), pl(2), pl(3)), None);
        parse_test_helper(parse_grid_index, "l1c3", pi(pl(1), None, pl(3)), None);
        parse_test_helper(parse_grid_index, "f2c3", pi(None, pl(2), pl(3)), None);
        parse_test_helper(parse_grid_index, "c3", pi(None, None, pl(3)), None);
        parse_test_helper(parse_grid_index, "l1c3f2", pi(pl(1), pl(2), pl(3)), None);
        parse_test_helper(parse_grid_index, "c3f2", pi(None, pl(2), pl(3)), None);
        parse_test_helper(parse_grid_index, "f2l1c3", pi(pl(1), pl(2), pl(3)), None);
        parse_test_helper(parse_grid_index, "c3f2l1", pi(pl(1), pl(2), pl(3)), None);
        parse_test_helper(parse_grid_index, "c3l1f2", pi(pl(1), pl(2), pl(3)), None);
    }

    #[test]
    fn parse_grid_slice_impl_test() {
        let pl = ParsedGridInteger::LowercaseInteger;
        let pu = ParsedGridInteger::UppercaseInteger;
        fn pi(
            line: Option<ParsedGridInteger>,
            field: Option<ParsedGridInteger>,
            character: Option<ParsedGridInteger>,
        ) -> ParsedGridIndex {
            ParsedGridIndex {
                line: line,
                field: field,
                character: character,
            }
        }

        fn ps(
            from: ParsedGridIndex,
            to: ParsedGridIndex,
            step: ParsedGridIndex,
        ) -> ParsedGridSlice {
            ParsedGridSlice {
                from: from,
                to: to,
                step: step,
            }
        }
        parse_test_helper(parse_grid_slice_impl, "", None, None);
        parse_test_helper(
            parse_grid_slice_impl,
            "l42",
            Some(ps(
                pi(Some(pl(42)), None, None),
                pi(None, None, None),
                pi(None, None, None),
            )),
            None,
        );
        parse_test_helper(
            parse_grid_slice_impl,
            "l42F0",
            Some(ps(
                pi(Some(pl(42)), Some(pu(0)), None),
                pi(None, None, None),
                pi(None, None, None),
            )),
            None,
        );
        parse_test_helper(
            parse_grid_slice_impl,
            "l42F0:",
            Some(ps(
                pi(Some(pl(42)), Some(pu(0)), None),
                pi(None, None, None),
                pi(None, None, None),
            )),
            None,
        );
        parse_test_helper(
            parse_grid_slice_impl,
            "l42F0::",
            Some(ps(
                pi(Some(pl(42)), Some(pu(0)), None),
                pi(None, None, None),
                pi(None, None, None),
            )),
            None,
        );
        parse_test_helper(
            parse_grid_slice_impl,
            "l42F0:::",
            Some(ps(
                pi(Some(pl(42)), Some(pu(0)), None),
                pi(None, None, None),
                pi(None, None, None),
            )),
            Some(':'),
        );
        parse_test_helper(
            parse_grid_slice_impl,
            ":l42F0",
            Some(ps(
                pi(None, None, None),
                pi(Some(pl(42)), Some(pu(0)), None),
                pi(None, None, None),
            )),
            None,
        );
        parse_test_helper(
            parse_grid_slice_impl,
            ":l42F0:",
            Some(ps(
                pi(None, None, None),
                pi(Some(pl(42)), Some(pu(0)), None),
                pi(None, None, None),
            )),
            None,
        );
        parse_test_helper(
            parse_grid_slice_impl,
            "::l2F-1a",
            Some(ps(
                pi(None, None, None),
                pi(None, None, None),
                pi(Some(pl(2)), Some(pu(-1)), None),
            )),
            Some('a'),
        );
    }
}
