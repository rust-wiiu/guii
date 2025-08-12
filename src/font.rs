use crate::GuiiError;
use core::alloc::GlobalAlloc;
use fontdue::{self, Metrics};
use hashbrown::HashMap;
use wut::{
    self,
    gx2::{
        shader::{
            surface::{self, Surface},
            texture::{self, Texture},
        },
        types::Vec2,
    },
    sys::GLOBAL_ALLOCATOR,
    vec::Vec,
};

#[derive(Debug, Clone, Copy)]
pub struct TexCoords {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
}

impl TexCoords {
    pub fn lb(&self) -> Vec2<f32> {
        Vec2::new(self.left, self.bottom)
    }

    pub fn lt(&self) -> Vec2<f32> {
        Vec2::new(self.left, self.top)
    }

    pub fn rb(&self) -> Vec2<f32> {
        Vec2::new(self.right, self.bottom)
    }

    pub fn rt(&self) -> Vec2<f32> {
        Vec2::new(self.right, self.top)
    }
}

/// Font atlus
///
/// All glyphs are prerendered into a single texture and rendered by their location on this texture. Allows for single draw call text rendering.
pub struct Atlus {
    tex: Texture,
    coords: HashMap<char, (TexCoords, Metrics)>,
}

impl Atlus {
    const DEFAULT_CHARS: &'static [char] = &[
        // region: special
        '�',
        ' ',
        '!',
        '"',
        '$',
        '#',
        '%',
        '&',
        '\'',
        '(',
        ')',
        '*',
        '+',
        ',',
        '-',
        '.',
        '/',
        ':',
        ';',
        '<',
        '=',
        '>',
        '?',
        '@',
        '[',
        '\\',
        ']',
        '^',
        '_',
        '`',
        '{',
        '|',
        '}',
        '~',
        // endregion
        // region: 0-9
        '0',
        '1',
        '2',
        '3',
        '4',
        '5',
        '6',
        '7',
        '8',
        '9',
        // endregion
        // region: A-Z
        'A',
        'B',
        'C',
        'D',
        'E',
        'F',
        'G',
        'H',
        'I',
        'J',
        'K',
        'L',
        'M',
        'N',
        'O',
        'P',
        'Q',
        'R',
        'S',
        'T',
        'U',
        'V',
        'W',
        'X',
        'Y',
        'Z', // endregion
        // region: a-z
        'a',
        'b',
        'c',
        'd',
        'e',
        'f',
        'g',
        'h',
        'i',
        'j',
        'k',
        'l',
        'm',
        'n',
        'o',
        'p',
        'q',
        'r',
        's',
        't',
        'u',
        'v',
        'w',
        'x',
        'y',
        'z',
        // endregion
        // region: custom
        // gamepad
        wut::font::icons::gamepad::A,
        wut::font::icons::gamepad::B,
        wut::font::icons::gamepad::X,
        wut::font::icons::gamepad::Y,
        wut::font::icons::gamepad::DPAD,
        wut::font::icons::gamepad::DPAD_EMPTY,
        wut::font::icons::gamepad::UP,
        wut::font::icons::gamepad::DOWN,
        wut::font::icons::gamepad::LEFT,
        wut::font::icons::gamepad::RIGHT,
        wut::font::icons::gamepad::UP_DOWN,
        wut::font::icons::gamepad::LEFT_RIGHT,
        wut::font::icons::gamepad::STICK,
        wut::font::icons::gamepad::STICK_L,
        wut::font::icons::gamepad::STICK_R,
        wut::font::icons::gamepad::STICK_L_PRESS,
        wut::font::icons::gamepad::STICK_R_PRESS,
        wut::font::icons::gamepad::L,
        wut::font::icons::gamepad::R,
        wut::font::icons::gamepad::ZL,
        wut::font::icons::gamepad::ZR,
        wut::font::icons::gamepad::TV,
        // root
        wut::font::icons::TARGET,
        wut::font::icons::SCREEN_SHOT,
        wut::font::icons::SCREEN_CAPTURE,
        // wiimote (avoiding duplicates)
        wut::font::icons::wiimote::POWER,
        wut::font::icons::wiimote::A,
        wut::font::icons::wiimote::B,
        wut::font::icons::wiimote::HOME,
        wut::font::icons::wiimote::PLUS,
        wut::font::icons::wiimote::MINUS,
        wut::font::icons::wiimote::ONE,
        wut::font::icons::wiimote::TWO,
        // nunchuk
        wut::font::icons::nunchuk::STICK,
        wut::font::icons::nunchuk::C,
        wut::font::icons::nunchuk::Z,
        // classic (avoiding duplicates)
        wut::font::icons::classic::A,
        wut::font::icons::classic::B,
        wut::font::icons::classic::X,
        wut::font::icons::classic::Y,
        wut::font::icons::classic::STICK_L,
        wut::font::icons::classic::STICK_R,
        wut::font::icons::classic::L,
        wut::font::icons::classic::R,
        wut::font::icons::classic::ZL,
        wut::font::icons::classic::ZR,
        // keyboard
        wut::font::icons::keyboard::RETURN,
        wut::font::icons::keyboard::OPEN_BOX,
        wut::font::icons::keyboard::BLANK,
        // pointer
        wut::font::icons::pointer::point::ALL,
        wut::font::icons::pointer::point::P1,
        wut::font::icons::pointer::point::P2,
        wut::font::icons::pointer::point::P3,
        wut::font::icons::pointer::point::P4,
        wut::font::icons::pointer::fist::ALL,
        wut::font::icons::pointer::fist::P1,
        wut::font::icons::pointer::fist::P2,
        wut::font::icons::pointer::fist::P3,
        wut::font::icons::pointer::fist::P4,
        wut::font::icons::pointer::open::ALL,
        wut::font::icons::pointer::open::P1,
        wut::font::icons::pointer::open::P2,
        wut::font::icons::pointer::open::P3,
        wut::font::icons::pointer::open::P4,
        wut::font::icons::WII,
        wut::font::icons::HELP,
        wut::font::icons::CLOSE,
        wut::font::icons::CLOSE_ALT,
        wut::font::icons::BACK,
        wut::font::icons::HOME,
        wut::font::icons::GAMEPAD,
        wut::font::icons::WIIMOTE,
        // 3ds
        wut::font::icons::three_ds::CIRCLEPAD,
        wut::font::icons::three_ds::BTN_POWER,
        wut::font::icons::three_ds::STEPS,
        wut::font::icons::three_ds::PLAYCOIN,
        // arrows
        wut::font::icons::arrow::circle::CW,
        wut::font::icons::arrow::circle::CCW,
        wut::font::icons::arrow::LEFT_RIGHT,
        wut::font::icons::arrow::UP_DOWN,
        wut::font::icons::arrow::RIGHT,
        wut::font::icons::arrow::LEFT,
        wut::font::icons::arrow::UP,
        wut::font::icons::arrow::DOWN,
        wut::font::icons::arrow::UP_RIGHT,
        wut::font::icons::arrow::DOWN_RIGHT,
        wut::font::icons::arrow::DOWN_LEFT,
        wut::font::icons::arrow::UP_LEFT,
        wut::font::icons::CANCEL,
        wut::font::icons::NFC,
        // endregion
    ];

    // should be a power of two for nice alignment
    pub const PX: usize = 64;

    /// Create a font atlus with ASCII characters and Wii U special chars.
    pub fn new() -> Result<Self, GuiiError> {
        Self::from_charset(Self::DEFAULT_CHARS)
    }

    /// Extends the default font atlus ([Atlus::new]) with custom chars.
    pub fn new_with(charset: impl AsRef<[char]>) -> Result<Self, GuiiError> {
        let mut chars = Vec::from(Self::DEFAULT_CHARS);
        chars.extend_from_slice(charset.as_ref());
        Self::from_charset(chars)
    }

    /// Create a font atlus with a custom character set.
    ///
    /// `charset` must only contain unique characters and should contain the '�' char.
    pub fn from_charset(charset: impl AsRef<[char]>) -> Result<Self, GuiiError> {
        let chars = charset.as_ref();

        let font = fontdue::Font::from_bytes(
            wut::font::system_font(wut::font::FontType::Standard)?,
            fontdue::FontSettings::default(),
        )
        .map_err(|e| GuiiError::FontdueError(e))?;

        let mut coords = HashMap::new();

        const PITCH: usize = 1024;

        let length = chars.len();

        let columns = PITCH / Self::PX;
        let rows = (length + columns - 1) / columns;

        let height = rows * Self::PX;
        let width = columns * Self::PX;

        let mut tex = Texture::new(
            Surface::new()
                .size(width, height)
                .depth(1)
                .mip_levels(1)
                .format(surface::Format::UnormR8)
                .aa(surface::AntiAliasing::X1)
                .usage(surface::Usage::Texture)
                .dim(surface::Dim::Tex2D)
                .tile_mode(surface::Tiling::LinearAligned)
                .swizzle(0)
                .build(),
        )
        .mip(0, 1)
        .slice(0, 1)
        .comp_map(texture::CompMap::xyzw())
        .build();

        let layout = tex.surface().layout();

        tex.as_raw_mut().surface.image = unsafe { GLOBAL_ALLOCATOR.alloc(layout) } as *mut _;

        assert_eq!(tex.as_raw_mut().surface.image.is_null(), false);
        assert_eq!(tex.surface().as_raw().pitch as usize, PITCH);

        let img = tex.surface_mut().image_mut().unwrap();

        for (i, c) in chars.iter().copied().enumerate() {
            let row = i / columns;
            let col = i % columns;

            let (metrics, bitmap) = font.rasterize(c, Self::PX as f32);

            let start_x = col * Self::PX;
            let start_y = row * Self::PX;

            for y in 0..metrics.height {
                for x in 0..metrics.width {
                    let src_idx = y * metrics.width + x;
                    let dst_x = start_x + x;
                    let dst_y = start_y + y;
                    let dst_idx = dst_y * PITCH + dst_x;

                    assert!(dst_idx < img.len());

                    img[dst_idx] = bitmap[src_idx];
                }
            }

            coords.insert(
                c,
                (
                    TexCoords {
                        left: start_x as f32 / width as f32,
                        right: (start_x + metrics.width) as f32 / width as f32,
                        top: start_y as f32 / height as f32,
                        bottom: (start_y + metrics.height) as f32 / height as f32,
                    },
                    metrics,
                ),
            );
        }

        tex.invalidate();

        Ok(Self { tex, coords })
    }

    pub fn get(&self, character: char) -> &(TexCoords, Metrics) {
        match self.coords.get(&character) {
            Some(coords) => coords,
            None => self.coords.get(&'�').unwrap(),
        }
    }

    pub fn texture(&self) -> &Texture {
        &self.tex
    }
}
