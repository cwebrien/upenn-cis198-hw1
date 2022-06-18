/*
    CIS198 Homework 1
    Part 1: Implementing functions

    Complete and write at least one unit test for each function you implement.
    If it already has a unit test, either add assertions to it or add a new one.
    Also answer the questions in text.
*/

// Remove these once you are done editing the file!
// This will result in useful warnings if you missed something.
#![allow(dead_code)]
#![allow(unused_variables)]

/*
    Problem 1: Double

    Implement the function that doubles an integer in three different ways.

    What are some differences between them? Can you write unit tests
    which fail (or fail to compile) for some but not others?

    Which of the three do you prefer?
*/

pub fn double_v1(n: i32) -> i32 {
    n * 2
}

pub fn double_v2(n: &i32) -> i32 {
    *n * 2
}

pub fn double_v3(n: &mut i32) {
    *n = *n * 2;
}

#[test]
fn test_double_v1() {
    assert_eq!(double_v1(2), 4);
    assert_eq!(double_v1(-3), -6);
}

#[test]
fn test_double_v2() {
    let a: i32 = 2;
    let b: i32 = -3;
    assert_eq!(double_v2(&a), 4);
    assert_eq!(double_v2(&b), -6);
}

#[test]
fn test_double_v3() {
    let mut a: i32 = 2;
    let mut b: i32 = -3;
    double_v3(&mut a);
    double_v3(&mut b);
    assert_eq!(a, 4);
    assert_eq!(b, -6);
}

/*
    Problem 2: Integer square root

    Implement the integer square root function: sqrt(n) should return the
    largest m such that m * m <= n. For a 'harder' version, try to do it more
    efficiently than trying every possibility.
*/
pub fn sqrt(n: usize) -> usize {
    for i in 1..(n + 1) {
        if i * i > n {
            return i - 1
        }
    }
    return 1
}

#[test]
fn test_sqrt() {
    assert_eq!(sqrt(1), 1);
    assert_eq!(sqrt(2), 1);

    assert_eq!(sqrt(4), 2);
    assert_eq!(sqrt(10000), 100);

    assert_eq!(sqrt(8), 2);
    assert_eq!(sqrt(10), 3);
    assert_eq!(sqrt(10001), 100);
}

// Remember to write unit tests here (and on all future functions)

/*
    Problem 3: Slice sum

    Implement the sum function on slices in two different ways
    (using different for loop patterns).
    Do not use the predefined sum function.
    Also, try to do it without an unnecessary `return` statement at the end --
    Clippy should detect if you mess this up.

    Which of the two ways do you prefer?
*/
pub fn sum_v1(nums: &[i32]) -> i32 {
    let mut result = 0;
    for &v in nums {
        result = result + v;
    }
    result
}

pub fn sum_v2(nums: &[i32]) -> i32 {
    let mut result: i32 = 0;
    // do some initialization...
    for v in nums {
        result = result + v;
    }
    result
}

#[test]
fn test_sum_v1() {
    let nums = [1, 2, 3, 5, 8];
    assert_eq!(sum_v1(&nums), 19);
}

#[test]
fn test_sum_v2() {
    let nums = [1, 2, 3, 5, 8];
    assert_eq!(sum_v2(&nums), 19);
}

/*
    Problem 4: Unique

    Make unique. Create a new vector which contains each item in the vector
    only once! Much like a set would.
    This doesn't need to be efficient; you can use a for loop.
*/

pub fn unique(nums: &[i32]) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    for n in nums {
        let mut already_contains: bool = false;
        for i in result.iter() {
            if i == n {
                already_contains = true;
                break;
            }
        };
        
        if !already_contains {
            result.push(*n);
        }
    }

    result
}

#[test]
fn test_unique() {
    let case = [1, -1, -2, -1, 5, 10, 10, 12, 15];
    let expected = [1, -1, -2, 5, 10, 12, 15];

    let unique_nums = unique(&case);
    assert_eq!(unique_nums, expected);
}

/*
    Problem 5: Filter

    Return a new vector containing only elements that satisfy `pred`.
    This uses some unfamiliar syntax for the type of pred -- all you need
    to know is that pred is a function from i32 to bool.
*/
pub fn filter(slice: &[i32], pred: impl Fn(i32) -> bool) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for n in slice {
        if pred(*n) {
            result.push(*n);
        }
    }
    result
}

#[test]
fn test_filter() {
    fn is_even(n: i32) -> bool {
        n % 2 == 0
    }
    assert_eq!(filter(&vec![1, 2, 3, 4, 5, 6], &is_even), vec![2, 4, 6]);
}

/*
    Problem 6: Fibonacci

    Given starting fibonacci numbers n1 and n2, compute a vector of
    length 'out_size'
    where v[i] is the ith fibonacci number.
*/
pub fn fibonacci(n1: i32, n2: i32, out_size: usize) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::from([n1, n2]);
    for i in 1 .. out_size - 1 {
        result.push(result[i] + result[i - 1]);
    }

    result
}

#[test]
fn test_fibonaci() {
    assert_eq!(fibonacci(1, 2, 7), vec![1, 2, 3, 5, 8, 13, 21]);
    assert_eq!(fibonacci(4, 6, 8), vec![4, 6, 10, 16, 26, 42, 68, 110]);
}

/*
    Problem 7: String concatenation

    Create a function which concats 2 &strs and returns a String,
    and a function which concats 2 Strings and returns a String.

    You may use any standard library function you wish.

    What are some reasons the second function is not efficient?
*/
pub fn str_concat(s1: &str, s2: &str) -> String {
    let result = format!("{}{}", s1, s2);
    return result;
}

#[test]
fn test_str_concat() {
    let expected = String::from("ABCdef");
    assert_eq!(str_concat("ABC", "def"), expected);
}

pub fn string_concat(s1: String, s2: String) -> String {   
    let mut result: String = String::new();
    result += &s1;
    result += &s2; 
    return result;
}

#[test]
fn test_string_concat() {
    let expected = String::from("ABCdef");
    assert_eq!(str_concat("ABC", "def"), expected);
}

/*
    Problem 8: String concatenation continued

    Convert a Vec<String> into a String.
    Your answer to the previous part may help.
*/

pub fn concat_all(v: Vec<String>) -> String {
    let mut result = String::new();
    for s in v {
        result = string_concat(result, s);
    }
    return result;
}

#[test]
fn test_concat_all() {
    let test_case = vec!["abc".to_string(), "DEF".to_string(), "g!h".to_string()];
    assert_eq!(concat_all(test_case), "abcDEFg!h");
}

/*
    Problem 9: Parsing

    Convert a Vec<String> into a Vec<i32> and vice versa.

    Assume all strings are correct numbers! We will do error handling later.
    Use `.expect("ignoring error")` to ignore Result from parse()
    See https://doc.rust-lang.org/std/primitive.str.html#method.parse

    The unit tests check if your functions are inverses of each other.

    A useful macro: format! is like println! but returns a String.
*/

pub fn parse_all(v: Vec<String>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    for s in v {
        let numeric: i32 = s.parse().expect("igoring error");
        result.push(numeric);
    }
    result
}

#[test]
fn test_parse_all() {
    let test_case: Vec<String> = vec!["123".to_string(), "-213".to_string(), "-3923".to_string()];
    let expected: Vec<i32> = vec![123, -213, -3923];
    assert_eq!(parse_all(test_case), expected);
}

pub fn print_all(v: Vec<i32>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    for i in v {
        result.push(i.to_string());
    }
    result
}

#[test]
fn test_print_parse() {
    assert_eq!(parse_all(print_all(vec![1, 2])), vec![1, 2]);
}

#[test]
fn test_parse_print() {
    let v = vec!["1".to_string(), "2".to_string()];
    assert_eq!(print_all(parse_all(v.clone())), v);
}

/*
    Problem 10: Composing functions

    Implement a function which concatenates the even Fibonacci
    numbers out of the first n Fibonacci numbers.

    For example: if n = 6, the first 5 Fibonacci numbers are 1, 1, 2, 3, 5, 8,
    so the function should return the String "28".

    Don't use a for loop! Your previous functions should be sufficient.
*/

pub fn concat_even_fibonaccis(n: usize) -> String {
    let sequence = fibonacci(1, 1, n);
    fn is_even(i: i32) -> bool {i % 2 == 0}
    let filtered = filter(&sequence, is_even);
    let filtered = print_all(filtered);
    let filtered = concat_all(filtered);
    filtered
}

#[test]
fn test_concat_even_fibonaccis() {
    assert_eq!(&concat_even_fibonaccis(6), "28");
    assert_eq!(&concat_even_fibonaccis(9), "2834");
}
