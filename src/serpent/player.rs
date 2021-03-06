use bracket_lib::prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Segment {
    pub x: i32,
    pub y: i32,
    pub direction_now: Direction,
    pub direction_next: Direction,
    pub glyph: u16,
}

impl Segment {
    pub fn new(
        x: i32,
        y: i32,
        direction_now: Direction,
        direction_next: Direction,
        glyph: u16,
    ) -> Self {
        Segment {
            x,
            y,
            direction_now,
            direction_next,
            glyph,
        }
    }

    pub fn update_direction(&mut self, previous: &Segment) {
        match self.direction_now {
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Stopped => {}
        }
        self.direction_now = self.direction_next;
        self.direction_next = previous.direction_now;
    }
}

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub direction: Direction,

    frame: usize,
    pub length: i32,
    pub segments: Vec<Segment>,
    pub(crate) symbol: Option<u16>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
    Stopped,
}

impl Player {
    pub fn new(x: i32, y: i32, symbol: Option<u16>) -> Self {
        Self {
            x,
            y,
            symbol,
            frame: 0,
            length: 1,
            direction: Direction::Stopped,
            segments: vec![Segment {
                x, //as f32,
                y, //as f32,
                direction_now: Direction::Stopped,
                direction_next: Direction::Stopped,
                glyph: 16,
            }],
        }
    }

    pub fn render(&mut self, ctx: &mut BTerm) {
        let mut glyph_idx = match self.direction {
            Direction::Left => 17,
            Direction::Right => 16,
            Direction::Up => 4,
            Direction::Down => 4,
            _ => 16,
        };

        if let Some(symbol) = self.symbol {
            glyph_idx = symbol;
        }

        ctx.set_active_console(1);
        ctx.cls();

        let head = *self.segments.clone().get(0).unwrap();
        ctx.set_fancy(
            PointF::new(head.x as f32, head.y as f32),
            1,
            Degrees::new(0.0),
            PointF::new(1.0, 1.0),
            WHITE,
            DARK_GRAY,
            glyph_idx,
        );
        for segment in self.segments.clone().iter().skip(1) {
            ctx.set_fancy(
                PointF::new(segment.x as f32, segment.y as f32),
                1,
                Degrees::new(0.0),
                PointF::new(1.0, 1.0),
                WHITE,
                DARK_GRAY,
                7, //glyph_idx, //0 as u16, //self.symbol //DRAGON_FRAMES[self.frame]
            );
        }
        ctx.set_active_console(0);
    }
    pub fn gravity_and_move(&mut self) {
        match self.direction {
            Direction::Stopped => {
                self.segments.get_mut(0).unwrap().direction_next = Direction::Stopped
            }
            Direction::Left => {
                self.segments.get_mut(0).unwrap().x -= 1;
                self.segments.get_mut(0).unwrap().direction_now = Direction::Left;
            }
            Direction::Right => {
                self.segments.get_mut(0).unwrap().x += 1;
                self.segments.get_mut(0).unwrap().direction_now = Direction::Right;
            }
            Direction::Up => {
                self.segments.get_mut(0).unwrap().y -= 1;
                self.segments.get_mut(0).unwrap().direction_now = Direction::Up;
            }
            Direction::Down => {
                self.segments.get_mut(0).unwrap().y += 1;
                self.segments.get_mut(0).unwrap().direction_now = Direction::Down;
            }
        }

        self.frame += 1;
        self.frame %= 2;

        let seg = self.segments.clone();
        for (i, s) in self.segments.iter_mut().enumerate().skip(1) {
            s.update_direction(seg.get(i - 1).unwrap())
        }
    }
    pub fn append(&mut self) {
        self.length += 1;

        let last_segment_x = self.segments.last().unwrap().x;
        let last_segment_y = self.segments.last().unwrap().y;
        let next_seg_x = last_segment_x;
        let next_seg_y = last_segment_y;

        self.segments.push(Segment {
            x: next_seg_x,
            y: next_seg_y,
            direction_next: Direction::Stopped,
            direction_now: Direction::Stopped,
            glyph: 3,
        })
    }
}

pub fn hello_player() {
    println!("hello player");
}
