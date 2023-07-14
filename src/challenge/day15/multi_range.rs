use std::collections::VecDeque;

#[derive(Debug, Default)]
pub struct MultiRange {
    sub: VecDeque<ClosedRange>,
}

impl MultiRange {
    // pub fn add_range2(&mut self, start: i64, end: i64) {
    //     if start > end {
    //         return;
    //     }

    //     if self.sub.is_empty() {
    //         self.sub.push_back((start, end));
    //         return;
    //     }
    //     let start_search = self.check_point(start);
    //     let end_search = self.check_point(end);

    //     log::debug!("start: {:?} end: {:?}", start_search, end_search);
    //     match (self.check_point(start), self.check_point(end)) {
    //         (Ok(s), Ok(e)) => {
    //             if s != e {
    //                 todo!("start: {:?} end: {:?}", start_search, end_search);
    //             }
    //         }
    //         (Ok(_), Err(_)) => todo!("start: {:?} end: {:?}", start_search, end_search),
    //         (Err(_), Ok(_)) => todo!("start: {:?} end: {:?}", start_search, end_search),
    //         (Err(s), Err(e)) => {
    //             if s == e {
    //                 self.sub.insert(s, (start, end));
    //             } else {
    //                 todo!("start: {:?} end: {:?}", start_search, end_search);
    //             }
    //         }
    //     }
    // }

    pub fn add_range(&mut self, start: i64, end: i64) {
        if start > end {
            return;
        }

        let mut write_ptr = match self.check_point(start) {
            Ok(idx) => idx,
            Err(idx) => idx,
        }
        .saturating_sub(1);

        let mut range = Some(ClosedRange { start, end });

        let mut leftmost_unchecked = write_ptr;

        while write_ptr < self.sub.len() && leftmost_unchecked < self.sub.len() {
            let cmp = match range.take().or_else(|| {
                leftmost_unchecked += 1;
                self.sub.get(leftmost_unchecked).cloned()
            }) {
                Some(c) => c,
                None => break,
            };

            log::trace!("current state: {:?}", self.sub);
            log::trace!(
                "current pointers: write_ptr={}, unchecked={} cmp={:?}",
                write_ptr,
                leftmost_unchecked,
                cmp,
            );

            match extend_range(self.sub[write_ptr], cmp) {
                RangeCombine::Skip(e) => {
                    log::trace!("SKIP");
                    write_ptr += 1;
                    if write_ptr <= leftmost_unchecked {
                        self.sub[write_ptr] = e;
                    } else {
                        range = Some(e);
                    }
                }
                RangeCombine::Insert(lhs, rhs) => {
                    log::trace!("INSERT lhs={:?} rhs={:?}", lhs, rhs);
                    self.sub[write_ptr] = lhs;
                    write_ptr += 1;
                    range = Some(rhs);
                }
                RangeCombine::Merge(m) => {
                    log::trace!("MERGE={:?}", m);
                    self.sub[write_ptr] = m;
                }
            }
        }

        if let Some(r) = range {
            log::trace!("range still exists, pushing to the end: {:?}", range);
            self.sub.push_back(r);
        }
        if write_ptr < self.sub.len() {
            self.sub.truncate(write_ptr + 1)
        }
    }

    pub fn iter_ranges(&self) -> impl Iterator<Item = (i64, i64)> + '_ {
        self.sub.iter().map(|r| (r.start, r.end))
    }

    pub fn count(&self) -> usize {
        self.iter_ranges()
            .fold(0, |acc, (s, e)| acc + (e - s + 1) as usize)
    }

    pub fn contains(&self, x: &i64) -> bool {
        self.check_point(*x).is_ok()
    }

    fn check_point(&self, x: i64) -> Result<usize, usize> {
        self.sub
            .binary_search_by(|r| match (x.cmp(&r.start), x.cmp(&r.end)) {
                (std::cmp::Ordering::Less, _) => std::cmp::Ordering::Greater,
                (_, std::cmp::Ordering::Greater) => std::cmp::Ordering::Less,
                (std::cmp::Ordering::Equal, _) => std::cmp::Ordering::Equal,
                (_, std::cmp::Ordering::Equal) => std::cmp::Ordering::Equal,
                (std::cmp::Ordering::Greater, std::cmp::Ordering::Less) => {
                    std::cmp::Ordering::Equal
                }
            })
    }
}

#[derive(Clone, Copy)]
struct ClosedRange {
    start: i64,
    end: i64,
}

impl std::fmt::Debug for ClosedRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.start, self.end)
    }
}

enum RangeCombine {
    Insert(ClosedRange, ClosedRange),
    Merge(ClosedRange),
    Skip(ClosedRange),
}

fn extend_range(src: ClosedRange, r: ClosedRange) -> RangeCombine {
    log::trace!("how should we combine existing={:?} new={:?}", src, r);
    if r.end <= src.end && r.start >= src.start {
        RangeCombine::Merge(src)
    } else if r.end < src.start - 1 {
        RangeCombine::Insert(r, src)
    } else if r.start <= src.end + 1 {
        RangeCombine::Merge(ClosedRange {
            start: std::cmp::min(r.start, src.start),
            end: std::cmp::max(r.end, src.end),
        })
    } else {
        RangeCombine::Skip(r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_ranges(mr: &MultiRange, expected: &[(i64, i64)]) {
        let actual = mr.iter_ranges().collect::<Vec<_>>();
        assert_eq!(actual.as_slice(), expected);
    }

    #[test]
    fn empty() {
        let mr = MultiRange::default();
        check_ranges(&mr, &[]);
    }

    #[test]
    fn single_range() {
        let mut mr = MultiRange::default();
        mr.add_range(0, 4);
        check_ranges(&mr, &[(0, 4)]);
    }

    #[test]
    fn add_distinct_after() {
        let mut mr = MultiRange::default();
        mr.add_range(0, 4);
        mr.add_range(6, 10);
        check_ranges(&mr, &[(0, 4), (6, 10)]);
    }

    #[test]
    fn add_distinct_before() {
        let mut mr = MultiRange::default();
        mr.add_range(0, 4);
        mr.add_range(-6, -4);
        check_ranges(&mr, &[(-6, -4), (0, 4)]);
    }

    #[test]
    fn add_adjacent_after() {
        let mut mr = MultiRange::default();
        mr.add_range(0, 4);
        mr.add_range(4, 10);
        check_ranges(&mr, &[(0, 10)]);
    }

    #[test]
    fn add_adjacent_before() {
        let mut mr = MultiRange::default();
        mr.add_range(0, 4);
        mr.add_range(-4, 0);
        check_ranges(&mr, &[(-4, 4)]);
    }

    #[test]
    fn add_two_single_sized() {
        let mut mr = MultiRange::default();
        mr.add_range(0, 0);
        mr.add_range(1, 1);
        check_ranges(&mr, &[(0, 1)]);
    }

    #[test]
    fn add_inside() {
        let mut mr = MultiRange::default();
        mr.add_range(0, 4);
        mr.add_range(1, 2);
        check_ranges(&mr, &[(0, 4)]);
    }

    #[test]
    fn add_inside_match_start() {
        let mut mr = MultiRange::default();
        mr.add_range(0, 4);
        mr.add_range(0, 2);
        check_ranges(&mr, &[(0, 4)]);
    }

    #[test]
    fn add_inside_match_end() {
        let mut mr = MultiRange::default();
        mr.add_range(0, 4);
        mr.add_range(2, 4);
        check_ranges(&mr, &[(0, 4)]);
    }

    #[test]
    fn add_surround() {
        let mut mr = MultiRange::default();
        mr.add_range(0, 4);
        mr.add_range(-4, 10);
        check_ranges(&mr, &[(-4, 10)]);
    }

    #[test]
    fn add_straddle_after() {
        let mut mr = MultiRange::default();
        mr.add_range(0, 4);
        mr.add_range(2, 10);
        check_ranges(&mr, &[(0, 10)]);
    }

    #[test]
    fn add_straddle_before() {
        let mut mr = MultiRange::default();
        mr.add_range(0, 4);
        mr.add_range(-4, 2);
        check_ranges(&mr, &[(-4, 4)]);
    }

    #[test]
    fn add_thrid_in_the_middle() {
        let mut mr = MultiRange::default();
        mr.add_range(0, 3);
        mr.add_range(7, 10);
        mr.add_range(5, 5);
        check_ranges(&mr, &[(0, 3), (5, 5), (7, 10)]);
    }

    #[test]
    fn add_thrid_in_the_middle_adj() {
        let mut mr = MultiRange::default();
        mr.add_range(0, 3);
        mr.add_range(7, 10);
        mr.add_range(4, 6);
        check_ranges(&mr, &[(0, 10)]);
    }

    #[test]
    fn add_thrid_with_foot_in_each() {
        let mut mr = MultiRange::default();
        mr.add_range(0, 3);
        mr.add_range(7, 10);
        mr.add_range(2, 8);
        check_ranges(&mr, &[(0, 10)]);
    }

    #[test]
    fn add_inside_merge() {
        let mut mr = MultiRange::default();
        mr.add_range(0, 3);
        mr.add_range(10, 13);
        mr.add_range(20, 23);
        mr.add_range(30, 33);
        mr.add_range(3, 10);
        check_ranges(&mr, &[(0, 13), (20, 23), (30, 33)]);
    }

    #[test]
    fn add_multiple_merge() {
        let mut mr = MultiRange::default();
        mr.add_range(0, 3);
        mr.add_range(10, 13);
        mr.add_range(20, 23);
        mr.add_range(30, 33);
        mr.add_range(3, 21);
        check_ranges(&mr, &[(0, 23), (30, 33)]);
    }

    #[test]
    fn add_merge_all() {
        let mut mr = MultiRange::default();
        mr.add_range(0, 3);
        mr.add_range(10, 13);
        mr.add_range(20, 23);
        mr.add_range(30, 33);
        mr.add_range(-50, 50);
        check_ranges(&mr, &[(-50, 50)]);
    }

    #[test]
    fn add_contained_range_with_tail() {
        let mut mr = MultiRange::default();
        mr.add_range(0, 3);
        mr.add_range(10, 13);
        mr.add_range(20, 23);
        mr.add_range(30, 33);
        mr.add_range(1, 2);
        check_ranges(&mr, &[(0, 3), (10, 13), (20, 23), (30, 33)]);
    }

    #[test]
    fn add_merge_range_with_head_and_tail() {
        let mut mr = MultiRange::default();
        mr.add_range(0, 3);
        mr.add_range(10, 13);
        mr.add_range(20, 23);
        mr.add_range(30, 33);
        mr.add_range(19, 21);
        check_ranges(&mr, &[(0, 3), (10, 13), (19, 23), (30, 33)]);
    }

    #[test]
    fn add_insert_range_with_head_and_tail() {
        let mut mr = MultiRange::default();
        mr.add_range(0, 3);
        mr.add_range(10, 13);
        mr.add_range(20, 23);
        mr.add_range(30, 33);
        mr.add_range(25, 26);
        check_ranges(&mr, &[(0, 3), (10, 13), (20, 23), (25, 26), (30, 33)]);
    }
}
