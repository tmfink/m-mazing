use crate::prelude::*;

const GRID_WIDTH: f32 = Tile::CELL_GRID_WIDTH as f32;
const GRID_HALF_WIDTH: f32 = 0.5 * GRID_WIDTH;
const CELL_WIDTH: f32 = 1.0;
const CELL_HALF_WIDTH: f32 = 0.5 * CELL_WIDTH;

/*
fn render_timer(render: &RenderState, x: f32, y: f32) {
    let x_left = x + 0.25 * CELL_WIDTH;
    let x_right = x + 0.75 * CELL_WIDTH;
    let y_top = y + 0.2 * CELL_WIDTH;
    let y_bottom = y + 0.8 * CELL_WIDTH;

    let points = [
        Vec2::new(x_left, y_top),
        Vec2::new(x_right, y_top),
        Vec2::new(x_left, y_bottom),
        Vec2::new(x_right, y_bottom),
        Vec2::new(x_left, y_top),
    ];
    shape::draw_connected_line(
        points.iter().copied(),
        render.theme.wall_thickness,
        render.theme.timer_color,
    );
}

fn render_used_marker(render: &RenderState, x: f32, y: f32) {
    let x_left = x + 0.1;
    let x_right = x + 0.9;
    let y_top = y + 0.1;
    let y_bottom = y + 0.9;

    let thickness = render.theme.used_marker_thickness;
    let color = render.theme.used_marker_color;
    draw_line(x_left, y_top, x_right, y_bottom, thickness, color);
    draw_line(x_left, y_bottom, x_right, y_top, thickness, color);
}

fn render_warp(render: &RenderState, x: f32, y: f32, pawn: Pawn) {
    let center = Vec2::new(x + CELL_HALF_WIDTH, y + CELL_HALF_WIDTH);

    const NUM_ANGLES: u32 = 8;
    const NUM_RADII: u32 = 24;

    let angles = (0..NUM_ANGLES)
        .map(|x| x as f32 * 2.0 * std::f32::consts::PI / NUM_ANGLES as f32)
        .cycle();
    let radii = (0..NUM_RADII).map(|x| x as f32 * CELL_HALF_WIDTH * 0.8 / NUM_RADII as f32);
    let points = angles
        .zip(radii)
        .map(|(angle, radius)| polar_to_cartesian(radius, angle) + center);
    shape::draw_connected_line(points, render.theme.warp_thickness, pawn.as_color(render));
}

fn render_loot(render: &RenderState, x: f32, y: f32, pawn: Pawn) {
    let x_left = x + 0.1 * CELL_WIDTH;
    let x_mid = x + 0.5 * CELL_WIDTH;
    let x_right = x + 0.9 * CELL_WIDTH;
    let y_top = y + 0.1 * CELL_WIDTH;
    let y_mid = y + 0.5 * CELL_WIDTH;
    let y_bottom = y + 0.9 * CELL_WIDTH;

    let points = [
        Vec2::new(x_right, y_mid),
        Vec2::new(x_mid, y_top),
        Vec2::new(x_left, y_mid),
        Vec2::new(x_mid, y_bottom),
        Vec2::new(x_right, y_mid),
    ];
    shape::draw_connected_line(
        points.iter().copied(),
        render.theme.wall_thickness,
        pawn.as_color(render),
    );
}

fn render_camera(render: &RenderState, gl: &mut QuadGl, x: f32, y: f32) {
    let scale = Vec3::new(CELL_WIDTH, CELL_WIDTH, 1.);
    let translation = Vec3::new(x + 0.5, y + 0.5, 0.);
    let rotation = Quat::IDENTITY;
    gl.push_model_matrix(Mat4::from_scale_rotation_translation(
        scale,
        rotation,
        translation,
    ));

    let points = [
        Vec2::new(-0.35, 0.0),
        Vec2::new(-0.175, 0.15),
        Vec2::new(0.0, 0.2),
        Vec2::new(0.175, 0.15),
        Vec2::new(0.35, 0.0),
    ];

    let color = render.theme.camera_color;
    shape::draw_connected_line(points.iter().copied(), render.theme.warp_thickness, color);
    shape::draw_connected_line(
        points.iter().copied().map(|mut v| {
            v.y *= -1.0;
            v
        }),
        render.theme.warp_thickness,
        color,
    );
    draw_circle(0.0, 0.0, 0.15, color);

    gl.pop_model_matrix();
}

fn render_crystal_ball(render: &RenderState, gl: &mut QuadGl, x: f32, y: f32) {
    let scale = Vec3::new(CELL_WIDTH, CELL_WIDTH, 1.);
    let translation = Vec3::new(x + 0.5, y + 0.5, 0.);
    let rotation = Quat::IDENTITY;
    gl.push_model_matrix(Mat4::from_scale_rotation_translation(
        scale,
        rotation,
        translation,
    ));

    let color = render.theme.crystal_ball_color;
    draw_hexagon(0.0, 0.0, 0.4, 0.05, false, color, mq::WHITE);
    draw_circle_lines(0.0, 0.0, 0.3, 0.05, color);

    gl.pop_model_matrix();
}

fn render_escalator(render: &RenderState, escalator: EscalatorLocation) {
    let [a, b] = escalator.0;
    let offset = CELL_HALF_WIDTH - GRID_HALF_WIDTH;
    draw_line(
        a.x() as f32 + offset,
        a.y() as f32 + offset,
        b.x() as f32 + offset,
        b.y() as f32 + offset,
        render.theme.escalator_thickness,
        render.theme.escalator_color,
    );
}

#[allow(clippy::too_many_arguments)]
fn render_final_exit(
    render: &RenderState,
    gl: &mut QuadGl,
    x: f32,
    y: f32,
    pawn: Pawn,
    tile: &Tile,
    col_idx: usize,
    row_idx: usize,
) {
    let scale = Vec3::new(CELL_WIDTH, CELL_WIDTH, 1.);

    let point = TileGridCoord::new(col_idx as u8, row_idx as u8)
        .expect("could not convert row/col idx to tile");

    // Z-axis goes "into" the screen since this is right-handed
    let angle = -tile.cell_exit_direction(point).as_angle();
    let rotation = Quat::from_rotation_z(angle);

    let translation = Vec3::new(x + 0.5, y + 0.5, 0.);
    gl.push_model_matrix(Mat4::from_scale_rotation_translation(
        scale,
        rotation,
        translation,
    ));

    let offset = 0.5 * render.theme.wall_thickness;
    let end = 1.0 - 2.0 * offset;
    draw_rectangle(
        -0.5 + offset,
        -0.5 + offset,
        end,
        end,
        pawn.as_color(render),
    );

    let color = render.theme.final_exit_arrow_color;
    let thickness = render.theme.warp_thickness;
    let endpoint = 0.5 - 2.0 * offset;
    let arrowhead_width = 0.3;
    let arrowhead_halfwidth = 0.5 * arrowhead_width;
    let arrowhead_length = 0.25;
    let arrowhead_back_x = endpoint - arrowhead_length;
    draw_line(0.0, 0.0, endpoint, 0.0, thickness, color);
    draw_line(
        endpoint,
        0.0,
        arrowhead_back_x,
        -arrowhead_halfwidth,
        thickness,
        color,
    );
    draw_line(
        endpoint,
        0.0,
        arrowhead_back_x,
        arrowhead_halfwidth,
        thickness,
        color,
    );

    gl.pop_model_matrix();
}

fn render_wall(
    render: &RenderState,
    a: Vec2,
    b: Vec2,
    wall: WallState,
    tile_bg_color: Color,
) {
    if wall == WallState::OrangeOnly {
        let hole_halfwidth = 0.5 * render.theme.wall_orange_only_hole_width;
        let hole_a = a.lerp(b, 0.5 - hole_halfwidth);
        let hole_b = a.lerp(b, 0.5 + hole_halfwidth);
        draw_line(
            a.x,
            a.y,
            hole_a.x,
            hole_a.y,
            render.theme.wall_thickness,
            render.theme.wall_orange_only_color,
        );
        draw_line(
            hole_b.x,
            hole_b.y,
            b.x,
            b.y,
            render.theme.wall_thickness,
            render.theme.wall_orange_only_color,
        );
    } else {
        draw_line(
            a.x,
            a.y,
            b.x,
            b.y,
            render.theme.wall_thickness,
            wall.wall_color(render, tile_bg_color),
        );
    }
}
*/

#[derive(Component)]
struct TileShape;

impl Tile {
    pub fn spawn(&self, pos: Vec2, render: &RenderState, commands: &mut Commands) -> Entity {
        let is_reachable_coord = self.reachable_coords();

        let tile_bg_color = if self.has_camera() {
            render.theme.tile_camera_bg_color
        } else {
            render.theme.tile_normal_bg_color
        };

        let shape = shapes::Rectangle {
            extents: Vec2::new(Self::CELL_GRID_WIDTH as f32, Self::CELL_GRID_WIDTH as f32),
            origin: RectangleOrigin::Center,
        };
        let tile_entity = commands
            .spawn_bundle(GeometryBuilder::build_as(
                &shape,
                DrawMode::Outlined {
                    fill_mode: FillMode::color(tile_bg_color),
                    outline_mode: StrokeMode::new(
                        render.theme.wall_open_color,
                        render.theme.wall_thickness,
                    ),
                },
                Transform::default(),
            ))
            .insert(TileShape)
            .id();

        /*
        let render_walls = |pred: fn(WallState) -> bool| {
            // horizontal walls
            for (row_idx, row) in self.horz_walls().iter().enumerate() {
                let row_idx = row_idx as f32;
                let y = -GRID_HALF_WIDTH + row_idx * CELL_WIDTH;
                for (col_idx, wall) in row.iter().copied().enumerate() {
                    let col_idx = col_idx as f32;
                    let x = -GRID_HALF_WIDTH + col_idx * CELL_WIDTH;
                    if pred(wall) {
                        render_wall(
                            render,
                            Vec2::new(x, y),
                            Vec2::new(x + CELL_WIDTH, y),
                            wall,
                            tile_bg_color,
                        );
                    }
                }
            }

            // vertical walls
            for (row_idx, row) in self.vert_walls().iter().enumerate() {
                let row_idx = row_idx as f32;
                let y = -GRID_HALF_WIDTH + row_idx * CELL_WIDTH;
                for (col_idx, wall) in row.iter().copied().enumerate() {
                    let col_idx = col_idx as f32;
                    let x = -GRID_HALF_WIDTH + col_idx * CELL_WIDTH;
                    if pred(wall) {
                        render_wall(
                            render,
                            Vec2::new(x, y),
                            Vec2::new(x, y + CELL_WIDTH),
                            wall,
                            tile_bg_color,
                        );
                    }
                }
            }
        };

        // Render open walls before other walls
        render_walls(|wall| wall == WallState::Open);
        render_walls(|wall| wall != WallState::Open);

        // todo: render cells
        for (row_idx, row) in self.cell_grid().iter().enumerate() {
            let row_idx_float = row_idx as f32;
            let y = -GRID_HALF_WIDTH + row_idx_float * CELL_WIDTH;
            for (col_idx, cell) in row.iter().copied().enumerate() {
                let col_idx_float = col_idx as f32;
                let x = -GRID_HALF_WIDTH + col_idx_float * CELL_WIDTH;

                let is_reachable = is_reachable_coord[row_idx][col_idx];
                if !is_reachable {
                    draw_rectangle(x, y, 1.0, 1.0, render.theme.unreachable_cell_color);
                }

                match cell {
                    TileCell::TimerFlip(_) => render_timer(render, x, y),
                    TileCell::Warp(pawn) => render_warp(render, x, y, pawn),
                    TileCell::Loot(pawn) => render_loot(render, x, y, pawn),
                    TileCell::FinalExit(pawn) => {
                        render_final_exit(render, gl, x, y, pawn, self, col_idx, row_idx)
                    }
                    TileCell::Camera(_) => render_camera(render, gl, x, y),
                    TileCell::CrystalBall(_) => render_crystal_ball(render, gl, x, y),
                    TileCell::Empty => (),
                }

                if cell.is_used() {
                    render_used_marker(render, x, y);
                }
            }
        }

        for escalator in self.escalators() {
            render_escalator(render, *escalator);
        }

        gl.pop_model_matrix();
        */

        tile_entity
    }
}
