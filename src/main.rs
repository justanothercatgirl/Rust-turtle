use std::f32::consts;
use std::mem;
use std::io::{self, Write};

#[allow(unused)]
fn transform(n: i32) -> i32 {
    let (x, y) = (n/1000+n%1000/100, n%100/10+n%10);
    if x < y {x*100 + y}
    else {y*100 + x}
}

const SIZE_X: usize = 25;
const SIZE_Y: usize = 55;


struct Turtle {
    dir_cmplx: (f32, f32),
    position: (f32, f32),
    tail_up: bool,
    field: [[u8; SIZE_Y]; SIZE_X],
}

#[allow(dead_code)]
impl Turtle {
    fn forward(&mut self, distance: f32) {
        let (x,  y) = (self.position.0, self.position.1); 
        let (dx, dy) = (self.dir_cmplx.0 * distance, self.dir_cmplx.1 * distance);
        
        if !self.tail_up {
            self.draw_field_line(x, y, x+dx, y+dy);
        }

        self.position.0 += dx;
        self.position.1 += dy;
    }

    fn backward(&mut self, distance: f32) {
        self.forward(-1.0 * distance)
    }

    fn left(&mut self, angle_degrees: f32) {
        // (a + bi)(c + di) = ac + (ab+cd)i - bd = ac-bd + (ab+cd)i
        let angle_radians = angle_degrees*consts::PI/180.0;
        let (a, b, c, d) = (self.dir_cmplx.0, 
                            self.dir_cmplx.1,
                            angle_radians.cos(),
                            angle_radians.sin());
        self.dir_cmplx = (a*c-b*d, a*d+b*c);
    }

    fn right(&mut self, angle_degrees: f32) {
        self.left(-1.0 * angle_degrees)
    }

    fn plot_p(&mut self, x: i32, y: i32) {
        let x = x + SIZE_X as i32 / 2;
        let y = y + SIZE_Y as i32 / 2;
        if let Some(row) = self.field.get_mut(x as usize) {
            if let Some(cell) = row.get_mut(y as usize) {
                *cell = b'@';
            }
        }
    }

    fn draw_field_line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        // re-define as closest integers
        let mut x1 = x1.round() as i32;
        let mut y1 = y1.round() as i32;
        let mut x2 = x2.round() as i32;
        let mut y2 = y2.round() as i32;

        let gt45: bool = (y2-y1).abs() > (x2-x1).abs(); // slope greater than 45;
        if gt45 {
            mem::swap(&mut x1, &mut y1);
            mem::swap(&mut x2, &mut y2);
        }

        //swap if needed
        if x1 > x2 {
            mem::swap(&mut x1, &mut x2);
            mem::swap(&mut y1, &mut y2);
        }

        let d_err = (y2-y1).abs();
        let dx = x2-x1;

        let iy = if y1 < y2 {1} else {-1};

        let mut err = dx >> 1; // dx / 2;
        let mut y = y1;
       
        for x in x1..=x2 {
            if gt45 {
                self.plot_p(x, y);
            } else {
                self.plot_p(y, x);
            }
            err -= d_err;
            if err < 0 {
                y += iy;
                err += dx;
            }
        }

    }
    fn print_info(&self) {
        println!("self.direction: ({};{})\n\
                self.position: ({};{})\n\
                self.tail_up: {}",
                self.dir_cmplx.0, self.dir_cmplx.1,
                self.position.0, self.position.1,
                self.tail_up);
        io::stdout().flush().ok();
    }
    fn new() -> Self {
        Turtle{dir_cmplx:(0.0, 1.0), 
            position:(0.0, 0.0), 
            tail_up:false, 
            field:[[b'.'; SIZE_Y]; SIZE_X]}
    }
    fn print_field(&self) {
        println!();
        for i in self.field.iter().rev() {
            for j in i {
                print!("{}", *j as char);
            }
            println!();
        }
        io::stdout().flush().ok();
    }
}

fn main()
{
    let mut drawer = Turtle::new();
    for _ in 0..2 {
        drawer.forward(3.0);
        drawer.left(90.0);
        drawer.backward(10.0);
        drawer.left(90.0);
    }
    drawer.tail_up = true;
    drawer.backward(10.0);
    drawer.right(90.0);
    drawer.forward(8.0);
    drawer.left(90.0);
    drawer.tail_up = false;
    for _ in 0..2 {
        drawer.forward(16.0);
        drawer.right(90.0);
        drawer.forward(8.0);
        drawer.right(90.0);
    }

    drawer.print_field();
    println!("");
}
