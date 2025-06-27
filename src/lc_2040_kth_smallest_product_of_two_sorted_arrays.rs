pub fn check(nums2: &Vec<i32>, x1: i64, v: i64) -> i64 {
    let length = nums2.len();
    let mut left = 0;
    let mut right = length as i32 - 1;
    while left <= right {
        let mid = left + (right - left) / 2;
        let product = x1 * nums2[mid as usize] as i64;
        if (x1 >= 0 && product <= v) || (x1 < 0 && product > v) {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    if x1 >= 0 {
        left as i64
    } else {
        length as i64 - left as i64
    }
}

pub fn kth_smallest_product(nums1: Vec<i32>, nums2: Vec<i32>, k: i64) -> i64 {
    let mut left = -1e10 as i64;
    let mut right = 1e10 as i64;
    while left <= right {
        let mid = left + (right - left) / 2;
        let mut count = 0;
        for &x in &nums1 {
            count += check(&nums2, x as i64, mid);
        }
        if count < k {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    left
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(kth_smallest_product(vec![-4, -2, 0, 3], vec![2, 4], 6), 0);
    }
}

/* initial try with iterators
    let mut product: Vec<i64> = nums1
        .iter()
        .flat_map(|&a| nums2.iter().map(move |&b| a as i64 * b as i64))
        .collect();
    product.sort_unstable();
    product[k as usize - 1]
// O(N * M * log(N * M))
*/
