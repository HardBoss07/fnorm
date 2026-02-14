pub fn render(
    template: &str,
    parent: &str,
    n: usize,
    ext: &str,
    dims: Option<(u32, u32)>,
) -> String {
    let mut result = String::new();
    let mut chars = template.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '{' {
            let mut key = String::new();

            while let Some(&next) = chars.peek() {
                chars.next();
                if next == '}' {
                    break;
                }
                key.push(next);
            }

            let replacement = match key.as_str() {
                "parent" => parent.to_lowercase(),
                "PARENT" => parent.to_uppercase(),
                "N" => n.to_string(),
                "ext" => ext.to_string(),
                "width" => dims.map(|d| d.0.to_string()).unwrap_or_default(),
                "height" => dims.map(|d| d.1.to_string()).unwrap_or_default(),
                _ => format!("{{{}}}", key),
            };

            result.push_str(&replacement);
        } else {
            result.push(c);
        }
    }

    result
}
