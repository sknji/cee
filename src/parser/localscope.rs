pub struct LocalScope {
    pub local_count: i8,
    pub scope_depth: i8,
    pub locals: Vec<Local>,
    pub offset: i32,
}

pub struct Local {
    id: i8,
    name: String,
    depth: i8,
    offset: i32,
}

impl Local {
    pub fn new(name: String, depth: i8, id: i8) -> Self {
        Self {
            name,
            depth,
            offset: 0,
            id,
        }
    }
}

impl LocalScope {
    pub fn new() -> Self {
        Self {
            local_count: 0,
            locals: Vec::new(),
            scope_depth: 0,
            offset: 0,
        }
    }

    pub fn begin_scope(&mut self) {
        self.scope_depth += 1;
    }

    pub fn end_scope(&mut self) -> u8 {
        self.scope_depth -= 1;
        let mut counts = 0;
        while self.local_count > 0
            && self
                .locals
                .get((self.local_count - 1) as usize)
                .unwrap()
                .depth
                > self.scope_depth
        {
            counts += 1;
            self.local_count -= 1
        }
        counts
    }

    pub fn add_local_if_not_exist(&mut self, name: &str) -> i8 {
        let (exist, id) = self.contains(name);
        if exist {
            return id;
        }

        self.add_local(name)
    }

    pub fn add_local(&mut self, name: &str) -> i8 {
        self.local_count += 1;
        let local = Local::new(name.to_string(), self.scope_depth, self.local_count);
        self.locals.push(local);
        self.local_count
    }

    pub fn offset_by_id(&self, id: i8) -> (bool, i32) {
        let mut counter = self.local_count - 1;

        while counter >= 0 {
            let l = self.locals.get(counter as usize).unwrap();
            if l.depth != -1 && l.depth < self.scope_depth {
                break;
            }

            if l.id == id {
                return (true, l.offset);
            }

            counter -= 1;
        }
        (false, 0)
    }

    pub fn contains(&self, name: &str) -> (bool, i8) {
        let mut counter = self.local_count - 1;

        while counter >= 0 {
            let l = self.locals.get(counter as usize).unwrap();
            if l.depth != -1 && l.depth < self.scope_depth {
                break;
            }

            if l.name.eq(name) {
                return (true, l.id);
            }

            counter -= 1;
        }

        (false, 0)
    }

    pub fn resolve_local(&self, name: &str) -> Option<u8> {
        let mut counter = self.local_count - 1;

        while counter >= 0 {
            let l = self.locals.get(counter as usize);
            match l {
                None => {}
                Some(v) => {
                    if v.name.eq(name) {
                        return Some(counter as u8);
                    }
                }
            }

            counter -= 1;
        }

        return None;
    }

    pub fn assign_offsets(&mut self) -> i32 {
        let mut offset = 0;
        for loc in self.locals.iter_mut() {
            offset += 8;
            loc.offset = -offset;
        }

        self.offset = offset;

        self.offset
    }
}
