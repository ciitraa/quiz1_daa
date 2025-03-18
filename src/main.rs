fn min_heapify(arr: &mut [i32], n: usize, i: usize) {
    let mut smallest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && arr[left] < arr[smallest] {
        smallest = left;
    }
    if right < n && arr[right] < arr[smallest] {
        smallest = right;
    }

    if smallest != i {
        arr.swap(i, smallest);
        min_heapify(arr, n, smallest);
    }
}

fn build_min_heap(arr: &mut [i32]) {
    let n = arr.len();
    for i in (0..n / 2).rev() {
        min_heapify(arr, n, i);
    }
}

fn main() {
    let mut arr = [9, 5, 6, 2, 3, 8, 7, 1, 4];

    println!("Heap: {:?}", arr);
    build_min_heap(&mut arr);
    println!("Min Heap: {:?}", arr);
}
