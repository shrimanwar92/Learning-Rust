use std::mem;

fn analyze_slice(arr: &[i32]) {
	println!("first element in sliced array: {:?}", arr[0]);
	println!("Length of sliced array: {:?}", arr.len());
}

fn main() {
    // fixed size arrays
    let arr1: [i32; 5] = [1,2,3,4,5];

    // all elements can be instatiated to the same value
    let arr2: [i32; 500] = [0; 500];

    //elements in array
    println!("{:#?}", arr1[0]);
    println!("{:#?}", arr2[499]);

    // len returns size of array
    println!("Length of arr1: {:?}", arr1.len());
    println!("Length of arr2: {:?}", arr2.len());

    // memory taken by array
    println!("memory taken by arr1: {:?} bytes", mem::size_of_val(&arr1));
    println!("memory taken by arr2: {:?} bytes", mem::size_of_val(&arr2));

    // arrays can be borrowed as slices
    // Slices are similar to arrays, but their size is not known at compile time. 
    // a slice is a two-word object, the first word is a pointer to the data, 
    // and the second word is the length of the slice

    // borrow a section of array as slice
    println!("Analyzing slice by passing 9 elements from arr2");
    analyze_slice(&arr2[1..9])

}
