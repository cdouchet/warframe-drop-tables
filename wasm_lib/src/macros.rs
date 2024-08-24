#[macro_export]
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => ($crate::log(&format_args!($($t)*).to_string()))
}

#[macro_export]
macro_rules! parse_xml_name {
    ($bytes:expr) => {{
        let bytes = $bytes.name().0.to_vec();
        String::from_utf8(bytes).expect("Failed to parse bytes as string")
    }};
}

#[macro_export]
macro_rules! warframe_parse_error {
    () => {
        console_log!("Fatal: Unexpected error while parsing warframe data")
    };
}
