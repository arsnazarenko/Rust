const N: usize = 15;

fn merge(array: &mut [i32], aux: &mut [i32], l: i32, m: i32, r: i32)  {
    let mut  i: i32 = l;
    let mut j: i32 = m + 1;

    for k in l..r + 1 {
        if (i > m) {
            array[k as usize] = aux[j as usize];
            j += 1;
            continue;
        }
        if (j > r) {
            array[k as usize] = aux[i as usize];
            i += 1;
            continue;
        }
        if (aux[j as usize] < aux[i as usize]) {
            array[k as usize] = aux[j as usize];
            j += 1;
        } else {
            array[k as usize] = aux[i as usize];
            i += 1;
        }
    }

}

fn merge_sort(arr: &mut [i32], aux: &mut [i32], l: i32, r: i32) {
    if (l < r) {
        let m: i32 = (l + r) / 2;
        merge_sort(aux, arr, l, m);
        merge_sort(aux, arr, m + 1, r);
        merge(arr, aux, l, m, r);
    }
}

fn sort(array: &mut [i32]) {
    let mut aux: [i32; N] = [0; N];
    aux.clone_from_slice(array);
    merge_sort(array, &mut aux[..], 0, (N - 1) as i32);
}

pub fn main() {
    let mut array: [i32; N] = [12, 23, 3, 13, 2, 1, 11, 45, 56, 67, 34, 29, 21, 44, 1000];
    sort(&mut array[..]);
    println!("{:?}", array);
}