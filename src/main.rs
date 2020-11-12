use std::fs::File;

use graphics::{prelude::*, turtle::Turtle};

fn main() {
    let width = 500;
    let height = 500;
    let bg_color = Rgb::WHITE;

    let mut img = PpmBuilder::new(height, width, 255)
        .bg_color(bg_color)
        .build();
    let xmax = width as f64;
    let ymax = height as f64;

    // declaring Rgb with (my) function
    let mut color = Rgb::new(0, 255, 0);
    // octants 1 and 5
    img.draw_line((0., 0., 0.), (xmax - 1., ymax - 1., 0.), color);
    img.draw_line((0., 0., 0.), (xmax - 1., ymax / 2., 0.), color);
    img.draw_line((xmax - 1., ymax - 1., 0.), (0., ymax / 2., 0.), color);

    color.blue = 255; // mutating value

    // octants 8 and 4
    img.draw_line((0., ymax - 1., 0.), (xmax - 1., 0., 0.), color);
    img.draw_line((0., ymax - 1., 0.), (xmax - 1., ymax / 2., 0.), color);
    img.draw_line((xmax - 1., 0., 0.), (0., ymax / 2., 0.), color);

    // declaring an Rgb with the struct notation
    let color = Rgb {
        red: 255,
        green: 0,
        blue: 0,
    };

    // octants 2 and 6
    img.draw_line((0., 0., 0.), (xmax / 2., ymax - 1., 0.), color);
    img.draw_line((xmax - 1., ymax - 1., 0.), (xmax / 2., 0., 0.), color);

    let color = Rgb {
        red: 255,
        green: 0,
        blue: 255,
    };

    // octants 7 and 3
    img.draw_line((0., ymax - 1., 0.), (xmax / 2., 0., 0.), color);
    img.draw_line((xmax - 1., 0., 0.), (xmax / 2., ymax - 1., 0.), color);

    let color = Rgb {
        red: 255,
        blue: 0,
        green: 255,
    };

    // horizontal and vertical
    img.draw_line((0., ymax / 2., 0.), (xmax - 1., ymax / 2., 0.), color);
    img.draw_line((xmax / 2., 0., 0.), (xmax / 2., ymax - 1., 0.), color);

    // just for fun: draw a Circle with our Turtle
    let mut turtle = Turtle::new(img, xmax / 2., ymax / 2., Rgb::BLACK);

    turtle.direction = 0.;
    let radius = xmax / 4.;
    turtle.forward(radius);
    turtle.turn_left(90.);

    let circumf = radius * std::f64::consts::TAU;
    let total_steps = 360;

    turtle.pen_down = true;

    for _ in 0..total_steps {
        turtle.forward(circumf / total_steps as f64);
        turtle.turn_left(360. / total_steps as f64);
    }

    let img = turtle.get_screen();

    // Result (Error) handling is required in Rust
    img.display().expect("error displaying ppm");

    img.write_binary_to_buf(&mut File::create("binary.ppm").expect("error creating binary.ppm"))
        .expect("error writing to file");

    img.write_ascii_to_buf(&mut File::create("ascii.ppm").expect("error creating ascii.ppm"))
        .expect("error writing to file");
    img.save("img.png").expect("error saving file as png");

    // You can also add return type io::Result<()> to main(), and use ? where you see `expect`
}
