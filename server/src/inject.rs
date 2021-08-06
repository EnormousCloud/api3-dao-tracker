pub fn it(content: &str, start: &'static str, end: &'static str, replacement: &str) -> String {
    if let Some(start_tag) = content.find(start) {
        let before: Vec<u8> = content.bytes().take(start_tag + start.len()).collect();
        let before_str = match std::str::from_utf8(before.as_ref()) {
            Ok(x) => x,
            Err(_) => return content.to_string(),
        };
        let after_before: Vec<u8> = content.bytes().skip(before.len()).collect();
        let after_before_str: String = match std::str::from_utf8(after_before.as_ref()) {
            Ok(x) => x.to_string(),
            Err(_) => return content.to_string(),
        };

        if let Some(end_tag) = after_before_str.find(end) {
            let after: Vec<u8> = after_before.iter().skip(end_tag).map(|ch| *ch).collect();
            let after_str: String = match std::str::from_utf8(after.as_ref()) {
                Ok(x) => x.to_string(),
                Err(_) => return content.to_string(),
            };
            return format!("{}{}{}", before_str, replacement, after_str);
        }
    }
    content.to_string()
}

pub fn replace(content: &str, start: &'static str, end: &'static str, replacement: &str) -> String {
    if let Some(start_tag) = content.find(start) {
        let before: Vec<u8> = content.bytes().take(start_tag).collect();
        let before_str = match std::str::from_utf8(before.as_ref()) {
            Ok(x) => x,
            Err(_) => return content.to_string(),
        };
        let after_before: Vec<u8> = content.bytes().skip(before.len() + start.len()).collect();
        let after_before_str: String = match std::str::from_utf8(after_before.as_ref()) {
            Ok(x) => x.to_string(),
            Err(_) => return content.to_string(),
        };
        if let Some(end_tag) = after_before_str.find(end) {
            let after: Vec<u8> = after_before
                .iter()
                .skip(end_tag + end.len())
                .map(|ch| *ch)
                .collect();
            let after_str: String = match std::str::from_utf8(after.as_ref()) {
                Ok(x) => x.to_string(),
                Err(_) => return content.to_string(),
            };
            return format!("{}{}{}", before_str, replacement, after_str);
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
            "this © can(`{}`) ©r should(`{}`) a © lot",
            "can(`",
            "`)",
            "change",
        );
        assert_eq!(res.as_str(), "this © can(`change`) ©r should(`{}`) a © lot");
    }

    #[test]
    fn it_replaces_tag() {
        let res = replace(
            "this © does <main>b©d</main>a © lot",
            "<main>",
            "</main>",
            "",
        );
        assert_eq!(res.as_str(), "this © does a © lot");
    }
}
