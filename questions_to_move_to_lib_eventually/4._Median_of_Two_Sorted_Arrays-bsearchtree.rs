impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // Ensure nums1 is the shorter array for binary search efficiency
        let (nums1, nums2) = if nums1.len() > nums2.len() { (nums2, nums1) } else { (nums1, nums2) };
        let m = nums1.len();
        let n = nums2.len();
        let total = m + n;
        let half = total / 2;

        // Binary search on nums1
        let mut left = 0;
        let mut right = m;

        while left <= right {
            // Partition point for nums1
            let i = left + (right - left) / 2;
            // Partition point for nums2
            let j = half - i;

            // Check if partition is correct
            let left_max = if i == 0 { i32::MIN } else { nums1[i - 1] };
            let right_min = if i == m { i32::MAX } else { nums1[i] };
            let left_max2 = if j == 0 { i32::MIN } else { nums2[j - 1] };
            let right_min2 = if j == n { i32::MAX } else { nums2[j] };

            if left_max <= right_min2 && left_max2 <= right_min {
                // Found the correct partition
                if total % 2 == 0 {
                    return (left_max.max(left_max2) as f64 + right_min.min(right_min2) as f64) / 2.0;
                } else {
                    return right_min.min(right_min2) as f64;
                }
            } else if left_max > right_min2 {
                // Left partition of nums1 is too large, reduce i
                right = i - 1;
            } else {
                // Left partition of nums1 is too small, increase i
                left = i + 1;
            }
        }
        0.0
    }
}
