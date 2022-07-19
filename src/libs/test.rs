use serde_json::Value;

pub trait TestObject {
    fn call(&mut self, method: &str, args: &[Value]) -> Option<Value>;
}

fn into_array(value: Value) -> Vec<Value> {
    match value {
        Value::Array(a) => a,
        _ => unreachable!(),
    }
}

pub fn assert_object<O: TestObject>(mut obj: O, methods: Value, params: Value, excepts: Value) {
    let iter = into_array(methods)
        .into_iter()
        .map(|v| match v {
            Value::String(s) => s,
            _ => unreachable!(),
        })
        .zip(into_array(params).into_iter().map(into_array))
        .zip(into_array(excepts).into_iter().map(|v| match v {
            Value::String(s) => s,
            _ => unreachable!(),
        }));

    for (i, bb) in iter.enumerate() {
        let ((m, p), e) = bb;
        let now = std::time::Instant::now();
        let result = obj.call(&m, &p);
        println!(
            "{}. call {} {:?} --> {:?}, used {:?}",
            i,
            m,
            p,
            result,
            now.elapsed()
        );
        if let Some(r) = result {
            assert_eq!(r, e);
        }
    }
}
