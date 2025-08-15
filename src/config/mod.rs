pub mod controls;
pub mod layout;
pub mod pallet;

#[derive(Debug)]
pub struct Config {
    pub layout: layout::Layout,
    pub pallet: pallet::Pallet,
    pub controls: controls::Controls,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            layout: layout::Layout::new(100.0, 100.0),
            pallet: pallet::Pallet::default(),
            controls: controls::Controls::default(),
        }
    }
}
