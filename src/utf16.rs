quick_error! {
    #[derive(Debug)]
    pub enum SubstringError {
        EncodeUtf16 { }
    }
}

pub fn substring(txt: &str, s: usize, e: usize) -> Result<String, SubstringError> {
    let a: Vec<_> = txt.encode_utf16().collect();
    let mut sub_a = vec![];
    for i in s..e {
        sub_a.push(a[i]);
    }
    String::from_utf16(&sub_a[..]).map_err(|_| SubstringError::EncodeUtf16)
}

pub fn utf16_len(txt: &str) -> usize {
    txt.encode_utf16().count()
}
