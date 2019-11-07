use std::collections::BTreeMap as Map;

pub fn split<'s, 't>(mut text: &'t str, sep: &'s str) -> Map<usize, &'t str> {
    let seplen = sep.len();
    let mut map = Map::new();
    let mut offset = 0;
    while let Some(idx) = text.find(sep) {
        let skip = idx+seplen;
        if idx>0 {
            map.insert(offset, &text[0..idx]);
        }
        text = &text[skip..];
        offset += skip;
    }
    if text.len()>0 {
        map.insert(offset, &text);
    }
    map
}