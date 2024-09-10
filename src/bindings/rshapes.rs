use super::{FnOutput, Memory, Pointer};

/// Set texture and rectangle to be used on shapes drawing
pub unsafe fn set_shapes_texture(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw a pixel
pub unsafe fn draw_pixel(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw a pixel (Vector version)
pub unsafe fn draw_pixel_v(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw a line
pub unsafe fn draw_line(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw a line (using gl lines)
pub unsafe fn draw_line_v(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw a line (using triangles/quads)
pub unsafe fn draw_line_ex(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw lines sequence (using gl lines)
pub unsafe fn draw_line_strip(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw line segment cubic-bezier in-out interpolation
pub unsafe fn draw_line_bezier(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw a color-filled circle
pub unsafe fn draw_circle(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw a piece of a circle
pub unsafe fn draw_circle_sector(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw circle sector outline
pub unsafe fn draw_circle_sector_lines(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw a gradient-filled circle
pub unsafe fn draw_circle_gradient(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw a color-filled circle (Vector version)
pub unsafe fn draw_circle_v(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw circle outline
pub unsafe fn draw_circle_lines(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw circle outline (Vector version)
pub unsafe fn draw_circle_lines_v(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw ellipse
pub unsafe fn draw_ellipse(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw ellipse outline
pub unsafe fn draw_ellipse_lines(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw ring
pub unsafe fn draw_ring(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw ring outline
pub unsafe fn draw_ring_lines(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw a color-filled rectangle
pub unsafe fn draw_rectangle(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw a color-filled rectangle (Vector version)
pub unsafe fn draw_rectangle_v(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw a color-filled rectangle
pub unsafe fn draw_rectangle_rec(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw a color-filled rectangle with pro parameters
pub unsafe fn draw_rectangle_pro(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw a vertical-gradient-filled rectangle
pub unsafe fn draw_rectangle_gradient_v(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw a horizontal-gradient-filled rectangle
pub unsafe fn draw_rectangle_gradient_h(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw a gradient-filled rectangle with custom vertex colors
pub unsafe fn draw_rectangle_gradient_ex(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw rectangle outline
pub unsafe fn draw_rectangle_lines(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw rectangle outline with extended parameters
pub unsafe fn draw_rectangle_lines_ex(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw rectangle with rounded edges
pub unsafe fn draw_rectangle_rounded(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw rectangle with rounded edges outline
pub unsafe fn draw_rectangle_rounded_lines(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw a color-filled triangle (vertex in counter-clockwise order!)
pub unsafe fn draw_triangle(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw triangle outline (vertex in counter-clockwise order!)
pub unsafe fn draw_triangle_lines(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw a triangle fan defined by points (first vertex is the center)
pub unsafe fn draw_triangle_fan(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw a triangle strip defined by points
pub unsafe fn draw_triangle_strip(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw a regular polygon (Vector version)
pub unsafe fn draw_poly(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw a polygon outline of n sides
pub unsafe fn draw_poly_lines(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw a polygon outline of n sides with extended parameters
pub unsafe fn draw_poly_lines_ex(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw spline: Linear, minimum 2 points
pub unsafe fn draw_spline_linear(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw spline: B-Spline, minimum 4 points
pub unsafe fn draw_spline_basis(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw spline: Catmull-Rom, minimum 4 points
pub unsafe fn draw_spline_catmull_rom(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw spline: Quadratic Bezier, minimum 3 points (1 control point): [p1, c2, p3, c4...]
pub unsafe fn draw_spline_bezier_quadratic(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw spline: Cubic Bezier, minimum 4 points (2 control points): [p1, c2, c3, p4, c5, c6...]
pub unsafe fn draw_spline_bezier_cubic(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw spline segment: Linear, 2 points
pub unsafe fn draw_spline_segment_linear(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw spline segment: B-Spline, 4 points
pub unsafe fn draw_spline_segment_basis(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw spline segment: Catmull-Rom, 4 points
pub unsafe fn draw_spline_segment_catmull_rom(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw spline segment: Quadratic Bezier, 2 points, 1 control point
pub unsafe fn draw_spline_segment_bezier_quadratic(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Draw spline segment: Cubic Bezier, 2 points, 2 control points
pub unsafe fn draw_spline_segment_bezier_cubic(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get (evaluate) spline point: Linear
pub unsafe fn get_spline_point_linear(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get (evaluate) spline point: B-Spline
pub unsafe fn get_spline_point_basis(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get (evaluate) spline point: Catmull-Rom
pub unsafe fn get_spline_point_catmull_rom(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get (evaluate) spline point: Quadratic Bezier
pub unsafe fn get_spline_point_bezier_quad(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get (evaluate) spline point: Cubic Bezier
pub unsafe fn get_spline_point_bezier_cubic(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check collision between two rectangles
pub unsafe fn check_collision_recs(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check collision between two circles
pub unsafe fn check_collision_circles(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check collision between circle and rectangle
pub unsafe fn check_collision_circle_rec(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if point is inside rectangle
pub unsafe fn check_collision_point_rec(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if point is inside circle
pub unsafe fn check_collision_point_circle(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if point is inside a triangle
pub unsafe fn check_collision_point_triangle(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if point is within a polygon described by array of vertices
pub unsafe fn check_collision_point_poly(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check the collision between two lines defined by two points each, returns collision point by reference
pub unsafe fn check_collision_lines(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Check if point belongs to line created between two points [p1] and [p2] with defined margin in pixels [threshold]
pub unsafe fn check_collision_point_line(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}

/// Get collision rectangle for two rectangles collision
pub unsafe fn get_collision_rec(_mem: &mut Memory, _index: Pointer) -> FnOutput {
    unimplemented!()
}
