use std::collections::{HashMap, HashSet};
use unicode_segmentation::UnicodeSegmentation;

pub fn extract_tags(names: Vec<String>) -> HashSet<String> {
    let mut checked_map: HashMap<usize, String> = HashMap::new();

    check_prefix_suffix(&names, &mut checked_map, true);
    check_prefix_suffix(&names, &mut checked_map, false);

    checked_map.values().map(|s| s.trim().to_string()).collect()
}

fn check_prefix_suffix(
    names: &[String],
    checked_map: &mut HashMap<usize, String>,
    is_prefix: bool,
) {
    let mut checked_set: HashSet<usize> = HashSet::new();
    let mut suffix_tags: HashSet<String> = HashSet::new();
    for (i, name) in names.iter().enumerate() {
        let mut cursor = 0;
        let mut rejected_set: HashSet<usize> = HashSet::new();
        if checked_map.contains_key(&i) {
            continue;
        }
        for (j, name2) in names.iter().enumerate() {
            loop {
                if i == j || checked_set.contains(&j) || rejected_set.contains(&j) {
                    break;
                }
                let c = grapheme_nth(name, cursor, is_prefix);
                let c2 = grapheme_nth(name2, cursor, is_prefix);
                if c != c2 || (c.is_none() && c2.is_none()) {
                    if cursor == 0 {
                        rejected_set.insert(j);
                        break;
                    }
                    checked_set.insert(i);
                    checked_set.insert(j);
                    let tag = take_tag(name, cursor, is_prefix);
                    insert_check_map(checked_map, &mut suffix_tags, i, j, tag, is_prefix);
                    cursor = 0;
                    break;
                }
                cursor += 1;
            }
        }
    }
}

fn grapheme_nth(name: &str, cursor: usize, is_prefix: bool) -> Option<&str> {
    let count = name.graphemes(true).count();
    if is_prefix {
        name.graphemes(true).nth(cursor)
    } else {
        if count > cursor {
            name.graphemes(true).nth(count - cursor - 1)
        } else {
            None
        }
    }
}

fn take_tag(name: &str, cursor: usize, is_prefix: bool) -> String {
    if is_prefix {
        name.graphemes(true).take(cursor).collect()
    } else {
        let count = name.graphemes(true).count();
        name.graphemes(true).skip(count - cursor).collect()
    }
}

fn insert_check_map(
    checked_map: &mut HashMap<usize, String>,
    suffix_tags: &mut HashSet<String>,
    i: usize,
    j: usize,
    mut tag: String,
    is_prefix: bool,
) {
    if is_prefix {
        checked_map.insert(i, tag.clone());
        checked_map.insert(j, tag);
        return;
    }
    // ここからsuffixの場合
    // もしprefixで発見したtagよりsuffixの方が長い場合はsuffixを優先する
    if let Some(prefix) = checked_map.get(&i) {
        if prefix.len() > tag.len() {
            return;
        }

        // すでに格納済みのタグが 今のタグ.ends_with(すでに格納済みのタグ) である場合、今のタグ候補はすでに格納済みのタグで代入する
        for v in suffix_tags.iter() {
            if tag.ends_with(v) {
                tag = v.clone();
            }
        }

        suffix_tags.insert(tag.clone());
        checked_map.insert(i, tag.clone());
        checked_map.insert(j, tag.clone());
        // すでに格納したsuffixが今のタグ候補に含まれている場合、今のタグ候補の方がタグとして適していると判断する
        for (_, v) in checked_map.iter_mut() {
            if v.ends_with(&tag) {
                *v = tag.clone();
            }
        }
    } else {
        suffix_tags.insert(tag.clone());
        checked_map.insert(i, tag.clone());
        checked_map.insert(j, tag.clone());
    }
}

#[cfg(test)]
mod tests_extract_tag {
    use super::*;

    fn assert_eq_tags(names: Vec<&str>, mut right: Vec<&str>) {
        let tags = extract_tags(names.iter().map(|s| s.to_string()).collect());
        right.sort();
        assert_eq!(
            tags,
            right
                .iter()
                .map(|s| s.to_string())
                .collect::<HashSet<String>>()
        );
    }

    #[test]
    fn test_2v2() {
        assert_eq_tags(
            vec![
                "AA Cynda",
                "AA Dugo",
                "BE naari",
                "BE",
                "あいしてる",
                "あいうえお",
                "Saru FM",
                "ぱーぷる FM",
                "X",
                "X",
                "RR★",
                "RR",
            ],
            vec!["AA", "BE", "あい", "FM", "X", "RR"],
        );
    }

    #[test]
    fn test_3v3() {
        assert_eq_tags(
            vec![
                "AAA",
                "AAAA",
                "AAAGGGAAA",
                "ランドロス",
                "ボルトロス",
                "トルネロス",
                "ending",
                "evening",
                "beginning",
                "mamama",
                "mamamama",
                "mamamamama",
            ],
            vec!["AAA", "ロス", "ing", "mamama"],
        );
    }

    #[test]
    fn test_4v4() {
        assert_eq_tags(
            vec![
                "ロトのつるぎ",
                "ロトのたて",
                "ロトのかぶと",
                "ロトのよろい",
                "(・3・)ちゅん。",
                "(・3・)ゆる。",
                "(・3・)ななせ。",
                "(・3・)あも。",
                "gnidne",
                "gnineve",
                "gninnigeb",
            ],
            vec!["ロトの", "(・3・)", "gni"],
        )
    }

    #[test]
    fn test_6v6() {
        assert_eq_tags(
            vec![
                "OtM 1", "OtM 2", "OtM 3", "OtM 4", "OtM 5", "OtM 6", "RR 1", "RR 2", "RR 3",
                "RR 4", "RR 5", "RR 6",
            ],
            vec!["OtM", "RR"],
        );
    }
}
