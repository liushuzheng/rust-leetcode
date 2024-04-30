struct SnapshotArray {
    snap_id: i32,
    array: Vec<i32>,
}

impl SnapshotArray {
    fn new(length: i32) -> Self {
        SnapshotArray {
            snap_id: -1,
            array: vec![0; length as usize],
        }
    }

    fn set(&self, index: i32, val: i32) {

    }

    fn snap(&self) -> i32 {
        0
    }

    fn get(&self, index: i32, snap_id: i32) -> i32 {
        0
    }
}
