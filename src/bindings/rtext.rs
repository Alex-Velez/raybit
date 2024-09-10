use super::{FnOutput, Memory, Pointer};
use crate::read;

/// Get the default Font
pub unsafe fn get_font_default(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load font from file into GPU memory (VRAM)
pub unsafe fn load_font(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load font from file with extended parameters, use NULL for codepoints and 0 for codepointCount to load the default character set
pub unsafe fn load_font_ex(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load font from Image (XNA style)
pub unsafe fn load_font_from_image(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load font from memory buffer, fileType refers to extension: i.e. '.ttf'
pub unsafe fn load_font_from_memory(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if a font is ready
pub unsafe fn is_font_ready(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load font data for further use
pub unsafe fn load_font_data(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Generate image font atlas using chars info
pub unsafe fn gen_image_font_atlas(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Unload font chars info data (RAM)
pub unsafe fn unload_font_data(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Unload font from GPU memory (VRAM)
pub unsafe fn unload_font(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Export font as code file, returns true on success
pub unsafe fn export_font_as_code(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw current FPS
pub unsafe fn draw_fps(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw text (using default font)
pub unsafe fn draw_text(mem: &mut Memory, index: Pointer) -> FnOutput {
    // [@](text: str, posX: i32, posY: i32, fontSize: i32, color: Color)
    // [text:ptr][posX:ptr][posY:ptr][fontSize:ptr][color:ptr]
    let arg_ptrs = read::ptrs::<5>(mem, index);
    let text = read::c_string(mem, arg_ptrs[0])?;
    let pos_x = read::i32(mem, arg_ptrs[1]);
    let pos_y = read::i32(mem, arg_ptrs[2]);
    let font_size = read::i32(mem, arg_ptrs[3]);
    let color = read::color(mem, arg_ptrs[4]);
    raylib_ffi::DrawText(text.as_ptr(), pos_x, pos_y, font_size, color);
    Ok(None)
}

/// Draw text using font and additional parameters
pub unsafe fn draw_text_ex(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw text using Font and pro parameters (rotation)
pub unsafe fn draw_text_pro(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw one character (codepoint)
pub unsafe fn draw_text_codepoint(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw multiple character (codepoint)
pub unsafe fn draw_text_codepoints(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set vertical line spacing when drawing with line-breaks
pub unsafe fn set_text_line_spacing(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Measure string width for default font
pub unsafe fn measure_text(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Measure string size for Font
pub unsafe fn measure_text_ex(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get glyph index position in font for a codepoint (unicode character), fallback to '?' if not found
pub unsafe fn get_glyph_index(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get glyph font info data for a codepoint (unicode character), fallback to '?' if not found
pub unsafe fn get_glyph_info(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get glyph rectangle in font atlas for a codepoint (unicode character), fallback to '?' if not found
pub unsafe fn get_glyph_atlas_rec(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load UTF-8 text encoded from codepoints array
pub unsafe fn load_utf8(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Unload UTF-8 text encoded from codepoints array
pub unsafe fn unload_utf8(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load all codepoints from a UTF-8 text string, codepoints count returned by parameter
pub unsafe fn load_codepoints(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Unload codepoints data from memory
pub unsafe fn unload_codepoints(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get total number of codepoints in a UTF-8 encoded string
pub unsafe fn get_codepoint_count(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get next codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
pub unsafe fn get_codepoint(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get next codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
pub unsafe fn get_codepoint_next(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get previous codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
pub unsafe fn get_codepoint_previous(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Encode one codepoint into UTF-8 byte array (array length returned as parameter)
pub unsafe fn codepoint_to_utf8(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Copy one string to another, returns bytes copied
pub unsafe fn text_copy(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if two text string are equal
pub unsafe fn text_is_equal(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get text length, checks for '\0' ending
pub unsafe fn text_length(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Text formatting with variables (sprintf() style)
pub unsafe fn text_format(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get a piece of a text string
pub unsafe fn text_subtext(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Replace text string (WARNING: memory must be freed!)
pub unsafe fn text_replace(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Insert text in a position (WARNING: memory must be freed!)
pub unsafe fn text_insert(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Join text strings with delimiter
pub unsafe fn text_join(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Split text into multiple strings
pub unsafe fn text_split(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Append text at specific position and move cursor!
pub unsafe fn text_append(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Find first text occurrence within a string
pub unsafe fn text_find_index(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get upper case version of provided string
pub unsafe fn text_to_upper(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get lower case version of provided string
pub unsafe fn text_to_lower(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get Pascal case notation version of provided string
pub unsafe fn text_to_pascal(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get integer value from text (negative values not supported)
pub unsafe fn text_to_integer(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}
