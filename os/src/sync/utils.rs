use alloc::{collections::VecDeque, sync::Arc};
use crate::task::{TaskControlBlock};

pub fn get_next_queue_id(wait_queue: &VecDeque<Arc<TaskControlBlock>>) -> isize {
    if wait_queue.is_empty() {
        -1
    } else {
        if let Some(waking_task) = wait_queue.front() {
            waking_task.inner_exclusive_access().res.as_ref().unwrap().tid as isize
        } else {
            -1
        }
    }
}