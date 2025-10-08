fn bubble_sort(arr: &mut [i32]) {
	let n = arr.len();
	if n <= 1 {
		return;
	}

	for i in 0..n {
		let mut swapped = false;
		for j in 0..(n - 1 - i) {
			if arr[j] > arr[j + 1] {
				arr.swap(j, j + 1);
				swapped = true;
			}
		}
		if !swapped {
			break;
		}
	}
}

fn partition(arr: &mut [i32]) -> usize {
	let len = arr.len();
	let pivot = arr[len - 1];
	let mut i = 0usize;

	for j in 0..len - 1 {
		if arr[j] <= pivot {
			arr.swap(i, j);
			i += 1;
		}
	}

	arr.swap(i, len - 1);
	i
}

fn quick_sort(arr: &mut [i32]) {
	if arr.len() <= 1 {
		return;
	}

	let pivot_index = partition(arr);

	let (left, right) = arr.split_at_mut(pivot_index);
	quick_sort(left);
	if right.len() > 1 {
		quick_sort(&mut right[1..]);
	}
}

fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
	let mut res = Vec::with_capacity(left.len() + right.len());
	let mut i = 0;
	let mut j = 0;

	while i < left.len() && j < right.len() {
		if left[i] <= right[j] {
			res.push(left[i]);
			i += 1;
		} else {
			res.push(right[j]);
			j += 1;
		}
	}

	if i < left.len() {
		res.extend_from_slice(&left[i..]);
	}
	if j < right.len() {
		res.extend_from_slice(&right[j..]);
	}

	res
}

fn merge_sort(arr: &[i32]) -> Vec<i32> {
	let n = arr.len();
	if n <= 1 {
		return arr.to_vec();
	}

	let mid = n / 2;
	let left = merge_sort(&arr[..mid]);
	let right = merge_sort(&arr[mid..]);
	merge(&left, &right)
}

fn main() {
	let original = vec![45, 92, 10, 77, 5, 36, 21, 88, 12, 70, 99, 53, 18, 64, 3, 81, 29, 42, 7, 96];

	println!("Vetor original: {:?}", original);

	// Bubble Sort
	let mut b = original.clone();
	bubble_sort(&mut b);
	println!("Bubble Sort:   {:?}", b);

	// Quick Sort (in-place)
	let mut q = original.clone();
	quick_sort(&mut q);
	println!("Quick Sort:    {:?}", q);

	// Merge Sort (returns new Vec)
	let m = merge_sort(&original);
	println!("Merge Sort:    {:?}", m);
}