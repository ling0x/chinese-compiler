use std::collections::HashMap;

pub fn chinese_to_number(s: &str) -> Option<i64> {
    let digit_map: HashMap<char, i64> = [
        ('零', 0),
        ('〇', 0),
        ('一', 1),
        ('二', 2),
        ('三', 3),
        ('四', 4),
        ('五', 5),
        ('六', 6),
        ('七', 7),
        ('八', 8),
        ('九', 9),
    ]
    .iter()
    .cloned()
    .collect();

    let unit_map: HashMap<char, i64> = [('十', 10), ('百', 100), ('千', 1000), ('万', 10000)]
        .iter()
        .cloned()
        .collect();

    let chars: Vec<char> = s.chars().collect();

    // Handle single digit
    if chars.len() == 1 {
        if let Some(&val) = digit_map.get(&chars[0]) {
            return Some(val);
        }
        // Handle standalone unit like "十" = 10
        if let Some(&val) = unit_map.get(&chars[0]) {
            return Some(val);
        }
    }

    let mut result: i64 = 0;
    let mut current: i64 = 0;
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];

        if let Some(&digit) = digit_map.get(&ch) {
            current = digit;
        } else if let Some(&unit) = unit_map.get(&ch) {
            // Handle implicit "一" before unit
            if current == 0 {
                current = 1;
            }

            if unit >= 10000 {
                // Handle 万
                result = (result + current) * unit;
                current = 0;
            } else {
                current *= unit;
                result += current;
                current = 0;
            }
        }
        i += 1;
    }

    Some(result + current)
}
