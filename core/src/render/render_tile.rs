use bevy::math::{const_vec2, const_vec3};

use crate::{prelude::*, render::polar_to_cartesian};

const GRID_WIDTH: f32 = Tile::CELL_GRID_WIDTH as f32;
const GRID_HALF_WIDTH: f32 = 0.5 * GRID_WIDTH;
const CELL_WIDTH: f32 = 1.0;
const CELL_HALF_WIDTH: f32 = 0.5 * CELL_WIDTH;

const WALL_Z: f32 = 0.1;
const WALL_TRANSFORM: Transform = Transform {
    translation: const_vec3!([0., 0., WALL_Z]),
    ..Transform::identity()
};

const CELL_ITEM_Z: f32 = 0.2;

fn render_timer(
    render: &RenderState,
    location: Vec2,
    commands: &mut Commands,
    tile_entity: Entity,
) {
    const X_LEFT: f32 = 0.25;
    const X_RIGHT: f32 = 0.75;
    const Y_TOP: f32 = 0.2;
    const Y_BOTTOM: f32 = 0.8;

    const TILE_POINTS: [Vec2; 5] = [
        const_vec2!([X_LEFT, Y_TOP]),
        const_vec2!([X_RIGHT, Y_TOP]),
        const_vec2!([X_LEFT, Y_BOTTOM]),
        const_vec2!([X_RIGHT, Y_BOTTOM]),
        const_vec2!([X_LEFT, Y_TOP]),
    ];

    let builder = shape::draw_connected_line(TILE_POINTS.iter().copied(), GeometryBuilder::new());
    let transform = Transform::from_translation(location.extend(CELL_ITEM_Z));
    let geo = commands
        .spawn_bundle(builder.build(
            DrawMode::Stroke(StrokeMode::new(
                render.theme.timer_color,
                render.theme.wall_thickness,
            )),
            transform,
        ))
        .id();
    commands.entity(tile_entity).push_children(&[geo]);
}

/*
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
*/

fn render_warp(
    render: &RenderState,
    location: Vec2,
    pawn: Pawn,
    commands: &mut Commands,
    tile_entity: Entity,
) {
    let center = Vec2::new(CELL_HALF_WIDTH, CELL_HALF_WIDTH);

    const NUM_ANGLES: u32 = 8;
    const NUM_RADII: u32 = 24;

    let angles = (0..NUM_ANGLES)
        .map(|x| x as f32 * 2.0 * std::f32::consts::PI / NUM_ANGLES as f32)
        .cycle();
    let radii = (0..NUM_RADII).map(|x| x as f32 * CELL_HALF_WIDTH * 0.8 / NUM_RADII as f32);
    let points = angles
        .zip(radii)
        .map(|(angle, radius)| polar_to_cartesian(radius, angle) + center);

    let builder = shape::draw_connected_line(points, GeometryBuilder::new());
    let transform = Transform::from_translation(location.extend(CELL_ITEM_Z));
    let geo = commands
        .spawn_bundle(builder.build(
            DrawMode::Stroke(StrokeMode::new(
                pawn.as_color(render),
                render.theme.warp_thickness,
            )),
            transform,
        ))
        .id();
    commands.entity(tile_entity).push_children(&[geo]);
}

fn render_loot(
    render: &RenderState,
    location: Vec2,
    pawn: Pawn,
    commands: &mut Commands,
    tile_entity: Entity,
) {
    let shape = shapes::Rectangle {
        extents: Vec2::new(0.5, 0.5),
        origin: RectangleOrigin::Center,
    };
    let rot = Quat::from_rotation_z(std::f32::consts::FRAC_PI_4);
    let translation = (location + Vec2::new(0.5, 0.5)).extend(CELL_ITEM_Z);
    let transform = Transform::from_rotation(rot).with_translation(translation);
    let entity = commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            DrawMode::Stroke(StrokeMode::new(
                pawn.as_color(render),
                render.theme.wall_thickness,
            )),
            transform,
        ))
        .id();
    commands.entity(tile_entity).push_children(&[entity]);
}

/*
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
*/

fn render_wall(
    render: &RenderState,
    a: Vec2,
    b: Vec2,
    wall: WallState,
    tile_bg_color: Color,
    commands: &mut Commands,
    tile_entity: Entity,
) {
    let mut builder = GeometryBuilder::new();
    let color;
    if wall == WallState::OrangeOnly {
        let hole_halfwidth = 0.5 * render.theme.wall_orange_only_hole_width;
        let hole_a = a.lerp(b, 0.5 - hole_halfwidth);
        let hole_b = a.lerp(b, 0.5 + hole_halfwidth);
        let line1 = shapes::Line(a, hole_a);
        let line2 = shapes::Line(hole_b, b);
        builder = builder.add(&line1).add(&line2);
        color = render.theme.wall_orange_only_color;
    } else {
        let line = shapes::Line(a, b);
        builder = builder.add(&line);
        color = wall.wall_color(render, tile_bg_color);
    }
    let geo = commands
        .spawn_bundle(builder.build(
            DrawMode::Stroke(StrokeMode::new(color, render.theme.wall_thickness)),
            WALL_TRANSFORM,
        ))
        .id();
    commands.entity(tile_entity).push_children(&[geo]);
}

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
                DrawMode::Fill(FillMode::color(tile_bg_color)),
                Transform::default(),
            ))
            .insert(TileShape)
            .id();

        let render_walls = |pred: fn(WallState) -> bool, commands: &mut Commands| {
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
                            commands,
                            tile_entity,
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
                            commands,
                            tile_entity,
                        );
                    }
                }
            }
        };

        // Render open walls before other walls
        render_walls(|wall| wall == WallState::Open, commands);
        render_walls(|wall| wall != WallState::Open, commands);

        // todo: render cells
        for (row_idx, row) in self.cell_grid().iter().enumerate() {
            let row_idx_float = row_idx as f32;
            let y = -GRID_HALF_WIDTH + row_idx_float * CELL_WIDTH;
            for (col_idx, cell) in row.iter().copied().enumerate() {
                let col_idx_float = col_idx as f32;
                let x = -GRID_HALF_WIDTH + col_idx_float * CELL_WIDTH;

                let is_reachable = is_reachable_coord[row_idx][col_idx];
                if !is_reachable {
                    let covered_cell = shapes::Rectangle {
                        extents: Vec2::new(1., 1.),
                        // this looks like TopLeft, but we are flipping the Y axis in the camera right now
                        origin: RectangleOrigin::BottomLeft,
                    };
                    let covered_cell_id = commands
                        .spawn_bundle(GeometryBuilder::build_as(
                            &covered_cell,
                            DrawMode::Fill(FillMode::color(render.theme.unreachable_cell_color)),
                            Transform::from_xyz(x, y, CELL_ITEM_Z),
                        ))
                        .id();
                    commands
                        .entity(tile_entity)
                        .push_children(&[covered_cell_id]);
                }

                let cell_location = Vec2::new(x, y);
                match cell {
                    TileCell::TimerFlip(_) => {
                        render_timer(render, cell_location, commands, tile_entity)
                    }
                    TileCell::Warp(pawn) => {
                        render_warp(render, cell_location, pawn, commands, tile_entity)
                    }
                    TileCell::Loot(pawn) => {
                        render_loot(render, cell_location, pawn, commands, tile_entity)
                    }
                    _ => (),
                    /*
                    TileCell::FinalExit(pawn) => {
                        render_final_exit(render, gl, x, y, pawn, self, col_idx, row_idx)
                    }
                    TileCell::Camera(_) => render_camera(render, gl, x, y),
                    TileCell::CrystalBall(_) => render_crystal_ball(render, gl, x, y),
                    TileCell::Empty => (),
                    */
                }

                /*
                if cell.is_used() {
                    render_used_marker(render, x, y);
                }
                */
            }
        }

        /*
        for escalator in self.escalators() {
            render_escalator(render, *escalator);
        }

        gl.pop_model_matrix();
        */

        tile_entity
    }
}
