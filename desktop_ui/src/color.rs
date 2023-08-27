pub fn color_from_web(web_color: &str) -> iced::Color {
    let re = regex::Regex::new(r"^#([A-Fa-f0-9]{2})([A-Fa-f0-9]{2})([A-Fa-f0-9]{2})$").
    unwrap();
    let groups = re.captures(web_color).unwrap();
    let r = u8::from_str_radix(&groups[1], 16).unwrap();
    let g = u8::from_str_radix(&groups[2], 16).unwrap();
    let b = u8::from_str_radix(&groups[3], 16).unwrap();
    iced::Color::from_rgb(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0)
}