// Given an array of integers, find the pair of 
// adjacent elements that has the largest product 
// and return that product.

fn adjacentElementsProduct(inputArray: Vec<i32>) -> i32 {
    inputArray.windows(2).map(|x| x.iter().product()).max().unwrap()

}


pub fn run() {
    let array = vec![0, 5, -7, 3, 7];
    let answer = adjacentElementsProduct(array);
    println!("{}", answer);
}