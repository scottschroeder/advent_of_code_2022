use anyhow::{Context, Result};

const P1_MAX: usize = 100_000;

const DISK_SIZE: usize = 70_000_000;
const DISK_NEED: usize = 30_000_000;

pub fn part1(input: &str) -> Result<String> {
    let fs = parse(input)?;
    let (_, output) = calculate_dir_totals(&fs)?;
    let p1_sum = output
        .iter()
        .map(|(_, t)| *t)
        .filter(|t| *t <= P1_MAX)
        .sum::<usize>();
    Ok(format!("{:?}", p1_sum))
}

pub fn part2(input: &str) -> Result<String> {
    let fs = parse(input)?;
    let (root_size, output) = calculate_dir_totals(&fs)?;
    let min_free = (root_size + DISK_NEED) - DISK_SIZE;
    let mut min_delete = None;

    for (_, total) in output {
        if total < min_free {
            continue;
        }
        let delta = total - min_free;
        if let Some((min_delta, _)) = min_delete {
            if delta >= min_delta {
                continue;
            }
        }
        min_delete = Some((delta, total));
    }

    let (_, total) =
        min_delete.ok_or_else(|| anyhow::anyhow!("no directory will free up enough space"))?;
    Ok(format!("{:?}", total))
}

fn calculate_dir_totals<'a>(fs: &'a FileSystem) -> Result<(usize, Vec<(&'a Inode<'a>, usize)>)> {
    let mut output = Vec::new();
    let cwd = InodeID(0);
    let stats = dir_total_recurse(fs, cwd, &mut output)?;
    Ok((stats, output))
}

fn dir_total_recurse<'a>(
    fs: &'a FileSystem,
    cwd: InodeID,
    output: &mut Vec<(&'a Inode<'a>, usize)>,
) -> Result<usize> {
    let inode = fs.get(cwd)?;
    let mut total = 0;
    for cid in inode
        .listing()
        .ok_or_else(|| anyhow::anyhow!("attempted to list regular file"))?
    {
        let child_inode = fs.get(*cid)?;
        if child_inode.listing.is_some() {
            total += dir_total_recurse(fs, *cid, output)?;
        } else {
            total += child_inode.size;
        }
    }
    output.push((inode, total));
    Ok(total)
}

fn parse(input: &str) -> Result<FileSystem> {
    let mut fs = FileSystem::default();
    let mut stack = DirStack::new(fs.insert_dir("/"));
    for l in input.lines() {
        let l = l.trim();
        if l == "$ ls" {
            continue;
        } else if l.starts_with("$ cd ") {
            let dir = parse_cd(l)?;
            if dir == ".." {
                stack.pop();
            } else if dir == "/" {
                stack.root();
            } else {
                let sub_inode = fs
                    .get_subdir(stack.cwd(), dir)
                    .ok_or_else(|| anyhow::anyhow!("fs directory not found: {}/{}", stack, dir))?;
                stack.push(sub_inode, dir);
            }
            log::trace!("cwd => {}", stack);
        } else if l.starts_with("dir ") {
            let dir = parse_dir_listing(l)?;
            let child = fs.insert_dir(dir);
            fs.add_to_dir(stack.cwd(), child);
            log::trace!("dir {}/{}", stack, dir);
        } else {
            let (sz, filename) = parse_file_listing(l)?;
            let child = fs.insert_file(filename, sz);
            fs.add_to_dir(stack.cwd(), child);
            log::trace!("file {}/{} [{} bytes]", stack, filename, sz);
        }
    }
    Ok(fs)
}

struct DirStack<'a> {
    root: InodeID,
    stack: Vec<(InodeID, &'a str)>,
}

impl<'a> std::fmt::Display for DirStack<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, d) in self.stack.iter().enumerate() {
            if !(idx == 0 && d.1 == "/") {
                write!(f, "/")?;
            }
            write!(f, "{}", d.1)?;
        }
        Ok(())
    }
}

impl<'a> DirStack<'a> {
    fn new(root: InodeID) -> DirStack<'a> {
        DirStack {
            root,
            stack: Vec::new(),
        }
    }

    fn cwd(&self) -> InodeID {
        if let Some(last) = self.stack.last() {
            last.0
        } else {
            self.root
        }
    }

    fn push(&mut self, node: InodeID, dir: &'a str) {
        self.stack.push((node, dir))
    }

    fn pop(&mut self) {
        self.stack.pop();
    }
    fn root(&mut self) {
        self.stack.truncate(0)
    }
}

#[derive(Debug, Clone, Copy)]
struct InodeID(usize);

#[derive(Debug, Clone)]
struct Inode<'a> {
    name: &'a str,
    listing: Option<Directory>,
    size: usize,
}

impl<'a> Inode<'a> {
    fn listing(&self) -> Option<&[InodeID]> {
        self.listing.as_ref().map(|l| l.listing.as_slice())
    }
}

impl<'a> std::fmt::Display for Inode<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, Default)]
struct Directory {
    listing: Vec<InodeID>,
}

#[derive(Debug, Default, Clone)]
struct FileSystem<'a> {
    inodes: Vec<Inode<'a>>,
}

impl<'a> FileSystem<'a> {
    fn get(&self, id: InodeID) -> Result<&Inode<'a>> {
        self.inodes
            .get(id.0)
            .ok_or_else(|| anyhow::anyhow!("missing inode: {:?}", id))
    }
    fn insert_dir(&mut self, name: &'a str) -> InodeID {
        self.inodes.push(Inode {
            name,
            listing: Some(Directory::default()),
            size: 0,
        });
        InodeID(self.inodes.len() - 1)
    }
    fn insert_file(&mut self, name: &'a str, size: usize) -> InodeID {
        self.inodes.push(Inode {
            name,
            listing: None,
            size,
        });
        InodeID(self.inodes.len() - 1)
    }
    fn add_to_dir(&mut self, parent: InodeID, child: InodeID) {
        if let Some(dir) = self.inodes[parent.0].listing.as_mut() {
            dir.listing.push(child)
        }
    }
    fn get_subdir(&self, node: InodeID, name: &str) -> Option<InodeID> {
        self.inodes
            .get(node.0)
            .and_then(|inode| inode.listing.as_ref())
            .iter()
            .flat_map(|dir| dir.listing.iter())
            .find(|id| self.inodes[id.0].name == name)
            .cloned()
    }
}

fn parse_cd(input: &str) -> Result<&str> {
    let mut split = input.split_whitespace();
    aoc::parse::expect_str_literal(&mut split, "$")?;
    aoc::parse::expect_str_literal(&mut split, "cd")?;
    aoc::parse::expect_word(&mut split, "directory")
}

fn parse_dir_listing(input: &str) -> Result<&str> {
    let mut split = input.split_whitespace();
    aoc::parse::expect_str_literal(&mut split, "dir")?;
    aoc::parse::expect_word(&mut split, "directory")
}

fn parse_file_listing(input: &str) -> Result<(usize, &str)> {
    let mut split = input.split_whitespace();
    let sz: usize = aoc::parse::expect_parse(&mut split, "file size")?;
    let filename = aoc::parse::expect_word(&mut split, "file name")?;
    Ok((sz, filename))
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../../../input/day7");
    const EX: &str = include_str!("../../../input/day7_ex");

    #[test]
    fn verify_p1() {
        assert_eq!(part1(INPUT).unwrap().as_str(), "1443806")
    }
    #[test]
    fn verify_p2() {
        assert_eq!(part2(INPUT).unwrap().as_str(), "942298")
    }
    #[test]
    fn p1_ex() {
        assert_eq!(part1(EX).unwrap().as_str(), "95437")
    }
    #[test]
    fn p2_ex() {
        assert_eq!(part2(EX).unwrap().as_str(), "24933642")
    }
}
