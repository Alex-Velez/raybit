use super::{raudio, rcore, rmodels, rshapes, rtext, rtextures, FnOutput, Memory, Pointer};

// modules: rcore + rshapes + rtextures + rtext + rmodels + raudio
pub const FN_COUNT: usize = 552;
pub const FUNCTIONS: [unsafe fn(mem: &mut Memory, index: Pointer) -> FnOutput; FN_COUNT] = [
    /* ID: 0 */ raudio::attach_audio_mixed_processor,
    /* ID: 1 */ raudio::attach_audio_stream_processor,
    /* ID: 2 */ rcore::begin_blend_mode,
    /* ID: 3 */ rcore::begin_drawing,
    /* ID: 4 */ rcore::begin_mode2_d,
    /* ID: 5 */ rcore::begin_mode3_d,
    /* ID: 6 */ rcore::begin_scissor_mode,
    /* ID: 7 */ rcore::begin_shader_mode,
    /* ID: 8 */ rcore::begin_texture_mode,
    /* ID: 9 */ rcore::begin_vr_stereo_mode,
    /* ID: 10 */ rcore::change_directory,
    /* ID: 11 */ rmodels::check_collision_box_sphere,
    /* ID: 12 */ rmodels::check_collision_boxes,
    /* ID: 13 */ rshapes::check_collision_circle_rec,
    /* ID: 14 */ rshapes::check_collision_circles,
    /* ID: 15 */ rshapes::check_collision_lines,
    /* ID: 16 */ rshapes::check_collision_point_circle,
    /* ID: 17 */ rshapes::check_collision_point_line,
    /* ID: 18 */ rshapes::check_collision_point_poly,
    /* ID: 19 */ rshapes::check_collision_point_rec,
    /* ID: 20 */ rshapes::check_collision_point_triangle,
    /* ID: 21 */ rshapes::check_collision_recs,
    /* ID: 22 */ rmodels::check_collision_spheres,
    /* ID: 23 */ rcore::clear_background,
    /* ID: 24 */ rcore::clear_window_state,
    /* ID: 25 */ raudio::close_audio_device,
    /* ID: 26 */ rcore::close_window,
    /* ID: 27 */ rtext::codepoint_to_utf8,
    /* ID: 28 */ rtextures::color_alpha,
    /* ID: 29 */ rtextures::color_alpha_blend,
    /* ID: 30 */ rtextures::color_brightness,
    /* ID: 31 */ rtextures::color_contrast,
    /* ID: 32 */ rtextures::color_from_hsv,
    /* ID: 33 */ rtextures::color_from_normalized,
    /* ID: 34 */ rtextures::color_normalize,
    /* ID: 35 */ rtextures::color_tint,
    /* ID: 36 */ rtextures::color_to_hsv,
    /* ID: 37 */ rtextures::color_to_int,
    /* ID: 38 */ rcore::compress_data,
    /* ID: 39 */ rcore::decode_data_base64,
    /* ID: 40 */ rcore::decompress_data,
    /* ID: 41 */ raudio::detach_audio_mixed_processor,
    /* ID: 42 */ raudio::detach_audio_stream_processor,
    /* ID: 43 */ rcore::directory_exists,
    /* ID: 44 */ rcore::disable_cursor,
    /* ID: 45 */ rcore::disable_event_waiting,
    /* ID: 46 */ rmodels::draw_billboard,
    /* ID: 47 */ rmodels::draw_billboard_pro,
    /* ID: 48 */ rmodels::draw_billboard_rec,
    /* ID: 49 */ rmodels::draw_bounding_box,
    /* ID: 50 */ rmodels::draw_capsule,
    /* ID: 51 */ rmodels::draw_capsule_wires,
    /* ID: 52 */ rshapes::draw_circle,
    /* ID: 53 */ rmodels::draw_circle3_d,
    /* ID: 54 */ rshapes::draw_circle_gradient,
    /* ID: 55 */ rshapes::draw_circle_lines,
    /* ID: 56 */ rshapes::draw_circle_lines_v,
    /* ID: 57 */ rshapes::draw_circle_sector,
    /* ID: 58 */ rshapes::draw_circle_sector_lines,
    /* ID: 59 */ rshapes::draw_circle_v,
    /* ID: 60 */ rmodels::draw_cube,
    /* ID: 61 */ rmodels::draw_cube_v,
    /* ID: 62 */ rmodels::draw_cube_wires,
    /* ID: 63 */ rmodels::draw_cube_wires_v,
    /* ID: 64 */ rmodels::draw_cylinder,
    /* ID: 65 */ rmodels::draw_cylinder_ex,
    /* ID: 66 */ rmodels::draw_cylinder_wires,
    /* ID: 67 */ rmodels::draw_cylinder_wires_ex,
    /* ID: 68 */ rshapes::draw_ellipse,
    /* ID: 69 */ rshapes::draw_ellipse_lines,
    /* ID: 70 */ rtext::draw_fps,
    /* ID: 71 */ rmodels::draw_grid,
    /* ID: 72 */ rshapes::draw_line,
    /* ID: 73 */ rmodels::draw_line3_d,
    /* ID: 74 */ rshapes::draw_line_bezier,
    /* ID: 75 */ rshapes::draw_line_ex,
    /* ID: 76 */ rshapes::draw_line_strip,
    /* ID: 77 */ rshapes::draw_line_v,
    /* ID: 78 */ rmodels::draw_mesh,
    /* ID: 79 */ rmodels::draw_mesh_instanced,
    /* ID: 80 */ rmodels::draw_model,
    /* ID: 81 */ rmodels::draw_model_ex,
    /* ID: 82 */ rmodels::draw_model_wires,
    /* ID: 83 */ rmodels::draw_model_wires_ex,
    /* ID: 84 */ rshapes::draw_pixel,
    /* ID: 85 */ rshapes::draw_pixel_v,
    /* ID: 86 */ rmodels::draw_plane,
    /* ID: 87 */ rmodels::draw_point3_d,
    /* ID: 88 */ rshapes::draw_poly,
    /* ID: 89 */ rshapes::draw_poly_lines,
    /* ID: 90 */ rshapes::draw_poly_lines_ex,
    /* ID: 91 */ rmodels::draw_ray,
    /* ID: 92 */ rshapes::draw_rectangle,
    /* ID: 93 */ rshapes::draw_rectangle_gradient_ex,
    /* ID: 94 */ rshapes::draw_rectangle_gradient_h,
    /* ID: 95 */ rshapes::draw_rectangle_gradient_v,
    /* ID: 96 */ rshapes::draw_rectangle_lines,
    /* ID: 97 */ rshapes::draw_rectangle_lines_ex,
    /* ID: 98 */ rshapes::draw_rectangle_pro,
    /* ID: 99 */ rshapes::draw_rectangle_rec,
    /* ID: 100 */ rshapes::draw_rectangle_rounded,
    /* ID: 101 */ rshapes::draw_rectangle_rounded_lines,
    /* ID: 102 */ rshapes::draw_rectangle_v,
    /* ID: 103 */ rshapes::draw_ring,
    /* ID: 104 */ rshapes::draw_ring_lines,
    /* ID: 105 */ rmodels::draw_sphere,
    /* ID: 106 */ rmodels::draw_sphere_ex,
    /* ID: 107 */ rmodels::draw_sphere_wires,
    /* ID: 108 */ rshapes::draw_spline_basis,
    /* ID: 109 */ rshapes::draw_spline_bezier_cubic,
    /* ID: 110 */ rshapes::draw_spline_bezier_quadratic,
    /* ID: 111 */ rshapes::draw_spline_catmull_rom,
    /* ID: 112 */ rshapes::draw_spline_linear,
    /* ID: 113 */ rshapes::draw_spline_segment_basis,
    /* ID: 114 */ rshapes::draw_spline_segment_bezier_cubic,
    /* ID: 115 */ rshapes::draw_spline_segment_bezier_quadratic,
    /* ID: 116 */ rshapes::draw_spline_segment_catmull_rom,
    /* ID: 117 */ rshapes::draw_spline_segment_linear,
    /* ID: 118 */ rtext::draw_text,
    /* ID: 119 */ rtext::draw_text_codepoint,
    /* ID: 120 */ rtext::draw_text_codepoints,
    /* ID: 121 */ rtext::draw_text_ex,
    /* ID: 122 */ rtext::draw_text_pro,
    /* ID: 123 */ rtextures::draw_texture,
    /* ID: 124 */ rtextures::draw_texture_ex,
    /* ID: 125 */ rtextures::draw_texture_n_patch,
    /* ID: 126 */ rtextures::draw_texture_pro,
    /* ID: 127 */ rtextures::draw_texture_rec,
    /* ID: 128 */ rtextures::draw_texture_v,
    /* ID: 129 */ rshapes::draw_triangle,
    /* ID: 130 */ rmodels::draw_triangle3_d,
    /* ID: 131 */ rshapes::draw_triangle_fan,
    /* ID: 132 */ rshapes::draw_triangle_lines,
    /* ID: 133 */ rshapes::draw_triangle_strip,
    /* ID: 134 */ rmodels::draw_triangle_strip3_d,
    /* ID: 135 */ rcore::enable_cursor,
    /* ID: 136 */ rcore::enable_event_waiting,
    /* ID: 137 */ rcore::encode_data_base64,
    /* ID: 138 */ rcore::end_blend_mode,
    /* ID: 139 */ rcore::end_drawing,
    /* ID: 140 */ rcore::end_mode2_d,
    /* ID: 141 */ rcore::end_mode3_d,
    /* ID: 142 */ rcore::end_scissor_mode,
    /* ID: 143 */ rcore::end_shader_mode,
    /* ID: 144 */ rcore::end_texture_mode,
    /* ID: 145 */ rcore::end_vr_stereo_mode,
    /* ID: 146 */ rcore::export_automation_event_list,
    /* ID: 147 */ rcore::export_data_as_code,
    /* ID: 148 */ rtext::export_font_as_code,
    /* ID: 149 */ rtextures::export_image,
    /* ID: 150 */ rtextures::export_image_as_code,
    /* ID: 151 */ rtextures::export_image_to_memory,
    /* ID: 152 */ rmodels::export_mesh,
    /* ID: 153 */ raudio::export_wave,
    /* ID: 154 */ raudio::export_wave_as_code,
    /* ID: 155 */ rtextures::fade,
    /* ID: 156 */ rcore::file_exists,
    /* ID: 157 */ rtextures::gen_image_cellular,
    /* ID: 158 */ rtextures::gen_image_checked,
    /* ID: 159 */ rtextures::gen_image_color,
    /* ID: 160 */ rtext::gen_image_font_atlas,
    /* ID: 161 */ rtextures::gen_image_gradient_linear,
    /* ID: 162 */ rtextures::gen_image_gradient_radial,
    /* ID: 163 */ rtextures::gen_image_gradient_square,
    /* ID: 164 */ rtextures::gen_image_perlin_noise,
    /* ID: 165 */ rtextures::gen_image_text,
    /* ID: 166 */ rtextures::gen_image_white_noise,
    /* ID: 167 */ rmodels::gen_mesh_cone,
    /* ID: 168 */ rmodels::gen_mesh_cube,
    /* ID: 169 */ rmodels::gen_mesh_cubicmap,
    /* ID: 170 */ rmodels::gen_mesh_cylinder,
    /* ID: 171 */ rmodels::gen_mesh_heightmap,
    /* ID: 172 */ rmodels::gen_mesh_hemi_sphere,
    /* ID: 173 */ rmodels::gen_mesh_knot,
    /* ID: 174 */ rmodels::gen_mesh_plane,
    /* ID: 175 */ rmodels::gen_mesh_poly,
    /* ID: 176 */ rmodels::gen_mesh_sphere,
    /* ID: 177 */ rmodels::gen_mesh_tangents,
    /* ID: 178 */ rmodels::gen_mesh_torus,
    /* ID: 179 */ rtextures::gen_texture_mipmaps,
    /* ID: 180 */ rcore::get_application_directory,
    /* ID: 181 */ rcore::get_camera_matrix,
    /* ID: 182 */ rcore::get_camera_matrix2_d,
    /* ID: 183 */ rcore::get_char_pressed,
    /* ID: 184 */ rcore::get_clipboard_text,
    /* ID: 185 */ rtext::get_codepoint,
    /* ID: 186 */ rtext::get_codepoint_count,
    /* ID: 187 */ rtext::get_codepoint_next,
    /* ID: 188 */ rtext::get_codepoint_previous,
    /* ID: 189 */ rshapes::get_collision_rec,
    /* ID: 190 */ rtextures::get_color,
    /* ID: 191 */ rcore::get_current_monitor,
    /* ID: 192 */ rcore::get_directory_path,
    /* ID: 193 */ rcore::get_file_extension,
    /* ID: 194 */ rcore::get_file_length,
    /* ID: 195 */ rcore::get_file_mod_time,
    /* ID: 196 */ rcore::get_file_name,
    /* ID: 197 */ rcore::get_file_name_without_ext,
    /* ID: 198 */ rtext::get_font_default,
    /* ID: 199 */ rcore::get_fps,
    /* ID: 200 */ rcore::get_frame_time,
    /* ID: 201 */ rcore::get_gamepad_axis_count,
    /* ID: 202 */ rcore::get_gamepad_axis_movement,
    /* ID: 203 */ rcore::get_gamepad_button_pressed,
    /* ID: 204 */ rcore::get_gamepad_name,
    /* ID: 205 */ rcore::get_gesture_detected,
    /* ID: 206 */ rcore::get_gesture_drag_angle,
    /* ID: 207 */ rcore::get_gesture_drag_vector,
    /* ID: 208 */ rcore::get_gesture_hold_duration,
    /* ID: 209 */ rcore::get_gesture_pinch_angle,
    /* ID: 210 */ rcore::get_gesture_pinch_vector,
    /* ID: 211 */ rtext::get_glyph_atlas_rec,
    /* ID: 212 */ rtext::get_glyph_index,
    /* ID: 213 */ rtext::get_glyph_info,
    /* ID: 214 */ rtextures::get_image_alpha_border,
    /* ID: 215 */ rtextures::get_image_color,
    /* ID: 216 */ rcore::get_key_pressed,
    /* ID: 217 */ raudio::get_master_volume,
    /* ID: 218 */ rmodels::get_mesh_bounding_box,
    /* ID: 219 */ rmodels::get_model_bounding_box,
    /* ID: 220 */ rcore::get_monitor_count,
    /* ID: 221 */ rcore::get_monitor_height,
    /* ID: 222 */ rcore::get_monitor_name,
    /* ID: 223 */ rcore::get_monitor_physical_height,
    /* ID: 224 */ rcore::get_monitor_physical_width,
    /* ID: 225 */ rcore::get_monitor_position,
    /* ID: 226 */ rcore::get_monitor_refresh_rate,
    /* ID: 227 */ rcore::get_monitor_width,
    /* ID: 228 */ rcore::get_mouse_delta,
    /* ID: 229 */ rcore::get_mouse_position,
    /* ID: 230 */ rcore::get_mouse_ray,
    /* ID: 231 */ rcore::get_mouse_wheel_move,
    /* ID: 232 */ rcore::get_mouse_wheel_move_v,
    /* ID: 233 */ rcore::get_mouse_x,
    /* ID: 234 */ rcore::get_mouse_y,
    /* ID: 235 */ raudio::get_music_time_length,
    /* ID: 236 */ raudio::get_music_time_played,
    /* ID: 237 */ rtextures::get_pixel_color,
    /* ID: 238 */ rtextures::get_pixel_data_size,
    /* ID: 239 */ rcore::get_prev_directory_path,
    /* ID: 240 */ rcore::get_random_value,
    /* ID: 241 */ rmodels::get_ray_collision_box,
    /* ID: 242 */ rmodels::get_ray_collision_mesh,
    /* ID: 243 */ rmodels::get_ray_collision_quad,
    /* ID: 244 */ rmodels::get_ray_collision_sphere,
    /* ID: 245 */ rmodels::get_ray_collision_triangle,
    /* ID: 246 */ rcore::get_render_height,
    /* ID: 247 */ rcore::get_render_width,
    /* ID: 248 */ rcore::get_screen_height,
    /* ID: 249 */ rcore::get_screen_to_world2_d,
    /* ID: 250 */ rcore::get_screen_width,
    /* ID: 251 */ rcore::get_shader_location,
    /* ID: 252 */ rcore::get_shader_location_attrib,
    /* ID: 253 */ rshapes::get_spline_point_basis,
    /* ID: 254 */ rshapes::get_spline_point_bezier_cubic,
    /* ID: 255 */ rshapes::get_spline_point_bezier_quad,
    /* ID: 256 */ rshapes::get_spline_point_catmull_rom,
    /* ID: 257 */ rshapes::get_spline_point_linear,
    /* ID: 258 */ rcore::get_time,
    /* ID: 259 */ rcore::get_touch_point_count,
    /* ID: 260 */ rcore::get_touch_point_id,
    /* ID: 261 */ rcore::get_touch_position,
    /* ID: 262 */ rcore::get_touch_x,
    /* ID: 263 */ rcore::get_touch_y,
    /* ID: 264 */ rcore::get_window_handle,
    /* ID: 265 */ rcore::get_window_position,
    /* ID: 266 */ rcore::get_window_scale_dpi,
    /* ID: 267 */ rcore::get_working_directory,
    /* ID: 268 */ rcore::get_world_to_screen,
    /* ID: 269 */ rcore::get_world_to_screen2_d,
    /* ID: 270 */ rcore::get_world_to_screen_ex,
    /* ID: 271 */ rcore::hide_cursor,
    /* ID: 272 */ rtextures::image_alpha_clear,
    /* ID: 273 */ rtextures::image_alpha_crop,
    /* ID: 274 */ rtextures::image_alpha_mask,
    /* ID: 275 */ rtextures::image_alpha_premultiply,
    /* ID: 276 */ rtextures::image_blur_gaussian,
    /* ID: 277 */ rtextures::image_clear_background,
    /* ID: 278 */ rtextures::image_color_brightness,
    /* ID: 279 */ rtextures::image_color_contrast,
    /* ID: 280 */ rtextures::image_color_grayscale,
    /* ID: 281 */ rtextures::image_color_invert,
    /* ID: 282 */ rtextures::image_color_replace,
    /* ID: 283 */ rtextures::image_color_tint,
    /* ID: 284 */ rtextures::image_copy,
    /* ID: 285 */ rtextures::image_crop,
    /* ID: 286 */ rtextures::image_dither,
    /* ID: 287 */ rtextures::image_draw,
    /* ID: 288 */ rtextures::image_draw_circle,
    /* ID: 289 */ rtextures::image_draw_circle_lines,
    /* ID: 290 */ rtextures::image_draw_circle_lines_v,
    /* ID: 291 */ rtextures::image_draw_circle_v,
    /* ID: 292 */ rtextures::image_draw_line,
    /* ID: 293 */ rtextures::image_draw_line_v,
    /* ID: 294 */ rtextures::image_draw_pixel,
    /* ID: 295 */ rtextures::image_draw_pixel_v,
    /* ID: 296 */ rtextures::image_draw_rectangle,
    /* ID: 297 */ rtextures::image_draw_rectangle_lines,
    /* ID: 298 */ rtextures::image_draw_rectangle_rec,
    /* ID: 299 */ rtextures::image_draw_rectangle_v,
    /* ID: 300 */ rtextures::image_draw_text,
    /* ID: 301 */ rtextures::image_draw_text_ex,
    /* ID: 302 */ rtextures::image_flip_horizontal,
    /* ID: 303 */ rtextures::image_flip_vertical,
    /* ID: 304 */ rtextures::image_format,
    /* ID: 305 */ rtextures::image_from_image,
    /* ID: 306 */ rtextures::image_mipmaps,
    /* ID: 307 */ rtextures::image_resize,
    /* ID: 308 */ rtextures::image_resize_canvas,
    /* ID: 309 */ rtextures::image_resize_nn,
    /* ID: 310 */ rtextures::image_rotate,
    /* ID: 311 */ rtextures::image_rotate_ccw,
    /* ID: 312 */ rtextures::image_rotate_cw,
    /* ID: 313 */ rtextures::image_text,
    /* ID: 314 */ rtextures::image_text_ex,
    /* ID: 315 */ rtextures::image_to_pot,
    /* ID: 316 */ raudio::init_audio_device,
    /* ID: 317 */ rcore::init_window,
    /* ID: 318 */ raudio::is_audio_device_ready,
    /* ID: 319 */ raudio::is_audio_stream_playing,
    /* ID: 320 */ raudio::is_audio_stream_processed,
    /* ID: 321 */ raudio::is_audio_stream_ready,
    /* ID: 322 */ rcore::is_cursor_hidden,
    /* ID: 323 */ rcore::is_cursor_on_screen,
    /* ID: 324 */ rcore::is_file_dropped,
    /* ID: 325 */ rcore::is_file_extension,
    /* ID: 326 */ rtext::is_font_ready,
    /* ID: 327 */ rcore::is_gamepad_available,
    /* ID: 328 */ rcore::is_gamepad_button_down,
    /* ID: 329 */ rcore::is_gamepad_button_pressed,
    /* ID: 330 */ rcore::is_gamepad_button_released,
    /* ID: 331 */ rcore::is_gamepad_button_up,
    /* ID: 332 */ rcore::is_gesture_detected,
    /* ID: 333 */ rtextures::is_image_ready,
    /* ID: 334 */ rcore::is_key_down,
    /* ID: 335 */ rcore::is_key_pressed,
    /* ID: 336 */ rcore::is_key_pressed_repeat,
    /* ID: 337 */ rcore::is_key_released,
    /* ID: 338 */ rcore::is_key_up,
    /* ID: 339 */ rmodels::is_material_ready,
    /* ID: 340 */ rmodels::is_model_animation_valid,
    /* ID: 341 */ rmodels::is_model_ready,
    /* ID: 342 */ rcore::is_mouse_button_down,
    /* ID: 343 */ rcore::is_mouse_button_pressed,
    /* ID: 344 */ rcore::is_mouse_button_released,
    /* ID: 345 */ rcore::is_mouse_button_up,
    /* ID: 346 */ raudio::is_music_ready,
    /* ID: 347 */ raudio::is_music_stream_playing,
    /* ID: 348 */ rcore::is_path_file,
    /* ID: 349 */ rtextures::is_render_texture_ready,
    /* ID: 350 */ rcore::is_shader_ready,
    /* ID: 351 */ raudio::is_sound_playing,
    /* ID: 352 */ raudio::is_sound_ready,
    /* ID: 353 */ rtextures::is_texture_ready,
    /* ID: 354 */ raudio::is_wave_ready,
    /* ID: 355 */ rcore::is_window_focused,
    /* ID: 356 */ rcore::is_window_fullscreen,
    /* ID: 357 */ rcore::is_window_hidden,
    /* ID: 358 */ rcore::is_window_maximized,
    /* ID: 359 */ rcore::is_window_minimized,
    /* ID: 360 */ rcore::is_window_ready,
    /* ID: 361 */ rcore::is_window_resized,
    /* ID: 362 */ rcore::is_window_state,
    /* ID: 363 */ raudio::load_audio_stream,
    /* ID: 364 */ rcore::load_automation_event_list,
    /* ID: 365 */ rtext::load_codepoints,
    /* ID: 366 */ rcore::load_directory_files,
    /* ID: 367 */ rcore::load_directory_files_ex,
    /* ID: 368 */ rcore::load_dropped_files,
    /* ID: 369 */ rcore::load_file_data,
    /* ID: 370 */ rcore::load_file_text,
    /* ID: 371 */ rtext::load_font,
    /* ID: 372 */ rtext::load_font_data,
    /* ID: 373 */ rtext::load_font_ex,
    /* ID: 374 */ rtext::load_font_from_image,
    /* ID: 375 */ rtext::load_font_from_memory,
    /* ID: 376 */ rtextures::load_image,
    /* ID: 377 */ rtextures::load_image_anim,
    /* ID: 378 */ rtextures::load_image_colors,
    /* ID: 379 */ rtextures::load_image_from_memory,
    /* ID: 380 */ rtextures::load_image_from_screen,
    /* ID: 381 */ rtextures::load_image_from_texture,
    /* ID: 382 */ rtextures::load_image_palette,
    /* ID: 383 */ rtextures::load_image_raw,
    /* ID: 384 */ rtextures::load_image_svg,
    /* ID: 385 */ rmodels::load_material_default,
    /* ID: 386 */ rmodels::load_materials,
    /* ID: 387 */ rmodels::load_model,
    /* ID: 388 */ rmodels::load_model_animations,
    /* ID: 389 */ rmodels::load_model_from_mesh,
    /* ID: 390 */ raudio::load_music_stream,
    /* ID: 391 */ raudio::load_music_stream_from_memory,
    /* ID: 392 */ rcore::load_random_sequence,
    /* ID: 393 */ rtextures::load_render_texture,
    /* ID: 394 */ rcore::load_shader,
    /* ID: 395 */ rcore::load_shader_from_memory,
    /* ID: 396 */ raudio::load_sound,
    /* ID: 397 */ raudio::load_sound_alias,
    /* ID: 398 */ raudio::load_sound_from_wave,
    /* ID: 399 */ rtextures::load_texture,
    /* ID: 400 */ rtextures::load_texture_cubemap,
    /* ID: 401 */ rtextures::load_texture_from_image,
    /* ID: 402 */ rtext::load_utf8,
    /* ID: 403 */ rcore::load_vr_stereo_config,
    /* ID: 404 */ raudio::load_wave,
    /* ID: 405 */ raudio::load_wave_from_memory,
    /* ID: 406 */ raudio::load_wave_samples,
    /* ID: 407 */ rcore::maximize_window,
    /* ID: 408 */ rtext::measure_text,
    /* ID: 409 */ rtext::measure_text_ex,
    /* ID: 410 */ rcore::mem_alloc,
    /* ID: 411 */ rcore::mem_free,
    /* ID: 412 */ rcore::mem_realloc,
    /* ID: 413 */ rcore::minimize_window,
    /* ID: 414 */ rcore::open_url,
    /* ID: 415 */ raudio::pause_audio_stream,
    /* ID: 416 */ raudio::pause_music_stream,
    /* ID: 417 */ raudio::pause_sound,
    /* ID: 418 */ raudio::play_audio_stream,
    /* ID: 419 */ rcore::play_automation_event,
    /* ID: 420 */ raudio::play_music_stream,
    /* ID: 421 */ raudio::play_sound,
    /* ID: 422 */ rcore::poll_input_events,
    /* ID: 423 */ rcore::restore_window,
    /* ID: 424 */ raudio::resume_audio_stream,
    /* ID: 425 */ raudio::resume_music_stream,
    /* ID: 426 */ raudio::resume_sound,
    /* ID: 427 */ rcore::save_file_data,
    /* ID: 428 */ rcore::save_file_text,
    /* ID: 429 */ raudio::seek_music_stream,
    /* ID: 430 */ raudio::set_audio_stream_buffer_size_default,
    /* ID: 431 */ raudio::set_audio_stream_callback,
    /* ID: 432 */ raudio::set_audio_stream_pan,
    /* ID: 433 */ raudio::set_audio_stream_pitch,
    /* ID: 434 */ raudio::set_audio_stream_volume,
    /* ID: 435 */ rcore::set_automation_event_base_frame,
    /* ID: 436 */ rcore::set_automation_event_list,
    /* ID: 437 */ rcore::set_clipboard_text,
    /* ID: 438 */ rcore::set_config_flags,
    /* ID: 439 */ rcore::set_exit_key,
    /* ID: 440 */ rcore::set_gamepad_mappings,
    /* ID: 441 */ rcore::set_gestures_enabled,
    /* ID: 442 */ rcore::set_load_file_data_callback,
    /* ID: 443 */ rcore::set_load_file_text_callback,
    /* ID: 444 */ raudio::set_master_volume,
    /* ID: 445 */ rmodels::set_material_texture,
    /* ID: 446 */ rmodels::set_model_mesh_material,
    /* ID: 447 */ rcore::set_mouse_cursor,
    /* ID: 448 */ rcore::set_mouse_offset,
    /* ID: 449 */ rcore::set_mouse_position,
    /* ID: 450 */ rcore::set_mouse_scale,
    /* ID: 451 */ raudio::set_music_pan,
    /* ID: 452 */ raudio::set_music_pitch,
    /* ID: 453 */ raudio::set_music_volume,
    /* ID: 454 */ rtextures::set_pixel_color,
    /* ID: 455 */ rcore::set_random_seed,
    /* ID: 456 */ rcore::set_save_file_data_callback,
    /* ID: 457 */ rcore::set_save_file_text_callback,
    /* ID: 458 */ rcore::set_shader_value,
    /* ID: 459 */ rcore::set_shader_value_matrix,
    /* ID: 460 */ rcore::set_shader_value_texture,
    /* ID: 461 */ rcore::set_shader_value_v,
    /* ID: 462 */ rshapes::set_shapes_texture,
    /* ID: 463 */ raudio::set_sound_pan,
    /* ID: 464 */ raudio::set_sound_pitch,
    /* ID: 465 */ raudio::set_sound_volume,
    /* ID: 466 */ rcore::set_target_fps,
    /* ID: 467 */ rtext::set_text_line_spacing,
    /* ID: 468 */ rtextures::set_texture_filter,
    /* ID: 469 */ rtextures::set_texture_wrap,
    /* ID: 470 */ rcore::set_trace_log_callback,
    /* ID: 471 */ rcore::set_trace_log_level,
    /* ID: 472 */ rcore::set_window_focused,
    /* ID: 473 */ rcore::set_window_icon,
    /* ID: 474 */ rcore::set_window_icons,
    /* ID: 475 */ rcore::set_window_max_size,
    /* ID: 476 */ rcore::set_window_min_size,
    /* ID: 477 */ rcore::set_window_monitor,
    /* ID: 478 */ rcore::set_window_opacity,
    /* ID: 479 */ rcore::set_window_position,
    /* ID: 480 */ rcore::set_window_size,
    /* ID: 481 */ rcore::set_window_state,
    /* ID: 482 */ rcore::set_window_title,
    /* ID: 483 */ rcore::show_cursor,
    /* ID: 484 */ rcore::start_automation_event_recording,
    /* ID: 485 */ raudio::stop_audio_stream,
    /* ID: 486 */ rcore::stop_automation_event_recording,
    /* ID: 487 */ raudio::stop_music_stream,
    /* ID: 488 */ raudio::stop_sound,
    /* ID: 489 */ rcore::swap_screen_buffer,
    /* ID: 490 */ rcore::take_screenshot,
    /* ID: 491 */ rtext::text_append,
    /* ID: 492 */ rtext::text_copy,
    /* ID: 493 */ rtext::text_find_index,
    /* ID: 494 */ rtext::text_format,
    /* ID: 495 */ rtext::text_insert,
    /* ID: 496 */ rtext::text_is_equal,
    /* ID: 497 */ rtext::text_join,
    /* ID: 498 */ rtext::text_length,
    /* ID: 499 */ rtext::text_replace,
    /* ID: 500 */ rtext::text_split,
    /* ID: 501 */ rtext::text_subtext,
    /* ID: 502 */ rtext::text_to_integer,
    /* ID: 503 */ rtext::text_to_lower,
    /* ID: 504 */ rtext::text_to_pascal,
    /* ID: 505 */ rtext::text_to_upper,
    /* ID: 506 */ rcore::toggle_borderless_windowed,
    /* ID: 507 */ rcore::toggle_fullscreen,
    /* ID: 508 */ rcore::trace_log,
    /* ID: 509 */ raudio::unload_audio_stream,
    /* ID: 510 */ rcore::unload_automation_event_list,
    /* ID: 511 */ rtext::unload_codepoints,
    /* ID: 512 */ rcore::unload_directory_files,
    /* ID: 513 */ rcore::unload_dropped_files,
    /* ID: 514 */ rcore::unload_file_data,
    /* ID: 515 */ rcore::unload_file_text,
    /* ID: 516 */ rtext::unload_font,
    /* ID: 517 */ rtext::unload_font_data,
    /* ID: 518 */ rtextures::unload_image,
    /* ID: 519 */ rtextures::unload_image_colors,
    /* ID: 520 */ rtextures::unload_image_palette,
    /* ID: 521 */ rmodels::unload_material,
    /* ID: 522 */ rmodels::unload_mesh,
    /* ID: 523 */ rmodels::unload_model,
    /* ID: 524 */ rmodels::unload_model_animation,
    /* ID: 525 */ rmodels::unload_model_animations,
    /* ID: 526 */ raudio::unload_music_stream,
    /* ID: 527 */ rcore::unload_random_sequence,
    /* ID: 528 */ rtextures::unload_render_texture,
    /* ID: 529 */ rcore::unload_shader,
    /* ID: 530 */ raudio::unload_sound,
    /* ID: 531 */ raudio::unload_sound_alias,
    /* ID: 532 */ rtextures::unload_texture,
    /* ID: 533 */ rtext::unload_utf8,
    /* ID: 534 */ rcore::unload_vr_stereo_config,
    /* ID: 535 */ raudio::unload_wave,
    /* ID: 536 */ raudio::unload_wave_samples,
    /* ID: 537 */ raudio::update_audio_stream,
    /* ID: 538 */ rcore::update_camera,
    /* ID: 539 */ rcore::update_camera_pro,
    /* ID: 540 */ rmodels::update_mesh_buffer,
    /* ID: 541 */ rmodels::update_model_animation,
    /* ID: 542 */ raudio::update_music_stream,
    /* ID: 543 */ raudio::update_sound,
    /* ID: 544 */ rtextures::update_texture,
    /* ID: 545 */ rtextures::update_texture_rec,
    /* ID: 546 */ rmodels::upload_mesh,
    /* ID: 547 */ rcore::wait_time,
    /* ID: 548 */ raudio::wave_copy,
    /* ID: 549 */ raudio::wave_crop,
    /* ID: 550 */ raudio::wave_format,
    /* ID: 551 */ rcore::window_should_close,
];
