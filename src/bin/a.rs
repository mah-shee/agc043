#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    }
    let dx = vec![1, 0];
    let dy = vec![0, 1];
    let mut dp = vec![vec![std::usize::MAX; w]; h];
    if s[0][0] == '#' {
        dp[0][0] = 1;
    } else {
        dp[0][0] = 0;
    }

    for i in 0..h {
        for j in 0..w {
            for k in 0..2 {
                let ci = i + dx[k];
                let cj = j + dy[k];
                if ci >= h || cj >= w {
                    continue;
                }
                let mut add = 0;
                if s[ci][cj] == '#' && s[i][j] == '.' {
                    add = 1;
                }
                dp[ci][cj] = std::cmp::min(dp[ci][cj], dp[i][j] + add);
            }
        }
    }
    println!("{}", dp[h - 1][w - 1]);
}
