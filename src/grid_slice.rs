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

impl<I: Iterator<Item = Vec<String>>> Iterator for GridSlice<I> {
    type Item = Vec<String>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let words: Vec<String> = match self.source {
                GridSliceSource::Iter(ref mut i) => i.next()?,
                GridSliceSource::SavedLines(ref l) => l.get(self.num_line)?.to_vec(),
            };
            self.num_line += 1;
            if is_inside_range(&self.grid_slice.line, self.num_line as i64 - 1) {
                let field_range = normalize_range(&self.grid_slice.field, words.len());
                if field_range.step > 0 {
                    return Some(filter_line(&field_range, words.into_iter()));
                } else {
                    return Some(filter_line(&field_range, words.into_iter().rev()));
                }
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
    let from = if range.from < 0 {
        (length - (range.from.abs() as usize % length)) as i64
    } else {
        range.from
    };
    let to = if range.to < 0 {
        (length - (range.to.abs() as usize % length)) as i64
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
            from: length as i64 - to - 1,
            to: length as i64 - from - 1,
            step: range.step,
        }
    }
}

fn is_inside_range(range: &grid_slice_parser::GridSliceRange, current: i64) -> bool {
    (current >= range.from)
        && (current <= range.to || range.to == -1)
        && (((range.from - current) % range.step) == 0)
}

fn filter_line<I: Iterator<Item = String>>(
    range: &grid_slice_parser::GridSliceRange,
    word: I,
) -> Vec<String> {
    word.enumerate()
        .filter(|(n, _)| is_inside_range(range, *n as i64))
        .map(|(_, w)| w)
        .collect::<Vec<String>>()
}
