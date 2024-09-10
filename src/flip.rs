use crate::{bbio, bbmem};

const FN_MAP: [unsafe fn(memory: &mut Vec<u8>, pointer: usize, width: usize) -> Option<Vec<u8>>;
    552] = [
    attach_audio_mixed_processor,
    attach_audio_stream_processor,
    begin_blend_mode,
    begin_drawing,
    begin_mode2_d,
    begin_mode3_d,
    begin_scissor_mode,
    begin_shader_mode,
    begin_texture_mode,
    begin_vr_stereo_mode,
    change_directory,
    check_collision_box_sphere,
    check_collision_boxes,
    check_collision_circle_rec,
    check_collision_circles,
    check_collision_lines,
    check_collision_point_circle,
    check_collision_point_line,
    check_collision_point_poly,
    check_collision_point_rec,
    check_collision_point_triangle,
    check_collision_recs,
    check_collision_spheres,
    clear_background,
    clear_window_state,
    close_audio_device,
    close_window,
    codepoint_to_u_t_f8,
    color_alpha,
    color_alpha_blend,
    color_brightness,
    color_contrast,
    color_from_h_s_v,
    color_from_normalized,
    color_normalize,
    color_tint,
    color_to_h_s_v,
    color_to_int,
    compress_data,
    decode_data_base64,
    decompress_data,
    detach_audio_mixed_processor,
    detach_audio_stream_processor,
    directory_exists,
    disable_cursor,
    disable_event_waiting,
    draw_billboard,
    draw_billboard_pro,
    draw_billboard_rec,
    draw_bounding_box,
    draw_capsule,
    draw_capsule_wires,
    draw_circle,
    draw_circle3_d,
    draw_circle_gradient,
    draw_circle_lines,
    draw_circle_lines_v,
    draw_circle_sector,
    draw_circle_sector_lines,
    draw_circle_v,
    draw_cube,
    draw_cube_v,
    draw_cube_wires,
    draw_cube_wires_v,
    draw_cylinder,
    draw_cylinder_ex,
    draw_cylinder_wires,
    draw_cylinder_wires_ex,
    draw_ellipse,
    draw_ellipse_lines,
    draw_f_p_s,
    draw_grid,
    draw_line,
    draw_line3_d,
    draw_line_bezier,
    draw_line_ex,
    draw_line_strip,
    draw_line_v,
    draw_mesh,
    draw_mesh_instanced,
    draw_model,
    draw_model_ex,
    draw_model_wires,
    draw_model_wires_ex,
    draw_pixel,
    draw_pixel_v,
    draw_plane,
    draw_point3_d,
    draw_poly,
    draw_poly_lines,
    draw_poly_lines_ex,
    draw_ray,
    draw_rectangle,
    draw_rectangle_gradient_ex,
    draw_rectangle_gradient_h,
    draw_rectangle_gradient_v,
    draw_rectangle_lines,
    draw_rectangle_lines_ex,
    draw_rectangle_pro,
    draw_rectangle_rec,
    draw_rectangle_rounded,
    draw_rectangle_rounded_lines,
    draw_rectangle_v,
    draw_ring,
    draw_ring_lines,
    draw_sphere,
    draw_sphere_ex,
    draw_sphere_wires,
    draw_spline_basis,
    draw_spline_bezier_cubic,
    draw_spline_bezier_quadratic,
    draw_spline_catmull_rom,
    draw_spline_linear,
    draw_spline_segment_basis,
    draw_spline_segment_bezier_cubic,
    draw_spline_segment_bezier_quadratic,
    draw_spline_segment_catmull_rom,
    draw_spline_segment_linear,
    draw_text,
    draw_text_codepoint,
    draw_text_codepoints,
    draw_text_ex,
    draw_text_pro,
    draw_texture,
    draw_texture_ex,
    draw_texture_n_patch,
    draw_texture_pro,
    draw_texture_rec,
    draw_texture_v,
    draw_triangle,
    draw_triangle3_d,
    draw_triangle_fan,
    draw_triangle_lines,
    draw_triangle_strip,
    draw_triangle_strip3_d,
    enable_cursor,
    enable_event_waiting,
    encode_data_base64,
    end_blend_mode,
    end_drawing,
    end_mode2_d,
    end_mode3_d,
    end_scissor_mode,
    end_shader_mode,
    end_texture_mode,
    end_vr_stereo_mode,
    export_automation_event_list,
    export_data_as_code,
    export_font_as_code,
    export_image,
    export_image_as_code,
    export_image_to_memory,
    export_mesh,
    export_wave,
    export_wave_as_code,
    fade,
    file_exists,
    gen_image_cellular,
    gen_image_checked,
    gen_image_color,
    gen_image_font_atlas,
    gen_image_gradient_linear,
    gen_image_gradient_radial,
    gen_image_gradient_square,
    gen_image_perlin_noise,
    gen_image_text,
    gen_image_white_noise,
    gen_mesh_cone,
    gen_mesh_cube,
    gen_mesh_cubicmap,
    gen_mesh_cylinder,
    gen_mesh_heightmap,
    gen_mesh_hemi_sphere,
    gen_mesh_knot,
    gen_mesh_plane,
    gen_mesh_poly,
    gen_mesh_sphere,
    gen_mesh_tangents,
    gen_mesh_torus,
    gen_texture_mipmaps,
    get_application_directory,
    get_camera_matrix,
    get_camera_matrix2_d,
    get_char_pressed,
    get_clipboard_text,
    get_codepoint,
    get_codepoint_count,
    get_codepoint_next,
    get_codepoint_previous,
    get_collision_rec,
    get_color,
    get_current_monitor,
    get_directory_path,
    get_f_p_s,
    get_file_extension,
    get_file_length,
    get_file_mod_time,
    get_file_name,
    get_file_name_without_ext,
    get_font_default,
    get_frame_time,
    get_gamepad_axis_count,
    get_gamepad_axis_movement,
    get_gamepad_button_pressed,
    get_gamepad_name,
    get_gesture_detected,
    get_gesture_drag_angle,
    get_gesture_drag_vector,
    get_gesture_hold_duration,
    get_gesture_pinch_angle,
    get_gesture_pinch_vector,
    get_glyph_atlas_rec,
    get_glyph_index,
    get_glyph_info,
    get_image_alpha_border,
    get_image_color,
    get_key_pressed,
    get_master_volume,
    get_mesh_bounding_box,
    get_model_bounding_box,
    get_monitor_count,
    get_monitor_height,
    get_monitor_name,
    get_monitor_physical_height,
    get_monitor_physical_width,
    get_monitor_position,
    get_monitor_refresh_rate,
    get_monitor_width,
    get_mouse_delta,
    get_mouse_position,
    get_mouse_ray,
    get_mouse_wheel_move,
    get_mouse_wheel_move_v,
    get_mouse_x,
    get_mouse_y,
    get_music_time_length,
    get_music_time_played,
    get_pixel_color,
    get_pixel_data_size,
    get_prev_directory_path,
    get_random_value,
    get_ray_collision_box,
    get_ray_collision_mesh,
    get_ray_collision_quad,
    get_ray_collision_sphere,
    get_ray_collision_triangle,
    get_render_height,
    get_render_width,
    get_screen_height,
    get_screen_to_world2_d,
    get_screen_width,
    get_shader_location,
    get_shader_location_attrib,
    get_spline_point_basis,
    get_spline_point_bezier_cubic,
    get_spline_point_bezier_quad,
    get_spline_point_catmull_rom,
    get_spline_point_linear,
    get_time,
    get_touch_point_count,
    get_touch_point_id,
    get_touch_position,
    get_touch_x,
    get_touch_y,
    get_window_handle,
    get_window_position,
    get_window_scale_d_p_i,
    get_working_directory,
    get_world_to_screen,
    get_world_to_screen2_d,
    get_world_to_screen_ex,
    hide_cursor,
    image_alpha_clear,
    image_alpha_crop,
    image_alpha_mask,
    image_alpha_premultiply,
    image_blur_gaussian,
    image_clear_background,
    image_color_brightness,
    image_color_contrast,
    image_color_grayscale,
    image_color_invert,
    image_color_replace,
    image_color_tint,
    image_copy,
    image_crop,
    image_dither,
    image_draw,
    image_draw_circle,
    image_draw_circle_lines,
    image_draw_circle_lines_v,
    image_draw_circle_v,
    image_draw_line,
    image_draw_line_v,
    image_draw_pixel,
    image_draw_pixel_v,
    image_draw_rectangle,
    image_draw_rectangle_lines,
    image_draw_rectangle_rec,
    image_draw_rectangle_v,
    image_draw_text,
    image_draw_text_ex,
    image_flip_horizontal,
    image_flip_vertical,
    image_format,
    image_from_image,
    image_mipmaps,
    image_resize,
    image_resize_canvas,
    image_resize_n_n,
    image_rotate,
    image_rotate_c_c_w,
    image_rotate_c_w,
    image_text,
    image_text_ex,
    image_to_p_o_t,
    init_audio_device,
    init_window,
    is_audio_device_ready,
    is_audio_stream_playing,
    is_audio_stream_processed,
    is_audio_stream_ready,
    is_cursor_hidden,
    is_cursor_on_screen,
    is_file_dropped,
    is_file_extension,
    is_font_ready,
    is_gamepad_available,
    is_gamepad_button_down,
    is_gamepad_button_pressed,
    is_gamepad_button_released,
    is_gamepad_button_up,
    is_gesture_detected,
    is_image_ready,
    is_key_down,
    is_key_pressed,
    is_key_pressed_repeat,
    is_key_released,
    is_key_up,
    is_material_ready,
    is_model_animation_valid,
    is_model_ready,
    is_mouse_button_down,
    is_mouse_button_pressed,
    is_mouse_button_released,
    is_mouse_button_up,
    is_music_ready,
    is_music_stream_playing,
    is_path_file,
    is_render_texture_ready,
    is_shader_ready,
    is_sound_playing,
    is_sound_ready,
    is_texture_ready,
    is_wave_ready,
    is_window_focused,
    is_window_fullscreen,
    is_window_hidden,
    is_window_maximized,
    is_window_minimized,
    is_window_ready,
    is_window_resized,
    is_window_state,
    load_audio_stream,
    load_automation_event_list,
    load_codepoints,
    load_directory_files,
    load_directory_files_ex,
    load_dropped_files,
    load_file_data,
    load_file_text,
    load_font,
    load_font_data,
    load_font_ex,
    load_font_from_image,
    load_font_from_memory,
    load_image,
    load_image_anim,
    load_image_colors,
    load_image_from_memory,
    load_image_from_screen,
    load_image_from_texture,
    load_image_palette,
    load_image_raw,
    load_image_svg,
    load_material_default,
    load_materials,
    load_model,
    load_model_animations,
    load_model_from_mesh,
    load_music_stream,
    load_music_stream_from_memory,
    load_random_sequence,
    load_render_texture,
    load_shader,
    load_shader_from_memory,
    load_sound,
    load_sound_alias,
    load_sound_from_wave,
    load_texture,
    load_texture_cubemap,
    load_texture_from_image,
    load_u_t_f8,
    load_vr_stereo_config,
    load_wave,
    load_wave_from_memory,
    load_wave_samples,
    maximize_window,
    measure_text,
    measure_text_ex,
    mem_alloc,
    mem_free,
    mem_realloc,
    minimize_window,
    open_u_r_l,
    pause_audio_stream,
    pause_music_stream,
    pause_sound,
    play_audio_stream,
    play_automation_event,
    play_music_stream,
    play_sound,
    poll_input_events,
    restore_window,
    resume_audio_stream,
    resume_music_stream,
    resume_sound,
    save_file_data,
    save_file_text,
    seek_music_stream,
    set_audio_stream_buffer_size_default,
    set_audio_stream_callback,
    set_audio_stream_pan,
    set_audio_stream_pitch,
    set_audio_stream_volume,
    set_automation_event_base_frame,
    set_automation_event_list,
    set_clipboard_text,
    set_config_flags,
    set_exit_key,
    set_gamepad_mappings,
    set_gestures_enabled,
    set_load_file_data_callback,
    set_load_file_text_callback,
    set_master_volume,
    set_material_texture,
    set_model_mesh_material,
    set_mouse_cursor,
    set_mouse_offset,
    set_mouse_position,
    set_mouse_scale,
    set_music_pan,
    set_music_pitch,
    set_music_volume,
    set_pixel_color,
    set_random_seed,
    set_save_file_data_callback,
    set_save_file_text_callback,
    set_shader_value,
    set_shader_value_matrix,
    set_shader_value_texture,
    set_shader_value_v,
    set_shapes_texture,
    set_sound_pan,
    set_sound_pitch,
    set_sound_volume,
    set_target_f_p_s,
    set_text_line_spacing,
    set_texture_filter,
    set_texture_wrap,
    set_trace_log_callback,
    set_trace_log_level,
    set_window_focused,
    set_window_icon,
    set_window_icons,
    set_window_max_size,
    set_window_min_size,
    set_window_monitor,
    set_window_opacity,
    set_window_position,
    set_window_size,
    set_window_state,
    set_window_title,
    show_cursor,
    start_automation_event_recording,
    stop_audio_stream,
    stop_automation_event_recording,
    stop_music_stream,
    stop_sound,
    swap_screen_buffer,
    take_screenshot,
    text_append,
    text_copy,
    text_find_index,
    text_format,
    text_insert,
    text_is_equal,
    text_join,
    text_length,
    text_replace,
    text_split,
    text_subtext,
    text_to_integer,
    text_to_lower,
    text_to_pascal,
    text_to_upper,
    toggle_borderless_windowed,
    toggle_fullscreen,
    trace_log,
    unload_audio_stream,
    unload_automation_event_list,
    unload_codepoints,
    unload_directory_files,
    unload_dropped_files,
    unload_file_data,
    unload_file_text,
    unload_font,
    unload_font_data,
    unload_image,
    unload_image_colors,
    unload_image_palette,
    unload_material,
    unload_mesh,
    unload_model,
    unload_model_animation,
    unload_model_animations,
    unload_music_stream,
    unload_random_sequence,
    unload_render_texture,
    unload_shader,
    unload_sound,
    unload_sound_alias,
    unload_texture,
    unload_u_t_f8,
    unload_vr_stereo_config,
    unload_wave,
    unload_wave_samples,
    update_audio_stream,
    update_camera,
    update_camera_pro,
    update_mesh_buffer,
    update_model_animation,
    update_music_stream,
    update_sound,
    update_texture,
    update_texture_rec,
    upload_mesh,
    wait_time,
    wave_copy,
    wave_crop,
    wave_format,
    window_should_close,
];

pub unsafe fn call(id: usize, memory: &mut Vec<u8>, pointer: &usize, width: usize) {
    if id >= FN_MAP.len() {
        bbio::hintexit("function index out of bounds", id);
    }

    if let Some(result_cells) = FN_MAP[id](memory, *pointer, width) {
        for x in 1..=result_cells.len() {
            (*memory)[pointer - x] = result_cells[x - 1];
        }
    }
}

// Attach audio stream processor to the entire audio pipeline, receives the samples as s
unsafe fn attach_audio_mixed_processor(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Attach audio stream processor to stream, receives the samples as s
unsafe fn attach_audio_stream_processor(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Begin blending mode (alpha, additive, multiplied, subtract, custom)
unsafe fn begin_blend_mode(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Setup canvas (framebuffer) to start drawing
unsafe fn begin_drawing(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    raylib_ffi::BeginDrawing();
    None
}

// Begin 2D mode with custom camera (2D)
unsafe fn begin_mode2_d(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Begin 3D mode with custom camera (3D)
unsafe fn begin_mode3_d(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Begin scissor mode (define screen area for following drawing)
unsafe fn begin_scissor_mode(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Begin custom shader drawing
unsafe fn begin_shader_mode(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Begin drawing to render texture
unsafe fn begin_texture_mode(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Begin stereo rendering (requires VR simulator)
unsafe fn begin_vr_stereo_mode(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Change working directory, return true on success
unsafe fn change_directory(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check collision between box and sphere
unsafe fn check_collision_box_sphere(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check collision between two bounding boxes
unsafe fn check_collision_boxes(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check collision between circle and rectangle
unsafe fn check_collision_circle_rec(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check collision between two circles
unsafe fn check_collision_circles(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check the collision between two lines defined by two points each, returns collision point by reference
unsafe fn check_collision_lines(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if point is inside circle
unsafe fn check_collision_point_circle(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if point belongs to line created between two points [p1] and [p2] with defined margin in pixels [threshold]
unsafe fn check_collision_point_line(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if point is within a polygon described by array of vertices
unsafe fn check_collision_point_poly(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if point is inside rectangle
unsafe fn check_collision_point_rec(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if point is inside a triangle
unsafe fn check_collision_point_triangle(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check collision between two rectangles
unsafe fn check_collision_recs(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check collision between two spheres
unsafe fn check_collision_spheres(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set background color (framebuffer clear color)
unsafe fn clear_background(memory: &mut Vec<u8>, pointer: usize, width: usize) -> Option<Vec<u8>> {
    // [@](color: Color)
    // [@][color:ptr][color:ptr]
    let input_cells = bbmem::get_input_cells(memory, pointer, 1 * width);
    let color_ptr = bbmem::read_unsigned(memory, pointer, width);
    let color = bbmem::read_color(memory, color_ptr);
    raylib_ffi::ClearBackground(color);
    None
}

// Clear window configuration state flags
unsafe fn clear_window_state(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Close the audio device and context
unsafe fn close_audio_device(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Close window and unload OpenGL context
unsafe fn close_window(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    raylib_ffi::CloseWindow();
    None
}

// Encode one codepoint into UTF-8 byte array (array length returned as parameter)
unsafe fn codepoint_to_u_t_f8(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get color with alpha applied, alpha goes from 0.0f to 1.0f
unsafe fn color_alpha(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get src alpha-blended into dst color with tint
unsafe fn color_alpha_blend(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get color with brightness correction, brightness factor goes from -1.0f to 1.0f
unsafe fn color_brightness(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get color with contrast correction, contrast values between -1.0f and 1.0f
unsafe fn color_contrast(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get a Color from HSV values, hue [0..360], saturation/value [0..1]
unsafe fn color_from_h_s_v(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get Color from normalized values [0..1]
unsafe fn color_from_normalized(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get Color normalized as float [0..1]
unsafe fn color_normalize(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get color multiplied with another color
unsafe fn color_tint(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get HSV values for a Color, hue [0..360], saturation/value [0..1]
unsafe fn color_to_h_s_v(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get hexadecimal value for a Color
unsafe fn color_to_int(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Compress data (DEFLATE algorithm), memory must be MemFree()
unsafe fn compress_data(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Decode Base64 string data, memory must be MemFree()
unsafe fn decode_data_base64(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Decompress data (DEFLATE algorithm), memory must be MemFree()
unsafe fn decompress_data(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Detach audio stream processor from the entire audio pipeline
unsafe fn detach_audio_mixed_processor(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Detach audio stream processor from stream
unsafe fn detach_audio_stream_processor(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if a directory path exists
unsafe fn directory_exists(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Disables cursor (lock cursor)
unsafe fn disable_cursor(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Disable waiting for events on EndDrawing(), automatic events polling
unsafe fn disable_event_waiting(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a billboard texture
unsafe fn draw_billboard(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a billboard texture defined by source and rotation
unsafe fn draw_billboard_pro(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a billboard texture defined by source
unsafe fn draw_billboard_rec(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw bounding box (wires)
unsafe fn draw_bounding_box(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a capsule with the center of its sphere caps at startPos and endPos
unsafe fn draw_capsule(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw capsule wireframe with the center of its sphere caps at startPos and endPos
unsafe fn draw_capsule_wires(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a color-filled circle
unsafe fn draw_circle(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a circle in 3D world space
unsafe fn draw_circle3_d(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a gradient-filled circle
unsafe fn draw_circle_gradient(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw circle outline
unsafe fn draw_circle_lines(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw circle outline (Vector version)
unsafe fn draw_circle_lines_v(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a piece of a circle
unsafe fn draw_circle_sector(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw circle sector outline
unsafe fn draw_circle_sector_lines(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a color-filled circle (Vector version)
unsafe fn draw_circle_v(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw cube
unsafe fn draw_cube(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw cube (Vector version)
unsafe fn draw_cube_v(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw cube wires
unsafe fn draw_cube_wires(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw cube wires (Vector version)
unsafe fn draw_cube_wires_v(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a cylinder/cone
unsafe fn draw_cylinder(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a cylinder with base at startPos and top at endPos
unsafe fn draw_cylinder_ex(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a cylinder/cone wires
unsafe fn draw_cylinder_wires(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a cylinder wires with base at startPos and top at endPos
unsafe fn draw_cylinder_wires_ex(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw ellipse
unsafe fn draw_ellipse(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw ellipse outline
unsafe fn draw_ellipse_lines(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw current FPS
unsafe fn draw_f_p_s(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a grid (centered at (0, 0, 0))
unsafe fn draw_grid(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a line
unsafe fn draw_line(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a line in 3D world space
unsafe fn draw_line3_d(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw line segment cubic-bezier in-out interpolation
unsafe fn draw_line_bezier(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a line (using triangles/quads)
unsafe fn draw_line_ex(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw lines sequence (using gl lines)
unsafe fn draw_line_strip(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a line (using gl lines)
unsafe fn draw_line_v(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a 3d mesh with material and transform
unsafe fn draw_mesh(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw multiple mesh instances with material and different transforms
unsafe fn draw_mesh_instanced(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a model (with texture if set)
unsafe fn draw_model(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a model with extended parameters
unsafe fn draw_model_ex(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a model wires (with texture if set)
unsafe fn draw_model_wires(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a model wires (with texture if set) with extended parameters
unsafe fn draw_model_wires_ex(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a pixel
unsafe fn draw_pixel(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a pixel (Vector version)
unsafe fn draw_pixel_v(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a plane XZ
unsafe fn draw_plane(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a point in 3D space, actually a small line
unsafe fn draw_point3_d(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a regular polygon (Vector version)
unsafe fn draw_poly(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a polygon outline of n sides
unsafe fn draw_poly_lines(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a polygon outline of n sides with extended parameters
unsafe fn draw_poly_lines_ex(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a ray line
unsafe fn draw_ray(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a color-filled rectangle
unsafe fn draw_rectangle(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a gradient-filled rectangle with custom vertex colors
unsafe fn draw_rectangle_gradient_ex(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a horizontal-gradient-filled rectangle
unsafe fn draw_rectangle_gradient_h(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a vertical-gradient-filled rectangle
unsafe fn draw_rectangle_gradient_v(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw rectangle outline
unsafe fn draw_rectangle_lines(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw rectangle outline with extended parameters
unsafe fn draw_rectangle_lines_ex(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a color-filled rectangle with pro parameters
unsafe fn draw_rectangle_pro(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a color-filled rectangle
unsafe fn draw_rectangle_rec(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw rectangle with rounded edges
unsafe fn draw_rectangle_rounded(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw rectangle with rounded edges outline
unsafe fn draw_rectangle_rounded_lines(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a color-filled rectangle (Vector version)
unsafe fn draw_rectangle_v(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw ring
unsafe fn draw_ring(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw ring outline
unsafe fn draw_ring_lines(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw sphere
unsafe fn draw_sphere(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw sphere with extended parameters
unsafe fn draw_sphere_ex(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw sphere wires
unsafe fn draw_sphere_wires(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw spline: B-Spline, minimum 4 points
unsafe fn draw_spline_basis(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw spline: Cubic Bezier, minimum 4 points (2 control points): [p1, c2, c3, p4, c5, c6...]
unsafe fn draw_spline_bezier_cubic(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw spline: Quadratic Bezier, minimum 3 points (1 control point): [p1, c2, p3, c4...]
unsafe fn draw_spline_bezier_quadratic(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw spline: Catmull-Rom, minimum 4 points
unsafe fn draw_spline_catmull_rom(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw spline: Linear, minimum 2 points
unsafe fn draw_spline_linear(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw spline segment: B-Spline, 4 points
unsafe fn draw_spline_segment_basis(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw spline segment: Cubic Bezier, 2 points, 2 control points
unsafe fn draw_spline_segment_bezier_cubic(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw spline segment: Quadratic Bezier, 2 points, 1 control point
unsafe fn draw_spline_segment_bezier_quadratic(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw spline segment: Catmull-Rom, 4 points
unsafe fn draw_spline_segment_catmull_rom(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw spline segment: Linear, 2 points
unsafe fn draw_spline_segment_linear(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw text (using default font)
unsafe fn draw_text(memory: &mut Vec<u8>, pointer: usize, width: usize) -> Option<Vec<u8>> {
    // [@](text: str, x: i32, y: i32, fontSize: i32, color: Color)
    // [@][text:ptr][text:ptr][x:ptr][x:ptr][y:ptr][y:ptr][fontSize:ptr][fontSize:ptr][color:ptr][color:ptr]
    const ARGS: usize = 5;
    let input_cells = bbmem::get_input_cells(memory, pointer, ARGS * width);
    let text_ptr = bbmem::_read_ptr(memory, pointer, width);
    let pos_x_ptr = bbmem::read_unsigned(memory, pointer + width, width);

    let pos_x_ptr = bbmem::read_ptr(&input_cells[2..4]);
    let pos_y_ptr = bbmem::read_ptr(&input_cells[4..6]);
    let font_size_ptr = bbmem::read_ptr(&input_cells[6..8]);
    let color_ptr = bbmem::read_ptr(&input_cells[8..10]);
    let text = bbmem::read_string(memory, text_ptr);
    let pos_x = bbmem::read_i32(memory, pos_x_ptr);
    let pos_y = bbmem::read_i32(memory, pos_y_ptr);
    let font_size = bbmem::read_i32(memory, font_size_ptr);
    let color = bbmem::read_color(memory, color_ptr);
    raylib_ffi::DrawText(
        text.as_ptr() as *const std::os::raw::c_char,
        pos_x,
        pos_y,
        font_size,
        color,
    );
    None
}

// Draw one character (codepoint)
unsafe fn draw_text_codepoint(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw multiple character (codepoint)
unsafe fn draw_text_codepoints(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw text using font and additional parameters
unsafe fn draw_text_ex(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw text using Font and pro parameters (rotation)
unsafe fn draw_text_pro(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a Texture2D
unsafe fn draw_texture(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a Texture2D with extended parameters
unsafe fn draw_texture_ex(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draws a texture (or part of it) that stretches or shrinks nicely
unsafe fn draw_texture_n_patch(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a part of a texture defined by a rectangle with 'pro' parameters
unsafe fn draw_texture_pro(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a part of a texture defined by a rectangle
unsafe fn draw_texture_rec(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a Texture2D with position defined as Vector2
unsafe fn draw_texture_v(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a color-filled triangle (vertex in counter-clockwise order!)
unsafe fn draw_triangle(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a color-filled triangle (vertex in counter-clockwise order!)
unsafe fn draw_triangle3_d(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a triangle fan defined by points (first vertex is the center)
unsafe fn draw_triangle_fan(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw triangle outline (vertex in counter-clockwise order!)
unsafe fn draw_triangle_lines(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a triangle strip defined by points
unsafe fn draw_triangle_strip(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a triangle strip defined by points
unsafe fn draw_triangle_strip3_d(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Enables cursor (unlock cursor)
unsafe fn enable_cursor(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Enable waiting for events on EndDrawing(), no automatic event polling
unsafe fn enable_event_waiting(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Encode data to Base64 string, memory must be MemFree()
unsafe fn encode_data_base64(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// End blending mode (reset to default: alpha blending)
unsafe fn end_blend_mode(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// End canvas drawing and swap buffers (double buffering)
unsafe fn end_drawing(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    raylib_ffi::EndDrawing();
    None
}

// Ends 2D mode with custom camera
unsafe fn end_mode2_d(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Ends 3D mode and returns to default 2D orthographic mode
unsafe fn end_mode3_d(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// End scissor mode
unsafe fn end_scissor_mode(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// End custom shader drawing (use default shader)
unsafe fn end_shader_mode(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Ends drawing to render texture
unsafe fn end_texture_mode(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// End stereo rendering (requires VR simulator)
unsafe fn end_vr_stereo_mode(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Export automation events list as text file
unsafe fn export_automation_event_list(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Export data to code (.h), returns true on success
unsafe fn export_data_as_code(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Export font as code file, returns true on success
unsafe fn export_font_as_code(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Export image data to file, returns true on success
unsafe fn export_image(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Export image as code file defining an array of bytes, returns true on success
unsafe fn export_image_as_code(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Export image to memory buffer
unsafe fn export_image_to_memory(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Export mesh data to file, returns true on success
unsafe fn export_mesh(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Export wave data to file, returns true on success
unsafe fn export_wave(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Export wave sample data to code (.h), returns true on success
unsafe fn export_wave_as_code(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get color with alpha applied, alpha goes from 0.0f to 1.0f
unsafe fn fade(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if file exists
unsafe fn file_exists(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Generate image: cellular algorithm, bigger tileSize means bigger cells
unsafe fn gen_image_cellular(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Generate image: checked
unsafe fn gen_image_checked(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Generate image: plain color
unsafe fn gen_image_color(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Generate image font atlas using chars info
unsafe fn gen_image_font_atlas(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Generate image: linear gradient, direction in degrees [0..360], 0=Vertical gradient
unsafe fn gen_image_gradient_linear(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Generate image: radial gradient
unsafe fn gen_image_gradient_radial(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Generate image: square gradient
unsafe fn gen_image_gradient_square(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Generate image: perlin noise
unsafe fn gen_image_perlin_noise(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Generate image: grayscale image from text data
unsafe fn gen_image_text(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Generate image: white noise
unsafe fn gen_image_white_noise(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Generate cone/pyramid mesh
unsafe fn gen_mesh_cone(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Generate cuboid mesh
unsafe fn gen_mesh_cube(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Generate cubes-based map mesh from image data
unsafe fn gen_mesh_cubicmap(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Generate cylinder mesh
unsafe fn gen_mesh_cylinder(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Generate heightmap mesh from image data
unsafe fn gen_mesh_heightmap(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Generate half-sphere mesh (no bottom cap)
unsafe fn gen_mesh_hemi_sphere(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Generate trefoil knot mesh
unsafe fn gen_mesh_knot(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Generate plane mesh (with subdivisions)
unsafe fn gen_mesh_plane(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Generate polygonal mesh
unsafe fn gen_mesh_poly(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Generate sphere mesh (standard sphere)
unsafe fn gen_mesh_sphere(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Compute mesh tangents
unsafe fn gen_mesh_tangents(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Generate torus mesh
unsafe fn gen_mesh_torus(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Generate GPU mipmaps for a texture
unsafe fn gen_texture_mipmaps(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get the directory of the running application (uses static string)
unsafe fn get_application_directory(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get camera transform matrix (view matrix)
unsafe fn get_camera_matrix(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get camera 2d transform matrix
unsafe fn get_camera_matrix2_d(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get char pressed (unicode), call it multiple times for chars queued, returns 0 when the queue is empty
unsafe fn get_char_pressed(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get clipboard text content
unsafe fn get_clipboard_text(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get next codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
unsafe fn get_codepoint(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get total number of codepoints in a UTF-8 encoded string
unsafe fn get_codepoint_count(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get next codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
unsafe fn get_codepoint_next(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get previous codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
unsafe fn get_codepoint_previous(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get collision rectangle for two rectangles collision
unsafe fn get_collision_rec(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get Color structure from hexadecimal value
unsafe fn get_color(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get current connected monitor
unsafe fn get_current_monitor(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get full path for a given fileName with path (uses static string)
unsafe fn get_directory_path(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get current FPS
unsafe fn get_f_p_s(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get pointer to extension for a filename string (includes dot: '.png')
unsafe fn get_file_extension(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get file length in bytes (NOTE: GetFileSize() conflicts with windows.h)
unsafe fn get_file_length(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get file modification time (last write time)
unsafe fn get_file_mod_time(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get pointer to filename for a path string
unsafe fn get_file_name(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get filename string without extension (uses static string)
unsafe fn get_file_name_without_ext(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get the default Font
unsafe fn get_font_default(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get time in seconds for last frame drawn (delta time)
unsafe fn get_frame_time(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get gamepad axis count for a gamepad
unsafe fn get_gamepad_axis_count(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get axis movement value for a gamepad axis
unsafe fn get_gamepad_axis_movement(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get the last gamepad button pressed
unsafe fn get_gamepad_button_pressed(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get gamepad internal name id
unsafe fn get_gamepad_name(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get latest detected gesture
unsafe fn get_gesture_detected(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get gesture drag angle
unsafe fn get_gesture_drag_angle(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get gesture drag vector
unsafe fn get_gesture_drag_vector(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get gesture hold time in milliseconds
unsafe fn get_gesture_hold_duration(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get gesture pinch angle
unsafe fn get_gesture_pinch_angle(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get gesture pinch delta
unsafe fn get_gesture_pinch_vector(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get glyph rectangle in font atlas for a codepoint (unicode character), fallback to '?' if not found
unsafe fn get_glyph_atlas_rec(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get glyph index position in font for a codepoint (unicode character), fallback to '?' if not found
unsafe fn get_glyph_index(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get glyph font info data for a codepoint (unicode character), fallback to '?' if not found
unsafe fn get_glyph_info(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get image alpha border rectangle
unsafe fn get_image_alpha_border(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get image pixel color at (x, y) position
unsafe fn get_image_color(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get key pressed (keycode), call it multiple times for keys queued, returns 0 when the queue is empty
unsafe fn get_key_pressed(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get master volume (listener)
unsafe fn get_master_volume(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Compute mesh bounding box limits
unsafe fn get_mesh_bounding_box(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Compute model bounding box limits (considers all meshes)
unsafe fn get_model_bounding_box(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get number of connected monitors
unsafe fn get_monitor_count(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get specified monitor height (current video mode used by monitor)
unsafe fn get_monitor_height(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get the human-readable, UTF-8 encoded name of the specified monitor
unsafe fn get_monitor_name(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get specified monitor physical height in millimetres
unsafe fn get_monitor_physical_height(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get specified monitor physical width in millimetres
unsafe fn get_monitor_physical_width(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get specified monitor position
unsafe fn get_monitor_position(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get specified monitor refresh rate
unsafe fn get_monitor_refresh_rate(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get specified monitor width (current video mode used by monitor)
unsafe fn get_monitor_width(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get mouse delta between frames
unsafe fn get_mouse_delta(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get mouse position XY
unsafe fn get_mouse_position(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get a ray trace from mouse position
unsafe fn get_mouse_ray(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get mouse wheel movement for X or Y, whichever is larger
unsafe fn get_mouse_wheel_move(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get mouse wheel movement for both X and Y
unsafe fn get_mouse_wheel_move_v(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get mouse position X
unsafe fn get_mouse_x(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get mouse position Y
unsafe fn get_mouse_y(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get music time length (in seconds)
unsafe fn get_music_time_length(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get current music time played (in seconds)
unsafe fn get_music_time_played(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get Color from a source pixel pointer of certain format
unsafe fn get_pixel_color(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get pixel data size in bytes for certain format
unsafe fn get_pixel_data_size(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get previous directory path for a given path (uses static string)
unsafe fn get_prev_directory_path(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get a random value between min and max (both included)
unsafe fn get_random_value(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get collision info between ray and box
unsafe fn get_ray_collision_box(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get collision info between ray and mesh
unsafe fn get_ray_collision_mesh(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get collision info between ray and quad
unsafe fn get_ray_collision_quad(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get collision info between ray and sphere
unsafe fn get_ray_collision_sphere(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get collision info between ray and triangle
unsafe fn get_ray_collision_triangle(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get current render height (it considers HiDPI)
unsafe fn get_render_height(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get current render width (it considers HiDPI)
unsafe fn get_render_width(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get current screen height
unsafe fn get_screen_height(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get the world space position for a 2d camera screen space position
unsafe fn get_screen_to_world2_d(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get current screen width
unsafe fn get_screen_width(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get shader uniform location
unsafe fn get_shader_location(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get shader attribute location
unsafe fn get_shader_location_attrib(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get (evaluate) spline point: B-Spline
unsafe fn get_spline_point_basis(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get (evaluate) spline point: Cubic Bezier
unsafe fn get_spline_point_bezier_cubic(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get (evaluate) spline point: Quadratic Bezier
unsafe fn get_spline_point_bezier_quad(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get (evaluate) spline point: Catmull-Rom
unsafe fn get_spline_point_catmull_rom(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get (evaluate) spline point: Linear
unsafe fn get_spline_point_linear(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get elapsed time in seconds since InitWindow()
unsafe fn get_time(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get number of touch points
unsafe fn get_touch_point_count(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get touch point identifier for given index
unsafe fn get_touch_point_id(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get touch position XY for a touch point index (relative to screen size)
unsafe fn get_touch_position(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get touch position X for touch point 0 (relative to screen size)
unsafe fn get_touch_x(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get touch position Y for touch point 0 (relative to screen size)
unsafe fn get_touch_y(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get native window handle
unsafe fn get_window_handle(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get window position XY on monitor
unsafe fn get_window_position(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get window scale DPI factor
unsafe fn get_window_scale_d_p_i(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get current working directory (uses static string)
unsafe fn get_working_directory(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get the screen space position for a 3d world space position
unsafe fn get_world_to_screen(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get the screen space position for a 2d camera world space position
unsafe fn get_world_to_screen2_d(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get size position for a 3d world space position
unsafe fn get_world_to_screen_ex(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Hides cursor
unsafe fn hide_cursor(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Clear alpha channel to desired color
unsafe fn image_alpha_clear(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Crop image depending on alpha value
unsafe fn image_alpha_crop(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Apply alpha mask to image
unsafe fn image_alpha_mask(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Premultiply alpha channel
unsafe fn image_alpha_premultiply(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Apply Gaussian blur using a box blur approximation
unsafe fn image_blur_gaussian(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Clear image background with given color
unsafe fn image_clear_background(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Modify image color: brightness (-255 to 255)
unsafe fn image_color_brightness(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Modify image color: contrast (-100 to 100)
unsafe fn image_color_contrast(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Modify image color: grayscale
unsafe fn image_color_grayscale(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Modify image color: invert
unsafe fn image_color_invert(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Modify image color: replace color
unsafe fn image_color_replace(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Modify image color: tint
unsafe fn image_color_tint(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Create an image duplicate (useful for transformations)
unsafe fn image_copy(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Crop an image to a defined rectangle
unsafe fn image_crop(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Dither image data to 16bpp or lower (Floyd-Steinberg dithering)
unsafe fn image_dither(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a source image within a destination image (tint applied to source)
unsafe fn image_draw(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a filled circle within an image
unsafe fn image_draw_circle(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw circle outline within an image
unsafe fn image_draw_circle_lines(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw circle outline within an image (Vector version)
unsafe fn image_draw_circle_lines_v(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw a filled circle within an image (Vector version)
unsafe fn image_draw_circle_v(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw line within an image
unsafe fn image_draw_line(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw line within an image (Vector version)
unsafe fn image_draw_line_v(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw pixel within an image
unsafe fn image_draw_pixel(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw pixel within an image (Vector version)
unsafe fn image_draw_pixel_v(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw rectangle within an image
unsafe fn image_draw_rectangle(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw rectangle lines within an image
unsafe fn image_draw_rectangle_lines(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw rectangle within an image
unsafe fn image_draw_rectangle_rec(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw rectangle within an image (Vector version)
unsafe fn image_draw_rectangle_v(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw text (using default font) within an image (destination)
unsafe fn image_draw_text(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Draw text (custom sprite font) within an image (destination)
unsafe fn image_draw_text_ex(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Flip image horizontally
unsafe fn image_flip_horizontal(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Flip image vertically
unsafe fn image_flip_vertical(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Convert image data to desired format
unsafe fn image_format(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Create an image from another image piece
unsafe fn image_from_image(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Compute all mipmap levels for a provided image
unsafe fn image_mipmaps(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Resize image (Bicubic scaling algorithm)
unsafe fn image_resize(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Resize canvas and fill with color
unsafe fn image_resize_canvas(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Resize image (Nearest-Neighbor scaling algorithm)
unsafe fn image_resize_n_n(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Rotate image by input angle in degrees (-359 to 359)
unsafe fn image_rotate(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Rotate image counter-clockwise 90deg
unsafe fn image_rotate_c_c_w(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Rotate image clockwise 90deg
unsafe fn image_rotate_c_w(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Create an image from text (default font)
unsafe fn image_text(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Create an image from text (custom sprite font)
unsafe fn image_text_ex(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Convert image to POT (power-of-two)
unsafe fn image_to_p_o_t(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Initialize audio device and context
unsafe fn init_audio_device(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Initialize window and OpenGL context
unsafe fn init_window(memory: &mut Vec<u8>, pointer: usize) -> Option<Vec<u8>> {
    // [@](width: i32, height: i32, title: str)
    // [@][width:ptr][width:ptr][height:ptr][height:ptr][str:ptr][str:ptr]
    let input_cells = bbmem::get_input_cells(memory, pointer, 6);
    let width_ptr = bbmem::read_ptr(&input_cells[0..2]);
    let height_ptr = bbmem::read_ptr(&input_cells[2..4]);
    let title_ptr = bbmem::read_ptr(&input_cells[4..6]);
    let width = bbmem::read_i32(memory, width_ptr);
    let height = bbmem::read_i32(memory, height_ptr);
    let title = bbmem::read_string(memory, title_ptr);
    raylib_ffi::InitWindow(width, height, title.as_ptr() as *const std::os::raw::c_char);
    None
}

// Check if audio device has been initialized successfully
unsafe fn is_audio_device_ready(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if audio stream is playing
unsafe fn is_audio_stream_playing(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if any audio stream buffers requires refill
unsafe fn is_audio_stream_processed(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Checks if an audio stream is ready
unsafe fn is_audio_stream_ready(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if cursor is not visible
unsafe fn is_cursor_hidden(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if cursor is on the screen
unsafe fn is_cursor_on_screen(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if a file has been dropped into window
unsafe fn is_file_dropped(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check file extension (including point: .png, .wav)
unsafe fn is_file_extension(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if a font is ready
unsafe fn is_font_ready(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if a gamepad is available
unsafe fn is_gamepad_available(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if a gamepad button is being pressed
unsafe fn is_gamepad_button_down(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if a gamepad button has been pressed once
unsafe fn is_gamepad_button_pressed(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if a gamepad button has been released once
unsafe fn is_gamepad_button_released(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if a gamepad button is NOT being pressed
unsafe fn is_gamepad_button_up(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if a gesture have been detected
unsafe fn is_gesture_detected(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if an image is ready
unsafe fn is_image_ready(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if a key is being pressed
unsafe fn is_key_down(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if a key has been pressed once
unsafe fn is_key_pressed(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if a key has been pressed again (Only PLATFORM_DESKTOP)
unsafe fn is_key_pressed_repeat(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if a key has been released once
unsafe fn is_key_released(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if a key is NOT being pressed
unsafe fn is_key_up(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if a material is ready
unsafe fn is_material_ready(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check model animation skeleton match
unsafe fn is_model_animation_valid(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if a model is ready
unsafe fn is_model_ready(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if a mouse button is being pressed
unsafe fn is_mouse_button_down(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if a mouse button has been pressed once
unsafe fn is_mouse_button_pressed(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if a mouse button has been released once
unsafe fn is_mouse_button_released(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if a mouse button is NOT being pressed
unsafe fn is_mouse_button_up(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Checks if a music stream is ready
unsafe fn is_music_ready(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if music is playing
unsafe fn is_music_stream_playing(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if a given path is a file or a directory
unsafe fn is_path_file(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if a render texture is ready
unsafe fn is_render_texture_ready(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if a shader is ready
unsafe fn is_shader_ready(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if a sound is currently playing
unsafe fn is_sound_playing(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Checks if a sound is ready
unsafe fn is_sound_ready(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if a texture is ready
unsafe fn is_texture_ready(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Checks if wave data is ready
unsafe fn is_wave_ready(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if window is currently focused (only PLATFORM_DESKTOP)
unsafe fn is_window_focused(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if window is currently fullscreen
unsafe fn is_window_fullscreen(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if window is currently hidden (only PLATFORM_DESKTOP)
unsafe fn is_window_hidden(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if window is currently maximized (only PLATFORM_DESKTOP)
unsafe fn is_window_maximized(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if window is currently minimized (only PLATFORM_DESKTOP)
unsafe fn is_window_minimized(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if window has been initialized successfully
unsafe fn is_window_ready(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if window has been resized last frame
unsafe fn is_window_resized(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if one specific window flag is enabled
unsafe fn is_window_state(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load audio stream (to stream raw audio pcm data)
unsafe fn load_audio_stream(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load automation events list from file, NULL for empty list, capacity = MAX_AUTOMATION_EVENTS
unsafe fn load_automation_event_list(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load all codepoints from a UTF-8 text string, codepoints count returned by parameter
unsafe fn load_codepoints(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load directory filepaths
unsafe fn load_directory_files(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load directory filepaths with extension filtering and recursive directory scan
unsafe fn load_directory_files_ex(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load dropped filepaths
unsafe fn load_dropped_files(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load file data as byte array (read)
unsafe fn load_file_data(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load text data from file (read), returns a '\0' terminated string
unsafe fn load_file_text(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load font from file into GPU memory (VRAM)
unsafe fn load_font(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load font data for further use
unsafe fn load_font_data(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load font from file with extended parameters, use NULL for codepoints and 0 for codepointCount to load the default character set
unsafe fn load_font_ex(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load font from Image (XNA style)
unsafe fn load_font_from_image(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load font from memory buffer, fileType refers to extension: i.e. '.ttf'
unsafe fn load_font_from_memory(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load image from file into CPU memory (RAM)
unsafe fn load_image(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load image sequence from file (frames appended to image.data)
unsafe fn load_image_anim(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load color data from image as a Color array (RGBA - 32bit)
unsafe fn load_image_colors(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load image from memory buffer, fileType refers to extension: i.e. '.png'
unsafe fn load_image_from_memory(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load image from screen buffer and (screenshot)
unsafe fn load_image_from_screen(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load image from GPU texture data
unsafe fn load_image_from_texture(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load colors palette from image as a Color array (RGBA - 32bit)
unsafe fn load_image_palette(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load image from RAW file data
unsafe fn load_image_raw(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load image from SVG file data or string with specified size
unsafe fn load_image_svg(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load default material (Supports: DIFFUSE, SPECULAR, NORMAL maps)
unsafe fn load_material_default(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load materials from model file
unsafe fn load_materials(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load model from files (meshes and materials)
unsafe fn load_model(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load model animations from file
unsafe fn load_model_animations(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load model from generated mesh (default material)
unsafe fn load_model_from_mesh(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load music stream from file
unsafe fn load_music_stream(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load music stream from data
unsafe fn load_music_stream_from_memory(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load random values sequence, no values repeated
unsafe fn load_random_sequence(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load texture for rendering (framebuffer)
unsafe fn load_render_texture(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load shader from files and bind default locations
unsafe fn load_shader(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load shader from code strings and bind default locations
unsafe fn load_shader_from_memory(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load sound from file
unsafe fn load_sound(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Create a new sound that shares the same sample data as the source sound, does not own the sound data
unsafe fn load_sound_alias(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load sound from wave data
unsafe fn load_sound_from_wave(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load texture from file into GPU memory (VRAM)
unsafe fn load_texture(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load cubemap from image, multiple image cubemap layouts supported
unsafe fn load_texture_cubemap(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load texture from image data
unsafe fn load_texture_from_image(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load UTF-8 text encoded from codepoints array
unsafe fn load_u_t_f8(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load VR stereo config for VR simulator device parameters
unsafe fn load_vr_stereo_config(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load wave data from file
unsafe fn load_wave(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load wave from memory buffer, fileType refers to extension: i.e. '.wav'
unsafe fn load_wave_from_memory(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Load samples data from wave as a 32bit float data array
unsafe fn load_wave_samples(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set window state: maximized, if resizable (only PLATFORM_DESKTOP)
unsafe fn maximize_window(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Measure string width for default font
unsafe fn measure_text(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Measure string size for Font
unsafe fn measure_text_ex(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Internal memory allocator
unsafe fn mem_alloc(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Internal memory free
unsafe fn mem_free(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Internal memory reallocator
unsafe fn mem_realloc(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set window state: minimized, if resizable (only PLATFORM_DESKTOP)
unsafe fn minimize_window(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Open URL with default system browser (if available)
unsafe fn open_u_r_l(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Pause audio stream
unsafe fn pause_audio_stream(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Pause music playing
unsafe fn pause_music_stream(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Pause a sound
unsafe fn pause_sound(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Play audio stream
unsafe fn play_audio_stream(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Play a recorded automation event
unsafe fn play_automation_event(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Start music playing
unsafe fn play_music_stream(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Play a sound
unsafe fn play_sound(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Register all input events
unsafe fn poll_input_events(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set window state: not minimized/maximized (only PLATFORM_DESKTOP)
unsafe fn restore_window(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Resume audio stream
unsafe fn resume_audio_stream(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Resume playing paused music
unsafe fn resume_music_stream(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Resume a paused sound
unsafe fn resume_sound(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Save data to file from byte array (write), returns true on success
unsafe fn save_file_data(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Save text data to file (write), string must be '\0' terminated, returns true on success
unsafe fn save_file_text(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Seek music to a position (in seconds)
unsafe fn seek_music_stream(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Default size for new audio streams
unsafe fn set_audio_stream_buffer_size_default(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Audio thread callback to request new data
unsafe fn set_audio_stream_callback(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set pan for audio stream (0.5 is centered)
unsafe fn set_audio_stream_pan(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set pitch for audio stream (1.0 is base level)
unsafe fn set_audio_stream_pitch(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set volume for audio stream (1.0 is max level)
unsafe fn set_audio_stream_volume(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set automation event internal base frame to start recording
unsafe fn set_automation_event_base_frame(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set automation event list to record to
unsafe fn set_automation_event_list(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set clipboard text content
unsafe fn set_clipboard_text(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Setup init configuration flags (view FLAGS)
unsafe fn set_config_flags(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set a custom key to exit program (default is ESC)
unsafe fn set_exit_key(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set internal gamepad mappings (SDL_GameControllerDB)
unsafe fn set_gamepad_mappings(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Enable a set of gestures using flags
unsafe fn set_gestures_enabled(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set custom file binary data loader
unsafe fn set_load_file_data_callback(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set custom file text data loader
unsafe fn set_load_file_text_callback(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set master volume (listener)
unsafe fn set_master_volume(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set texture for a material map type (MATERIAL_MAP_DIFFUSE, MATERIAL_MAP_SPECULAR...)
unsafe fn set_material_texture(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set material for a mesh
unsafe fn set_model_mesh_material(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set mouse cursor
unsafe fn set_mouse_cursor(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set mouse offset
unsafe fn set_mouse_offset(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set mouse position XY
unsafe fn set_mouse_position(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set mouse scaling
unsafe fn set_mouse_scale(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set pan for a music (0.5 is center)
unsafe fn set_music_pan(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set pitch for a music (1.0 is base level)
unsafe fn set_music_pitch(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set volume for music (1.0 is max level)
unsafe fn set_music_volume(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set color formatted into destination pixel pointer
unsafe fn set_pixel_color(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set the seed for the random number generator
unsafe fn set_random_seed(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set custom file binary data saver
unsafe fn set_save_file_data_callback(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set custom file text data saver
unsafe fn set_save_file_text_callback(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set shader uniform value
unsafe fn set_shader_value(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set shader uniform value (matrix 4x4)
unsafe fn set_shader_value_matrix(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set shader uniform value for texture (sampler2d)
unsafe fn set_shader_value_texture(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set shader uniform value vector
unsafe fn set_shader_value_v(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set texture and rectangle to be used on shapes drawing
unsafe fn set_shapes_texture(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set pan for a sound (0.5 is center)
unsafe fn set_sound_pan(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set pitch for a sound (1.0 is base level)
unsafe fn set_sound_pitch(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set volume for a sound (1.0 is max level)
unsafe fn set_sound_volume(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set target FPS (maximum)
unsafe fn set_target_f_p_s(memory: &mut Vec<u8>, pointer: usize) -> Option<Vec<u8>> {
    // [@](fps: i32)
    // [@][fps: ptr][fps: ptr]
    let input_cells = bbmem::get_input_cells(memory, pointer, 2);
    let fps_ptr = bbmem::read_ptr(&input_cells[0..2]);
    let fps = bbmem::read_i32(memory, fps_ptr);
    raylib_ffi::SetTargetFPS(fps);
    None
}

// Set vertical line spacing when drawing with line-breaks
unsafe fn set_text_line_spacing(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set texture scaling filter mode
unsafe fn set_texture_filter(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set texture wrapping mode
unsafe fn set_texture_wrap(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set custom trace log
unsafe fn set_trace_log_callback(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set the current threshold (minimum) log level
unsafe fn set_trace_log_level(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set window focused (only PLATFORM_DESKTOP)
unsafe fn set_window_focused(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set icon for window (single image, RGBA 32bit, only PLATFORM_DESKTOP)
unsafe fn set_window_icon(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set icon for window (multiple images, RGBA 32bit, only PLATFORM_DESKTOP)
unsafe fn set_window_icons(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set window maximum dimensions (for FLAG_WINDOW_RESIZABLE)
unsafe fn set_window_max_size(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set window minimum dimensions (for FLAG_WINDOW_RESIZABLE)
unsafe fn set_window_min_size(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set monitor for the current window
unsafe fn set_window_monitor(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set window opacity [0.0f..1.0f] (only PLATFORM_DESKTOP)
unsafe fn set_window_opacity(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set window position on screen (only PLATFORM_DESKTOP)
unsafe fn set_window_position(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set window dimensions
unsafe fn set_window_size(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set window configuration state using flags (only PLATFORM_DESKTOP)
unsafe fn set_window_state(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Set title for window (only PLATFORM_DESKTOP and PLATFORM_WEB)
unsafe fn set_window_title(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Shows cursor
unsafe fn show_cursor(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Start recording automation events (AutomationEventList must be set)
unsafe fn start_automation_event_recording(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Stop audio stream
unsafe fn stop_audio_stream(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Stop recording automation events
unsafe fn stop_automation_event_recording(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Stop music playing
unsafe fn stop_music_stream(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Stop playing a sound
unsafe fn stop_sound(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Swap back buffer with front buffer (screen drawing)
unsafe fn swap_screen_buffer(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Takes a screenshot of current screen (filename extension defines format)
unsafe fn take_screenshot(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Append text at specific position and move cursor!
unsafe fn text_append(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Copy one string to another, returns bytes copied
unsafe fn text_copy(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Find first text occurrence within a string
unsafe fn text_find_index(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Text formatting with variables (sprintf() style)
unsafe fn text_format(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Insert text in a position (WARNING: memory must be freed!)
unsafe fn text_insert(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if two text string are equal
unsafe fn text_is_equal(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Join text strings with delimiter
unsafe fn text_join(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get text length, checks for '\0' ending
unsafe fn text_length(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Replace text string (WARNING: memory must be freed!)
unsafe fn text_replace(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Split text into multiple strings
unsafe fn text_split(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get a piece of a text string
unsafe fn text_subtext(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get integer value from text (negative values not supported)
unsafe fn text_to_integer(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get lower case version of provided string
unsafe fn text_to_lower(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get Pascal case notation version of provided string
unsafe fn text_to_pascal(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Get upper case version of provided string
unsafe fn text_to_upper(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Toggle window state: borderless windowed (only PLATFORM_DESKTOP)
unsafe fn toggle_borderless_windowed(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Toggle window state: fullscreen/windowed (only PLATFORM_DESKTOP)
unsafe fn toggle_fullscreen(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Show trace log messages (LOG_DEBUG, LOG_INFO, LOG_WARNING, LOG_ERROR...)
unsafe fn trace_log(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Unload audio stream and free memory
unsafe fn unload_audio_stream(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Unload automation events list from file
unsafe fn unload_automation_event_list(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Unload codepoints data from memory
unsafe fn unload_codepoints(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Unload filepaths
unsafe fn unload_directory_files(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Unload dropped filepaths
unsafe fn unload_dropped_files(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Unload file data allocated by LoadFileData()
unsafe fn unload_file_data(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Unload file text data allocated by LoadFileText()
unsafe fn unload_file_text(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Unload font from GPU memory (VRAM)
unsafe fn unload_font(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Unload font chars info data (RAM)
unsafe fn unload_font_data(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Unload image from CPU memory (RAM)
unsafe fn unload_image(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Unload color data loaded with LoadImageColors()
unsafe fn unload_image_colors(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Unload colors palette loaded with LoadImagePalette()
unsafe fn unload_image_palette(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Unload material from GPU memory (VRAM)
unsafe fn unload_material(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Unload mesh data from CPU and GPU
unsafe fn unload_mesh(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Unload model (including meshes) from memory (RAM and/or VRAM)
unsafe fn unload_model(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Unload animation data
unsafe fn unload_model_animation(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Unload animation array data
unsafe fn unload_model_animations(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Unload music stream
unsafe fn unload_music_stream(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Unload random values sequence
unsafe fn unload_random_sequence(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Unload render texture from GPU memory (VRAM)
unsafe fn unload_render_texture(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Unload shader from GPU memory (VRAM)
unsafe fn unload_shader(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Unload sound
unsafe fn unload_sound(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Unload a sound alias (does not deallocate sample data)
unsafe fn unload_sound_alias(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Unload texture from GPU memory (VRAM)
unsafe fn unload_texture(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Unload UTF-8 text encoded from codepoints array
unsafe fn unload_u_t_f8(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Unload VR stereo config
unsafe fn unload_vr_stereo_config(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Unload wave data
unsafe fn unload_wave(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Unload samples data loaded with LoadWaveSamples()
unsafe fn unload_wave_samples(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Update audio stream buffers with data
unsafe fn update_audio_stream(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Update camera position for selected mode
unsafe fn update_camera(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Update camera movement/rotation
unsafe fn update_camera_pro(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Update mesh vertex data in GPU for a specific buffer index
unsafe fn update_mesh_buffer(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Update model animation pose
unsafe fn update_model_animation(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Updates buffers for music streaming
unsafe fn update_music_stream(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Update sound buffer with new data
unsafe fn update_sound(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Update GPU texture with new data
unsafe fn update_texture(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Update GPU texture rectangle with new data
unsafe fn update_texture_rec(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    unimplemented!()
}

// Upload mesh vertex data in GPU and provide VAO/VBO ids
unsafe fn upload_mesh(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Wait for some time (halt program execution)
unsafe fn wait_time(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Copy a wave to a new wave
unsafe fn wave_copy(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Crop a wave to defined samples range
unsafe fn wave_crop(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Convert wave data to desired format
unsafe fn wave_format(_memory: &mut Vec<u8>, _pointer: usize, width: usize) -> Option<Vec<u8>> {
    unimplemented!()
}

// Check if application should close (KEY_ESCAPE pressed or windows close icon clicked)
unsafe fn window_should_close(
    _memory: &mut Vec<u8>,
    _pointer: usize,
    width: usize,
) -> Option<Vec<u8>> {
    Some(vec![raylib_ffi::WindowShouldClose() as u8])
}
