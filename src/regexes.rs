use std::collections::HashMap;
use lazy_static::lazy_static;
use fancy_regex::Regex;

lazy_static!{
    pub static ref REGEXES: HashMap<String, (Regex, Vec<String>)> = {
        let mut res = HashMap::new();
        macro_rules! regex {
            ($name: ident: $regex: literal $($alias: ident)*) => {
                res.insert(
                    stringify!($name).to_string(),
                    (
                        Regex::new($regex).expect("should compile"),
                        vec![$(stringify!($alias).to_string()),*]
                    )
                )
            };
        }

        regex!(integer: r"[0-9]+" int ints i);
        regex!(float: r"([0-9]*[.])?[0-9]+" double);
        regex!(number: r"([0-9]*[.])?[0-9]+" num);
        regex!(telephone: r"[+]*[(]{0,1}[0-9]{1,4}[)]{0,1}[-\s\./0-9]*" phone);
        regex!(email: r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"# mail);

        res
    };
}