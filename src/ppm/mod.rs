use crate::color::rgb::RgbColor;
use std::fs::File;
use std::io::prelude::*;

pub fn generate_image(
    render: &Vec<RgbColor>,
    filename: String,
    dimensions: (u32, u32),
) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    file.write_all(b"P3\n")?;
    file.write_all(format!("{} {}", dimensions.0, dimensions.1).as_bytes())?;
    file.write_all(b"\n255\n")?;
    for pixel in render {
        file.write_all(format!("{}\n", pixel).as_bytes())?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_image() {
        let render = vec![RgbColor::default(); 1600];
        assert!(generate_image(&render, "test.ppm".to_string(), (40, 40)).is_ok());
    }
}
