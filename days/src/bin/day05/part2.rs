use core::fmt;
use std::cmp::{min, max};

#[derive(Clone, Copy)]
struct Range {
    start: usize,
    end: usize
}

impl Range {
    fn can_merge(&self, other: &Range) -> bool {
        if (self.start <= other.end && self.end >= other.start) || 
            (self.start >= other.end && self.end <= other.start) ||
            (self.start == other.end + 1) ||
            (self.end == other.start + 1) {
                return true;
        }

        return false;
    }

    fn merge(&mut self, other: &Range) {
        self.start = min(self.start, other.start);
        self.end = max(self.end, other.end);
    }

    fn size(&self) -> usize {
        (self.end - self.start) + 1
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}->{}", self.start, self.end)
    }
}

pub fn run(input: &str) -> usize {
    let mut count = 0;
    let mut ranges: Vec<Range> = Vec::new();
    let mut planning = true;

    for line in input.lines() {
        let line = line.trim();

        if line == "" {
            planning = false;
            continue;
        }

        if !planning {continue}

        if planning {
            let (start_str, end_str) = line.split_once("-").unwrap();

            ranges.push(Range {
                start: start_str.parse().unwrap(),
                end: end_str.parse().unwrap()
            });
            ranges.sort_by_key(|r| r.start);

            let mut i = 0;

            while i < ranges.len() - 1 {
                let range = ranges[i];
                let next_range = ranges[i+1];

                if range.can_merge(&next_range) {
                    ranges[i].merge(&next_range);
                    ranges.remove(i + 1);
                    i = 0;
                    continue;
                }

                i += 1;
            }
        }
    }

    for range in ranges {
        count += range.size();
    }

   return count;
}
