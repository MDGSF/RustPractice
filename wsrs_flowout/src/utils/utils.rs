pub fn is_string_wildcard_match_array(s: &str, array: &[&str]) -> bool {
    for item in array {
        if wildcard_match(item, s) {
            return true;
        }
    }
    false
}

pub fn wildcard_match(pattern: &str, name: &str) -> bool {
    if pattern.len() == 0 {
        return pattern == name;
    }
    if pattern == "*" {
        return true;
    }
    let rpattern = String::from(pattern);
    let rname = String::from(name);
    wildcard_deep_match(rpattern.as_bytes(), rname.as_bytes(), false)
}

pub fn wildcard_deep_match(mut pattern: &[u8], mut name: &[u8], plus: bool) -> bool {
    while pattern.len() > 0 {
        match pattern[0] {
            b'?' => {
                /*
                 * '?' 表示 0 个或者 1 个字符
                 */

                return wildcard_deep_match(&pattern[1..], &name, plus)
                    || (name.len() > 0 && wildcard_deep_match(&pattern[1..], &name[1..], plus));
            }
            b'*' => {
                /*
                 * '*' 表示 0 个，1 个字符或者任意个字符
                 */

                return wildcard_deep_match(&pattern[1..], &name, plus)
                    || (name.len() > 0 && wildcard_deep_match(&pattern, &name[1..], plus));
            }
            b'+' => {
                /*
                 * '+' 表示 1 个字符或者任意个字符，至少一个字符
                 *
                 * plus = true 表示上一次匹配的 pattern 字符是 '+' 并且 '+'
                 * 已经匹配了至少一个字符了。
                 */

                if plus {
                    return wildcard_deep_match(&pattern[1..], &name, false)
                        || (name.len() > 0 && wildcard_deep_match(&pattern, &name[1..], true));
                } else {
                    return name.len() > 0 && wildcard_deep_match(&pattern, &name[1..], true);
                }
            }
            _ => {
                if name.len() == 0 || pattern[0] != name[0] {
                    return false;
                }
            }
        }

        pattern = &pattern[1..];
        name = &name[1..];
    }
    return name.len() == 0 && pattern.len() == 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wildcard_match_1() {
        let pattern = "aa";
        let name = "bb";
        let result = wildcard_match(pattern, name);
        assert_eq!(result, false);
    }

    #[test]
    fn test_wildcard_match_2() {
        let pattern = "b?b";
        let name = "bbb";
        let result = wildcard_match(pattern, name);
        assert_eq!(result, true);
    }

    #[test]
    fn test_wildcard_match_3() {
        let pattern = "";
        let name = "";
        let result = wildcard_match(pattern, name);
        assert_eq!(result, true);
    }

    #[test]
    fn test_wildcard_match_4() {
        let pattern = "a";
        let name = "a";
        let result = wildcard_match(pattern, name);
        assert_eq!(result, true);
    }

    #[test]
    fn test_wildcard_match_5() {
        let pattern = "?";
        let name = "a";
        let result = wildcard_match(pattern, name);
        assert_eq!(result, true);
    }

    #[test]
    fn test_wildcard_match_6() {
        let pattern = "*";
        let name = "";
        let result = wildcard_match(pattern, name);
        assert_eq!(result, true);
    }

    #[test]
    fn test_wildcard_match_7() {
        let pattern = "*";
        let name = "aaaaabbbbb";
        let result = wildcard_match(pattern, name);
        assert_eq!(result, true);
    }

    #[test]
    fn test_wildcard_match_8() {
        let pattern = "aa*";
        let name = "aaaaab";
        let result = wildcard_match(pattern, name);
        assert_eq!(result, true);

        let pattern = "aa*";
        let name = "aa";
        let result = wildcard_match(pattern, name);
        assert_eq!(result, true);
    }

    #[test]
    fn test_wildcard_match_9() {
        let pattern = "aa*b";
        let name = "aaaaabaadfsacxvb";
        let result = wildcard_match(pattern, name);
        assert_eq!(result, true);
    }

    #[test]
    fn test_wildcard_match_10() {
        let pattern = "aa?";
        let name = "aa";
        let result = wildcard_match(pattern, name);
        assert_eq!(result, true);

        let pattern = "aa?bb";
        let name = "aabb";
        let result = wildcard_match(pattern, name);
        assert_eq!(result, true);

        let pattern = "aa?bb";
        let name = "aadbb";
        let result = wildcard_match(pattern, name);
        assert_eq!(result, true);

        let pattern = "aa?bb";
        let name = "aaabb";
        let result = wildcard_match(pattern, name);
        assert_eq!(result, true);

        let pattern = "aa?bb";
        let name = "aabbb";
        let result = wildcard_match(pattern, name);
        assert_eq!(result, true);
    }

    #[test]
    fn test_wildcard_match_11() {
        let pattern = "aa+bb";
        let name = "aabb";
        let result = wildcard_match(pattern, name);
        assert_eq!(result, false);

        let pattern = "+";
        let name = "a";
        let result = wildcard_match(pattern, name);
        assert_eq!(result, true);

        let pattern = "+";
        let name = "";
        let result = wildcard_match(pattern, name);
        assert_eq!(result, false);

        let pattern = "+";
        let name = "ab";
        let result = wildcard_match(pattern, name);
        assert_eq!(result, true);

        let pattern = "a+b+c";
        let name = "aqwerbpoiu3314c";
        let result = wildcard_match(pattern, name);
        assert_eq!(result, true);
    }

    #[test]
    fn test_wildcard_match_array_1() {
        let s = "aaa";
        let array = &["aa", "bb"];
        let result = is_string_wildcard_match_array(s, array);
        assert_eq!(result, false);
    }

    #[test]
    fn test_wildcard_match_array_2() {
        let s = "aaa";
        let array = &["aa", "bb", "a*a"];
        let result = is_string_wildcard_match_array(s, array);
        assert_eq!(result, true);
    }

    #[test]
    fn test_route_1() {
        let pattern = "/api/v1/users/*";
        let path = "/api/v1/users/123";
        let result = wildcard_match(pattern, path);
        assert_eq!(result, true);
    }

    #[test]
    fn test_route_2() {
        let pattern = "/api/v1/drawings/*/shapes/*";
        let path = "/api/v1/drawings/123/shapes/333";
        let result = wildcard_match(pattern, path);
        assert_eq!(result, true);
    }

    #[test]
    fn test_route_3() {
        let pattern = "/api/v1/drawings/*/shapes/*";
        let path = "/api/v1/drawings/123";
        let result = wildcard_match(pattern, path);
        assert_eq!(result, false);
    }
}
