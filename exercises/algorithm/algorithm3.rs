/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
use std::mem::swap;
fn sort<T: std::cmp::PartialOrd>(array: &mut [T]){
	let (start,end) = (0,array.len()); // 0 1
    let (mut i,mut j) = (start,end-1); // 0 0
    let mut status = true;
    while i<j{
        if status{
            while j>i && array[j]>=array[i]{
                j -= 1;
            }
            if j>i{
                array.swap(i,j);
                status = ! status;
            }
        }
        else{
            while i<j && array[i]<=array[j]{
                i += 1;
            }
            if i<j{
                array.swap(i,j);
                status = ! status;
            }
        }
        if i != end-1{
            sort(&mut array[i+1..end]);
        }
        if i != start{
            sort(&mut array[0..i]);
        }
    }
    println!("i={i} j={j}");
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}