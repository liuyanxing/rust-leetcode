pub fn backtrace(candidate: &Vec<i32>, target: i32, start: usize, res: &mut Vec<Vec<i32>>, temp: &mut Vec<i32>) {
	if target == 0 {
		res.push(temp.clone());
		return;
	}
	if target < 0 {
		return;
	}
	
	for i in start..candidate.len() {
		temp.push(candidate[i]);
		backtrace(candidate, target - candidate[i], i, res, temp);
		temp.pop();
	}
}

pub fn combination_sum(candidate: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
	let mut res = Vec::with_capacity(100);
	backtrace(&candidate, target, 0, &mut res, &mut Vec::with_capacity(60));
	return res;
}