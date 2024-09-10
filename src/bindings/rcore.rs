use super::{FnOutput, Memory, Pointer};
use crate::read;

/// Initialize window and OpenGL context
pub unsafe fn init_window(mem: &mut Memory, index: Pointer) -> FnOutput {
    // [@](width: i32, height: i32, title: str)
    // [width:ptr][height:ptr][str:ptr]
    let arg_ptrs = read::ptrs::<3>(mem, index);
    let width = read::i32(mem, arg_ptrs[0]);
    let height = read::i32(mem, arg_ptrs[1]);
    let title = read::c_string(mem, arg_ptrs[2])?;
    raylib_ffi::InitWindow(width, height, title.as_ptr());
    Ok(None)
}

/// Close window and unload OpenGL context
pub unsafe fn close_window(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    // [@]()
    raylib_ffi::CloseWindow();
    Ok(None)
}

/// Check if application should close (KEY_ESCAPE pressed or windows close icon clicked)
pub unsafe fn window_should_close(mem: &mut Memory, index: Pointer) -> FnOutput {
    // [@]() -> bool
    // [output:ptr]
    let arg_ptrs = read::ptrs::<1>(mem, index);
    let output_ptr = arg_ptrs[0];
    let result = raylib_ffi::WindowShouldClose();
    let output_data = vec![result as u8];
    Ok(Some((output_ptr, output_data)))
}

/// Check if window has been initialized successfully
pub unsafe fn is_window_ready(mem: &mut Memory, index: Pointer) -> FnOutput {
    // [@]() -> bool
    let arg_ptrs = read::ptrs::<1>(mem, index);
    let output_ptr = arg_ptrs[0];
    let result = raylib_ffi::IsWindowReady();
    let output_data = vec![result as u8];
    Ok(Some((output_ptr, output_data)))
}

/// Check if window is currently fullscreen
pub unsafe fn is_window_fullscreen(mem: &mut Memory, index: Pointer) -> FnOutput {
    // [@]() -> bool
    let arg_ptrs = read::ptrs::<1>(mem, index);
    let output_ptr = arg_ptrs[0];
    let result = raylib_ffi::IsWindowFullscreen();
    let output_data = vec![result as u8];
    Ok(Some((output_ptr, output_data)))
}

/// Check if window is currently hidden (only PLATFORM_DESKTOP)
pub unsafe fn is_window_hidden(mem: &mut Memory, index: Pointer) -> FnOutput {
    // [@]() -> bool
    let arg_ptrs = read::ptrs::<1>(mem, index);
    let output_ptr = arg_ptrs[0];
    let result = raylib_ffi::IsWindowHidden();
    let output_data = vec![result as u8];
    Ok(Some((output_ptr, output_data)))
}

/// Check if window is currently minimized (only PLATFORM_DESKTOP)
pub unsafe fn is_window_minimized(mem: &mut Memory, index: Pointer) -> FnOutput {
    // [@]() -> bool
    let arg_ptrs = read::ptrs::<1>(mem, index);
    let output_ptr = arg_ptrs[0];
    let result = raylib_ffi::IsWindowMinimized();
    let output_data = vec![result as u8];
    Ok(Some((output_ptr, output_data)))
}

/// Check if window is currently maximized (only PLATFORM_DESKTOP)
pub unsafe fn is_window_maximized(mem: &mut Memory, index: Pointer) -> FnOutput {
    // [@]() -> bool
    let arg_ptrs = read::ptrs::<1>(mem, index);
    let output_ptr = arg_ptrs[0];
    let result = raylib_ffi::IsWindowMaximized();
    let output_data = vec![result as u8];
    Ok(Some((output_ptr, output_data)))
}

/// Check if window is currently focused (only PLATFORM_DESKTOP)
pub unsafe fn is_window_focused(mem: &mut Memory, index: Pointer) -> FnOutput {
    // [@]() -> bool
    let arg_ptrs = read::ptrs::<1>(mem, index);
    let output_ptr = arg_ptrs[0];
    let result = raylib_ffi::IsWindowFocused();
    let output_data = vec![result as u8];
    Ok(Some((output_ptr, output_data)))
}

/// Check if window has been resized last frame
pub unsafe fn is_window_resized(mem: &mut Memory, index: Pointer) -> FnOutput {
    // [@]() -> bool
    let arg_ptrs = read::ptrs::<1>(mem, index);
    let output_ptr = arg_ptrs[0];
    let result = raylib_ffi::IsWindowResized();
    let output_data = vec![result as u8];
    Ok(Some((output_ptr, output_data)))
}

/// Check if one specific window flag is enabled
pub unsafe fn is_window_state(mem: &mut Memory, index: Pointer) -> FnOutput {
    // [@](flag: u32) -> bool
    let arg_ptrs = read::ptrs::<2>(mem, index);
    let flag = read::u32(mem, arg_ptrs[0]);
    let output_ptr = arg_ptrs[1];
    let result = raylib_ffi::IsWindowState(flag);
    let output_data = vec![result as u8];
    Ok(Some((output_ptr, output_data)))
}

/// Set window configuration state using flags (only PLATFORM_DESKTOP)
pub unsafe fn set_window_state(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Clear window configuration state flags
pub unsafe fn clear_window_state(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Toggle window state: fullscreen/windowed (only PLATFORM_DESKTOP)
pub unsafe fn toggle_fullscreen(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Toggle window state: borderless windowed (only PLATFORM_DESKTOP)
pub unsafe fn toggle_borderless_windowed(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set window state: maximized, if resizable (only PLATFORM_DESKTOP)
pub unsafe fn maximize_window(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set window state: minimized, if resizable (only PLATFORM_DESKTOP)
pub unsafe fn minimize_window(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set window state: not minimized/maximized (only PLATFORM_DESKTOP)
pub unsafe fn restore_window(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set icon for window (single image, RGBA 32bit, only PLATFORM_DESKTOP)
pub unsafe fn set_window_icon(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set icon for window (multiple images, RGBA 32bit, only PLATFORM_DESKTOP)
pub unsafe fn set_window_icons(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set title for window (only PLATFORM_DESKTOP and PLATFORM_WEB)
pub unsafe fn set_window_title(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set window position on screen (only PLATFORM_DESKTOP)
pub unsafe fn set_window_position(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set monitor for the current window
pub unsafe fn set_window_monitor(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set window minimum dimensions (for FLAG_WINDOW_RESIZABLE)
pub unsafe fn set_window_min_size(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set window maximum dimensions (for FLAG_WINDOW_RESIZABLE)
pub unsafe fn set_window_max_size(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set window dimensions
pub unsafe fn set_window_size(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set window opacity [0.0f..1.0f] (only PLATFORM_DESKTOP)
pub unsafe fn set_window_opacity(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set window focused (only PLATFORM_DESKTOP)
pub unsafe fn set_window_focused(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get native window handle
pub unsafe fn get_window_handle(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get current screen width
pub unsafe fn get_screen_width(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get current screen height
pub unsafe fn get_screen_height(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get current render width (it considers HiDPI)
pub unsafe fn get_render_width(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get current render height (it considers HiDPI)
pub unsafe fn get_render_height(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get number of connected monitors
pub unsafe fn get_monitor_count(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get current connected monitor
pub unsafe fn get_current_monitor(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get specified monitor position
pub unsafe fn get_monitor_position(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get specified monitor width (current video mode used by monitor)
pub unsafe fn get_monitor_width(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get specified monitor height (current video mode used by monitor)
pub unsafe fn get_monitor_height(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get specified monitor physical width in millimetres
pub unsafe fn get_monitor_physical_width(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get specified monitor physical height in millimetres
pub unsafe fn get_monitor_physical_height(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get specified monitor refresh rate
pub unsafe fn get_monitor_refresh_rate(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get window position XY on monitor
pub unsafe fn get_window_position(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get window scale DPI factor
pub unsafe fn get_window_scale_dpi(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get the human-readable, UTF-8 encoded name of the specified monitor
pub unsafe fn get_monitor_name(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set clipboard text content
pub unsafe fn set_clipboard_text(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get clipboard text content
pub unsafe fn get_clipboard_text(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Enable waiting for events on EndDrawing(), no automatic event polling
pub unsafe fn enable_event_waiting(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Disable waiting for events on EndDrawing(), automatic events polling
pub unsafe fn disable_event_waiting(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Shows cursor
pub unsafe fn show_cursor(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Hides cursor
pub unsafe fn hide_cursor(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if cursor is not visible
pub unsafe fn is_cursor_hidden(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Enables cursor (unlock cursor)
pub unsafe fn enable_cursor(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Disables cursor (lock cursor)
pub unsafe fn disable_cursor(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if cursor is on the screen
pub unsafe fn is_cursor_on_screen(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set background color (framebuffer clear color)
pub unsafe fn clear_background(mem: &mut Memory, index: Pointer) -> FnOutput {
    // [@](color: Color)
    // [color:ptr]
    let arg_ptrs = read::ptrs::<1>(mem, index);
    let color = read::color(mem, arg_ptrs[0]);
    raylib_ffi::ClearBackground(color);
    Ok(None)
}

/// Setup canvas (framebuffer) to start drawing
pub unsafe fn begin_drawing(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    // [@]()
    raylib_ffi::BeginDrawing();
    Ok(None)
}

/// End canvas drawing and swap buffers (double buffering)
pub unsafe fn end_drawing(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    // [@]()
    raylib_ffi::EndDrawing();
    Ok(None)
}

/// Begin 2D mode with custom camera (2D)
pub unsafe fn begin_mode2_d(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Ends 2D mode with custom camera
pub unsafe fn end_mode2_d(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Begin 3D mode with custom camera (3D)
pub unsafe fn begin_mode3_d(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Ends 3D mode and returns to default 2D orthographic mode
pub unsafe fn end_mode3_d(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Begin drawing to render texture
pub unsafe fn begin_texture_mode(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Ends drawing to render texture
pub unsafe fn end_texture_mode(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Begin custom shader drawing
pub unsafe fn begin_shader_mode(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// End custom shader drawing (use default shader)
pub unsafe fn end_shader_mode(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Begin blending mode (alpha, additive, multiplied, subtract, custom)
pub unsafe fn begin_blend_mode(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// End blending mode (reset to default: alpha blending)
pub unsafe fn end_blend_mode(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Begin scissor mode (define screen area for following drawing)
pub unsafe fn begin_scissor_mode(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// End scissor mode
pub unsafe fn end_scissor_mode(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Begin stereo rendering (requires VR simulator)
pub unsafe fn begin_vr_stereo_mode(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// End stereo rendering (requires VR simulator)
pub unsafe fn end_vr_stereo_mode(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load VR stereo config for VR simulator device parameters
pub unsafe fn load_vr_stereo_config(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Unload VR stereo config
pub unsafe fn unload_vr_stereo_config(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load shader from files and bind default locations
pub unsafe fn load_shader(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load shader from code strings and bind default locations
pub unsafe fn load_shader_from_memory(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if a shader is ready
pub unsafe fn is_shader_ready(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get shader uniform location
pub unsafe fn get_shader_location(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get shader attribute location
pub unsafe fn get_shader_location_attrib(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set shader uniform value
pub unsafe fn set_shader_value(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set shader uniform value vector
pub unsafe fn set_shader_value_v(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set shader uniform value (matrix 4x4)
pub unsafe fn set_shader_value_matrix(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set shader uniform value for texture (sampler2d)
pub unsafe fn set_shader_value_texture(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Unload shader from GPU memory (VRAM)
pub unsafe fn unload_shader(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get a ray trace from mouse position
pub unsafe fn get_mouse_ray(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get camera transform matrix (view matrix)
pub unsafe fn get_camera_matrix(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get camera 2d transform matrix
pub unsafe fn get_camera_matrix2_d(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get the screen space position for a 3d world space position
pub unsafe fn get_world_to_screen(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get the world space position for a 2d camera screen space position
pub unsafe fn get_screen_to_world2_d(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get size position for a 3d world space position
pub unsafe fn get_world_to_screen_ex(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get the screen space position for a 2d camera world space position
pub unsafe fn get_world_to_screen2_d(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set target FPS (maximum)
pub unsafe fn set_target_fps(mem: &mut Memory, index: Pointer) -> FnOutput {
    // [@](fps: i32)
    // [fps:ptr]
    let arg_ptrs = read::ptrs::<1>(mem, index);
    let fps = read::i32(mem, arg_ptrs[0]);
    raylib_ffi::SetTargetFPS(fps);
    Ok(None)
}

/// Get time in seconds for last frame drawn (delta time)
pub unsafe fn get_frame_time(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get elapsed time in seconds since InitWindow()
pub unsafe fn get_time(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get current FPS
pub unsafe fn get_fps(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Swap back buffer with front buffer (screen drawing)
pub unsafe fn swap_screen_buffer(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Register all input events
pub unsafe fn poll_input_events(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Wait for some time (halt program execution)
pub unsafe fn wait_time(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set the seed for the random number generator
pub unsafe fn set_random_seed(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get a random value between min and max (both included)
pub unsafe fn get_random_value(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load random values sequence, no values repeated
pub unsafe fn load_random_sequence(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Unload random values sequence
pub unsafe fn unload_random_sequence(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Takes a screenshot of current screen (filename extension defines format)
pub unsafe fn take_screenshot(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Setup init configuration flags (view FLAGS)
pub unsafe fn set_config_flags(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Open URL with default system browser (if available)
pub unsafe fn open_url(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Show trace log messages (LOG_DEBUG, LOG_INFO, LOG_WARNING, LOG_ERROR...)
pub unsafe fn trace_log(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set the current threshold (minimum) log level
pub unsafe fn set_trace_log_level(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Internal memory allocator
pub unsafe fn mem_alloc(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Internal memory reallocator
pub unsafe fn mem_realloc(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Internal memory free
pub unsafe fn mem_free(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set custom trace log
pub unsafe fn set_trace_log_callback(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set custom file binary data loader
pub unsafe fn set_load_file_data_callback(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set custom file binary data saver
pub unsafe fn set_save_file_data_callback(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set custom file text data loader
pub unsafe fn set_load_file_text_callback(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set custom file text data saver
pub unsafe fn set_save_file_text_callback(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load file data as byte array (read)
pub unsafe fn load_file_data(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Unload file data allocated by LoadFileData()
pub unsafe fn unload_file_data(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Save data to file from byte array (write), returns true on success
pub unsafe fn save_file_data(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Export data to code (.h), returns true on success
pub unsafe fn export_data_as_code(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load text data from file (read), returns a '\0' terminated string
pub unsafe fn load_file_text(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Unload file text data allocated by LoadFileText()
pub unsafe fn unload_file_text(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Save text data to file (write), string must be '\0' terminated, returns true on success
pub unsafe fn save_file_text(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if file exists
pub unsafe fn file_exists(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if a directory path exists
pub unsafe fn directory_exists(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check file extension (including point: .png, .wav)
pub unsafe fn is_file_extension(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get file length in bytes (NOTE: GetFileSize() conflicts with windows.h)
pub unsafe fn get_file_length(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get pointer to extension for a filename string (includes dot: '.png')
pub unsafe fn get_file_extension(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get pointer to filename for a path string
pub unsafe fn get_file_name(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get filename string without extension (uses static string)
pub unsafe fn get_file_name_without_ext(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get full path for a given fileName with path (uses static string)
pub unsafe fn get_directory_path(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get previous directory path for a given path (uses static string)
pub unsafe fn get_prev_directory_path(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get current working directory (uses static string)
pub unsafe fn get_working_directory(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get the directory of the running application (uses static string)
pub unsafe fn get_application_directory(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Change working directory, return true on success
pub unsafe fn change_directory(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if a given path is a file or a directory
pub unsafe fn is_path_file(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load directory filepaths
pub unsafe fn load_directory_files(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load directory filepaths with extension filtering and recursive directory scan
pub unsafe fn load_directory_files_ex(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Unload filepaths
pub unsafe fn unload_directory_files(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if a file has been dropped into window
pub unsafe fn is_file_dropped(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load dropped filepaths
pub unsafe fn load_dropped_files(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Unload dropped filepaths
pub unsafe fn unload_dropped_files(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get file modification time (last write time)
pub unsafe fn get_file_mod_time(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Compress data (DEFLATE algorithm), memory must be MemFree()
pub unsafe fn compress_data(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Decompress data (DEFLATE algorithm), memory must be MemFree()
pub unsafe fn decompress_data(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Encode data to Base64 string, memory must be MemFree()
pub unsafe fn encode_data_base64(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Decode Base64 string data, memory must be MemFree()
pub unsafe fn decode_data_base64(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Load automation events list from file, NULL for empty list, capacity = MAX_AUTOMATION_EVENTS
pub unsafe fn load_automation_event_list(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Unload automation events list from file
pub unsafe fn unload_automation_event_list(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Export automation events list as text file
pub unsafe fn export_automation_event_list(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set automation event list to record to
pub unsafe fn set_automation_event_list(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set automation event internal base frame to start recording
pub unsafe fn set_automation_event_base_frame(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Start recording automation events (AutomationEventList must be set)
pub unsafe fn start_automation_event_recording(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Stop recording automation events
pub unsafe fn stop_automation_event_recording(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Play a recorded automation event
pub unsafe fn play_automation_event(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if a key has been pressed once
pub unsafe fn is_key_pressed(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if a key has been pressed again (Only PLATFORM_DESKTOP)
pub unsafe fn is_key_pressed_repeat(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if a key is being pressed
pub unsafe fn is_key_down(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if a key has been released once
pub unsafe fn is_key_released(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if a key is NOT being pressed
pub unsafe fn is_key_up(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get key pressed (keycode), call it multiple times for keys queued, returns 0 when the queue is empty
pub unsafe fn get_key_pressed(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get char pressed (unicode), call it multiple times for chars queued, returns 0 when the queue is empty
pub unsafe fn get_char_pressed(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set a custom key to exit program (default is ESC)
pub unsafe fn set_exit_key(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if a gamepad is available
pub unsafe fn is_gamepad_available(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get gamepad internal name id
pub unsafe fn get_gamepad_name(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if a gamepad button has been pressed once
pub unsafe fn is_gamepad_button_pressed(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if a gamepad button is being pressed
pub unsafe fn is_gamepad_button_down(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if a gamepad button has been released once
pub unsafe fn is_gamepad_button_released(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if a gamepad button is NOT being pressed
pub unsafe fn is_gamepad_button_up(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get the last gamepad button pressed
pub unsafe fn get_gamepad_button_pressed(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get gamepad axis count for a gamepad
pub unsafe fn get_gamepad_axis_count(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get axis movement value for a gamepad axis
pub unsafe fn get_gamepad_axis_movement(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set internal gamepad mappings (SDL_GameControllerDB)
pub unsafe fn set_gamepad_mappings(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if a mouse button has been pressed once
pub unsafe fn is_mouse_button_pressed(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if a mouse button is being pressed
pub unsafe fn is_mouse_button_down(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if a mouse button has been released once
pub unsafe fn is_mouse_button_released(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if a mouse button is NOT being pressed
pub unsafe fn is_mouse_button_up(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get mouse position X
pub unsafe fn get_mouse_x(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get mouse position Y
pub unsafe fn get_mouse_y(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get mouse position XY
pub unsafe fn get_mouse_position(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get mouse delta between frames
pub unsafe fn get_mouse_delta(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set mouse position XY
pub unsafe fn set_mouse_position(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set mouse offset
pub unsafe fn set_mouse_offset(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set mouse scaling
pub unsafe fn set_mouse_scale(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get mouse wheel movement for X or Y, whichever is larger
pub unsafe fn get_mouse_wheel_move(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get mouse wheel movement for both X and Y
pub unsafe fn get_mouse_wheel_move_v(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Set mouse cursor
pub unsafe fn set_mouse_cursor(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get touch position X for touch point 0 (relative to screen size)
pub unsafe fn get_touch_x(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get touch position Y for touch point 0 (relative to screen size)
pub unsafe fn get_touch_y(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get touch position XY for a touch point index (relative to screen size)
pub unsafe fn get_touch_position(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get touch point identifier for given index
pub unsafe fn get_touch_point_id(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get number of touch points
pub unsafe fn get_touch_point_count(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Enable a set of gestures using flags
pub unsafe fn set_gestures_enabled(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if a gesture have been detected
pub unsafe fn is_gesture_detected(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get latest detected gesture
pub unsafe fn get_gesture_detected(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get gesture hold time in milliseconds
pub unsafe fn get_gesture_hold_duration(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get gesture drag vector
pub unsafe fn get_gesture_drag_vector(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get gesture drag angle
pub unsafe fn get_gesture_drag_angle(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get gesture pinch delta
pub unsafe fn get_gesture_pinch_vector(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get gesture pinch angle
pub unsafe fn get_gesture_pinch_angle(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Update camera position for selected mode
pub unsafe fn update_camera(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Update camera movement/rotation
pub unsafe fn update_camera_pro(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}
