use std::cmp::Ordering;

/// Sorts the array with either quick sort or insertion sort, depending on the
/// array's size.
fn hybrid_sort<T, F>(arr: &mut [T], compare: &mut F)
    where F: FnMut(&T, &T) -> Ordering,
{
    if arr.len() <= 32 {
        insertion_sort(arr, compare);
    } else {
        quick_sort(arr, compare);
    }
}

/// Performs quick sort.
///
/// Quick sort is a sorting algorithm running in O(n * log n) average case. The
/// worst case is O(n²), though. A lot depends on the choice of the pivot
/// element. This implementation chooses the pivot in a rather stupid manner,
/// so the worst-case is only "highly unlikely" and not "extremely unlikely".
///
/// The algorithm works recursively by partitioning the array into two parts.
/// In the first, all elements have to be smaller than the pivot element, in
/// the second everything is greater than the pivot. Both parts can then be
/// sorted independent of each other. After the array has been partitioned it
/// looks somewhat like this:
///
/// ```
///              ▇ ▂   ▆
///    ▃ ▅ ▃ ▅ ▁ █ █ ▇ █ ▆
///
///    \       / \       /
///      less      greater
/// ```
fn quick_sort<T, F>(arr: &mut [T], compare: &mut F)
    where F: FnMut(&T, &T) -> Ordering,
{
    // If the array only contains 0 or 1 element, we can stop, since it's
    // already sorted.
    if arr.len() <= 1 {
        return;
    }

    // Choose a pivot element. This is a crucial part in getting the
    // performance right. This implementation just picks the element in the
    // middle. This is usually not the best idea! We will run into an O(n²)
    // worst case, if the pivot element is bad for partitioning.
    //
    // We reference the pivot element by index. Otherwise we would need to
    // require `T: Clone` or use unsafe hackery. Always using the index is
    // not the best solution regarding cache efficiency, but it works for us.
    // The best solution would require special care, because of Ownership and
    // Drop. We'll learn more about this later.
    let mut pivot_pos = arr.len() / 2;

    // Partition the array by maintaining two indices. Afterwards we want to
    // have two parts. In the first, all elements have to be smaller than the
    // pivot element, in the second everything is greater than the pivot.
    //
    // Note: This could be done by `Iterator::partition()`, but we can do it
    // without any additional memory requirement.
    let (mut i, mut j) = (0, arr.len() - 1);
    while i <= j {
        while compare(&arr[i], &arr[pivot_pos]) == Ordering::Less {
            i += 1;
        }
        while compare(&arr[j], &arr[pivot_pos]) == Ordering::Greater {
            j -= 1;
        }

        if i <= j {
            // Now we found two elements that can be swapped!
            arr.swap(i, j);

            // Check if we just swapped the pivot element and fix the index
            // accordingly.
            if i == pivot_pos {
                pivot_pos = j;
            } else if j == pivot_pos {
                pivot_pos = i;
            }

            i += 1;
            j -= 1;
        }
    }

    // The partitioning is done, i and j tell us the bounds of both parts. We
    // call `hybrid_sort()` to allow for algorithm dispatching. We want to sort
    // small arrays with another algoritm than big arrays.
    hybrid_sort(&mut arr[..j + 1], compare);
    hybrid_sort(&mut arr[i..], compare);
}

/// Performs insertion sort.
///
/// Insertion sort runs in O(n²) worst case, but is extremly fast on nearly
/// sorted or small inputs. It's commonly used to handle the small sub-arrays
/// of recursive algorithms such as quick sort or merge sort.
///
/// Insertion sort splits the array in two parts: a sorted first part and an
/// unsorted second part. In each iteration the first element of the unsorted
/// part is inserted at the correct position into the sorted part. The array
/// may look like this during the algorithm:
///
/// ```
///          ▂ ▆     ▇
///    ▃ ▆ ▇ █ █ ▅ ▁ █ ▅ ▃
///
///    \       / \       /
///     sorted    unsorted
/// ```
///
/// # Performance
///
/// To insert the next element at the correct position in the sorted part, we
/// have multiple possibilities:
///
/// 1. Always swap the element with the element left to it, if the former is
///    smaller. This is what this implementation does. It's fairly easy but
///    we do quite a few unnecessary writes. This method is *slow*!
///
/// 2. We can first find the correct position to insert by searching through
///    all elements first, then shift all elements to the right and insert the
///    new element. This is way better than the first method.
///
/// 3. Like (2.) but we already shift all elements to the right while searching
///    for the right insertion-position. This is the best method.
///
/// Sadly, (2.) and (3.) require some unsafe hackery. Why? Ownership and Drop!
///
/// We don't know what `Drop` is, yet, but we know that each resource should
/// have one owner. We *can't* move out of an array, so we can't really get
/// out the new element to do the shift with the rest. We will learn how to
/// theoretically optimize this method later.
///
fn insertion_sort<T, F>(arr: &mut [T], compare: &mut F)
    where F: FnMut(&T, &T) -> Ordering
{
    for mut new in 0..arr.len() {
        while new > 0 && compare(&arr[new - 1], &arr[new]) == Ordering::Greater {
            arr.swap(new, new - 1);
            new -= 1;
        }
    }
}

/// Sorts the given slice with the given comparator, which determines the
/// relation between two elements.
pub fn sort_by<T, F>(arr: &mut [T], mut f: F)
    where F: FnMut(&T, &T) -> Ordering,
{
    hybrid_sort(arr, &mut f);
}

/// Sorts the given slice by comparing the keys for each element according to
/// the given closure.
pub fn sort_by_key<T, F, K>(arr: &mut [T], mut map: F)
    where F: FnMut(&T) -> K,
          K: Ord,
{
    hybrid_sort(arr, &mut |a, b| map(a).cmp(&map(b)))
}

/// Sorts the array in natural order (with the `Ord::cmp` method).
pub fn sort<T: Ord>(arr: &mut [T]) {
    hybrid_sort(arr, &mut Ord::cmp);
}

fn main() {
    let mut arr = [
        90, 39, 49, 18, 43, 17, 38, 76, 24, 74, 56, 19, 32, 54, 33, 24, 47, 75,
        68, 22, 70, 58, 72, 89, 14, 69, 61, 84, 45, 44, 7, 96, 95, 22, 83, 93,
        14, 97, 17, 86, 92, 81, 79, 32, 30, 82, 8, 40, 7, 24, 87, 38, 27, 70,
        9, 18, 44, 89, 86, 17, 60, 93, 43, 39, 74, 42, 38, 40, 13, 29, 87, 95,
        73, 3, 75, 97, 87, 2, 96, 66, 37, 1, 73, 23, 43, 93, 20, 51, 40, 44,
        39, 13, 56, 38, 16, 90, 5,
    ];
    // let mut arr = [90, 39, 49, 18, 43, 17, 38, 76, 24, 74, 56, 19, 54, 33];

    println!("Before: {:?}", &arr[..]);
    println!("After: {:?}", &arr[..]);
}

#[test]
fn sort_short() {
    let mut a: [i32; 0] = [];
    let mut b = [27];

    sort(&mut a);
    sort(&mut b);
    assert_eq!(a, []);
    assert_eq!(b, [27]);

    sort_by(&mut a, |a, b| a.cmp(b));
    sort_by(&mut b, |a, b| a.cmp(b));
    assert_eq!(a, []);
    assert_eq!(b, [27]);

    sort_by_key(&mut a, |x| *x);
    sort_by_key(&mut b, |x| *x);
    assert_eq!(a, []);
    assert_eq!(b, [27]);
}

#[test]
fn sort_medium_long() {
    let arr = [-30, 31, 22, 21, 59, -64, -77, 98, -58, 55, -41, 49];
    let mut a = arr.to_vec();
    let mut b = arr.to_vec();
    let mut c = arr.to_vec();

    sort(&mut a);
    assert_eq!(a, &[-77, -64, -58, -41, -30, 21, 22, 31, 49, 55, 59, 98]);

    sort_by(&mut b, |a, b| b.cmp(a));  // reverse order
    assert_eq!(b, &[98, 59, 55, 49, 31, 22, 21, -30, -41, -58, -64, -77]);

    sort_by_key(&mut c, |x: &i64| x.abs());
    assert_eq!(c, &[21, 22, -30, 31, -41, 49, 55, -58, 59, -64, -77, 98]);
}

#[test]
fn sort_long() {
    let arr = [
        -27, -62, -21, 1, 0, 6, 17, -14, -98, -49, 8, 67, -82, -65, -53, 56,
        32, 0, 59, -12, 39, 15, 90, -80, 40, -63, 65, -69, -74, 56, 91, 63,
        -61, 68, -95, 17, -81, 35, 7, 77, 28, 76, 16, -81, -53, -25, -45, -27,
        60, -81, 86, 95, 85, -32, -53, -53, -98, 50, -92, 66, 44, 85, -52, 54,
        48, -14, 92, -80, -59, 20, 61, -36, -76, -34, 20, 76, 87, -38, -61, 12,
        -54, 6, 26, 48, 81, 98, -30, 31, 22, 21, 59, -64, -77, 98, 49, -58,
        -27, -44, 49, 55, -41, 49, -78, 50, -38, -8, -68, -31, 64, -52, 22, 0,
        58, -88, 90, -19, 42, 3, 54, -68, -24, 73, 0, 11, 77, -75, -48, -51,
        -52, 29, 12, 59, 50, -14, -70, 29, -18, 92, 0, -88, -73, -25, -32, -39,
        67, 18, -87, 93, 24, -80, 34, 92, 34, 60, 68, -79, 60, 72, 17, 82, -39,
        47, -83, -44, -57, 91, 33, 93, 41, 5, -26, 59, -25, 57, -39, -49, 84,
        -44, -39, 97, 12, -12, 85, 83, -11, -59, -60, -36, 98, -46, -18, 64,
        -88, -67, -24, 42, 39, 61, -63, 89, 39, 72, -12, 54, 87, 33, 92, -56,
        -6, -76, 73, -63, -58, 28, 86, -49, -11, 78, 38, 39, 26, -13, -73, 44,
        -46, -78, -61, -62, 19, 27, 96, 22, -86, -4, -1, -33, 74, 91, 83, 23,
        -40, -29, 90, 7, 30, -27, -90, 65, 79, 47, 45, 19, -27, 20, 11, -73,
        -84, -27, 58, -88, 94, -58, 56, -93, 86, 22, -43, 1, -21, -20, -64,
        -18, 54, -97, -63, -32, 57, -24, 29, 27, -53, -86, 17, -7, -78, -36,
        65, -9, 52, -86, 62, 21, 82, 10, -53, 80, 2, -87, -85, -51, 2, 45, 42,
        -85, 17, -63, -76, -46, 26, 55, 94, 80, -53, -26, -73, -43, 72, -45,
        14, -83, -69, -3, -77, 96, -95, -15, 4, -12, -54, 99, -94, 88, -56,
        -11, -36, -72, -91, -42, 95, 41, 23, 41, 44, -49, 93, 91, 22, 38, 96,
        7, -63, 55, -74, -77, -50, -90, 78, -56, -2, 30, 79, -42, -12, -46,
        -28, -73, -28, 61, -13, 4, -15, -96, -62, 93, -91, 41, -79, 1, 65, -49,
        35, -24, 88, 60, 24, -36, 27, 85, 80, -80, -65, 87, 14, 18, 79, 44, 39,
        -91, 81, -89, 70, 27, 49, -65, -33, 93, 60, -65, 85, 57, -62, -72, -90,
        -42, -62, 69, 40, 86, -43, -81, -52, 66, -5, -96, -47, -92, 36, 38,
        -21, 62, -35, 27, 65, 87, 68, -33, 0, -13, 89, -55, -95, 80, -14, 13,
        71, -95, -52, -60, -46, 14, 26, -89, -77, -52, -49, -53, -95, 60, 2,
        45, -98, -19, 49, -24, 1, -63, -51, 24, 83, 60, -92, 8, 17, -55, -64,
        42, 12, 3, 86, 78, 28, -36, 6, 55, -90, -33, -84, 34, 29, 47, 57, -80,
        -24, -31, -95, -8, -8, 21, -34, -68,
    ];

    let mut a = arr.to_vec();
    let mut b = arr.to_vec();
    let mut c = arr.to_vec();

    // a.sort();
    sort(&mut a);
    assert_eq!(a, &[
        -98i64, -98, -98, -97, -96, -96, -95, -95, -95, -95, -95, -95, -94,
        -93, -92, -92, -92, -91, -91, -91, -90, -90, -90, -90, -89, -89, -88,
        -88, -88, -88, -87, -87, -86, -86, -86, -85, -85, -84, -84, -83, -83,
        -82, -81, -81, -81, -81, -80, -80, -80, -80, -80, -79, -79, -78, -78,
        -78, -77, -77, -77, -77, -76, -76, -76, -75, -74, -74, -73, -73, -73,
        -73, -73, -72, -72, -70, -69, -69, -68, -68, -68, -67, -65, -65, -65,
        -65, -64, -64, -64, -63, -63, -63, -63, -63, -63, -63, -62, -62, -62,
        -62, -62, -61, -61, -61, -60, -60, -59, -59, -58, -58, -58, -57, -56,
        -56, -56, -55, -55, -54, -54, -53, -53, -53, -53, -53, -53, -53, -53,
        -52, -52, -52, -52, -52, -52, -51, -51, -51, -50, -49, -49, -49, -49,
        -49, -49, -48, -47, -46, -46, -46, -46, -46, -45, -45, -44, -44, -44,
        -43, -43, -43, -42, -42, -42, -41, -40, -39, -39, -39, -39, -38, -38,
         -36, -36, -36, -36, -36, -36, -35, -34, -34, -33, -33, -33, -33, -32,
         -32, -32, -31, -31, -30, -29, -28, -28, -27, -27, -27, -27, -27, -27,
         -26, -26, -25, -25, -25, -24, -24, -24, -24, -24, -24, -21, -21, -21,
         -20, -19, -19, -18, -18, -18, -15, -15, -14, -14, -14, -14, -13, -13,
         -13, -12, -12, -12, -12, -12, -11, -11, -11, -9, -8, -8, -8, -7, -6,
         -5, -4, -3, -2, -1, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 3, 3, 4, 4,
         5, 6, 6, 6, 7, 7, 7, 8, 8, 10, 11, 11, 12, 12, 12, 12, 13, 14, 14, 14,
         15, 16, 17, 17, 17, 17, 17, 17, 18, 18, 19, 19, 20, 20, 20, 21, 21,
         21, 22, 22, 22, 22, 22, 23, 23, 24, 24, 24, 26, 26, 26, 26, 27, 27,
         27, 27, 27, 28, 28, 28, 29, 29, 29, 29, 30, 30, 31, 32, 33, 33, 34,
         34, 34, 35, 35, 36, 38, 38, 38, 39, 39, 39, 39, 39, 40, 40, 41, 41,
         41, 41, 42, 42, 42, 42, 44, 44, 44, 44, 45, 45, 45, 47, 47, 47, 48,
         48, 49, 49, 49, 49, 49, 50, 50, 50, 52, 54, 54, 54, 54, 55, 55, 55,
         55, 56, 56, 56, 57, 57, 57, 57, 58, 58, 59, 59, 59, 59, 60, 60, 60,
         60, 60, 60, 60, 61, 61, 61, 62, 62, 63, 64, 64, 65, 65, 65, 65, 65,
         66, 66, 67, 67, 68, 68, 68, 69, 70, 71, 72, 72, 72, 73, 73, 74, 76,
         76, 77, 77, 78, 78, 78, 79, 79, 79, 80, 80, 80, 80, 81, 81, 82, 82,
         83, 83, 83, 84, 85, 85, 85, 85, 85, 86, 86, 86, 86, 86, 87, 87, 87,
         87, 88, 88, 89, 89, 90, 90, 90, 91, 91, 91, 91, 92, 92, 92, 92, 93,
         93, 93, 93, 93, 94, 94, 95, 95, 96, 96, 96, 97, 98, 98, 98, 99,
    ] as &[_]);

    // b.sort_by(|a, b| b.cmp(a));
    sort_by(&mut b, |a, b| b.cmp(a));  // reverse order
    assert_eq!(b, &[
        99i64, 98, 98, 98, 97, 96, 96, 96, 95, 95, 94, 94, 93, 93, 93, 93, 93,
        92, 92, 92, 92, 91, 91, 91, 91, 90, 90, 90, 89, 89, 88, 88, 87, 87, 87,
        87, 86, 86, 86, 86, 86, 85, 85, 85, 85, 85, 84, 83, 83, 83, 82, 82, 81,
        81, 80, 80, 80, 80, 79, 79, 79, 78, 78, 78, 77, 77, 76, 76, 74, 73, 73,
        72, 72, 72, 71, 70, 69, 68, 68, 68, 67, 67, 66, 66, 65, 65, 65, 65, 65,
        64, 64, 63, 62, 62, 61, 61, 61, 60, 60, 60, 60, 60, 60, 60, 59, 59, 59,
        59, 58, 58, 57, 57, 57, 57, 56, 56, 56, 55, 55, 55, 55, 54, 54, 54, 54,
        52, 50, 50, 50, 49, 49, 49, 49, 49, 48, 48, 47, 47, 47, 45, 45, 45, 44,
        44, 44, 44, 42, 42, 42, 42, 41, 41, 41, 41, 40, 40, 39, 39, 39, 39, 39,
        38, 38, 38, 36, 35, 35, 34, 34, 34, 33, 33, 32, 31, 30, 30, 29, 29, 29,
        29, 28, 28, 28, 27, 27, 27, 27, 27, 26, 26, 26, 26, 24, 24, 24, 23, 23,
        22, 22, 22, 22, 22, 21, 21, 21, 20, 20, 20, 19, 19, 18, 18, 17, 17, 17,
        17, 17, 17, 16, 15, 14, 14, 14, 13, 12, 12, 12, 12, 11, 11, 10, 8, 8,
        7, 7, 7, 6, 6, 6, 5, 4, 4, 3, 3, 2, 2, 2, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0,
        -1, -2, -3, -4, -5, -6, -7, -8, -8, -8, -9, -11, -11, -11, -12, -12,
        -12, -12, -12, -13, -13, -13, -14, -14, -14, -14, -15, -15, -18, -18,
        -18, -19, -19, -20, -21, -21, -21, -24, -24, -24, -24, -24, -24, -25,
        -25, -25, -26, -26, -27, -27, -27, -27, -27, -27, -28, -28, -29, -30,
        -31, -31, -32, -32, -32, -33, -33, -33, -33, -34, -34, -35, -36, -36,
        -36, -36, -36, -36, -38, -38, -39, -39, -39, -39, -40, -41, -42, -42,
        -42, -43, -43, -43, -44, -44, -44, -45, -45, -46, -46, -46, -46, -46,
        -47, -48, -49, -49, -49, -49, -49, -49, -50, -51, -51, -51, -52, -52,
        -52, -52, -52, -52, -53, -53, -53, -53, -53, -53, -53, -53, -54, -54,
        -55, -55, -56, -56, -56, -57, -58, -58, -58, -59, -59, -60, -60, -61,
        -61, -61, -62, -62, -62, -62, -62, -63, -63, -63, -63, -63, -63, -63,
        -64, -64, -64, -65, -65, -65, -65, -67, -68, -68, -68, -69, -69, -70,
        -72, -72, -73, -73, -73, -73, -73, -74, -74, -75, -76, -76, -76, -77,
        -77, -77, -77, -78, -78, -78, -79, -79, -80, -80, -80, -80, -80, -81,
        -81, -81, -81, -82, -83, -83, -84, -84, -85, -85, -86, -86, -86, -87,
        -87, -88, -88, -88, -88, -89, -89, -90, -90, -90, -90, -91, -91, -91,
        -92, -92, -92, -93, -94, -95, -95, -95, -95, -95, -95, -96, -96, -97,
        -98, -98, -98
    ] as &[_]);

    // c.sort_by_key(|x: &i64| x.abs());
    sort_by_key(&mut c, |x: &i64| x.abs());
    assert!(
        c.iter()
            .zip(&c[1..])
            .all(|(&a, &b)| a.abs() <= b.abs())
    );
}
