/// @author: Leon
/// https://leetcode.com/problems/maximum-points-you-can-obtain-from-cards/
/// Time Complexity:    O(`len_pts`)
/// Space Complexity:   O(1)
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let len_pts: usize = card_points.len();
        let pts_total: i32 = card_points.iter().sum::<i32>();
        let k: usize = k as usize;
        if k == len_pts {
            return pts_total;
        }
        let len: usize = len_pts - k;
        let mut most: i32 = 0;
        let mut sum: i32 = 0;
        for (idx, &pt) in card_points.iter().enumerate() {
            sum += pt;
            if idx + 1 >= len {
                most = std::cmp::max(most, pts_total - sum);
                sum -= card_points[idx + 1 - len];
            }
        }
        most
    }
}
