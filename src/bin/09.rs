use core::fmt;

advent_of_code::solution!(9);

#[derive(Clone)]
enum Block {
    Free,
    Used(usize),
}

#[derive(Clone)]
enum BlockSegment {
    Free(i32, usize),
    Used(i32, usize, usize),
}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            Block::Free => ".".to_owned(),
            Block::Used(id) => id.to_string(),
        };
        write!(f, "{}", out)
    }
}

impl fmt::Debug for BlockSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let out = match self {
            BlockSegment::Free(_bid, size) => ".".repeat(*size),
            BlockSegment::Used(_bid, sid, size) => sid.to_string().repeat(*size),
        };
        write!(f, "{}", out)
    }
}

fn dense_disk_map_to_sparse(disk_map: &str) -> Vec<Block> {
    disk_map
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .enumerate()
        .flat_map(|(ix, size)| {
            let block_type = match (ix % 2) == 0 {
                true => Block::Used(ix / 2),
                false => Block::Free,
            };
            std::iter::repeat(block_type).take(size)
        })
        .collect()
}

fn dense_disk_map_to_segments(disk_map: &str) -> Vec<BlockSegment> {
    disk_map
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .enumerate()
        .filter(|&(_, size)| size > 0)
        .map(|(ix, size)| match (ix % 2) == 0 {
            true => BlockSegment::Used(ix as i32, ix / 2, size),
            false => BlockSegment::Free(ix as i32, size),
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<usize> {
    let disk_map = input.lines().next()?;
    let blocks = dense_disk_map_to_sparse(disk_map);

    let mut checksum = 0;
    let mut end_of_disk = blocks.len() - 1;

    for (block_ix, block) in blocks.iter().enumerate() {
        match block {
            Block::Free => {
                // Find the last used block from the end
                end_of_disk = blocks[block_ix..=end_of_disk]
                    .iter()
                    .rposition(|b| matches!(b, Block::Used(_)))
                    .map_or(block_ix, |idx| block_ix + idx);

                if let Block::Used(block_id) = blocks[end_of_disk] {
                    checksum += block_ix * block_id;
                    end_of_disk -= 1;
                }
            }
            Block::Used(block_id) => {
                checksum += block_ix * block_id;
            }
        }

        if end_of_disk <= block_ix {
            break;
        }
    }

    Some(checksum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let disk_map = input.lines().next()?;
    let segments = dense_disk_map_to_segments(disk_map);
    let mut compacted = segments.clone();

    for segment in segments.iter().rev() {
        if let BlockSegment::Used(bid, _sid, size) = segment {
            // Find the first free segment with size at least segment_size
            let maybe_free_space_ix = compacted
                .iter()
                .take_while(
                    |segment| !matches!(segment, BlockSegment::Used(this_bid, ..) if this_bid==bid),
                )
                .position(|segment| matches!(segment, BlockSegment::Free(_, s) if s>=size));

            if let Some(free_space_ix) = maybe_free_space_ix {
                if let BlockSegment::Free(free_bid, free_size) = compacted[free_space_ix] {
                    if free_size > *size {
                        compacted[free_space_ix] = BlockSegment::Free(free_bid, *size);
                        compacted.insert(
                            free_space_ix + 1,
                            BlockSegment::Free(-free_bid, free_size - size),
                        );
                    }
                    let segment_ix = compacted
                        .iter()
                        .position(
                            |s| matches!(s, BlockSegment::Used(this_bid, ..) if this_bid==bid),
                        )
                        .unwrap();
                    compacted.swap(segment_ix, free_space_ix);
                }
            }
        }
    }

    let (_, checksum) = compacted
        .iter()
        .fold((0, 0), |(block_ix, checksum), s| match s {
            BlockSegment::Free(_, size) => (block_ix + size, checksum),
            BlockSegment::Used(_, sid, size) => (
                block_ix + size,
                checksum
                    + (block_ix..(block_ix + size))
                        .map(|ix| ix * sid)
                        .sum::<usize>(),
            ),
        });

    Some(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
