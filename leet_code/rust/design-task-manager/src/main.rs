//https://leetcode.com/problems/design-task-manager/

use std::cell::RefCell;

#[derive(Debug)]
struct TaskManager {
    tasks: RefCell<Vec<Option<(i32, i32)>>>,
    priority: RefCell<Vec<(i32, i32)>>,
}

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TaskManager {
    fn new(tasks: Vec<Vec<i32>>) -> Self {
        let mut t = vec![None;100001];
        let mut p = Vec::with_capacity(tasks.len());
        tasks.iter().for_each(|a| {
            t[a[1] as usize] = Some((a[2], a[0]));
            p.push((a[2], a[1]));
        });
        p.sort_by(|a, b| {
            if a.0 == b.0 {
                return a.1.cmp(&b.1);
            }
            a.0.cmp(&b.0)
        });
        TaskManager {
            tasks: RefCell::new(t),
            priority: RefCell::new(p),
        }
    }

    fn add(&self, user_id: i32, task_id: i32, priority: i32) {
        let mut t = self.tasks.borrow_mut();
        let mut p = self.priority.borrow_mut();
        let pos = match p.binary_search_by(|a| {
            if a.0 == priority {
                a.1.cmp(&task_id)
            } else {
                a.0.cmp(&priority)
            }
        }) {
            Ok(idx) => idx,
            Err(idx) => idx,
        };
        p.insert(pos, (priority, task_id));
        t[task_id as usize] = Some((priority, user_id));
    }

    fn edit(&self, task_id: i32, new_priority: i32) {
        let mut t = self.tasks.borrow_mut();
        if let Some((priority, user_id)) = t[task_id as usize] {
            let mut tp = self.priority.borrow_mut();
            match tp.binary_search_by(|a| {
                if a.0 == priority {
                        a.1.cmp(&task_id)
                } else {
                    a.0.cmp(&priority)
                }
            }) {
                Ok(pos) => {
                    let mut e = tp.remove(pos);
                    let pos = match tp.binary_search_by(|a| {
                        if a.0 == new_priority {
                            a.1.cmp(&task_id)
                        } else {
                            a.0.cmp(&new_priority)
                        }
                    }) {
                        Ok(idx) => idx,
                        Err(idx) => idx,
                    };
                    tp.insert(pos,(new_priority, task_id));
                    t[task_id as usize] = Some((new_priority, task_id));
                },
                Err(idx) => {},
            };
        }
    }

    fn rmv(&self, task_id: i32) {
        let mut t= self.tasks.borrow_mut();
        let mut p= self.priority.borrow_mut();
        t[task_id as usize] = None;
        p.retain(|p| p.1 != task_id);
    }

    fn exec_top(&self) -> i32 {
        let mut t = self.tasks.borrow_mut();
        let mut p  = self.priority.borrow_mut();
        if let Some(task) = p.pop() {
            if let Some((priority,user_id)) = t[task.1 as usize] {
                t[task.1 as usize] = None;
                return user_id;
            }
        }
        -1
    }
}

/**
 * Your TaskManager object will be instantiated and called as such:
 * let obj = TaskManager::new(tasks);
 * obj.add(userId, taskId, priority);
 * obj.edit(taskId, newPriority);
 * obj.rmv(taskId);
 * let ret_4: i32 = obj.exec_top();
 */
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    /*fn testcases() -> Vec<(&str, i32, )> {
    }
    */
}
