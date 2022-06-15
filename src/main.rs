fn main() {
    let a = 1;
    let b = 1;
    println!("I can add: {} + {} = {}", a,b,add(1,1));
}

pub fn add(a: i32, b:i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}