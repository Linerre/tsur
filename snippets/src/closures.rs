// Closures prefer to capture by sharing references (borrow)
// The below fn is rather a ref mess
pub fn capture_by_borrow1<'a>(lst: Vec<&'a u8>) -> Vec<&'a u8> { // <-- the fn expects to return T
    let new_lst: Vec<_> = lst.iter() // creates an iterator over refs to u8s in lst
        .filter(|num| num > 0) // <-- but the closure captures by ref: num is of type &&u8
        .collect();
    new_lst
}

// To correct it
pub fn capture_by_borrow2(lst: Vec<u8>) -> Vec<u8> { // <-- the fn expects to return T
    let new_lst: Vec<_> = lst.iter() // creates an iterator over &u8 in lst (1st &)
        .filter(|num| **num > 0u8) // <-- but closure captures by ref: &(&u8), (2nd &), thus two **
        .copied()                  // copy the value pointed by &u8; Without this copied() method,
        .collect();                // in filter(), `&' should be used to create a tmp val (rvalue)
    new_lst
}

#[cfg(test)]
mod tests {
    use super::capture_by_borrow;

    #[test]
    fn test1() {
        let v1 = vec![3u8, 2u8, 1u8];
        let v2 = capture_by_borrow(v1);

        assert_eq!(&"me", v2[0]);
    }
}
