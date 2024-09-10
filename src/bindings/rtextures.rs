use super::{FnOutput, Memory, Pointer};

/// Load image from file into CPU memory (RAM)
pub unsafe fn load_image(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load image from RAW file data
pub unsafe fn load_image_raw(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load image from SVG file data or string with specified size
pub unsafe fn load_image_svg(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load image sequence from file (frames appended to image.data)
pub unsafe fn load_image_anim(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load image from memory buffer, fileType refers to extension: i.e. '.png'
pub unsafe fn load_image_from_memory(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load image from GPU texture data
pub unsafe fn load_image_from_texture(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load image from screen buffer and (screenshot)
pub unsafe fn load_image_from_screen(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if an image is ready
pub unsafe fn is_image_ready(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Unload image from CPU memory (RAM)
pub unsafe fn unload_image(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Export image data to file, returns true on success
pub unsafe fn export_image(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Export image to memory buffer
pub unsafe fn export_image_to_memory(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Export image as code file defining an array of bytes, returns true on success
pub unsafe fn export_image_as_code(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Generate image: plain color
pub unsafe fn gen_image_color(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Generate image: linear gradient, direction in degrees [0..360], 0=Vertical gradient
pub unsafe fn gen_image_gradient_linear(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Generate image: radial gradient
pub unsafe fn gen_image_gradient_radial(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Generate image: square gradient
pub unsafe fn gen_image_gradient_square(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Generate image: checked
pub unsafe fn gen_image_checked(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Generate image: white noise
pub unsafe fn gen_image_white_noise(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Generate image: perlin noise
pub unsafe fn gen_image_perlin_noise(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Generate image: cellular algorithm, bigger tileSize means bigger cells
pub unsafe fn gen_image_cellular(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Generate image: grayscale image from text data
pub unsafe fn gen_image_text(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Create an image duplicate (useful for transformations)
pub unsafe fn image_copy(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Create an image from another image piece
pub unsafe fn image_from_image(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Create an image from text (default font)
pub unsafe fn image_text(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Create an image from text (custom sprite font)
pub unsafe fn image_text_ex(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Convert image data to desired format
pub unsafe fn image_format(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Convert image to POT (power-of-two)
pub unsafe fn image_to_pot(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Crop an image to a defined rectangle
pub unsafe fn image_crop(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Crop image depending on alpha value
pub unsafe fn image_alpha_crop(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Clear alpha channel to desired color
pub unsafe fn image_alpha_clear(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Apply alpha mask to image
pub unsafe fn image_alpha_mask(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Premultiply alpha channel
pub unsafe fn image_alpha_premultiply(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Apply Gaussian blur using a box blur approximation
pub unsafe fn image_blur_gaussian(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Resize image (Bicubic scaling algorithm)
pub unsafe fn image_resize(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Resize image (Nearest-Neighbor scaling algorithm)
pub unsafe fn image_resize_nn(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Resize canvas and fill with color
pub unsafe fn image_resize_canvas(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Compute all mipmap levels for a provided image
pub unsafe fn image_mipmaps(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Dither image data to 16bpp or lower (Floyd-Steinberg dithering)
pub unsafe fn image_dither(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Flip image vertically
pub unsafe fn image_flip_vertical(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Flip image horizontally
pub unsafe fn image_flip_horizontal(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Rotate image by input angle in degrees (-359 to 359)
pub unsafe fn image_rotate(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Rotate image clockwise 90deg
pub unsafe fn image_rotate_cw(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Rotate image counter-clockwise 90deg
pub unsafe fn image_rotate_ccw(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Modify image color: tint
pub unsafe fn image_color_tint(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Modify image color: invert
pub unsafe fn image_color_invert(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Modify image color: grayscale
pub unsafe fn image_color_grayscale(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Modify image color: contrast (-100 to 100)
pub unsafe fn image_color_contrast(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Modify image color: brightness (-255 to 255)
pub unsafe fn image_color_brightness(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Modify image color: replace color
pub unsafe fn image_color_replace(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load color data from image as a Color array (RGBA - 32bit)
pub unsafe fn load_image_colors(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load colors palette from image as a Color array (RGBA - 32bit)
pub unsafe fn load_image_palette(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Unload color data loaded with LoadImageColors()
pub unsafe fn unload_image_colors(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Unload colors palette loaded with LoadImagePalette()
pub unsafe fn unload_image_palette(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get image alpha border rectangle
pub unsafe fn get_image_alpha_border(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get image pixel color at (x, y) position
pub unsafe fn get_image_color(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Clear image background with given color
pub unsafe fn image_clear_background(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw pixel within an image
pub unsafe fn image_draw_pixel(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw pixel within an image (Vector version)
pub unsafe fn image_draw_pixel_v(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw line within an image
pub unsafe fn image_draw_line(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw line within an image (Vector version)
pub unsafe fn image_draw_line_v(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw a filled circle within an image
pub unsafe fn image_draw_circle(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw a filled circle within an image (Vector version)
pub unsafe fn image_draw_circle_v(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw circle outline within an image
pub unsafe fn image_draw_circle_lines(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw circle outline within an image (Vector version)
pub unsafe fn image_draw_circle_lines_v(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw rectangle within an image
pub unsafe fn image_draw_rectangle(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw rectangle within an image (Vector version)
pub unsafe fn image_draw_rectangle_v(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw rectangle within an image
pub unsafe fn image_draw_rectangle_rec(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw rectangle lines within an image
pub unsafe fn image_draw_rectangle_lines(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw a source image within a destination image (tint applied to source)
pub unsafe fn image_draw(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw text (using default font) within an image (destination)
pub unsafe fn image_draw_text(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw text (custom sprite font) within an image (destination)
pub unsafe fn image_draw_text_ex(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load texture from file into GPU memory (VRAM)
pub unsafe fn load_texture(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load texture from image data
pub unsafe fn load_texture_from_image(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load cubemap from image, multiple image cubemap layouts supported
pub unsafe fn load_texture_cubemap(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load texture for rendering (framebuffer)
pub unsafe fn load_render_texture(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if a texture is ready
pub unsafe fn is_texture_ready(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Unload texture from GPU memory (VRAM)
pub unsafe fn unload_texture(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if a render texture is ready
pub unsafe fn is_render_texture_ready(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Unload render texture from GPU memory (VRAM)
pub unsafe fn unload_render_texture(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Update GPU texture with new data
pub unsafe fn update_texture(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Update GPU texture rectangle with new data
pub unsafe fn update_texture_rec(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Generate GPU mipmaps for a texture
pub unsafe fn gen_texture_mipmaps(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set texture scaling filter mode
pub unsafe fn set_texture_filter(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set texture wrapping mode
pub unsafe fn set_texture_wrap(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw a Texture2D
pub unsafe fn draw_texture(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw a Texture2D with position defined as Vector2
pub unsafe fn draw_texture_v(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw a Texture2D with extended parameters
pub unsafe fn draw_texture_ex(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw a part of a texture defined by a rectangle
pub unsafe fn draw_texture_rec(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw a part of a texture defined by a rectangle with 'pro' parameters
pub unsafe fn draw_texture_pro(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draws a texture (or part of it) that stretches or shrinks nicely
pub unsafe fn draw_texture_n_patch(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get color with alpha applied, alpha goes from 0.0f to 1.0f
pub unsafe fn fade(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get hexadecimal value for a Color
pub unsafe fn color_to_int(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get Color normalized as float [0..1]
pub unsafe fn color_normalize(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get Color from normalized values [0..1]
pub unsafe fn color_from_normalized(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get HSV values for a Color, hue [0..360], saturation/value [0..1]
pub unsafe fn color_to_hsv(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get a Color from HSV values, hue [0..360], saturation/value [0..1]
pub unsafe fn color_from_hsv(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get color multiplied with another color
pub unsafe fn color_tint(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get color with brightness correction, brightness factor goes from -1.0f to 1.0f
pub unsafe fn color_brightness(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get color with contrast correction, contrast values between -1.0f and 1.0f
pub unsafe fn color_contrast(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get color with alpha applied, alpha goes from 0.0f to 1.0f
pub unsafe fn color_alpha(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get src alpha-blended into dst color with tint
pub unsafe fn color_alpha_blend(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get Color structure from hexadecimal value
pub unsafe fn get_color(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get Color from a source pixel pointer of certain format
pub unsafe fn get_pixel_color(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set color formatted into destination pixel pointer
pub unsafe fn set_pixel_color(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get pixel data size in bytes for certain format
pub unsafe fn get_pixel_data_size(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}
