pub fn format_damage(value: f64) -> String {
        if value >= 100_000_000_000.0 {
        // 100.0G-Inf, Greater than 999.9G will cause the legend misaligned
        let g = value / 1_000_000_000.0;
        format!("{g:.1}G", g=g)
    } else if value >= 1_000_000_000.0 {
        // 1000M-99999M
        let m = value / 1_000_000.0;
        format!("{m:.0}M", m=m)
    } else if value >= 1_000_000.0 {
        // 1.0M-999.9M
        let m = value / 1_000_000.0;
        format!("{m:.1}M", m=m)
    } else if value >= 1_000.0 {
        let k = value / 1_000.0;
        format!("{}K", k.floor())
    } else {
        format!("{}", value.floor())
    }
}

pub fn get_character_color(index: usize) -> egui::Color32 {
    const COLORS: &[egui::Color32] = &[
        egui::Color32::from_rgb(255, 99, 132),   
        egui::Color32::from_rgb(54, 162, 235),   
        egui::Color32::from_rgb(255, 206, 86),   
        egui::Color32::from_rgb(75, 192, 192),   
        egui::Color32::from_rgb(153, 102, 255),  
        egui::Color32::from_rgb(255, 159, 64),   
        egui::Color32::from_rgb(231, 233, 237),  
        egui::Color32::from_rgb(102, 255, 102),  
    ];
    
    COLORS[index % COLORS.len()]
}

pub fn wrap_character_name(name: &str, max_line_length: usize) -> String {
    if name.len() <= max_line_length {
        return name.to_string();
    }
    
    let words: Vec<&str> = name.split_whitespace().collect();
    if words.is_empty() {
        return name.to_string();
    }
    
    let mut lines = Vec::new();
    let mut current_line = String::new();
    
    for word in words {
        if !current_line.is_empty() && current_line.len() + 1 + word.len() > max_line_length {
            lines.push(current_line.clone());
            current_line.clear();
        }
        
        if !current_line.is_empty() {
            current_line.push(' ');
        }
        current_line.push_str(word);
    }
    
    if !current_line.is_empty() {
        lines.push(current_line);
    }
    
    lines.join("\n")
}