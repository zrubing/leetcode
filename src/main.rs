fn main() {}

pub fn tow_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (index, value) in nums.iter().enumerate() {
        let inner_vec = nums[index + 1..].iter();

        for (inner_index, inner_value) in inner_vec.enumerate() {
            if value + inner_value == target {
                println!("{},{}", value, inner_value);

                return vec![index as i32, (index + 1 + inner_index) as i32];
            }
        }
    }
    return vec![];
}
