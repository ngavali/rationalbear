//https://leetcode.com/problems/design-task-manager/

use std::collections::{HashMap,BTreeSet};

#[derive(Debug)]
struct TaskManager {
    tasks: HashMap<i32,(i32, i32)>,
    priority: BTreeSet<(i32, i32)>,
}

impl TaskManager {
    fn new(tasks: Vec<Vec<i32>>) -> Self {
        let mut t = HashMap::new();
        let mut p : BTreeSet<(i32,i32)> = BTreeSet::new();
        tasks.iter().for_each(|a| {
            t.insert(a[1],(a[2], a[0]));
            p.insert((a[2], a[1]));
        });
        TaskManager {
            tasks: t,
            priority: p,
        }
    }

    fn add(&mut self, user_id: i32, task_id: i32, priority: i32) {
        self.priority.insert((priority, task_id));
        self.tasks.insert(task_id,(priority, user_id));
    }

    fn edit(&mut self, task_id: i32, new_priority: i32) {
        if let Some((priority, user_id)) = self.tasks.get_mut(&task_id) {
            self.priority.remove(&(*priority, task_id));
            self.priority.insert((new_priority, task_id));
            *priority = new_priority;
        }
    }

    fn rmv(&mut self, task_id: i32) {
        if let Some((priority, user_id)) = self.tasks.get(&task_id) {
            self.priority.remove(&(*priority, task_id));
        }
    }

    fn exec_top(&mut self) -> i32 {
        let mut res = None;
        if let Some((_, tid)) = self.priority.pop_last() {
            if let Some((_, uid)) = self.tasks.get(&tid) {
                res = Some((tid, *uid));
            }
        }
        match res {
            Some((tid,uid)) => {
                uid
            },
            _ => -1,
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    /*fn testcases() -> Vec<(&str, i32, )> {
    }
    */
}

/*// Study best solution

struct TaskManager {
    heap: Vec<(i32, i32, i32)>,
    index: Vec<Option<usize>>,
    max_task_id: usize,
}

impl TaskManager {
    fn new(tasks: Vec<Vec<i32>>) -> Self {
        let max_task_id = tasks.iter().map(|t| t[1] as usize).max().unwrap_or(0) + 1;
        let mut tm = TaskManager {
            heap: Vec::with_capacity(tasks.len()),
            index: vec![None; max_task_id],
            max_task_id,
        };
        for t in tasks {
            tm.add(t[0], t[1], t[2]);
        }
        tm
    }

    fn add(&mut self, user: i32, task: i32, prio: i32) {
        let idx = self.heap.len();
        if (task as usize) >= self.index.len() {
            self.index.resize(task as usize + 1, None);
        }
        self.heap.push((prio, task, user));
        self.index[task as usize] = Some(idx);
        self.sift_up(idx);
    }

    fn edit(&mut self, task: i32, new_prio: i32) {
        if let Some(i) = self.index[task as usize] {
            self.heap[i].0 = new_prio;
            self.sift_up(i);
            self.sift_down(i);
        }
    }

    fn rmv(&mut self, task: i32) {
        if let Some(i) = self.index[task as usize] {
            let last = self.heap.len() - 1;
            if i != last {
                self.swap(i, last);
                self.heap.pop();
                self.sift_up(i);
                self.sift_down(i);
            } else {
                self.heap.pop();
            }
            self.index[task as usize] = None;
        }
    }

    fn exec_top(&mut self) -> i32 {
        if self.heap.is_empty() { return -1; }
        let (_, task, user) = self.heap[0];
        self.rmv(task);
        user
    }

    #[inline(always)]
    fn better(a: (i32,i32,i32), b: (i32,i32,i32)) -> bool {
        a.0 > b.0 || (a.0 == b.0 && a.1 > b.1)
    }

    #[inline(always)]
    fn swap(&mut self, i: usize, j: usize) {
        self.heap.swap(i, j);
        self.index[self.heap[i].1 as usize] = Some(i);
        self.index[self.heap[j].1 as usize] = Some(j);
    }

    fn sift_up(&mut self, mut i: usize) {
        while i > 0 {
            let p = (i - 1) >> 1;
            if Self::better(self.heap[i], self.heap[p]) {
                self.swap(i, p);
                i = p;
            } else { break; }
        }
    }

    fn sift_down(&mut self, mut i: usize) {
        let n = self.heap.len();
        loop {
            let l = (i << 1) + 1;
            let r = (i << 1) + 2;
            let mut best = i;
            if l < n && Self::better(self.heap[l], self.heap[best]) { best = l; }
            if r < n && Self::better(self.heap[r], self.heap[best]) { best = r; }
            if best == i { break; }
            self.swap(i, best);
            i = best;
        }
    }
}

*/
