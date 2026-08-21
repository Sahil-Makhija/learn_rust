use std::vec;

pub fn calculate_median(sorted_nums: &[i32]) -> f64 {
    let len = sorted_nums.len();
    if len == 0 {
        return 0.0;
    } else if len == 1 {
        return sorted_nums[0] as f64;
    } else {
        if len % 2 == 0 {
            let m1 = sorted_nums[len / 2];
            let m2 = sorted_nums[(len / 2) - 1];
            return ((m1 + m2) as f64 / 2.0) as f64;
        } else {
            return (sorted_nums[len / 2]) as f64;
        }
    }
}

pub fn find_median_sorted_arrays(nums1: &mut Vec<i32>, nums2: &mut Vec<i32>) -> f64 {
    if nums1.len() == 0 && nums2.len() != 0 {
        return calculate_median(nums2);
    } else if nums2.len() == 0 && nums1.len() != 0 {
        return calculate_median(nums1);
    } else if nums1.len() == 0 && nums2.len() == 0 {
        return 0.0;
    } else {
        let mut p1 = 0;
        let mut p2 = 0;

        let mut merged: Vec<i32> = vec![];

        while p1 < nums1.len() && p2 < nums2.len() {
            if nums1[p1] > nums2[p2] {
                merged.push(nums2[p2]);
                p2 += 1;
            } else if nums1[p1] < nums2[p2] {
                merged.push(nums1[p1]);
                p1 += 1;
            } else {
                merged.push(nums1[p1]);
                merged.push(nums2[p2]);
                p1 += 1;
                p2 += 1;
            }
        }
        merged.extend_from_slice(&nums1[p1..]);
        merged.extend_from_slice(&nums2[p2..]);

        calculate_median(&merged)
    }
}
