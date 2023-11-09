const SUBS_I: &str = "àáâäæãåāăąçćčđďèéêëēėęěğǵḧîïíīįìıİłḿñńǹňôöòóœøōõőṕŕřßśšşșťțûüùúūǘůűųẃẍÿýžźż";
const SUBS_O: &str = "aaaaaaaaaacccddeeeeeeeegghiiiiiiiilmnnnnoooooooooprrsssssttuuuuuuuuuwxyyzzz";

pub fn slugify(s: &str) -> String {

    let mut s_conv = String::new();

    for c in s.chars() {

        // Character conversion
        let x = conv(c);

        // There can be no more '-' neighbors in the output
        if x == '-' && s_conv.ends_with('-') {
            continue;
        }

        s_conv.push(x)
    }

    // The output can't ends with '-' unless it is the ony character in the string
    if s_conv.ends_with('-') && s_conv.len() != 1 {
        s_conv.pop();
    }

    s_conv
}

fn conv(c: char) -> char {

    let subs_i: Vec<char> = SUBS_I.chars().collect();

    // If there is a match in SUB_I, take the rispective char in SUB_O
    if let Some(index) = subs_i.iter().position(|&x| x == c.to_lowercase().nth(0).unwrap()) {
        SUBS_O.chars().nth(index).unwrap()

        // If the character isn't alphanumeric replace it with '-'
    } else if !c.is_alphanumeric() {
        '-'

        // In other cases get the char in lowercase format
    } else {
        c.to_lowercase().nth(0).unwrap()
    }
}