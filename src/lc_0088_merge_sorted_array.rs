pub fn merge(nums1: &mut Vec<i32>, _m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut i = 0;
    while i < n {
        i += 1;
        nums1.pop();
    }

    nums1.append(nums2);
    nums1.sort()
}
