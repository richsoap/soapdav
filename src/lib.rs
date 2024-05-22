mod label;
pub mod adapter;
mod core;


fn double(v: &u32) -> u32 {
    return v * 2;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_double() {
        let v = 4;
        assert_eq!(double(&v), 8);
    }
}