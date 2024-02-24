use std::ops::Deref;


pub const LUMA_FROM_R_COEFF: f32 = 0.299;
pub const LUMA_FROM_G_COEFF: f32 = 0.587;
pub const LUMA_FROM_B_COEFF: f32 = 0.114;

pub trait ToLuma: image::Pixel {
    fn to_luma(&self) -> f32;
}

impl ToLuma for image::Rgb<u8> {
    fn to_luma(&self) -> f32 {
        (self.0[0] as f32) * LUMA_FROM_R_COEFF
            + (self.0[1] as f32) * LUMA_FROM_G_COEFF
            + (self.0[2] as f32) * LUMA_FROM_B_COEFF
    }
}

impl ToLuma for image::Rgb<u16> {
    fn to_luma(&self) -> f32 {
        (self.0[0] as f32) / 256.0 * LUMA_FROM_R_COEFF
            + (self.0[1] as f32) / 256.0 * LUMA_FROM_G_COEFF
            + (self.0[2] as f32) / 256.0 * LUMA_FROM_B_COEFF
    }
}

impl ToLuma for image::Rgba<u8> {
    fn to_luma(&self) -> f32 {
        (self.0[0] as f32) * LUMA_FROM_R_COEFF
            + (self.0[1] as f32) * LUMA_FROM_G_COEFF
            + (self.0[2] as f32) * LUMA_FROM_B_COEFF
    }
}

impl ToLuma for image::Rgba<u16> {
    fn to_luma(&self) -> f32 {
        (self.0[0] as f32) / 256.0 * LUMA_FROM_R_COEFF
            + (self.0[1] as f32) / 256.0 * LUMA_FROM_G_COEFF
            + (self.0[2] as f32) / 256.0 * LUMA_FROM_B_COEFF
    }
}

impl ToLuma for image::Bgr<u8> {
    fn to_luma(&self) -> f32 {
        (self.0[0] as f32) * LUMA_FROM_B_COEFF
            + (self.0[1] as f32) * LUMA_FROM_G_COEFF
            + (self.0[2] as f32) * LUMA_FROM_R_COEFF
    }
}

impl ToLuma for image::Bgra<u8> {
    fn to_luma(&self) -> f32 {
        (self.0[0] as f32) * LUMA_FROM_B_COEFF
            + (self.0[1] as f32) * LUMA_FROM_G_COEFF
            + (self.0[2] as f32) * LUMA_FROM_R_COEFF
    }
}

impl ToLuma for image::Luma<u8> {
    fn to_luma(&self) -> f32 {
        self.0[0] as f32
    }
}

impl ToLuma for image::Luma<u16> {
    fn to_luma(&self) -> f32 {
        self.0[0] as f32 / 256.0
    }
}

impl ToLuma for image::LumaA<u8> {
    fn to_luma(&self) -> f32 {
        self.0[0] as f32
    }
}

impl ToLuma for image::LumaA<u16> {
    fn to_luma(&self) -> f32 {
        self.0[0] as f32 / 256.0
    }
}

pub trait ToLumaImage {
    fn to_luma_image(&self) -> (usize, usize, Vec<f32>);
}

impl<P, Container> ToLumaImage for image::ImageBuffer<P, Container>
where
    P: ToLuma + 'static,
    P::Subpixel: 'static,
    Container: Deref<Target = [P::Subpixel]>,
{
    fn to_luma_image(&self) -> (usize, usize, Vec<f32>) {
        let width = self.width();
        let height = self.height();
        let out = self.pixels().map(<P as ToLuma>::to_luma).collect();
        (width as usize, height as usize, out)
    }
}

pub fn to_luma_image(image: &image::DynamicImage) -> (usize, usize, Vec<f32>) {
    match image {
        image::DynamicImage::ImageLuma8(image) => image.to_luma_image(),
        image::DynamicImage::ImageLumaA8(image) => image.to_luma_image(),
        image::DynamicImage::ImageRgb8(image) => image.to_luma_image(),
        image::DynamicImage::ImageRgba8(image) => image.to_luma_image(),
        image::DynamicImage::ImageBgr8(image) => image.to_luma_image(),
        image::DynamicImage::ImageBgra8(image) => image.to_luma_image(),
        image::DynamicImage::ImageLuma16(image) => image.to_luma_image(),
        image::DynamicImage::ImageLumaA16(image) => image.to_luma_image(),
        image::DynamicImage::ImageRgb16(image) => image.to_luma_image(),
        image::DynamicImage::ImageRgba16(image) => image.to_luma_image(),
    }
}