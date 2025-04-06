mod geometrical_shapes;

use geometrical_shapes as gs;
use gs::{Displayable, Drawable};
use raster::{Color, Image};

fn main() {
    let mut image = Image::blank(1000, 1000);

    // Drawing a random line
    gs::Line::random(image.width, image.height).draw(&mut image);

// Drawing a random point
    gs::Point::random(image.width, image.height).draw(&mut image);

// Drawing a rectangle
    let rectangle = gs::Rectangle::new(&gs::Point::new(150, 150), &gs::Point::new(50, 50));
    rectangle.draw(&mut image);

// Drawing a triangle
    let triangle = gs::Triangle::new(
        &gs::Point::new(500, 500),
        &gs::Point::new(250, 700),
        &gs::Point::new(700, 800),
    );
    triangle.draw(&mut image);

    // Drawing random circles
    for _ in 1..50 {
        gs::Circle::random(image.width, image.height).draw(&mut image);
    }

    // Drawing random pentagons
    for _ in 1..10 {
        gs::Pentagon::random(image.width, image.height).draw(&mut image);
    }

    // Drawing random cubes
    for _ in 1..5 {
        gs::Cube::random(image.width, image.height).draw(&mut image);
    }

    // Save the image to a file
    raster::save(&image, "image.png").unwrap();
}

// impl Displayable for Image {
//     fn display(&mut self, x: i32, y: i32, color: Color) {
//         if x >= 0 && x < self.width && y >= 0 && y < self.height {
//             self.set_pixel(x, y, color).unwrap();
//         }
//     }
// }
