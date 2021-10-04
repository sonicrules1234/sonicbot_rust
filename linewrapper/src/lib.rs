use macroquad::prelude::*;
pub struct LineWrapper {
    lines: Vec<String>,
    cols: u8,
    rows: u8,
    charh: f32,
    charw: f32,
    textp: TextParams,
}
impl LineWrapper {
    pub fn new() -> Self {
        let ttffont = macroquad::text::Font::default();
        let mut lines: Vec<String> = Vec::new();
        let chardims = measure_text("a", Some(ttffont), 16, 1.0);
        let charh = chardims.height + 10.0;
        let charw = chardims.width;
        let textp = TextParams {
            font: ttffont,
            font_size: 16,
            font_scale: 1.0,
            font_scale_aspect: 1.0,
            color: WHITE,
        };
        let mut wcount = 0;
        let mut hcount = 0;
        let sw = screen_width();
        let sh = screen_height();
        while wcount as f32 * charw < sw {
            wcount += 1;
        }
        wcount -= 1;
        while hcount as f32 * charh < sh {
            hcount += 1;
        }
        hcount -= 1;
        Self {
            lines: lines,
            cols: wcount,
            rows: hcount,
            charh: charh,
            charw: charw,
            textp: textp,
        }
    }
    pub fn println(&mut self, data: String) -> () {
        let output = textwrap::fill(data.as_str(), (self.cols - 2) as usize).replace("\r", "");
        let lines = output.split("\n").collect::<Vec<&str>>();
        for line in lines {
            self.lines.push(line.to_string());
            if self.lines.len() > (self.rows - 2).into() {
                self.lines.remove(0);
            }
        }
    }
    pub fn showlines(&self) -> () {
        clear_background(BLACK);
        //print!("{}", &self.lines.join("\n"));
        let mut startx = self.charh;
        for line in &self.lines {
            draw_text_ex(&line, self.charw, startx, self.textp);
            startx = startx + self.charh
        }
    }
}
