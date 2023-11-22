/// @author: Leon
/// https://leetcode.com/problems/flood-fill/
/// Time Complexity:    O(`len_rs` * `len_cs`)
/// Space Complexity:   O(`len_rs` * `len_cs`)
struct Solution;

#[allow(dead_code)]
impl Solution {
    const DIRS: &'static [isize] = &[0, -1, 0, 1, 0];
    pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
        let sr: usize = sr as usize;
        let sc: usize = sc as usize;
        if image[sr][sc] == color {
            return image;
        }
        let original = image[sr][sc];
        Self::dfs((sr as isize, sc as isize), &mut image, original, color);
        return image;
    }
    fn dfs(coord: (isize, isize), image: &mut Vec<Vec<i32>>, original: i32, color: i32) {
        let len_rs: isize = image.len() as isize;
        let len_cs: isize = image[0].len() as isize;
        let (r, c) = coord;
        image[r as usize][c as usize] = color;
        for d in 0..4 {
            let r_nxt: isize = r + Self::DIRS[d];
            let c_nxt: isize = c + Self::DIRS[d + 1];
            if r_nxt >= 0
                && c_nxt >= 0
                && r_nxt < len_rs
                && c_nxt < len_cs
                && image[r_nxt as usize][c_nxt as usize] == original
            {
                Self::dfs((r_nxt, c_nxt), image, original, color);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works_with_sample_input_1() {
        let image: Vec<Vec<i32>> = vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]];
        let sr: i32 = 1;
        let sc: i32 = 1;
        let color: i32 = 2;
        let expected: Vec<Vec<i32>> = vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]];
        let actual: Vec<Vec<i32>> = Solution::flood_fill(image, sr, sc, color);
        assert_eq!(expected, actual);
    }
}
