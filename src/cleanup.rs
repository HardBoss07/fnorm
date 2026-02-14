pub fn cleanup(name: String, separators: &[String]) -> String {
    let mut result = name;

    // remove double separators like "__", "--"
    for sep in separators {
        let double = format!("{0}{0}", sep);
        while result.contains(&double) {
            result = result.replace(&double, sep);
        }
    }

    // remove separator before dot
    for sep in separators {
        let pattern = format!("{}.", sep);
        result = result.replace(&pattern, ".");
    }

    // remove trailing separators
    for sep in separators {
        while result.ends_with(sep) {
            result.pop();
        }
    }

    result
}
