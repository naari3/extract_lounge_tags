# mk8dx_lounge_tag_extractor

## Example

```rust
use mk8dx_lounge_tag_extractor::extract_tags;
use std::collections::HashSet;

let names = vec![
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
];
let tags = extract_tags(&names);

assert_eq!(tags, HashSet::from_iter(vec!["AA", "BE", "あい", "FM", "X", "RR"].iter().map(|s| s.to_string())));
```
