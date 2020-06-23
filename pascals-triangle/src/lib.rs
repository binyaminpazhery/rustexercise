pub struct PascalsTriangle(usize);

impl PascalsTriangle {
    pub fn new(row_count: usize) -> Self {
        PascalsTriangle(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows = Vec::new();

        if self.0 >= 1 {
            rows.push(vec![1]);

            while rows.len() < self.0 {
                let mut prev_row = rows.last().unwrap().to_vec();
                let mut current_row = Vec::new();

                prev_row.insert(0, 0);
                prev_row.push(0);

                for window in prev_row.windows(2) {
                    current_row.push(window[0] + window[1]);
                }

                rows.push(current_row);
            }
        }

        rows
    }
}