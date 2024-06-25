pub(crate) mod domain_id;
pub trait ValueObject {
    type Value: ?Sized;
    fn get_value(&self) -> &Self::Value;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestValue(String);

    impl ValueObject for TestValue {
        type Value = String;

        fn get_value(&self) -> &Self::Value {
            &self.0
        }
    }

    #[test]
    fn test_trait_when_get_value_is_called_should_return_value() {
        let expected = "test";
        let test_value = TestValue(expected.to_string());
        assert_eq!(expected, test_value.get_value());
    }
}
