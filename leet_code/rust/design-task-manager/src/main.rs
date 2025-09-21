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
