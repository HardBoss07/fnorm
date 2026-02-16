pub fn cleanup(name: String, separators: &[String]) -> String {
    if separators.is_empty() || name.is_empty() {
        return name;
    }

    let mut result = name;

    // Use the first separator in the list as the canonical one.
    let primary_separator = &separators[0];

    // Step 1: Unify all other separators to the primary one.
    // We iterate from the second separator onwards.
    if separators.len() > 1 {
        for other_sep in &separators[1..] {
            result = result.replace(other_sep, primary_separator);
        }
    }

    // Step 2: Collapse sequences of the primary separator into one.
    let double_sep = format!("{0}{0}", primary_separator);
    while result.contains(&double_sep) {
        result = result.replace(&double_sep, primary_separator);
    }

    // Step 3: Remove leading separator.
    if result.starts_with(primary_separator) {
        result = result.chars().skip(primary_separator.len()).collect();
    }

    // Step 4: Remove trailing separator.
    if result.ends_with(primary_separator) {
        result.truncate(result.len() - primary_separator.len());
    }

    // Step 5: Remove separator before any dot.
    let sep_dot = format!("{}.", primary_separator);
    while result.contains(&sep_dot) {
        result = result.replace(&sep_dot, ".");
    }

    result
}
