use raster::{Color, Image};
use rand::Rng;

// Step 1: Implementing the Traits

// The Drawable trait for all shapes
pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> Color;
}

// // The Displayable trait for displaying pixels
pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

// // Step 2: Implementing the Point Structure
#[derive(Clone)] // Deriving Clone for Point
pub struct Point {
    pub x: i32,
    pub y: i32,
    color: Color,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        let mut rng = rand::thread_rng();
        Point {
            x,
            y,
            color: Color::rgba(
                rng.gen_range(0..=255),
                rng.gen_range(0..=255),
                rng.gen_range(0..=255),
                255,
            ),
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Self::new(rng.gen_range(0..width), rng.gen_range(0..height))
    }
}
// impl Drawable for Point {
//     fn draw(&self, image: &mut Image) {
//         image.display(self.x, self.y, self.color());
//     }

//     fn color(&self) -> Color {
//         self.color.clone()
//     }
// }

// // Step 3: Implementing the Line Structure

// pub struct Line {
//     start: Point,
//     end: Point,
//     color: Color,
// }

// impl Line {
//     pub fn new(start: &Point, end: &Point) -> Self {
//         let mut rng = rand::thread_rng();
//         Line {
//             start: Point::new(start.x, start.y),
//             end: Point::new(end.x, end.y),
//             color: Color::rgba(
//                 rng.gen_range(0..=255),
//                 rng.gen_range(0..=255),
//                 rng.gen_range(0..=255),
//                 255,
//             ),
//         }
//     }

//     pub fn random(width: i32, height: i32) -> Self {
//         let mut rng = rand::thread_rng();
//         Self::new(
//             &Point::new(rng.gen_range(0..width), rng.gen_range(0..height)),
//             &Point::new(rng.gen_range(0..width), rng.gen_range(0..height)),
//         )
//     }
// }

// impl Drawable for Line {
//     fn draw(&self, image: &mut Image) {
//         // Bresenham's line algorithm
//         let mut x0 = self.start.x;
//         let mut y0 = self.start.y;
//         let x1 = self.end.x;
//         let y1 = self.end.y;

//         let dx = (x1 - x0).abs();
//         let dy = -(y1 - y0).abs();
//         let sx = if x0 < x1 { 1 } else { -1 };
//         let sy = if y0 < y1 { 1 } else { -1 };
//         let mut err = dx + dy;

//         loop {
//             image.display(x0, y0, self.color());
//             if x0 == x1 && y0 == y1 {
//                 break;
//             }
//             let e2 = 2 * err;
//             if e2 >= dy {
//                 if x0 == x1 {
//                     break;
//                 }
//                 err += dy;
//                 x0 += sx;
//             }
//             if e2 <= dx {
//                 if y0 == y1 {
//                     break;
//                 }
//                 err += dx;
//                 y0 += sy;
//             }
//         }
//     }

//     fn color(&self) -> Color {
//         self.color.clone()
//     }
// }

// // Step 4: Implementing the Triangle Structure

// pub struct Triangle {
//     p1: Point,
//     p2: Point,
//     p3: Point,
//     color: Color,
// }

// impl Triangle {
//     pub fn new(p1: &Point, p2: &Point, p3: &Point) -> Self {
//         let mut rng = rand::thread_rng();
//         Triangle {
//             p1: Point::new(p1.x, p1.y),
//             p2: Point::new(p2.x, p2.y),
//             p3: Point::new(p3.x, p3.y),
//             color: Color::rgba(
//                 rng.gen_range(0..=255),
//                 rng.gen_range(0..=255),
//                 rng.gen_range(0..=255),
//                 255,
//             ),
//         }
//     }
// }

// impl Drawable for Triangle {
//     fn draw(&self, image: &mut Image) {
//         // Draw the triangle as three lines
//         Line::new(&self.p1, &self.p2).draw(image);
//         Line::new(&self.p2, &self.p3).draw(image);
//         Line::new(&self.p3, &self.p1).draw(image);
//     }

//     fn color(&self) -> Color {
//         self.color.clone()
//     }
// }

// // Step 5: Implementing the Rectangle Structure

// pub struct Rectangle {
//     top_left: Point,
//     bottom_right: Point,
//     color: Color,
// }

// impl Rectangle {
//     pub fn new(p1: &Point, p2: &Point) -> Self {
//         let mut rng = rand::thread_rng();
        
//         // Ensure we have proper top-left and bottom-right points
//         let x1 = p1.x.min(p2.x);
//         let y1 = p1.y.min(p2.y);
//         let x2 = p1.x.max(p2.x);
//         let y2 = p1.y.max(p2.y);
        
//         Rectangle {
//             top_left: Point::new(x1, y1),
//             bottom_right: Point::new(x2, y2),
//             color: Color::rgba(
//                 rng.gen_range(0..=255),
//                 rng.gen_range(0..=255),
//                 rng.gen_range(0..=255),
//                 255,
//             ),
//         }
//     }
// }

// impl Drawable for Rectangle {
//     fn draw(&self, image: &mut Image) {
//         // Draw the rectangle as four lines
//         let top_right = Point::new(self.bottom_right.x, self.top_left.y);
//         let bottom_left = Point::new(self.top_left.x, self.bottom_right.y);
        
//         Line::new(&self.top_left, &top_right).draw(image);
//         Line::new(&top_right, &self.bottom_right).draw(image);
//         Line::new(&self.bottom_right, &bottom_left).draw(image);
//         Line::new(&bottom_left, &self.top_left).draw(image);
//     }

//     fn color(&self) -> Color {
//         self.color.clone()
//     }
// }

// // Step 6: Implementing the Circle Structure

// pub struct Circle {
//     center: Point,
//     radius: i32,
//     color: Color,
// }

// impl Circle {
//     pub fn new(center: &Point, radius: i32) -> Self {
//         let mut rng = rand::thread_rng();
//         Circle {
//             center: Point::new(center.x, center.y),
//             radius,
//             color: Color::rgba(
//                 rng.gen_range(0..=255),
//                 rng.gen_range(0..=255),
//                 rng.gen_range(0..=255),
//                 255,
//             ),
//         }
//     }

//     pub fn random(width: i32, height: i32) -> Self {
//         let mut rng = rand::thread_rng();
//         let center = Point::random(width, height);
//         let max_radius = width.min(height) / 10;
//         let radius = rng.gen_range(5..=max_radius);
//         Self::new(&center, radius)
//     }
// }

// impl Drawable for Circle {
//     fn draw(&self, image: &mut Image) {
//         // Midpoint circle algorithm
//         let mut x = self.radius;
//         let mut y = 0;
//         let mut err = 0;

//         while x >= y {
//             image.display(self.center.x + x, self.center.y + y, self.color());
//             image.display(self.center.x + y, self.center.y + x, self.color());
//             image.display(self.center.x - y, self.center.y + x, self.color());
//             image.display(self.center.x - x, self.center.y + y, self.color());
//             image.display(self.center.x - x, self.center.y - y, self.color());
//             image.display(self.center.x - y, self.center.y - x, self.color());
//             image.display(self.center.x + y, self.center.y - x, self.color());
//             image.display(self.center.x + x, self.center.y - y, self.color());

//             y += 1;
//             if err <= 0 {
//                 err += 2 * y + 1;
//             }
//             if err > 0 {
//                 x -= 1;
//                 err -= 2 * x + 1;
//             }
//         }
//     }

//     fn color(&self) -> Color {
//         self.color.clone()
//     }
// }

// // Step 7: Implementing the Pentagon Structure

// pub struct Pentagon {
//     center: Point,
//     radius: i32,
//     color: Color,
// }

// #[allow(dead_code)] // Silences unused function warnings

// impl Pentagon {
//     pub fn new(center: &Point, radius: i32) -> Self {
//         let mut rng = rand::thread_rng();
//         Pentagon {
//             center: Point::new(center.x, center.y),
//             radius,
//             color: Color::rgba(
//                 rng.gen_range(0..=255),
//                 rng.gen_range(0..=255),
//                 rng.gen_range(0..=255),
//                 255,
//             ),
//         }
//     }

//     pub fn random(width: i32, height: i32) -> Self {
//         let mut rng = rand::thread_rng();
//         let center = Point::random(width, height);
//         let max_radius = width.min(height) / 10;
//         let radius = rng.gen_range(10..=max_radius);
//         Self::new(&center, radius)
//     }

//     fn vertices(&self) -> Vec<Point> {
//         let mut vertices = Vec::new();
//         let angle_step = 2.0 * std::f32::consts::PI / 5.0; // 5 sides
//         for i in 0..5 {
//             let angle = i as f32 * angle_step;
//             let x = self.center.x + (self.radius as f32 * angle.cos()) as i32;
//             let y = self.center.y + (self.radius as f32 * angle.sin()) as i32;
//             vertices.push(Point::new(x, y));
//         }
//         vertices
//     }
// }

// impl Drawable for Pentagon {
//     fn draw(&self, image: &mut Image) {
//         let vertices = self.vertices();
//         for i in 0..5 {
//             let next = (i + 1) % 5;
//             Line::new(&vertices[i], &vertices[next]).draw(image);
//         }
//     }

//     fn color(&self) -> Color {
//         self.color.clone()
//     }
// }

// // Step 8: Implementing the Cube Structure

// pub struct Cube {
//     front_top_left: Point,
//     size: i32,
//     color: Color,
// }
// #[allow(dead_code)]
// impl Cube {
//     pub fn new(front_top_left: &Point, size: i32) -> Self {
//         let mut rng = rand::thread_rng();
//         Cube {
//             front_top_left: front_top_left.clone(), // Clone Point
//             size,
//             color: Color::rgba(
//                 rng.gen_range(0..=255),
//                 rng.gen_range(0..=255),
//                 rng.gen_range(0..=255),
//                 255,
//             ),
//         }
//     }

//     pub fn random(width: i32, height: i32) -> Self {
//         let mut rng = rand::thread_rng();
//         let size = rng.gen_range(20..=50);
//         let x = rng.gen_range(0..(width - size));
//         let y = rng.gen_range(0..(height - size));
//         Self::new(&Point::new(x, y), size)
//     }
// }

// impl Drawable for Cube {
//     fn draw(&self, image: &mut Image) {
//         let offset = self.size / 3;

//         // Front face corners
//         let p1 = &self.front_top_left; // Borrow instead of move
//         let p2 = Point::new(p1.x + self.size, p1.y);
//         let p3 = Point::new(p2.x, p2.y + self.size);
//         let p4 = Point::new(p1.x, p1.y + self.size);

//         // Back face corners (shifted by offset)
//         let p5 = Point::new(p1.x + offset, p1.y - offset);
//         let p6 = Point::new(p2.x + offset, p2.y - offset);
//         let p7 = Point::new(p3.x + offset, p3.y - offset);
//         let p8 = Point::new(p4.x + offset, p4.y - offset);

//         // Draw front face
//         Line::new(&p1, &p2).draw(image);
//         Line::new(&p2, &p3).draw(image);
//         Line::new(&p3, &p4).draw(image);
//         Line::new(&p4, &p1).draw(image);

//         // Draw back face
//         Line::new(&p5, &p6).draw(image);
//         Line::new(&p6, &p7).draw(image);
//         Line::new(&p7, &p8).draw(image);
//         Line::new(&p8, &p5).draw(image);

//         // Connect corresponding vertices
//         Line::new(&p1, &p5).draw(image);
//         Line::new(&p2, &p6).draw(image);
//         Line::new(&p3, &p7).draw(image);
//         Line::new(&p4, &p8).draw(image);
//     }

//     fn color(&self) -> Color {
//         self.color.clone() // Return color clone
//     }
// }
