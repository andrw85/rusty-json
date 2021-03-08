use std::string::ToString;

pub trait JsonValueView {
    fn to_string(&self) -> String;
}

struct JsonValue<T> {
    value: T,
}

pub trait ValueTypes {}
impl ValueTypes for u64 {}
impl ValueTypes for i64 {}
impl ValueTypes for f64 {}
impl ValueTypes for String {}
impl ValueTypes for bool {}

pub fn new<V>(v: V) -> JsonValue<V> {
    JsonValue { value: v }
}

impl<T> JsonValueView for JsonValue<T>
where
    T: ValueTypes + Default + ToString + PartialEq + Eq,
{
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}
