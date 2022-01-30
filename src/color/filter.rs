use std::vec::Vec;
use crate::color::rgb::RgbColor;

pub trait Filter {
    fn apply_filter(&self, render: &mut Vec<RgbColor>);
}