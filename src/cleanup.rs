pub fn cleanup(name: String, separators: &[String]) -> String {
    let mut result = name;

    for sep in separators {
        let pattern = format!("{}.", sep);
        result = result.replace(&pattern, ".");
    }

    result
}
