use std::collections::HashMap;

pub type ColorMap = HashMap<usize, Rgb>;

#[derive(Debug)]
pub struct Gif {
    signature: String,
    screen_descriptor: ScreenDescriptor,
    global_color_map: Option<ColorMap>,
    frames: Vec<Frame>,
}

impl Gif {
    pub fn signature(&self) -> &str {
        &self.signature
    }
    pub fn screen_descriptor(&self) -> &ScreenDescriptor {
        &self.screen_descriptor
    }
    pub fn global_color_map(&self) -> &Option<ColorMap> {
        &self.global_color_map
    }
    pub fn frames(&self) -> &Vec<Frame> {
        &self.frames
    }
    pub fn new(
        signature: String,
        screen_descriptor: ScreenDescriptor,
        global_color_map: Option<ColorMap>,
        frames: Vec<Frame>,
    ) -> Self {
        Gif {
            signature,
            screen_descriptor,
            global_color_map,
            frames,
        }
    }
}

#[derive(Debug)]
pub struct ScreenDescriptor {
    width: u16,
    height: u16,
    m: bool,
    cr: u8,
    pixel: u8,
    background: u8,
}

impl ScreenDescriptor {
    pub fn width(&self) -> u16 {
        self.width
    }
    pub fn height(&self) -> u16 {
        self.height
    }
    pub fn m(&self) -> bool {
        self.m
    }
    pub fn cr(&self) -> u8 {
        self.cr
    }
    pub fn pixel(&self) -> u8 {
        self.pixel
    }

    pub fn background(&self) -> u8 {
        self.background
    }
    pub fn new(width: u16, height: u16, m: bool, cr: u8, pixel: u8, background: u8) -> Self {
        ScreenDescriptor {
            width,
            height,
            m,
            cr,
            pixel,
            background,
        }
    }
    pub fn set_m(&mut self, m: bool) {
        self.m = m;
    }
}

#[derive(Debug)]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {
    pub fn r(&self) -> u8 {
        self.r
    }
    pub fn g(&self) -> u8 {
        self.g
    }
    pub fn b(&self) -> u8 {
        self.b
    }
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Rgb { r, g, b }
    }
}

#[derive(Debug)]
pub struct Frame {
    image_descriptor: ImageDescriptor,
    local_color_map: Option<ColorMap>,
    raster_data: Vec<u8>,
    rgba_raster_data: Option<Vec<u8>>,
    graphic_control_extension: Option<GraphicControlExtension>,
}

impl Frame {
    pub fn image_descriptor(&self) -> &ImageDescriptor {
        &self.image_descriptor
    }
    pub fn local_color_map(&self) -> &Option<ColorMap> {
        &self.local_color_map
    }
    /// Normal ColorMap index color mapping.
    /// Color maps / graphic control extension block usage is necessary to retrieve the pixel colors in rgba.
    pub fn raster_data(&self) -> &Vec<u8> {
        &self.raster_data
    }
    /// This is not a gif89a specification field, present only if requested in the decoding process.
    /// Every byte of the raster data expanded to 4 bytes (R G B A).
    /// Color maps / graphic control extension block has already been used internally to obtain this raster data representation.
    pub fn rgba_raster_data(&self) -> &Option<Vec<u8>> {
        &self.rgba_raster_data
    }
    pub fn graphic_control_extension(&self) -> &Option<GraphicControlExtension> {
        &self.graphic_control_extension
    }
    pub fn new(
        image_descriptor: ImageDescriptor,
        local_color_map: Option<ColorMap>,
        raster_data: Vec<u8>,
        rgba_raster_data: Option<Vec<u8>>,
        graphic_control_extension: Option<GraphicControlExtension>,
    ) -> Self {
        Frame {
            image_descriptor,
            local_color_map,
            raster_data,
            rgba_raster_data,
            graphic_control_extension,
        }
    }
}

#[derive(Debug)]
pub enum ExtensionBlock {
    GraphicControlExtension(GraphicControlExtension),
}

#[derive(Debug)]
pub struct GraphicControlExtension {
    disposal_method: u8,
    user_input: bool,
    transparent_color: bool,
    delay_time: u16,
    transparent_color_index: Option<u8>,
}

impl GraphicControlExtension {
    pub fn disposal_method(&self) -> u8 {
        self.disposal_method
    }
    pub fn user_input(&self) -> bool {
        self.user_input
    }
    pub fn transparent_color(&self) -> bool {
        self.transparent_color
    }
    pub fn delay_time(&self) -> u16 {
        self.delay_time
    }
    pub fn transparent_color_index(&self) -> Option<u8> {
        self.transparent_color_index
    }
    pub fn new(
        disposal_method: u8,
        user_input: bool,
        transparent_color: bool,
        delay_time: u16,
        transparent_color_index: Option<u8>,
    ) -> Self {
        GraphicControlExtension {
            disposal_method,
            user_input,
            transparent_color,
            delay_time,
            transparent_color_index,
        }
    }
}

#[derive(Debug)]
pub struct ImageDescriptor {
    image_left: u16,
    image_top: u16,
    image_width: u16,
    image_height: u16,
    m: bool,
    i: bool,
    pixel: u8,
}

impl ImageDescriptor {
    pub fn image_left(&self) -> u16 {
        self.image_left
    }
    pub fn image_top(&self) -> u16 {
        self.image_top
    }
    pub fn image_width(&self) -> u16 {
        self.image_width
    }
    pub fn image_height(&self) -> u16 {
        self.image_height
    }
    pub fn m(&self) -> bool {
        self.m
    }
    pub fn i(&self) -> bool {
        self.i
    }
    pub fn pixel(&self) -> u8 {
        self.pixel
    }

    pub fn new(
        image_left: u16,
        image_top: u16,
        image_width: u16,
        image_height: u16,
        m: bool,
        i: bool,
        pixel: u8,
    ) -> Self {
        ImageDescriptor {
            image_left,
            image_top,
            image_width,
            image_height,
            m,
            i,
            pixel,
        }
    }
}
