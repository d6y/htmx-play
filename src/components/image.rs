use std::ops::Range;

pub struct Image {
    pub name: String,
    pub url: String,
}

impl Image {
    pub fn make(range: Range<usize>) -> Vec<Image> {
        (range.start..range.end).map(make_image).collect()
    }
}

fn make_image(i: usize) -> Image {
    let (r, g, b) = int_to_rgb(i);

    let bg_colour = format!("{:02X}{:02X}{:02X}", r, g, b);
    let fg_colour = format!("{:02X}{:02X}{:02X}", 255 - r, 255 - g, 255 - b);

    let name = format!("Image {}", i);

    let url = format!(
        "https://placehold.co/320x240/{}/{}?text=Image+number+{}",
        bg_colour, fg_colour, i
    );

    Image { name, url }
}

// This code below mostly from ChatGPT 4o

fn int_to_rgb(n: usize) -> (u8, u8, u8) {
    // Increment the hue by a fixed step to spread colors evenly
    let hue = (n as f64 * 137.508) % 360.0; // 137.508 is the golden angle for more uniform distribution
    let saturation = 0.7; // Keep saturation constant
    let lightness = 0.5; // Keep lightness constant

    hsl_to_rgb(hue, saturation, lightness)
}

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let h_prime = h / 60.0;
    let x = c * (1.0 - (h_prime % 2.0 - 1.0).abs());

    let (r1, g1, b1) = match h_prime {
        h if (0.0..1.0).contains(&h) => (c, x, 0.0),
        h if (1.0..2.0).contains(&h) => (x, c, 0.0),
        h if (2.0..3.0).contains(&h) => (0.0, c, x),
        h if (3.0..4.0).contains(&h) => (0.0, x, c),
        h if (4.0..5.0).contains(&h) => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    let m = l - c / 2.0;
    let r = ((r1 + m) * 255.0).round() as u8;
    let g = ((g1 + m) * 255.0).round() as u8;
    let b = ((b1 + m) * 255.0).round() as u8;

    (r, g, b)
}
