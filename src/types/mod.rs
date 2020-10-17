use std::collections::HashMap;

pub type ColorMap = HashMap<usize, Rgb>;

#[derive(Debug)]
pub struct Gif {
    pub signature: String,
    pub screen_descriptor: ScreenDescriptor,
    pub global_color_map: Option<ColorMap>,
    pub frames: Vec<Frame>,
}

#[derive(Debug)]
pub struct ScreenDescriptor {
    pub width: u16,
    pub height: u16,
    pub m: bool,
    pub cr: u8,
    pub pixel: u8,
    pub background: u8,
}

#[derive(Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Debug)]
pub struct Frame {
    pub image_descriptor: ImageDescriptor,
    pub local_color_map: Option<ColorMap>,
    /// raster_data representation depends on the [super::decoder::ColorOutput] mode setting.
    /// - If [super::decoder::ColorOutput::ColorMap] is set, the nth byte of the raster_data contains the ColorMap index of the nth pixel.
    /// - If [super::decoder::ColorOutput::RGBA] is set, every nth byte of the raster_data is expanded to 4 bytes, the 4 bytes contains respectively the values R G B A
    pub raster_data: Vec<u8>,
    pub graphic_control_extension: Option<GraphicControlExtension>,
}

#[derive(Debug)]
pub enum ExtensionBlock {
    GraphicControlExtension(GraphicControlExtension),
}

#[derive(Debug)]
pub struct GraphicControlExtension {
    pub disposal_method: u8,
    pub user_input: bool,
    pub transparent_color: bool,
    pub delay_time: u16,
    pub transparent_color_index: Option<u8>,
}

#[derive(Debug)]
pub struct ImageDescriptor {
    pub image_left: u16,
    pub image_top: u16,
    pub image_width: u16,
    pub image_height: u16,
    pub m: bool,
    pub i: bool,
    pub pixel: u8,
}
