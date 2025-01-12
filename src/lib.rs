#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
    a: Option<f64>,
    h: f64,
    s: f64,
    l: f64,
}

fn float_to_u8(value: f64) -> u8 {
    (value.clamp(0.0, 1.0) * 255.0).round() as u8
}

impl Color {
    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        let (h, s, l) = rgb_to_hsl(r, g, b);
        Self {
            r,
            g,
            b,
            a: None,
            h,
            s,
            l,
        }
    }

    pub fn from_rgba(r: f64, g: f64, b: f64, a: f64) -> Self {
        let (h, s, l) = rgb_to_hsl(r, g, b);
        Self {
            r,
            g,
            b,
            a: Some(a),
            h,
            s,
            l,
        }
    }

    pub fn from_hex(hex: &str) -> Result<Self, String> {
        if !hex.starts_with('#') || (hex.len() != 7 && hex.len() != 9) {
            return Err("Invalid hex color format".to_string());
        }

        let r = u8::from_str_radix(&hex[1..3], 16).map_err(|_| "Invalid hex value")? as f64 / 255.0;
        let g = u8::from_str_radix(&hex[3..5], 16).map_err(|_| "Invalid hex value")? as f64 / 255.0;
        let b = u8::from_str_radix(&hex[5..7], 16).map_err(|_| "Invalid hex value")? as f64 / 255.0;

        let a = if hex.len() == 9 {
            Some(
                u8::from_str_radix(&hex[7..9], 16).map_err(|_| "Invalid hex value")? as f64 / 255.0,
            )
        } else {
            None
        };
        Ok(Self {
            r,
            g,
            b,
            a,
            h: 0.0,
            s: 0.0,
            l: 0.0,
        })
    }

    pub fn from_hsl(h: f64, s: f64, l: f64) -> Self {
        let (r, g, b) = hsl_to_rgb(h, s, l);
        Self {
            r,
            g,
            b,
            a: None,
            h,
            s,
            l,
        }
    }

    pub fn to_rgb_string(&self) -> String {
        format!(
            "rgba({}, {}, {})",
            (self.r * 255.0) as u8,
            (self.g * 255.0) as u8,
            (self.b * 255.0) as u8
        )
    }

    pub fn to_rgba_string(&self) -> String {
        match self.a {
            Some(a) => format!(
                "rgba({}, {}, {}, {})",
                float_to_u8(self.r),
                float_to_u8(self.g),
                float_to_u8(self.b),
                float_to_u8(a)
            ),
            None => self.to_rgb_string(),
        }
    }

    pub fn to_hex_string(&self) -> String {
        let r = float_to_u8(self.r);
        let g = float_to_u8(self.g);
        let b = float_to_u8(self.b);
        let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);
        if let Some(a) = self.a {
            format!("{}{:02X}", hex, float_to_u8(a))
        } else {
            hex
        }
    }

    pub fn to_hsl_string(&self) -> String {
        format!("hsl({}, {}%, {}%)", self.h, self.s * 100.0, self.l * 100.0)
    }
}

fn rgb_to_hsl(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;
    let l = (max + min) / 2.0;

    let s = if l == 0.0 || l == 1.0 {
        0.0
    } else {
        delta / (1.0 - (2.0 * l - 1.0).abs())
    };

    let h = if delta == 0.0 {
        0.0
    } else if max == r {
        60.0 * ((g - b) / delta) % 360.0
    } else if max == g {
        60.0 * ((b - r) / delta) + 120.0
    } else {
        60.0 * ((r - g) / delta) + 240.0
    };
    (h, s, l)
}

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (f64, f64, f64) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    (r + m, g + m, b + m)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let color = Color {
            r: 0.5,
            g: 0.2,
            b: 0.6,
            a: None,
            h: 0.0,
            s: 0.0,
            l: 0.0,
        };
        assert_eq!(color.r, 0.5);
        assert_eq!(color.g, 0.2);
        assert_eq!(color.b, 0.8);
        assert_eq!(color.a, None);
    }
}
