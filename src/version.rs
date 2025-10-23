pub fn extract_version_from_name(name: &str) -> Option<String> {
    if !name.contains("google-cloud-cli-") {
        return None;
    }

    let chars = name.chars().peekable();
    let mut current_token = String::new();
    let mut in_version = false;
    let mut dot_count = 0;

    for ch in chars {
        if ch.is_ascii_digit() {
            current_token.push(ch);
            in_version = true;
        } else if ch == '.' && in_version {
            current_token.push(ch);
            dot_count += 1;
        } else {
            if in_version && dot_count >= 2 && !current_token.is_empty() {
                let parts: Vec<&str> = current_token.split('.').collect();
                if parts.len() >= 3
                    && parts
                        .iter()
                        .all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
                {
                    return Some(current_token);
                }
            }
            current_token.clear();
            in_version = false;
            dot_count = 0;
        }
    }

    if in_version && dot_count >= 2 && !current_token.is_empty() {
        let parts: Vec<&str> = current_token.split('.').collect();
        if parts.len() >= 3
            && parts
                .iter()
                .all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
        {
            return Some(current_token);
        }
    }

    None
}
