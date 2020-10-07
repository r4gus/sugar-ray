pub trait Ppm {
    /** Returns a object in the Portable Pixmap (PPM) format.
     *
     * __Layout__:
     * P3
     * width height
     * maximum-color-value
     * r g b r g b r g b
     * r g b r g b r g b
     *
     * Color value: e.g. 255 -> each value (r,g,b) has a value between
     * 0 and 255.
     *
     * Please not that by convention each line should be less or equal
     * 70 characters.
     */
    fn to_ppm(&self) -> String;
}

pub trait PpmColor {
    /** Turns a color into the PPM color format
     *
     * red green blue
     *
     * Each value has to be between 0 and 255, seperated by a whitespace.
     */
    fn to_ppm_color(&self) -> String;
}

