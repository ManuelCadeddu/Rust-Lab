const SUBS_I: &str = "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
const SUBS_O: &str = "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";


pub fn slugify(s: &str) -> String {
    let mut s_conv = String::new();

    for c in s.chars() {
        let x = conv(c);
        if x == '-' && s_conv.ends_with('-') {
            continue;
        }
        s_conv.push(x)
    }

    if s_conv.ends_with('-') && s_conv.len() != 1 {
        s_conv.pop();
    }

    s_conv
}

fn conv(c: char) -> char {
    let subs_i: Vec<char> = SUBS_I.chars().collect();

    if let Some(index) = subs_i.iter().position(|&x| x == c.to_lowercase().nth(0).unwrap()) {
        SUBS_O.chars().nth(index).unwrap()
    } else if !c.is_alphanumeric() {
        '-'
    } else {
        c.to_lowercase().nth(0).unwrap()
    }
}
