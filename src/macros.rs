macro_rules! variable {
    ($name:expr) => { $crate::common::Variable::from_str($name).unwrap() }
}
