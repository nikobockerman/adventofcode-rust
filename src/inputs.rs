use std::collections::HashMap;

lazy_static::lazy_static! {
    static ref INPUTS: HashMap<(u16, u8), &'static str> = {
        let mut map = HashMap::new();
        map.insert((2024, 5), include_str!("y2024/input-d5.txt"));
        map
    };
}

pub(crate) fn get_input(year: u16, day: u8) -> Option<&'static str> {
    INPUTS.get(&(year, day)).copied().map(str::trim_end)
}
