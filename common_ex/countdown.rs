pub fn countdown(n: u32) -> Vec<u32> {
    let mut elements: Vec<u32> = vec![];
    let mut i = n;
    while true {
        elements.push(i);
        if i == 0 { break; }
        i -= 1;
    }
    elements
}

pub fn main() {
    println!("{:?}", countdown(5));
    println!("{:?}", countdown(10));
    println!("{:?}", countdown(0));
    println!("{:?}", countdown(1));
    println!("{:?}", countdown(2));
    println!("{:?}", countdown(3));
    println!("{:?}", countdown(4));
    println!("{:?}", countdown(5));
}