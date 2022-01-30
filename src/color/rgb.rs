pub use crate::geometry::vec3::Vector as RgbColor;

impl std::fmt::Display for RgbColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "#{:02x}{:02x}{:02x}", (255.999*self.x) as u64, (255.999*self.y) as u64, (255.999*self.z) as u64)
        write!(f, "{} {} {}", (255.999*self.x) as u64, (255.999*self.y) as u64, (255.999*self.z) as u64)
    }
}

impl RgbColor {
    pub fn hex(self) -> String {
        format!("#{:02x}{:02x}{:02x}", (255.999*self.x) as u64, (255.999*self.y) as u64, (255.999*self.z) as u64)
    }
}

