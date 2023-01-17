// The last else branch will error if a &&str, other than &str, is passed to it!
fn kind_to_order(card: &str) -> u8 {
    let (_, kind) = card.split_at(1);
    if kind == "a" { 14 }
    else if kind == "k" { 13 }
    else if kind == "q" { 12 }
    else if kind == "j" { 11 }
    else if kind == "t" { 10 }
    else { kind.parse::<u8>().unwrap() } // 2-9
}

fn comp_ref<T: PartialOrd>(num1: T, num2: T) -> bool {
    // Even works for &&, say, &&u8; thus NO need for **num1 > **num2
    num1 > num2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comp_ref() {
        let num1: u8 = 10;
        let num2: u8 = 9;

        assert!(comp_ref(num1, num2));
        assert!(comp_ref(&num1, &num2));
        assert!(comp_ref(&&num1, &&num2)); // Note the &&u8 is also a kind of generic T!
    }
}
