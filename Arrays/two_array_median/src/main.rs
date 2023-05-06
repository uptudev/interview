fn main() {

}

#[test]
fn test_1() {
    assert_eq!(2., find_median_sorted_arrays(vec![1,3], vec![2]));
}

#[test]
fn test_2() {
    assert_eq!(2.5, find_median_sorted_arrays(vec![1,2], vec![3, 4]));
}

fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // Ensure that nums1 is the smaller array
        let (nums1, nums2) = if nums1.len() <= nums2.len() {
            (nums1, nums2)
        } else {
            (nums2, nums1)
        };

        // Define the possible range of positions for the median element via two pointers, low and high.
        let mut low = 0;
        let mut high = nums1.len();

        // Binary search loop to find the median
        while low <= high {

            // Determine the current position of the median element
            let i = (low + high) / 2;
            let j = (nums1.len() + nums2.len() + 1) / 2 - i;

            // Determine the maximum element in the left half of each array and the minimum element in the right half of each array
            let max_left_1 = if i == 0 { std::i32::MIN } else { nums1[i - 1] };
            let min_right_1 = if i == nums1.len() { std::i32::MAX } else { nums1[i] };
            let max_left_2 = if j == 0 { std::i32::MIN } else { nums2[j - 1] };
            let min_right_2 = if j == nums2.len() { std::i32::MAX } else { nums2[j] };

            // If the current positions of the median element are valid, return the median
            if max_left_1 <= min_right_2 && max_left_2 <= min_right_1 {
                // If the array length is even, return the midpoint between the two middle values
                if (nums1.len() + nums2.len()) % 2 == 0 {
                    return ((max_left_1.max(max_left_2) + min_right_1.min(min_right_2)) as f64) / 2.0;
                // If odd, just return the midpoint value
                } else {
                    return max_left_1.max(max_left_2) as f64;
                }
            
            // If the left half of nums1 is too large, move the median position to the left
            } else if max_left_1 > min_right_2 {
                high = i - 1;

            // Otherwise, move the median position to the right
            } else {
                low = i + 1;
            }
        }
        // This should never be reached, but it's needed to satisfy the compiler's babyrage
        unreachable!();
    }
