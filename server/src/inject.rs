pub fn it(content: &str, start: &'static str, end: &'static str, replacement: &str) -> String {
    if let Some(start_tag) = content.find(start) {
        let before: String = content
            .chars()
            .into_iter()
            .take(start_tag + start.len())
            .collect();
        let after_before: String = content.chars().into_iter().skip(before.len()).collect();
        if let Some(end_tag) = after_before.find(end) {
            let after: String = after_before.chars().into_iter().skip(end_tag).collect();
            return format!("{}{}{}", before, replacement, after);
        }
    }
    content.to_string()
}

pub fn replace(content: &str, start: &'static str, end: &'static str, replacement: &str) -> String {
    if let Some(start_tag) = content.find(start) {
        let before: String = content.chars().into_iter().take(start_tag - 1).collect();
        let after_before: String = content
            .chars()
            .into_iter()
            .skip(before.len() + start.len())
            .collect();
        if let Some(end_tag) = after_before.find(end) {
            let after: String = after_before
                .chars()
                .into_iter()
                .skip(end_tag + end.len())
                .collect();
            return format!("{}{}{}", before, replacement, after);
        }
    }
    content.to_string()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn html_tags() {
        let res = it(
            "this <main>replaces</main> a lot",
            "<main>",
            "</main>",
            "changes",
        );
        assert_eq!(res.as_str(), "this <main>changes</main> a lot");
    }

    #[test]
    fn call_params() {
        let res = it("this can(`{}`) a lot", "can(`", "`)", "change");
        assert_eq!(res.as_str(), "this can(`change`) a lot");
    }

    #[test]
    fn few_params() {
        let res = it(
            "this can(`{}`) or should(`{}`) a lot",
            "can(`",
            "`)",
            "change",
        );
        assert_eq!(res.as_str(), "this can(`change`) or should(`{}`) a lot");
    }

    #[test]
    fn it_replaces_tag() {
        let res = replace("this does <main>bad</main>a lot", "<main>", "</main>", "");
        assert_eq!(res.as_str(), "this does a lot");
    }
}