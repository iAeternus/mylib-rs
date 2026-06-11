use std::{
    panic::{self, AssertUnwindSafe},
    sync::{Arc, atomic::Ordering},
    thread,
};

use crossbeam_deque::{Injector, Steal, Stealer, Worker};

use crate::thread_pool::pool::{PoolState, Task};

/// 按优先级查找任务：local LIFO -> 高优 Injector -> 低优 Injector -> 窃取其他 Worker。
pub(super) fn find_work(
    local: &Worker<Task>,
    injectors: &[Injector<Task>],
    stealers: &[Stealer<Task>],
    steal_order: &[usize],
) -> Option<Task> {
    if let Some(t) = local.pop() {
        return Some(t);
    }

    for inj in injectors {
        loop {
            match inj.steal_batch_and_pop(local) {
                Steal::Success(t) => return Some(t),
                Steal::Retry => thread::yield_now(),
                Steal::Empty => break,
            }
        }
    }

    for &victim in steal_order {
        loop {
            match stealers[victim].steal_batch_and_pop(local) {
                Steal::Success(t) => return Some(t),
                Steal::Retry => thread::yield_now(),
                Steal::Empty => break,
            }
        }
    }

    None
}

/// Worker 主循环：找任务 -> 执行 -> 统计 -> 循环，空闲时 park。
pub(super) fn worker_loop(
    id: usize,
    local: Worker<Task>,
    stealers: Arc<Vec<Stealer<Task>>>,
    injectors: Arc<Vec<Injector<Task>>>,
    state: Arc<PoolState>,
) {
    let steal_order: Vec<usize> = {
        let mut order: Vec<usize> = (0..stealers.len()).collect();
        if id < order.len() {
            order.swap_remove(id);
        }
        order
    };

    state.sleeper.register(thread::current());

    loop {
        if state.shutdown_now.load(Ordering::Acquire) {
            break;
        }

        let task = match find_work(&local, &injectors, &stealers, &steal_order) {
            Some(t) => t,
            None => {
                if state.shutdown.load(Ordering::Acquire)
                    && state.pending.load(Ordering::Acquire) == 0
                {
                    break;
                }
                state.sleeper.park();
                continue;
            }
        };

        state.stats.task_started();
        let result = panic::catch_unwind(AssertUnwindSafe(task));
        match result {
            Ok(()) => {
                state.stats.task_completed();
            }
            Err(e) => {
                state.stats.task_panicked();
                eprintln!("[worker-{}] 任务崩溃: {:?}", id, e);
            }
        }

        state.pending.fetch_sub(1, Ordering::Release);

        if state.capacity > 0 {
            let _guard = state.blocker_lock.lock().unwrap();
            state.blocker_cvar.notify_one();
        }
    }

    state.exited.fetch_add(1, Ordering::Release);
    {
        let _guard = state.blocker_lock.lock().unwrap();
        state.blocker_cvar.notify_all();
    }
}
