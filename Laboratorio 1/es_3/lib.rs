pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result = Vec::new();
    for (i, row) in minefield.iter().enumerate() {
        let mut result_row = String::new();
        for (j, ch) in row.chars().enumerate() {
            if ch == '*' {
                result_row.push('*');
            } else {
                let mut count = 0;
                for x in i.saturating_sub(1)..=i + 1 {
                    for y in j.saturating_sub(1)..=j + 1 {
                        if x < minefield.len() && y < row.len() && minefield[x].chars().nth(y) == Some('*') {
                            count += 1;
                        }
                    }
                }
                if count > 0 {
                    result_row.push_str(&count.to_string());
                } else {
                    result_row.push(' ');
                }
            }
        }
        result.push(result_row);
    }
    result
}
