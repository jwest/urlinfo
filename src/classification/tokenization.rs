use regex::Regex;

pub fn tokenize(input: String) -> String {
    let re = Regex::new(r"[|\s,-_/?=&#:’`]+").expect("reg err");
    let mut fields: Vec<&str> = re.split(&input).collect();
    fields.retain(|&x| x != "http" && x != "https" && x != "www" && x != "com");
    return fields.join(" ");
}
