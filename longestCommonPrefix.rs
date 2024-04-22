fn longest_common_prefix(strs: &[String]) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let mut ans = String::new();
    let n = strs[0].len();

    let mut i = 0;
    while i < n {
        let c = match strs[0].chars().nth(i) {
            Some(ch) => ch,
            None => break,
        };

        for s in strs.iter() {
            if s.chars().nth(i) != Some(c) {
                return ans;
            }
        }
        ans.push(c);
        i += 1;
    }
    ans
}
