use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().view(view).build().unwrap();
    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    // Get canvas to draw on
    let draw = app.draw();
    // Set background Color
    draw.background().color(WHEAT);

    let frame_size = frame.texture().size();
    draw_board(&draw, frame_size[0], frame_size[1]);

    // Put everything on to the frame
    draw.to_frame(app, &frame).expect("Failed to draw to frame");
}

fn draw_board(draw: &Draw, size_x: u32, size_y: u32) {
    let smaller_side = std::cmp::min(size_x, size_y);
    // let chess_array = [[0; 8]; 8];
    for mut row in 0..7 {
        for mut col in 0..7 {
            // Reverse one half of the board index 
            // To accomodate nannou coordinate system
            // Since one half is on negative side
            if row < 4 {
                row -= row;
            }
            if col < 4 {
                col -= col;
            }
            // Get the square coordinates
            let x = (row * 50) as f32;
            let y = (col * 50) as f32;
            let side = 50.0;
            draw.rect().color(STEELBLUE).x_y(x, y).w_h(side, side);
        }
    }
}
