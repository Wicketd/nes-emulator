use crate::types::{Result, Asset};
use piston::types::{Color, ColorComponent};

pub struct Palette(Vec<Color>);

impl Palette {
    pub fn new() -> Result<Self> {
        if let Some(pal_encoded) = Asset::get("ntsc.pal") {
            let colors = Self::decode_bytes(&pal_encoded);

            Ok(Self(colors))
        } else {
            Err(anyhow!("could not load encoded palette file"))
        }
    }

    fn decode_bytes(bytes: &[u8]) -> Vec<Color> {
        let mut colors = vec![];
        let mut byte_iter = bytes.iter();

        while let (Some(r), Some(g), Some(b)) = (byte_iter.next(), byte_iter.next(), byte_iter.next()) {
            colors.push([
                *r as ColorComponent / 255.0,
                *g as ColorComponent / 255.0,
                *b as ColorComponent / 255.0,
                1.0,
            ]);
        }

        colors
    }

    pub fn get(&self, idx: usize) -> Option<&Color> {
        self.0.get(idx)
    }

    pub fn colors(&self) -> &Vec<Color> {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const COMPONENT_BYTE: u8 = 240;
    const COMPONENT: ColorComponent = COMPONENT_BYTE as ColorComponent / 255.0;

    #[test]
    fn decode_bytes() {
        let bytes: Vec<u8> = vec![
            COMPONENT_BYTE, 0,              0,
            0,              COMPONENT_BYTE, 0,
            0,              0,              COMPONENT_BYTE,
            55,             55,
        ];

        assert_eq!(Palette::decode_bytes(&bytes), vec![
            [COMPONENT, 0.0,       0.0, 1.0],
            [0.0,       COMPONENT, 0.0, 1.0],
            [0.0,       0.0,       COMPONENT, 1.0],
        ]);
    }
}
