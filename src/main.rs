mod counting_sort;

fn main() {
    let v = vec![1723, 456, 76, 85, 431, 31, 904];
    let v2 = crate::counting_sort::sort(&v);

    println!("Yo: {:?}", v2);
}
