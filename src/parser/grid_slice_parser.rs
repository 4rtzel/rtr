#[derive(Debug)]
enum ParsedGridInteger {
    LowercaseInteger(i64),
    UppercaseInteger(i64),
}

#[derive(Debug)]
struct ParsedGridIndex {
    line: Option<ParsedGridInteger>,
    field: Option<ParsedGridInteger>,
}

/*
 * lowercase_line = "l" integer
 * uppercase_line = "L" integer
 * lowercase_field = "f" integer
 * uppercase_field = "F" integer
 * line = lowercase_line | uppercase_line
 * field = lowercase_field | uppercase_field
 * grid_index = line [field] | field [line]
 * grid_slice = [grid_index] ':' [grid_index] [':' [grid_index]] | grid_index
 */
#[derive(Debug)]
struct ParsedGridSlice {
    from: ParsedGridIndex,
    to: ParsedGridIndex,
    step: ParsedGridIndex,
}

fn parse_integer(it: &mut std::str::Chars) -> Option<i64> {
    let peek = it.clone();
    // take_while() advances iterator past TWO elements after the last one. To work
    // around this issue we will have to advance the iterator manually.
    let integer = peek.take_while(|c| c.is_digit(10) || *c == '-').collect::<String>();
    match integer.parse().ok() {
        Some(v) => {
            // poor man's advance_by()
            for _ in 0..integer.len() {
                it.next();
            }
            Some(v)
        },
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

fn parse_line_or_field(it: &mut std::str::Chars, prefix: char) -> Option<i64> {
    let mut peek = it.clone();
    if peek.next()? == prefix {
        if let Some(v) = parse_integer(&mut peek) {
            *it = peek;
            return Some(v);
        }
    }
    None
}

fn parse_line(it: &mut std::str::Chars) -> Option<ParsedGridInteger> {
    if let Some(lowercase_line) = parse_line_or_field(it, 'l') {
        Some(ParsedGridInteger::LowercaseInteger(lowercase_line))
    } else if let Some(uppercase_line) = parse_line_or_field(it, 'L') {
        Some(ParsedGridInteger::UppercaseInteger(uppercase_line))
    } else {
        None
    }
}

fn parse_field(it: &mut std::str::Chars) -> Option<ParsedGridInteger> {
    if let Some(lowercase_field) = parse_line_or_field(it, 'f') {
        Some(ParsedGridInteger::LowercaseInteger(lowercase_field))
    } else if let Some(uppercase_field) = parse_line_or_field(it, 'F') {
        Some(ParsedGridInteger::UppercaseInteger(uppercase_field))
    } else {
        None
    }
}

fn parse_grid_index(it: &mut std::str::Chars) -> Option<ParsedGridIndex> {
    let mut peek = it.clone();
    let gi: Option<ParsedGridIndex> = if let Some(line) = parse_line(&mut peek) {
        Some(ParsedGridIndex {
            line: Some(line),
            field: parse_field(&mut peek),
        })
    } else if let Some(field) = parse_field(&mut peek) {
        Some(ParsedGridIndex {
            line: parse_line(&mut peek),
            field: Some(field),
        })
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
        from: ParsedGridIndex { line: None, field: None },
        to: ParsedGridIndex { line: None, field: None },
        step: ParsedGridIndex { line: None, field: None },
    };
    if let Some(from_grid_index) = parse_grid_index(&mut peek) {
        gs.from = from_grid_index;
    }
    if !parse_separator(&mut peek) {
        *it = peek;
        return Some(gs);
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
    // TODO: looks ugly
    if gs.from.line.is_none() && gs.from.field.is_none() && gs.to.line.is_none() && gs.to.field.is_none() && gs.step.line.is_none() && gs.step.field.is_none() {
        None
    } else {
        *it = peek;
        Some(gs)
    }
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
}

fn extract_valid_range(from: &Option<ParsedGridInteger>, to: &Option<ParsedGridInteger>) -> Result<(Option<i64>, Option<i64>), &'static str> {
    let mut from_int: Option<i64> = None;
    let mut to_int: Option<i64> = None;
    if let Some(from) = from {
        match from {
            ParsedGridInteger::LowercaseInteger(i) => {
                from_int = Some(*i);
            },
            ParsedGridInteger::UppercaseInteger(i) => {
                from_int = Some(*i);
                to_int = Some(*i);
            },
        }
    }

    if let Some(to) = to {
        match to {
            ParsedGridInteger::LowercaseInteger(i) => {
                if to_int.is_some() {
                    return Err("Ambiguous range specified. 'L'/'F' was used in conjunction with 'l'/'f' for the same range");
                }
                to_int = Some(*i);
            },
            ParsedGridInteger::UppercaseInteger(i) => {
                if to_int.is_some() || from_int.is_some() {
                    return Err("Ambiguous range specified. 'L'/'F' was used in conjunction with 'l'/'f' for the same range");
                }
                from_int = Some(*i);
                to_int = Some(*i);
            },
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
    Ok(GridSliceFilter {
        line: GridSliceRange {
            from: line_range.0.unwrap_or(0),
            to: line_range.1.unwrap_or(-1),
            step: if let Some(step_line) = pgs.step.line {
                match step_line {
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
            step: if let Some(step_field) = pgs.step.field {
                match step_field {
                    ParsedGridInteger::LowercaseInteger(i) => i,
                    ParsedGridInteger::UppercaseInteger(_) => {
                        return Err("Step field cannot be 'F'");
                    }
                }
            } else {
                1
            },
        }
    })
}
