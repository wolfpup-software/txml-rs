// https://developer.mozilla.org/en-US/docs/Web/HTML/Element

use crate::sieves::SieveImpl;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct TagInfo {
    pub namespace: String,
    pub tag: String,
    pub last_descendant_tag: String,
    pub has_text: bool,
    pub indent_count: usize,
    pub void_el: bool,
    pub preserved_text_el: bool,
    pub inline_el: bool,
    pub void_path: bool,
    pub preserved_text_path: bool,
    pub banned_path: bool,
}

impl TagInfo {
    pub fn new(sieve: &impl SieveImpl, tag: &str) -> TagInfo {
        let mut namespace = "html".to_string();
        if sieve.namespace_el(tag) {
            namespace = tag.to_string()
        }

        TagInfo {
            namespace: namespace,
            tag: tag.to_string(),
            last_descendant_tag: "".to_string(),
            has_text: false,
            indent_count: 0,
            void_el: sieve.void_el(tag),
            preserved_text_el: sieve.preserved_text_el(tag),
            inline_el: sieve.inline_el(tag),
            void_path: false,
            preserved_text_path: false,
            banned_path: sieve.banned_el(tag),
        }
    }

    pub fn from(sieve: &impl SieveImpl, prev_tag_info: &TagInfo, tag: &str) -> TagInfo {
        // clone, then update values, then return
        let mut tag_info = prev_tag_info.clone();

        if sieve.namespace_el(tag) {
            tag_info.namespace = tag.to_string();
        }

        if sieve.void_el(&prev_tag_info.tag) {
            tag_info.void_path = true;
        }

        // preserved text happends _after_ tag
        if sieve.preserved_text_el(&prev_tag_info.tag) {
            tag_info.preserved_text_path = true;
        }

        if sieve.banned_el(tag) {
            tag_info.banned_path = true;
        }

        if sieve.void_el(tag) || !sieve.inline_el(tag) {
            tag_info.indent_count += 1;
        }

        tag_info.tag = tag.to_string();
        tag_info.last_descendant_tag = "".to_string();
        tag_info.inline_el = sieve.inline_el(tag);
        tag_info.has_text = false;

        tag_info
    }
}
