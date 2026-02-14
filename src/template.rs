use std::path::Path;

pub fn render(
    template: &str,
    parent: &str,
    n: usize,
    ext: &str,
    dims: Option<(u32, u32)>,
) -> String {
    let mut result = template.to_string();

    result = result.replace("{parent}", &parent.to_lowercase());
    result = result.replace("{PARENT}", &parent.to_uppercase());
    result = result.replace("{N}", &n.to_string());
    result = result.replace("{ext}", ext);

    if let Some((w, h)) = dims {
        result = result.replace("{width}", &w.to_string());
        result = result.replace("{height}", &h.to_string());
    } else {
        result = result.replace("{width}", "");
        result = result.replace("{height}", "");
    }

    result
}
