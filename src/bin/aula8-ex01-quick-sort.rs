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

	// left contains elements <= pivot
	quick_sort(left);

	// right starts with pivot at index 0, skip it when sorting the greater partition
	if right.len() > 1 {
		quick_sort(&mut right[1..]);
	}
}

fn main() {
	let mut v = vec![9, -3, 5, 2, 6, 8, -6, 1, 3];
	println!("Vetor antes: {:?}", v);

	quick_sort(&mut v);

	println!("Vetor depois: {:?}", v);
}