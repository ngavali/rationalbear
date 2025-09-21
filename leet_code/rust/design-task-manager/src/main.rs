//https://leetcode.com/problems/design-task-manager/

use std::cell::RefCell;
use std::collections::{HashMap,BTreeSet};

#[derive(Debug)]
struct TaskManager {
    tasks: RefCell<HashMap<i32,(i32, i32)>>,
    priority: RefCell<BTreeSet<(i32, i32)>>,
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
            tasks: RefCell::new(t),
            priority: RefCell::new(p),
        }
    }

    fn add(&self, user_id: i32, task_id: i32, priority: i32) {
        let mut t = self.tasks.borrow_mut();
        let mut p = self.priority.borrow_mut();
        p.insert((priority, task_id));
        t.insert(task_id,(priority, user_id));
    }

    fn edit(&self, task_id: i32, new_priority: i32) {
        let mut t = self.tasks.borrow_mut();
        if let Some((priority, user_id)) = t.get_mut(&task_id) {
            let mut tp = self.priority.borrow_mut();
            tp.remove(&(*priority, task_id));
            tp.insert((new_priority, task_id));
            *priority = new_priority;
        }
    }

    fn rmv(&self, task_id: i32) {
        let mut t= self.tasks.borrow_mut();
        let mut p= self.priority.borrow_mut();
        if let Some((priority, user_id)) = t.get(&task_id) {
            p.remove(&(*priority, task_id));
        }
    }

    fn exec_top(&self) -> i32 {
        let mut t = self.tasks.borrow_mut();
        let mut p  = self.priority.borrow_mut();
        let mut res = None;
        if let Some((_, tid)) = p.pop_last() {
            if let Some((_, uid)) = t.get(&tid) {
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
