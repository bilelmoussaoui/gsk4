// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use BlendMode;
use RenderNode;
use RenderNodeType;
use RoundedRect;
use cairo;
use gdk;
use glib::GString;
use glib::object::IsA;
use glib::translate::*;
use graphene;
use pango;

use std::convert::TryFrom;
use std::fmt;
use std::ops::Deref;

// RenderNode subtypes

macro_rules! convert {
    ($source: ident => $dest: ident = $( $variant: ident )|+ $( ($intermediate: ident) )*) => {
        impl TryFrom<$source> for $dest {
            type Error = $source;

            fn try_from(value: $source) -> Result<Self, $source> {
                if $( value.get_node_type() == RenderNodeType::$variant )||+ {
                    $(
                        let value = $intermediate(value);
                    )*
                    Ok($dest(value))
                }
                else {
                    Err(value)
                }
            }
        }
    };
}

macro_rules! subtype(
    ($subtype: ident $( = $variant: ident)*) => (

        #[derive(Debug, Clone)]
        pub struct $subtype(RenderNode);

        impl Deref for $subtype {
            type Target = RenderNode;

            fn deref(&self) -> &RenderNode {
                &self.0
            }
        }

        impl fmt::Display for $subtype {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, stringify!($subtype))
            }
        }

        $(
            convert!(RenderNode => $subtype = $variant);
        )*
    );
);

subtype!(BlendNode = BlendNode);
subtype!(BlurNode = BlurNode);
subtype!(BorderNode = BorderNode);
subtype!(CairoNode = CairoNode);
subtype!(ClipNode = ClipNode);
subtype!(ColorMatrixNode = ColorMatrixNode);
subtype!(ColorNode = ColorNode);
subtype!(ContainerNode = ContainerNode);
subtype!(CrossFadeNode = CrossFadeNode);
subtype!(DebugNode = DebugNode);
subtype!(InsetShadowNode = InsetShadowNode);
subtype!(LinearGradientNode = LinearGradientNode);
subtype!(OpacityNode = OpacityNode);
subtype!(OutsetShadowNode = OutsetShadowNode);
subtype!(RepeatNode = RepeatNode);
subtype!(RoundedClipNode = RoundedClipNode);
subtype!(TextNode = TextNode);
subtype!(TextureNode = TextureNode);

// TODO: border_node_new()
// TODO: border_node_peek_widths()
// TODO: cairo_node_peek_surface()
// TODO: container_node_new()
// TODO: linear_gradient_node_new()
// TODO: linear_gradient_node_peek_color_stops()
// TODO: repeating_linear_gradient_node_new()
// TODO: text_node_peek_glyphs()

impl BlendNode {
    pub fn get_blend_mode(node: &RenderNode) -> BlendMode {
        assert_initialized_main_thread!();
        unsafe {
            from_glib(gsk_sys::gsk_blend_node_get_blend_mode(node.to_glib_none().0))
        }
    }

    pub fn get_bottom_child(node: &RenderNode) -> Option<RenderNode> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gsk_sys::gsk_blend_node_get_bottom_child(node.to_glib_none().0))
        }
    }

    pub fn get_top_child(node: &RenderNode) -> Option<RenderNode> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gsk_sys::gsk_blend_node_get_top_child(node.to_glib_none().0))
        }
    }

    pub fn new(bottom: &RenderNode, top: &RenderNode, blend_mode: BlendMode) -> Option<RenderNode> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gsk_sys::gsk_blend_node_new(bottom.to_glib_none().0, top.to_glib_none().0, blend_mode.to_glib()))
        }
    }
}

impl BlurNode {
    pub fn get_child(node: &RenderNode) -> Option<RenderNode> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gsk_sys::gsk_blur_node_get_child(node.to_glib_none().0))
        }
    }

    pub fn get_radius(node: &RenderNode) -> f64 {
        assert_initialized_main_thread!();
        unsafe {
            gsk_sys::gsk_blur_node_get_radius(node.to_glib_none().0)
        }
    }

    pub fn new(child: &RenderNode, radius: f64) -> Option<RenderNode> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gsk_sys::gsk_blur_node_new(child.to_glib_none().0, radius))
        }
    }
}

impl BorderNode {
    pub fn peek_colors(node: &RenderNode) -> Option<gdk::RGBA> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(gsk_sys::gsk_border_node_peek_colors(node.to_glib_none().0))
        }
    }

    pub fn peek_outline(node: &RenderNode) -> Option<RoundedRect> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(gsk_sys::gsk_border_node_peek_outline(node.to_glib_none().0))
        }
    }
}

impl CairoNode {
    pub fn get_draw_context(node: &RenderNode) -> Option<cairo::Context> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gsk_sys::gsk_cairo_node_get_draw_context(node.to_glib_none().0))
        }
    }

    pub fn new(bounds: &graphene::Rect) -> Option<RenderNode> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gsk_sys::gsk_cairo_node_new(bounds.to_glib_none().0))
        }
    }
}

impl ClipNode {
    pub fn get_child(node: &RenderNode) -> Option<RenderNode> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(gsk_sys::gsk_clip_node_get_child(node.to_glib_none().0))
        }
    }

    pub fn new(child: &RenderNode, clip: &graphene::Rect) -> Option<RenderNode> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gsk_sys::gsk_clip_node_new(child.to_glib_none().0, clip.to_glib_none().0))
        }
    }

    pub fn peek_clip(node: &RenderNode) -> Option<graphene::Rect> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(gsk_sys::gsk_clip_node_peek_clip(node.to_glib_none().0))
        }
    }
}

impl ColorMatrixNode {
    pub fn get_child(node: &RenderNode) -> Option<RenderNode> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(gsk_sys::gsk_color_matrix_node_get_child(node.to_glib_none().0))
        }
    }

    pub fn new(child: &RenderNode, color_matrix: &graphene::Matrix, color_offset: &graphene::Vec4) -> Option<RenderNode> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gsk_sys::gsk_color_matrix_node_new(child.to_glib_none().0, color_matrix.to_glib_none().0, color_offset.to_glib_none().0))
        }
    }

    pub fn peek_color_matrix(node: &RenderNode) -> Option<graphene::Matrix> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(gsk_sys::gsk_color_matrix_node_peek_color_matrix(node.to_glib_none().0))
        }
    }

    pub fn peek_color_offset(node: &RenderNode) -> Option<graphene::Vec4> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(gsk_sys::gsk_color_matrix_node_peek_color_offset(node.to_glib_none().0))
        }
    }
}

impl ColorNode {
    pub fn new(rgba: &gdk::RGBA, bounds: &graphene::Rect) -> Option<RenderNode> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gsk_sys::gsk_color_node_new(rgba.to_glib_none().0, bounds.to_glib_none().0))
        }
    }

    pub fn peek_color(node: &RenderNode) -> Option<gdk::RGBA> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(gsk_sys::gsk_color_node_peek_color(node.to_glib_none().0))
        }
    }
}

impl ContainerNode {
    pub fn get_child(node: &RenderNode, idx: u32) -> Option<RenderNode> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gsk_sys::gsk_container_node_get_child(node.to_glib_none().0, idx))
        }
    }

    pub fn get_n_children(node: &RenderNode) -> u32 {
        assert_initialized_main_thread!();
        unsafe {
            gsk_sys::gsk_container_node_get_n_children(node.to_glib_none().0)
        }
    }
}

impl CrossFadeNode {
    pub fn get_end_child(node: &RenderNode) -> Option<RenderNode> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gsk_sys::gsk_cross_fade_node_get_end_child(node.to_glib_none().0))
        }
    }

    pub fn get_progress(node: &RenderNode) -> f64 {
        assert_initialized_main_thread!();
        unsafe {
            gsk_sys::gsk_cross_fade_node_get_progress(node.to_glib_none().0)
        }
    }

    pub fn get_start_child(node: &RenderNode) -> Option<RenderNode> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gsk_sys::gsk_cross_fade_node_get_start_child(node.to_glib_none().0))
        }
    }

    pub fn new(start: &RenderNode, end: &RenderNode, progress: f64) -> Option<RenderNode> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gsk_sys::gsk_cross_fade_node_new(start.to_glib_none().0, end.to_glib_none().0, progress))
        }
    }
}

impl DebugNode {
    pub fn get_child(node: &RenderNode) -> Option<RenderNode> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(gsk_sys::gsk_debug_node_get_child(node.to_glib_none().0))
        }
    }

    pub fn get_message(node: &RenderNode) -> Option<GString> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(gsk_sys::gsk_debug_node_get_message(node.to_glib_none().0))
        }
    }

    pub fn new(child: &RenderNode, message: &str) -> Option<RenderNode> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gsk_sys::gsk_debug_node_new(child.to_glib_none().0, message.to_glib_full()))
        }
    }
}

impl InsetShadowNode {
    pub fn get_blur_radius(node: &RenderNode) -> f32 {
        assert_initialized_main_thread!();
        unsafe {
            gsk_sys::gsk_inset_shadow_node_get_blur_radius(node.to_glib_none().0)
        }
    }

    pub fn get_dx(node: &RenderNode) -> f32 {
        assert_initialized_main_thread!();
        unsafe {
            gsk_sys::gsk_inset_shadow_node_get_dx(node.to_glib_none().0)
        }
    }

    pub fn get_dy(node: &RenderNode) -> f32 {
        assert_initialized_main_thread!();
        unsafe {
            gsk_sys::gsk_inset_shadow_node_get_dy(node.to_glib_none().0)
        }
    }

    pub fn get_spread(node: &RenderNode) -> f32 {
        assert_initialized_main_thread!();
        unsafe {
            gsk_sys::gsk_inset_shadow_node_get_spread(node.to_glib_none().0)
        }
    }

    pub fn new(outline: &RoundedRect, color: &gdk::RGBA, dx: f32, dy: f32, spread: f32, blur_radius: f32) -> Option<RenderNode> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gsk_sys::gsk_inset_shadow_node_new(outline.to_glib_none().0, color.to_glib_none().0, dx, dy, spread, blur_radius))
        }
    }

    pub fn peek_color(node: &RenderNode) -> Option<gdk::RGBA> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(gsk_sys::gsk_inset_shadow_node_peek_color(node.to_glib_none().0))
        }
    }

    pub fn peek_outline(node: &RenderNode) -> Option<RoundedRect> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(gsk_sys::gsk_inset_shadow_node_peek_outline(node.to_glib_none().0))
        }
    }
}

impl LinearGradientNode {
    pub fn get_n_color_stops(node: &RenderNode) -> usize {
        assert_initialized_main_thread!();
        unsafe {
            gsk_sys::gsk_linear_gradient_node_get_n_color_stops(node.to_glib_none().0)
        }
    }

    pub fn peek_end(node: &RenderNode) -> Option<graphene::Point> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(gsk_sys::gsk_linear_gradient_node_peek_end(node.to_glib_none().0))
        }
    }

    pub fn peek_start(node: &RenderNode) -> Option<graphene::Point> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(gsk_sys::gsk_linear_gradient_node_peek_start(node.to_glib_none().0))
        }
    }
}

impl OpacityNode {
    pub fn get_child(node: &RenderNode) -> Option<RenderNode> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(gsk_sys::gsk_opacity_node_get_child(node.to_glib_none().0))
        }
    }

    pub fn get_opacity(node: &RenderNode) -> f64 {
        assert_initialized_main_thread!();
        unsafe {
            gsk_sys::gsk_opacity_node_get_opacity(node.to_glib_none().0)
        }
    }

    pub fn new(child: &RenderNode, opacity: f64) -> Option<RenderNode> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gsk_sys::gsk_opacity_node_new(child.to_glib_none().0, opacity))
        }
    }
}

impl OutsetShadowNode {
    pub fn get_blur_radius(node: &RenderNode) -> f32 {
        assert_initialized_main_thread!();
        unsafe {
            gsk_sys::gsk_outset_shadow_node_get_blur_radius(node.to_glib_none().0)
        }
    }

    pub fn get_dx(node: &RenderNode) -> f32 {
        assert_initialized_main_thread!();
        unsafe {
            gsk_sys::gsk_outset_shadow_node_get_dx(node.to_glib_none().0)
        }
    }

    pub fn get_dy(node: &RenderNode) -> f32 {
        assert_initialized_main_thread!();
        unsafe {
            gsk_sys::gsk_outset_shadow_node_get_dy(node.to_glib_none().0)
        }
    }

    pub fn get_spread(node: &RenderNode) -> f32 {
        assert_initialized_main_thread!();
        unsafe {
            gsk_sys::gsk_outset_shadow_node_get_spread(node.to_glib_none().0)
        }
    }

    pub fn new(outline: &RoundedRect, color: &gdk::RGBA, dx: f32, dy: f32, spread: f32, blur_radius: f32) -> Option<RenderNode> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gsk_sys::gsk_outset_shadow_node_new(outline.to_glib_none().0, color.to_glib_none().0, dx, dy, spread, blur_radius))
        }
    }

    pub fn peek_color(node: &RenderNode) -> Option<gdk::RGBA> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(gsk_sys::gsk_outset_shadow_node_peek_color(node.to_glib_none().0))
        }
    }

    pub fn peek_outline(node: &RenderNode) -> Option<RoundedRect> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(gsk_sys::gsk_outset_shadow_node_peek_outline(node.to_glib_none().0))
        }
    }
}

impl RepeatNode {
    pub fn get_child(node: &RenderNode) -> Option<RenderNode> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gsk_sys::gsk_repeat_node_get_child(node.to_glib_none().0))
        }
    }

    pub fn new(bounds: &graphene::Rect, child: &RenderNode, child_bounds: Option<&graphene::Rect>) -> Option<RenderNode> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gsk_sys::gsk_repeat_node_new(bounds.to_glib_none().0, child.to_glib_none().0, child_bounds.to_glib_none().0))
        }
    }

    pub fn peek_child_bounds(node: &RenderNode) -> Option<graphene::Rect> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(gsk_sys::gsk_repeat_node_peek_child_bounds(node.to_glib_none().0))
        }
    }
}

impl RoundedClipNode {
    pub fn get_child(node: &RenderNode) -> Option<RenderNode> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(gsk_sys::gsk_rounded_clip_node_get_child(node.to_glib_none().0))
        }
    }

    pub fn new(child: &RenderNode, clip: &RoundedRect) -> Option<RenderNode> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gsk_sys::gsk_rounded_clip_node_new(child.to_glib_none().0, clip.to_glib_none().0))
        }
    }

    pub fn peek_clip(node: &RenderNode) -> Option<RoundedRect> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(gsk_sys::gsk_rounded_clip_node_peek_clip(node.to_glib_none().0))
        }
    }
}

impl TextNode {
    pub fn get_num_glyphs(node: &RenderNode) -> u32 {
        assert_initialized_main_thread!();
        unsafe {
            gsk_sys::gsk_text_node_get_num_glyphs(node.to_glib_none().0)
        }
    }

    pub fn get_x(node: &RenderNode) -> f32 {
        assert_initialized_main_thread!();
        unsafe {
            gsk_sys::gsk_text_node_get_x(node.to_glib_none().0)
        }
    }

    pub fn get_y(node: &RenderNode) -> f32 {
        assert_initialized_main_thread!();
        unsafe {
            gsk_sys::gsk_text_node_get_y(node.to_glib_none().0)
        }
    }

    pub fn new<P: IsA<pango::Font>>(font: &P, glyphs: &mut pango::GlyphString, color: &gdk::RGBA, x: f32, y: f32) -> Option<RenderNode> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gsk_sys::gsk_text_node_new(font.as_ref().to_glib_none().0, glyphs.to_glib_none_mut().0, color.to_glib_none().0, x, y))
        }
    }

    pub fn peek_color(node: &RenderNode) -> Option<gdk::RGBA> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(gsk_sys::gsk_text_node_peek_color(node.to_glib_none().0))
        }
    }

    pub fn peek_font(node: &RenderNode) -> Option<pango::Font> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(gsk_sys::gsk_text_node_peek_font(node.to_glib_none().0))
        }
    }
}

impl TextureNode {
    pub fn get_texture(node: &RenderNode) -> Option<gdk::Texture> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(gsk_sys::gsk_texture_node_get_texture(node.to_glib_none().0))
        }
    }

    pub fn new<P: IsA<gdk::Texture>>(texture: &P, bounds: &graphene::Rect) -> Option<RenderNode> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(gsk_sys::gsk_texture_node_new(texture.as_ref().to_glib_none().0, bounds.to_glib_none().0))
        }
    }
}