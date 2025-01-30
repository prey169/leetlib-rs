impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut newVec = [nums1, nums2].concat();
        newVec.sort();
        let lenght: f32 = newVec.len() as f32;
        match lenght % 2.0 {
            0.0 => {
                let x = lenght / 2.0;
                let y = (lenght / 2.0) - 1.0;
                let z = (newVec[x as usize] as f32 + newVec[y as usize] as f32) / 2.0;
                return z as f64;
            }
            _ => {
                let x = (lenght / 2.0) - 0.5;
                return newVec[x as usize] as f64;
            }
        };
    }
}
