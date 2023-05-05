use sdl2::pixels::Color;

use super::cell_type::CellType;
#[derive(Clone)]
pub struct Cell {
    pub cell_type: CellType,
    pub value: f64,
}

impl Cell {
    pub fn new(c: CellType) -> Self {
        Self {
            cell_type: c,
            value: 100.0,
        }
    }
    pub fn get_cell_color(&self) -> Color {
        match self.cell_type {
            CellType::Empty => Color::RGBA(255, 255, 255, 255),
            CellType::Grass => {
                let green = (255 - (200 - (self.value * 2.0)as u8)) as u8;
                Color::RGBA(0, green, 0, 255)
            }
            CellType::Sand => {
                let component = (255 - (200 - (self.value * 2.0) as u8)) as u8;
                Color::RGBA(component, component, 0, 255)
            }
            CellType::Water => Color::RGBA(0, 0, 255, 255),
            CellType::Structure => Color::RGBA(64, 64, 64, 255),
            CellType::Wood => Color::RGBA(165, 42, 42, 255), // Brown color
            CellType::Floor=>Color::RGBA(188,143,143,255),
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            value: 100.0,
            cell_type: CellType::Empty,
        }
    }
}
