pub fn parse(line: String) -> Option<i32> {
    line.parse::<i32>().ok()
}

pub fn solve1(nums: &[i32]) -> i32 {
    let mut increasing_count = 0;

    let mut prev = i32::MAX;
    for num in nums {
        if num > &prev {
            increasing_count += 1
        };
        prev = *num;
    }

    increasing_count
}

pub fn solve2(nums: &[i32]) -> i32 {
    let mut sums = vec![];

    for i in 0..nums.len() {
        sums.push(nums[i..std::cmp::min(i + 3, nums.len())].iter().sum());
    }

    solve1(&sums)
}
