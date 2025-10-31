//https://leetcode.com/problems/snapshot-array/

struct SnapshotArray {
    state: Vec<Vec<(i32, i32)>>,
    snap_idx: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl SnapshotArray {
    fn new(length: i32) -> Self {
        SnapshotArray {
            state: vec![Vec::new(); length as usize],
            snap_idx: 0,
        }
    }

    fn set(&mut self, index: i32, val: i32) {
        if let Some(latest) = self.state[index as usize].last_mut()
            && latest.0 == self.snap_idx
        {
            latest.1 = val;
        } else {
            self.state[index as usize].push((self.snap_idx, val));
        }
    }

    fn snap(&mut self) -> i32 {
        self.snap_idx += 1;
        self.snap_idx - 1
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        if snap_id < self.snap_idx {
            match self.state[index as usize].binary_search_by(|s_id| s_id.0.cmp(&snap_id)) {
                Ok(pos) => self.state[index as usize][pos].1,
                Err(pos) if pos > 0 => self.state[index as usize][pos - 1].1,
                _ => 0,
            }
        } else {
            0
        }
    }
}

/**
 * Your SnapshotArray object will be instantiated and called as such:
 * let obj = SnapshotArray::new(length);
 * obj.set(index, val);
 * let ret_2: i32 = obj.snap();
 * let ret_3: i32 = obj.get(index, snap_id);
 */

fn main() {
    println!("Hello, world!");
}
