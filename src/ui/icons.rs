use eframe::egui::{self, Color32, Pos2, Rect, Stroke, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IconKind {
    Search,
    Broom,
    Folder,
    FolderOpen,
    Lightning,
    Trash,
    Archive,
    Globe,
    FileText,
    Image,
    Disc,
    Wrench,
    Music,
    Refresh,
    Shield,
    AlertTriangle,
    Check,
    Close,
    ChevronRight,
    ChevronDown,
    Rocket,
    Sparkles,
}

/// Paints a vector icon scaled inside the given rectangle.
pub fn paint_icon(painter: &egui::Painter, rect: Rect, kind: IconKind, color: Color32) {
    let stroke = Stroke::new(1.6_f32, color);
    let center = rect.center();
    let w = rect.width();
    let h = rect.height();
    let s = w.min(h);

    match kind {
        IconKind::Search => {
            // Magnifying glass: circle + diagonal handle
            let r = s * 0.32;
            let c = Pos2::new(center.x - s * 0.1, center.y - s * 0.1);
            painter.circle_stroke(c, r, stroke);
            let handle_start = Pos2::new(c.x + r * 0.707, c.y + r * 0.707);
            let handle_end = Pos2::new(center.x + s * 0.42, center.y + s * 0.42);
            painter.line_segment([handle_start, handle_end], Stroke::new(2.0_f32, color));
        }

        IconKind::Broom => {
            // Sweeping broom: angled handle + angled bristles
            let p1 = Pos2::new(center.x + s * 0.35, center.y - s * 0.4);
            let p2 = Pos2::new(center.x - s * 0.05, center.y + s * 0.05);
            painter.line_segment([p1, p2], Stroke::new(2.2_f32, color));

            // Bristles fan
            let b1 = Pos2::new(center.x - s * 0.2, center.y - s * 0.05);
            let b2 = Pos2::new(center.x + s * 0.05, center.y + s * 0.2);
            let b3 = Pos2::new(center.x - s * 0.35, center.y + s * 0.4);
            let b4 = Pos2::new(center.x - s * 0.45, center.y + s * 0.25);
            painter.line_segment([p2, Pos2::new(center.x - s * 0.35, center.y + s * 0.35)], stroke);
            painter.line_segment([b1, b4], stroke);
            painter.line_segment([b2, b3], stroke);
            painter.line_segment([b4, b3], Stroke::new(1.8_f32, color));
        }

        IconKind::Folder => {
            // Folder: tab on top-left + main body
            let x0 = rect.min.x + s * 0.1;
            let x1 = rect.max.x - s * 0.1;
            let y0 = rect.min.y + s * 0.2;
            let y1 = rect.max.y - s * 0.2;
            let tab_w = s * 0.35;
            let tab_h = s * 0.12;

            // Tab
            painter.line_segment([Pos2::new(x0, y0), Pos2::new(x0 + tab_w, y0)], stroke);
            painter.line_segment([Pos2::new(x0 + tab_w, y0), Pos2::new(x0 + tab_w + s * 0.08, y0 + tab_h)], stroke);
            painter.line_segment([Pos2::new(x0 + tab_w + s * 0.08, y0 + tab_h), Pos2::new(x1, y0 + tab_h)], stroke);
            // Body
            painter.line_segment([Pos2::new(x0, y0), Pos2::new(x0, y1)], stroke);
            painter.line_segment([Pos2::new(x0, y1), Pos2::new(x1, y1)], stroke);
            painter.line_segment([Pos2::new(x1, y1), Pos2::new(x1, y0 + tab_h)], stroke);
        }

        IconKind::FolderOpen => {
            let x0 = rect.min.x + s * 0.1;
            let x1 = rect.max.x - s * 0.1;
            let y0 = rect.min.y + s * 0.2;
            let y1 = rect.max.y - s * 0.2;

            // Back folder
            painter.line_segment([Pos2::new(x0, y0), Pos2::new(x0 + s * 0.35, y0)], stroke);
            painter.line_segment([Pos2::new(x0 + s * 0.35, y0), Pos2::new(x1, y0 + s * 0.12)], stroke);
            painter.line_segment([Pos2::new(x0, y0), Pos2::new(x0, y1)], stroke);
            // Open flap
            let f0 = Pos2::new(x0 - s * 0.02, y1);
            let f1 = Pos2::new(x0 + s * 0.15, y0 + s * 0.35);
            let f2 = Pos2::new(x1 + s * 0.05, y0 + s * 0.35);
            let f3 = Pos2::new(x1 - s * 0.05, y1);
            painter.line_segment([f0, f1], stroke);
            painter.line_segment([f1, f2], stroke);
            painter.line_segment([f2, f3], stroke);
            painter.line_segment([f3, f0], stroke);
        }

        IconKind::Lightning => {
            // Bold dynamic lightning bolt
            let pts = [
                Pos2::new(center.x + s * 0.08, center.y - s * 0.44),
                Pos2::new(center.x - s * 0.25, center.y + s * 0.04),
                Pos2::new(center.x - s * 0.02, center.y + s * 0.04),
                Pos2::new(center.x - s * 0.18, center.y + s * 0.44),
                Pos2::new(center.x + s * 0.26, center.y - s * 0.05),
                Pos2::new(center.x + s * 0.05, center.y - s * 0.05),
            ];
            painter.add(egui::epaint::Shape::convex_polygon(pts.to_vec(), color, Stroke::NONE));
        }

        IconKind::Trash => {
            // Trash can: lid + handle + body + internal ribs
            let x0 = rect.min.x + s * 0.2;
            let x1 = rect.max.x - s * 0.2;
            let y_lid = center.y - s * 0.25;
            let y_bot = center.y + s * 0.38;

            // Lid line
            painter.line_segment([Pos2::new(x0 - s * 0.08, y_lid), Pos2::new(x1 + s * 0.08, y_lid)], stroke);
            // Lid handle
            let h0 = Pos2::new(center.x - s * 0.12, y_lid);
            let h1 = Pos2::new(center.x - s * 0.12, y_lid - s * 0.1);
            let h2 = Pos2::new(center.x + s * 0.12, y_lid - s * 0.1);
            let h3 = Pos2::new(center.x + s * 0.12, y_lid);
            painter.line_segment([h0, h1], stroke);
            painter.line_segment([h1, h2], stroke);
            painter.line_segment([h2, h3], stroke);

            // Can body (slight taper)
            let b0 = Pos2::new(x0, y_lid);
            let b1 = Pos2::new(x0 + s * 0.05, y_bot);
            let b2 = Pos2::new(x1 - s * 0.05, y_bot);
            let b3 = Pos2::new(x1, y_lid);
            painter.line_segment([b0, b1], stroke);
            painter.line_segment([b1, b2], stroke);
            painter.line_segment([b2, b3], stroke);

            // Center ribs
            painter.line_segment([Pos2::new(center.x, y_lid + s * 0.1), Pos2::new(center.x, y_bot - s * 0.08)], stroke);
        }

        IconKind::Archive => {
            // Storage box / Package
            let x0 = rect.min.x + s * 0.15;
            let x1 = rect.max.x - s * 0.15;
            let y0 = rect.min.y + s * 0.18;
            let y1 = rect.max.y - s * 0.18;

            // Box outline
            painter.rect_stroke(Rect::from_min_max(Pos2::new(x0, y0), Pos2::new(x1, y1)), 2.0, stroke);
            // Lid flap line
            let flap_y = y0 + (y1 - y0) * 0.32;
            painter.line_segment([Pos2::new(x0, flap_y), Pos2::new(x1, flap_y)], stroke);
            // Handle slot
            let h_w = (x1 - x0) * 0.35;
            painter.line_segment(
                [Pos2::new(center.x - h_w * 0.5, flap_y + s * 0.15), Pos2::new(center.x + h_w * 0.5, flap_y + s * 0.15)],
                Stroke::new(2.0_f32, color),
            );
        }

        IconKind::Globe => {
            // Sphere with equator and longitude ellipse
            let r = s * 0.38;
            painter.circle_stroke(center, r, stroke);
            // Equator
            painter.line_segment([Pos2::new(center.x - r, center.y), Pos2::new(center.x + r, center.y)], stroke);
            // Longitude line
            painter.line_segment([Pos2::new(center.x, center.y - r), Pos2::new(center.x, center.y + r)], stroke);
        }

        IconKind::FileText => {
            // Document with folded corner and lines
            let x0 = rect.min.x + s * 0.2;
            let x1 = rect.max.x - s * 0.2;
            let y0 = rect.min.y + s * 0.12;
            let y1 = rect.max.y - s * 0.12;
            let fold = s * 0.2;

            painter.line_segment([Pos2::new(x0, y0), Pos2::new(x1 - fold, y0)], stroke);
            painter.line_segment([Pos2::new(x1 - fold, y0), Pos2::new(x1, y0 + fold)], stroke);
            painter.line_segment([Pos2::new(x1, y0 + fold), Pos2::new(x1, y1)], stroke);
            painter.line_segment([Pos2::new(x1, y1), Pos2::new(x0, y1)], stroke);
            painter.line_segment([Pos2::new(x0, y1), Pos2::new(x0, y0)], stroke);
            // Corner fold
            painter.line_segment([Pos2::new(x1 - fold, y0), Pos2::new(x1 - fold, y0 + fold)], stroke);
            painter.line_segment([Pos2::new(x1 - fold, y0 + fold), Pos2::new(x1, y0 + fold)], stroke);

            // Horizontal text lines
            let l_x0 = x0 + s * 0.1;
            let l_x1 = x1 - s * 0.1;
            painter.line_segment([Pos2::new(l_x0, center.y), Pos2::new(l_x1, center.y)], stroke);
            painter.line_segment([Pos2::new(l_x0, center.y + s * 0.14), Pos2::new(l_x1 - s * 0.1, center.y + s * 0.14)], stroke);
        }

        IconKind::Image => {
            // Photo frame with hills/mountains
            let x0 = rect.min.x + s * 0.15;
            let x1 = rect.max.x - s * 0.15;
            let y0 = rect.min.y + s * 0.2;
            let y1 = rect.max.y - s * 0.2;

            painter.rect_stroke(Rect::from_min_max(Pos2::new(x0, y0), Pos2::new(x1, y1)), 2.0, stroke);
            // Sun
            painter.circle_filled(Pos2::new(x0 + s * 0.2, y0 + s * 0.18), s * 0.06, color);
            // Mountains
            let m0 = Pos2::new(x0, y1);
            let m1 = Pos2::new(x0 + s * 0.25, center.y);
            let m2 = Pos2::new(x0 + s * 0.45, y1);
            let m3 = Pos2::new(x0 + s * 0.55, center.y + s * 0.05);
            let m4 = Pos2::new(x1, y1);
            painter.line_segment([m0, m1], stroke);
            painter.line_segment([m1, m2], stroke);
            painter.line_segment([m2, m3], stroke);
            painter.line_segment([m3, m4], stroke);
        }

        IconKind::Disc => {
            // Compact disc / package installer
            let r = s * 0.38;
            painter.circle_stroke(center, r, stroke);
            painter.circle_stroke(center, r * 0.35, stroke);
            painter.circle_filled(center, r * 0.12, color);
        }

        IconKind::Wrench => {
            // Tools / Wrench
            let p1 = Pos2::new(center.x - s * 0.3, center.y + s * 0.3);
            let p2 = Pos2::new(center.x + s * 0.15, center.y - s * 0.15);
            painter.line_segment([p1, p2], Stroke::new(2.4_f32, color));
            // Head
            painter.circle_stroke(Pos2::new(center.x + s * 0.22, center.y - s * 0.22), s * 0.16, Stroke::new(2.0_f32, color));
        }

        IconKind::Music => {
            // Double musical note with beam
            let n1_c = Pos2::new(center.x - s * 0.2, center.y + s * 0.22);
            let n2_c = Pos2::new(center.x + s * 0.18, center.y + s * 0.12);
            let r = s * 0.1;
            painter.circle_filled(n1_c, r, color);
            painter.circle_filled(n2_c, r, color);

            // Stems
            let s1_top = Pos2::new(n1_c.x + r, center.y - s * 0.3);
            let s2_top = Pos2::new(n2_c.x + r, center.y - s * 0.4);
            painter.line_segment([Pos2::new(n1_c.x + r, n1_c.y), s1_top], stroke);
            painter.line_segment([Pos2::new(n2_c.x + r, n2_c.y), s2_top], stroke);
            // Connecting beam
            painter.line_segment([s1_top, s2_top], Stroke::new(2.6_f32, color));
        }

        IconKind::Refresh => {
            // Circular refresh arc + arrowhead
            let r = s * 0.32;
            let _arc_rect = Rect::from_center_size(center, Vec2::splat(r * 2.0));
            // Draw 3/4 circle
            let num_pts = 16;
            let start_angle = 0.4_f32;
            let end_angle = 5.6_f32;
            for i in 0..num_pts {
                let a1 = start_angle + (end_angle - start_angle) * (i as f32 / num_pts as f32);
                let a2 = start_angle + (end_angle - start_angle) * ((i + 1) as f32 / num_pts as f32);
                let p1 = Pos2::new(center.x + r * a1.cos(), center.y + r * a1.sin());
                let p2 = Pos2::new(center.x + r * a2.cos(), center.y + r * a2.sin());
                painter.line_segment([p1, p2], stroke);
            }
            // Arrowhead at start
            let tip = Pos2::new(center.x + r * start_angle.cos(), center.y + r * start_angle.sin());
            painter.line_segment([tip, Pos2::new(tip.x + s * 0.12, tip.y - s * 0.05)], stroke);
            painter.line_segment([tip, Pos2::new(tip.x + s * 0.12, tip.y + s * 0.12)], stroke);
        }

        IconKind::Shield => {
            // Security shield with check or exclamation
            let x0 = rect.min.x + s * 0.15;
            let x1 = rect.max.x - s * 0.15;
            let y0 = rect.min.y + s * 0.12;
            let y_bot = rect.max.y - s * 0.12;

            painter.line_segment([Pos2::new(x0, y0), Pos2::new(x1, y0)], stroke);
            painter.line_segment([Pos2::new(x0, y0), Pos2::new(x0, center.y)], stroke);
            painter.line_segment([Pos2::new(x1, y0), Pos2::new(x1, center.y)], stroke);
            painter.line_segment([Pos2::new(x0, center.y), Pos2::new(center.x, y_bot)], stroke);
            painter.line_segment([Pos2::new(x1, center.y), Pos2::new(center.x, y_bot)], stroke);

            // Center exclamation or mark
            painter.line_segment([Pos2::new(center.x, center.y - s * 0.15), Pos2::new(center.x, center.y + s * 0.05)], Stroke::new(2.0_f32, color));
            painter.circle_filled(Pos2::new(center.x, center.y + s * 0.15), 1.5, color);
        }

        IconKind::AlertTriangle => {
            // Triangle with exclamation
            let top = Pos2::new(center.x, rect.min.y + s * 0.12);
            let b_left = Pos2::new(rect.min.x + s * 0.12, rect.max.y - s * 0.14);
            let b_right = Pos2::new(rect.max.x - s * 0.12, rect.max.y - s * 0.14);

            painter.line_segment([top, b_left], stroke);
            painter.line_segment([b_left, b_right], stroke);
            painter.line_segment([b_right, top], stroke);

            // Exclamation point
            painter.line_segment([Pos2::new(center.x, center.y - s * 0.1), Pos2::new(center.x, center.y + s * 0.08)], stroke);
            painter.circle_filled(Pos2::new(center.x, center.y + s * 0.18), 1.4, color);
        }

        IconKind::Check => {
            // Clean checkmark
            let p1 = Pos2::new(center.x - s * 0.3, center.y);
            let p2 = Pos2::new(center.x - s * 0.05, center.y + s * 0.25);
            let p3 = Pos2::new(center.x + s * 0.35, center.y - s * 0.25);
            painter.line_segment([p1, p2], Stroke::new(2.0_f32, color));
            painter.line_segment([p2, p3], Stroke::new(2.0_f32, color));
        }

        IconKind::Close => {
            // Clean X mark
            let d = s * 0.26;
            painter.line_segment([Pos2::new(center.x - d, center.y - d), Pos2::new(center.x + d, center.y + d)], stroke);
            painter.line_segment([Pos2::new(center.x - d, center.y + d), Pos2::new(center.x + d, center.y - d)], stroke);
        }

        IconKind::ChevronRight => {
            // Right disclosure arrow >
            let p1 = Pos2::new(center.x - s * 0.15, center.y - s * 0.25);
            let p2 = Pos2::new(center.x + s * 0.15, center.y);
            let p3 = Pos2::new(center.x - s * 0.15, center.y + s * 0.25);
            painter.line_segment([p1, p2], stroke);
            painter.line_segment([p2, p3], stroke);
        }

        IconKind::ChevronDown => {
            // Down disclosure arrow v
            let p1 = Pos2::new(center.x - s * 0.25, center.y - s * 0.15);
            let p2 = Pos2::new(center.x, center.y + s * 0.15);
            let p3 = Pos2::new(center.x + s * 0.25, center.y - s * 0.15);
            painter.line_segment([p1, p2], stroke);
            painter.line_segment([p2, p3], stroke);
        }

        IconKind::Rocket => {
            // Sleek fast rocket shape
            let pts = [
                Pos2::new(center.x, center.y - s * 0.42),
                Pos2::new(center.x + s * 0.25, center.y + s * 0.15),
                Pos2::new(center.x + s * 0.18, center.y + s * 0.35),
                Pos2::new(center.x - s * 0.18, center.y + s * 0.35),
                Pos2::new(center.x - s * 0.25, center.y + s * 0.15),
            ];
            painter.add(egui::epaint::Shape::convex_polygon(pts.to_vec(), color, Stroke::NONE));
            // Thruster flame
            painter.line_segment(
                [Pos2::new(center.x, center.y + s * 0.35), Pos2::new(center.x, center.y + s * 0.46)],
                Stroke::new(2.0_f32, color),
            );
        }

        IconKind::Sparkles => {
            // Clean sparkle stars
            let draw_star = |p: Pos2, rad: f32| {
                painter.line_segment([Pos2::new(p.x - rad, p.y), Pos2::new(p.x + rad, p.y)], stroke);
                painter.line_segment([Pos2::new(p.x, p.y - rad), Pos2::new(p.x, p.y + rad)], stroke);
            };
            draw_star(Pos2::new(center.x - s * 0.1, center.y - s * 0.1), s * 0.32);
            draw_star(Pos2::new(center.x + s * 0.25, center.y + s * 0.2), s * 0.18);
        }
    }
}

/// Helper to render an icon as an egui response
pub fn render_icon(ui: &mut egui::Ui, kind: IconKind, size: f32, color: Color32) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(Vec2::splat(size), egui::Sense::hover());
    paint_icon(ui.painter(), rect, kind, color);
    resp
}

/// Helper to render an icon in a circular background badge
pub fn render_icon_circle(
    ui: &mut egui::Ui,
    kind: IconKind,
    circle_size: f32,
    icon_size: f32,
    bg_color: Color32,
    icon_color: Color32,
) {
    let (rect, _) = ui.allocate_exact_size(Vec2::splat(circle_size), egui::Sense::hover());
    ui.painter().circle_filled(rect.center(), circle_size * 0.5, bg_color);
    let icon_rect = Rect::from_center_size(rect.center(), Vec2::splat(icon_size));
    paint_icon(ui.painter(), icon_rect, kind, icon_color);
}
