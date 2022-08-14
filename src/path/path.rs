use glam::Vec2;

#[derive(Default, Debug)]
pub struct SubPath {
    pub(crate) current_point: Vec2,
    pub(crate) points: Vec<Vec2>,
    pub(crate) curve_points: Vec<Vec2>,
    pub(crate) closed: bool,
}

impl SubPath {
    pub fn len(&self) -> usize {
        self.points.len()
    }

    pub fn line_to(&mut self, p: Vec2) {
        self.points.push(p);
        self.current_point = p;
    }

    pub fn quadratic_curve_to(&mut self, c: Vec2, p: Vec2) {
        self.curve_points.push(self.current_point);
        self.curve_points.push(c);
        self.curve_points.push(p);

        self.points.push(p);
        self.current_point = p;
    }

    pub fn close(&mut self) {
        self.closed = true
    }
}

#[derive(Default, Debug)]
pub struct Path {
    pub(crate) current_origin: Vec2,
    pub(crate) sub_paths: Vec<SubPath>,
}

impl Path {
    pub fn new() -> Self {
        Self {
            current_origin: Vec2::default(),
            sub_paths: Vec::default(),
        }
    }

    pub fn move_to(&mut self, p: Vec2) {
        self.current_origin = p;

        // If last sub-path is not empty, start a new sub-path
        if self.sub_paths.len() == 0 || self.sub_paths[0].len() > 0 {
            self.sub_paths.push(SubPath::default());
        }

        if let Some(sub_path) = self.sub_paths.last_mut() {
            sub_path.current_point = p;
        }
    }

    pub fn line_to(&mut self, p: Vec2) {
        if self.sub_paths.len() == 0 {
            self.sub_paths.push(SubPath::default());
        }

        let cur_path = self.sub_paths.last_mut().unwrap();

        cur_path.line_to(p);
    }

    pub fn quadratic_curve_to(&mut self, control_point: Vec2, p: Vec2) {
        if self.sub_paths.len() == 0 {
            self.sub_paths.push(SubPath::default());
        }

        let cur_path = self.sub_paths.last_mut().unwrap();

        cur_path.quadratic_curve_to(control_point, p);
    }

    pub fn close_subpath(&mut self) {
        if let Some(sub_path) = self.sub_paths.last_mut() {
            sub_path.close();
        }
    }
}
