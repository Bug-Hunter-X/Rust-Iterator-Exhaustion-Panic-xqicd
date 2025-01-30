fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let mut iter = vec.iter();

    // Safe way to handle potential exhaustion
    if let Some(first) = iter.next() {
        if let Some(second) = iter.next() {
            if let Some(third) = iter.next() {
                println!("First: {}", first);
                println!("Second: {}", second);
                println!("Third: {}", third);
            }
        }
    }
    // Or using a loop 
    for (i, item) in vec.iter().enumerate(){
        println!("Value at index {}: {}", i, item);
    }
} 