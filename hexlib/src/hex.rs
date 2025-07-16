pub struct Hex;

#[allow(dead_code)]
impl Hex {
    pub fn encode<S: AsRef<str>>(str: S) -> String {
        let str = str.as_ref();
        let chars = str.chars();

        let mut hex_str = String::with_capacity(str.len() * 2);

        for char in chars {
            if char.is_ascii() {
                let chrs = Self::get_hex_chars_of(char);
                hex_str.push(chrs.get(0).unwrap().to_ascii_lowercase());
                hex_str.push(chrs.get(1).unwrap().to_ascii_lowercase());
            }
        }

        hex_str
    }

    pub fn decode<S: AsRef<str>>(hex: S) -> String {
        let hex = hex.as_ref();
        let mut bytes = Vec::new();
        let mut chars = hex.chars().peekable();

        while chars.peek().is_some() {
            let high = chars.next().unwrap();
            let low = chars.next().unwrap();

            let high_val = Self::reverse_table_match(high);
            let low_val = Self::reverse_table_match(low);

            let byte = (high_val << 4) | low_val;
            bytes.push(byte);
        }

        String::from_utf8(bytes).unwrap_or_else(|_| String::new())
    }

    fn color_from_hex<S: AsRef<str>>(hex: S) -> String {
        let hex = hex.as_ref();

        if hex.len() >= 6 {
            hex[0..6].to_string()
        } else {
            hex.to_string()
        }
    }

    fn alternate_color_from_hex<S: AsRef<str>>(hex: S) -> String {
        let hex = hex.as_ref();

        let len = hex.len();

        let lensix = len-6;

        if len >= 6 {
            hex[lensix..].to_string()
        } else {
            hex.to_string()
        }
    }

    pub fn color_from_str<S: AsRef<str>>(str: S) -> String {
        let hex = Self::encode(str.as_ref());

        Self::color_from_hex(hex)
    }

    pub fn alternate_color_from_str<S: AsRef<str>>(str: S) -> String {
        let hex = Self::encode(str.as_ref());

        Self::alternate_color_from_hex(hex)
    }

    #[inline]
    fn get_hex_chars_of(char: char) -> [char; 2] {
        let byte = Self::get_byte(char);
        let mut str = String::new();

        str.push(Self::get_from_table(byte, true));
        str.push(Self::get_from_table(byte, false));

        let mut chars = str.chars();

        [chars.next().unwrap(), chars.next().unwrap()]
    }

    #[inline]
    fn get_from_table(ch: u8, is_high: bool) -> char {
        let nm = if is_high { ch / 16 } else { ch % 16 };

        Self::table_match(nm)
    }

    #[inline]
    fn table_match(ch: u8) -> char {
        match ch {
            10u8 => 'a',
            11u8 => 'b',
            12u8 => 'c',
            13u8 => 'd',
            14u8 => 'e',
            15u8 => 'f',
            0_u8..=9_u8 => (ch + 48) as char,
            _ => unreachable!(),
        }
    }

    #[inline]
    fn get_byte(char: char) -> u8 {
        char as u8
    }

    #[inline]
    fn reverse_table_match(c: char) -> u8 {
        match c.to_ascii_lowercase() {
            'a' => 10,
            'b' => 11,
            'c' => 12,
            'd' => 13,
            'e' => 14,
            'f' => 15,
            '0'..='9' => c as u8 - b'0',
            _ => unreachable!(),
        }
    }
}
