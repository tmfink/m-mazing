/*
use crate::prelude::*;

/// Zoom camera to fit rectangle while maintaining aspect ratio
#[must_use]
pub fn camera_zoom_to_fit(fit: Rect) -> mq::Camera2D {
    let desired_aspect_ratio = fit.w / fit.h;
    let screen_width = screen_width();
    let screen_height = screen_height();
    let current_aspect_ratio = screen_width / screen_height;
    debug!("Fitting camera to {:?}", fit);

    let w;
    let h;
    let x;
    let y;

    if current_aspect_ratio >= desired_aspect_ratio {
        trace!(
            "    Window too wide; want {} but am currently {}",
            desired_aspect_ratio,
            current_aspect_ratio
        );
        w = fit.w * current_aspect_ratio;
        h = fit.h;
        let excess_width = w - fit.w;
        x = fit.x - excess_width * 0.5;
        y = fit.y;
    } else {
        trace!(
            "    Window too tall; want {} but am currently {}",
            desired_aspect_ratio,
            current_aspect_ratio
        );
        w = fit.w;
        h = fit.w / current_aspect_ratio;
        let excess_height = h - fit.h;
        x = fit.x;
        y = fit.y - excess_height * 0.5;
    }

    trace!("    x={}, y={}, w={}, h={}", x, y, w, h);
    Camera2D::from_display_rect(mq::Rect { x, y, w, h })
}

*/
