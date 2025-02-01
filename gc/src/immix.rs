use std::alloc::{alloc, dealloc, Layout};
use std::ptr::NonNull;

trait AllocRaw {
    fn alloc<T>(&self, object: T) -> *const T;
}

const BLOCK_SIZE_BITS: usize = 15;
const BLOCK_SIZE: usize = 1 << BLOCK_SIZE_BITS;
const LINE_SIZE_BITS: usize = 7;
const LINE_SIZE: usize = 1 << LINE_SIZE_BITS;
const LINE_COUNT: usize = BLOCK_SIZE / LINE_SIZE;
const BLOCK_CAPACITY: usize = BLOCK_SIZE - LINE_COUNT;

type BlockPtr = NonNull<u8>;
type BlockSize = usize;

fn alloc_block(size: BlockSize) -> Result<BlockPtr, BlockError> {
    unsafe {
        let layout = Layout::from_size_align_unchecked(size, size);

        let ptr = alloc(layout);
        if ptr.is_null() {
            Err(BlockError::OOM)
        } else {
            Ok(NonNull::new_unchecked(ptr))
        }
    }
}

fn dealloc_block(ptr: BlockPtr, size: BlockSize) {
    unsafe {
        let layout = Layout::from_size_align_unchecked(size, size);
        dealloc(ptr.as_ptr(), layout);
    }
}

struct Block {
    ptr: BlockPtr,
    size: BlockSize,
}

#[derive(Debug, PartialEq)]
enum BlockError {
    BadRequest,
    OOM,
}

struct BumpBlock {
    // the index into the block where the last object was written
    cursor: *const u8,
    limit: *const u8,
    //
    block: Block,
    meta: BlockMeta,
}

struct BlockMeta {
    lines: *mut u8,
}

impl Block {
    pub fn new(size: BlockSize) -> Result<Block, BlockError> {
        if !size.is_power_of_two() {
            return Err(BlockError::BadRequest);
        }

        Ok(Block {
            ptr: alloc_block(size)?,
            size,
        })
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.ptr.as_ptr()
    }
}

impl BumpBlock {
    pub fn inner_alloc(&mut self, alloc_size: usize) -> Option<*const u8> {
        let block_start_ptr = self.block.as_ptr() as usize;
        let cursor_ptr = self.cursor as usize;

        // align to word boundary
        let align_mask: usize = !(size_of::<usize>() - 1);

        let next_ptr = cursor_ptr.checked_sub(alloc_size)? & align_mask;

        if next_ptr < block_start_ptr {
            None
        } else {
            Some(next_ptr as *const u8)
        }
    }
}

impl BlockMeta {
    pub fn find_next_availble_hole(
        &self,
        starting_at: usize,
        alloc_size: usize,
    ) -> Option<(usize, usize)> {
        // The count of consecutive avaliable holes. Must take into account a conservatively marked
        // hole at the beginning of the sequence.
        let mut count = 0;
        let starting_line = starting_at / LINE_SIZE;
        let lines_required = (alloc_size + LINE_SIZE - 1) / LINE_SIZE;
        // Counting down from the given search start index
        let mut end = starting_line;

        for index in (0..starting_line).rev() {
            let marked = unsafe { *self.lines.add(index) };

            if marked == 0 {
                // count unmarked lines
                count += 1;

                if index == 0 && count >= lines_required {
                    let limit = index * LINE_SIZE;
                    let cursor = end * LINE_SIZE;
                    return Some((cursor, limit));
                }
            } else {
                // This block is marked
                if count > lines_required {
                    // But at least 2 previous blocks were not marked. Return the hole, considering the
                    // immediately preceding block as conservatively marked
                    let limit = (index + 2) * LINE_SIZE;
                    let cursor = end * LINE_SIZE;
                    return Some((cursor, limit));
                }

                // If this line is marked and we didn't return a new cursor/limit pair by now,
                // reset the hole search state
                count = 0;
                end = index;
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let block = Block::new(1024).unwrap();
    }
}
