pub trait TestObject {
    fn call(&mut self, method: &str, args: &[serde_json::Value]) -> Option<serde_json::Value>;
}

pub fn assert_object<O: TestObject>(mut obj: O, methods: &str, params: &str, excepts: &str) {
    let methods = serde_json::from_str::<Vec<String>>(methods).unwrap();
    let params = serde_json::from_str::<Vec<Vec<serde_json::Value>>>(params).unwrap();
    let excepts = serde_json::from_str::<Vec<serde_json::Value>>(excepts).unwrap();

    for (i, bb) in methods.into_iter().zip(params).zip(excepts).enumerate() {
        let ((m, p), e) = bb;
        let now = std::time::Instant::now();
        let result = obj.call(m.as_str(), &p);
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
