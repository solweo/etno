pub fn enhance(tacos: bool) -> &'static str {
    if tacos { "Enhanced Tacos, No Onions" }
    else { "only tacos!" }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = enhance(true);
        assert_eq!(result, "Enhanced Tacos, No Onions");
    }
}
