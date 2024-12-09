use core::fmt;

use pathfinding::num_traits::Zero;

advent_of_code::solution!(9);

#[derive(Clone)]
enum Block {
    Free,
    Used(usize),
}

#[derive(Clone)]
enum BlockSegment {
    Free(usize),
    Used(usize, usize),
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
            BlockSegment::Free(size) => size.to_string(),
            BlockSegment::Used(id, size) => format!("({}, {})", id.to_string(), size.to_string()),
        };
        write!(f, "{}", out)
    }
}

fn dense_disk_map_to_sparse(disk_map: &str) -> Vec<Block> {
    disk_map
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .enumerate()
        .flat_map(|(ix, digit)| {
            let block_type = match (ix % 2) == 0 {
                true => Block::Used(ix / 2),
                false => Block::Free,
            };
            std::iter::repeat(block_type).take(digit)
        })
        .collect()
}

fn dense_disk_map_to_segments(disk_map: &str) -> Vec<BlockSegment> {
    disk_map
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .enumerate()
        .map(|(ix, digit)| match (ix % 2) == 0 {
            true => BlockSegment::Used(ix / 2, digit),
            false => BlockSegment::Free(digit),
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

    let mut checksum = 0;
    let last_segment_ix = segments.len() - 1;
    let mut inserted = 0;

    for segment_ix in (0..=last_segment_ix).rev() {
        let segment = &segments[segment_ix];

        if let BlockSegment::Used(_id, segment_size) = segment {
            println!("Segment: {:?}", &segment);
            // Find the first free segment with size at least segment_size
            let maybe_free_space_ix = compacted[0..last_segment_ix+inserted]
                .iter()
                .position(|segment| matches!(segment, BlockSegment::Free(s) if s>=segment_size));

            if let Some(free_space_ix) = maybe_free_space_ix {
                println!("Free space found {:?}", compacted[free_space_ix]);
                compacted.swap(segment_ix+inserted, free_space_ix);
                if let BlockSegment::Free(free_size) = &compacted[free_space_ix] {
                    if free_size > segment_size {
                        compacted.insert(
                            free_space_ix + 1,
                            BlockSegment::Free(free_size - segment_size),
                        );
                        inserted += 1;
                    }
                }
            } else {
                println!("No free space found!");
            }
        }
    }

    dbg!(compacted);

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
        assert_eq!(result, None);
    }
}
