use crate::parser::grid_slice_parser;

enum GridSliceSource<I> {
    Iter(I),
    SavedLines(Vec<Vec<String>>),
}

pub struct GridSlice<I> {
    grid_slice: grid_slice_parser::GridSliceFilter,
    source: GridSliceSource<I>,
    num_line: usize,
}

impl<T> GridSlice<T> {
    fn slice_fields(&self, fields: Vec<String>) -> Vec<String> {
        let field_range = normalize_range(&self.grid_slice.field, fields.len());
        if field_range.step > 0 {
            self.slice_fields_from_iter(field_range, fields.into_iter())
        } else {
            self.slice_fields_from_iter(field_range, fields.into_iter().rev())
        }
    }

    fn slice_fields_from_iter<I: Iterator<Item = String>>(
        &self,
        field_range: grid_slice_parser::GridSliceRange,
        it: I,
    ) -> Vec<String> {
        it.enumerate()
            .filter(|(n, _)| is_inside_range(&field_range, *n as i64))
            .map(|(_, f)| self.slice_chars(f))
            .collect()
    }

    fn slice_chars(&self, field: String) -> String {
        let char_range = normalize_range(&self.grid_slice.character, field.len());
        if char_range.step > 0 {
            self.slice_chars_from_iter(char_range, field.chars())
        } else {
            self.slice_chars_from_iter(char_range, field.chars().rev())
        }
    }

    fn slice_chars_from_iter<I: Iterator<Item = char>>(
        &self,
        char_range: grid_slice_parser::GridSliceRange,
        it: I,
    ) -> String {
        it.enumerate()
            .filter(|(n, _)| is_inside_range(&char_range, *n as i64))
            .map(|(_, c)| c)
            .collect()
    }
}

impl<I: Iterator<Item = Vec<String>>> Iterator for GridSlice<I> {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let fields: Vec<String> = match self.source {
                GridSliceSource::Iter(ref mut i) => i.next()?,
                GridSliceSource::SavedLines(ref l) => l.get(self.num_line)?.to_vec(),
            };
            self.num_line += 1;
            if is_inside_range(&self.grid_slice.line, self.num_line as i64 - 1) {
                return Some(self.slice_fields(fields));
            }
        }
    }
}

pub fn grid_slice_iter<I: Iterator<Item = Vec<String>>>(
    mut grid_slice: grid_slice_parser::GridSliceFilter,
    iter: I,
) -> GridSlice<I> {
    let post_process: bool =
        grid_slice.line.from < 0 || grid_slice.line.to < -1 || grid_slice.line.step < 0;

    if post_process {
        let mut lines: Vec<Vec<String>> = iter.collect();
        grid_slice.line = normalize_range(&grid_slice.line, lines.len());
        if grid_slice.line.step < 0 {
            lines.reverse();
        }
        GridSlice {
            grid_slice: grid_slice,
            source: GridSliceSource::SavedLines(lines),
            num_line: 0,
        }
    } else {
        GridSlice {
            grid_slice: grid_slice,
            source: GridSliceSource::Iter(iter),
            num_line: 0,
        }
    }
}

fn normalize_range(
    range: &grid_slice_parser::GridSliceRange,
    length: usize,
) -> grid_slice_parser::GridSliceRange {
    let length = length as i64;

    let from = if range.from < 0 {
        if length < -range.from {
            length
        } else {
            length + range.from
        }
    } else {
        range.from
    };
    let to = if range.to < 0 {
        if length < -range.to {
            0
        } else {
            length + range.to
        }
    } else {
        range.to
    };
    if range.step > 0 {
        grid_slice_parser::GridSliceRange {
            from: from,
            to: to,
            step: range.step,
        }
    } else {
        grid_slice_parser::GridSliceRange {
            from: length - to - 1,
            to: length - from - 1,
            step: range.step,
        }
    }
}

fn is_inside_range(range: &grid_slice_parser::GridSliceRange, current: i64) -> bool {
    (current >= range.from)
        && (current <= range.to || range.to == -1)
        && (((range.from - current) % range.step) == 0)
}
