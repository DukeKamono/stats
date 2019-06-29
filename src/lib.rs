// Copyright © 2019 Bart Massey
// [This program is licensed under the "MIT License"]
// Please see the file LICENSE in the source
// distribution of this software for license terms.

///! Functions to compute various statistics on a slice of
///! floating-point numbers.

/// Type of statistics function. If the statistic
/// is ill-defined, `None` will be returned.
pub type StatFn = fn(&[f64]) -> Option<f64>;

/// Arithmetic mean of input values. The mean of an empty
/// list is 0.0.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), mean(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), mean(&[-1.0, 1.0]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(1.0), mean(&[1.0, 1.0, 1.0]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.5), mean(&[-3.0, -1.0, 1.0, 5.0]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(1.2), mean(&[-1.0, 1.0, 7.0, 2.0, -3.0]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(-1.6), mean(&[-1.0, 1.0, -7.0, 2.0, -3.0]));
/// ```
pub fn mean(nums: &[f64]) -> Option<f64> {
	if nums == &[] {
		Some(0.0)
	} else {
		let mut x: f64 = 0.0;
		let mut y: f64 = 0.0;
		for i in nums {
			x += 1.0;
			y += i;
		};
		
		let z: f64 = y / x;
		Some(z)
	}
}

/// Population standard deviation of input values. The
/// standard deviation of an empty list is undefined.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(None, stddev(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), stddev(&[1.0, 1.0]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(4.5), stddev(&[2.0, -1.0]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(12.0), stddev(&[1.0, 1.0, -5.0]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(28.25), stddev(&[1.0, 1.0, -5.0, -10.0]));
/// ```
pub fn stddev(nums: &[f64]) -> Option<f64> {
    if nums == &[] {
		None
	} else {
		Some(summation_power(nums, mean(nums).unwrap()) / (nums.len() - 1) as f64)
	}
}

/// Median value of input values, taking the value closer
/// to the beginning to break ties. The median
/// of an empty list is undefined.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(None, median(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), median(&[0.0, 0.5, -1.0, 1.0]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), median(&[0.0, -1.0, 1.0]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.2), median(&[0.0, -1.0, 1.0, 3.0, -3.0, 0.5, 0.2]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), median(&[1.2, 0.0, -1.0, 1.0, 5.0, -3.0]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(-0.2), median(&[1.2, 0.0, -1.0, 1.0, 5.0, -3.0, -0.2, -0.5]));
/// ```
pub fn median(nums: &[f64]) -> Option<f64> {
    // Make a sorted copy of the input floats.
    let mut nums = nums.to_owned();
    // https://users.rust-lang.org/t/how-to-sort-a-vec-of-floats/2838/2
    nums.sort_by(|a, b| a.partial_cmp(b).unwrap());

    if nums == &[] {
		None
	} else {
		let length = nums.len();
		let offset = length - 1;
		if length % 2 == 0 {
			let first = nums[offset / 2];
			//From the notes above the examples we are not dividing by 2.
			//let second = nums[(offset / 2) + 1];
			//Some((first + second) / 2.0)
			Some(first)
		} else {
			Some(nums[offset / 2])
		}
	}
}

/// L2 norm (Euclidean norm) of input values. The L2
/// norm of an empty list is 0.0.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(Some(0.0), l2(&[]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(3.0), l2(&[-3.0]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(5.0), l2(&[-3.0, 4.0]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(6.0), l2(&[-4.0, -2.0, 4.0]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(2.0), l2(&[-1.0, 1.0, 1.0, 1.0]));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(Some(8.0), l2(&[-3.0, 4.0, -3.0, 5.0, 1.0, -2.0]));
/// ```
pub fn l2(nums: &[f64]) -> Option<f64> {
    if nums == &[] {
		Some(0.0)
	} else {
		Some(summation_power(nums, 0.0).sqrt())
	}
}

/// This takes each array value, minuses it from the offset,
/// rasies the power by 2, and then adds it to the total list.
///
/// # Examples:
///
/// ```
/// # use stats::*;
/// assert_eq!(0.0, summation_power(&[], 0.0));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(9.0, summation_power(&[-3.0], 0.0));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(25.0, summation_power(&[-3.0], 2.0));
/// ```
/// ```
/// # use stats::*;
/// assert_eq!(29.0, summation_power(&[-3.0, 4.0], 2.0));
/// ```
pub fn summation_power(nums: &[f64], offset: f64) -> f64 {
	//took this to_owned form the median section
	let mut nums = nums.to_owned();
	let mut total: f64 = 0.0;
	
	for i in &mut nums {
		total += (*i - offset).powf(2.0);
	}
	
	total
}