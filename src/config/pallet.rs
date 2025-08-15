use wut::gx2::color::Color;

#[derive(Debug, Clone, Copy)]
pub struct ColorCombination {
    pub base: Color,
    pub content: Color,
}

impl ColorCombination {
    pub const fn new(base: Color, content: Color) -> Self {
        Self { base, content }
    }

    pub fn auto(base: Color) -> Self {
        let (r, g, b, _) = base.into();

        Self::new(
            base,
            if (0.299 * r + 0.587 * g + 0.114 * b) > 0.5 {
                Color::black()
            } else {
                Color::white()
            },
        )
    }
}

#[derive(Debug, Clone)]
pub struct Pallet {
    pub background: ColorCombination,
    pub widget: ColorCombination,
    pub highlight: ColorCombination,
}

impl Pallet {
    pub fn auto(background: Color, widget: Color, highlight: Color) -> Self {
        Self {
            background: ColorCombination::auto(background),
            widget: ColorCombination::auto(widget),
            highlight: ColorCombination::auto(highlight),
        }
    }
}

impl Default for Pallet {
    fn default() -> Self {
        Self::auto(Color::black(), Color::black(), Color::red())
    }
}
