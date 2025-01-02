#[derive(Debug)]
pub struct MinStack {
    stack: Vec<i32>,
    mstack: Vec<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            mstack: Vec::new(),
        }
    }
    pub fn push(&mut self, val: i32) {
        self.stack.push(val);
        if self.mstack.is_empty() {
            self.mstack.push(val);
        } else if val < *self.mstack.last().unwrap() {
            self.mstack.push(val);
        } else {
            self.mstack.push(*self.mstack.last().unwrap());
        }
    }
    pub fn pop(&mut self) {
        self.stack.pop();
        self.mstack.pop();
    }
    pub fn top(&self) -> i32 {
        self.stack.last().unwrap().clone()
    }
    pub fn get_min(&self) -> i32 {
        self.mstack.last().unwrap().clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ex1() {
        let cmds = [
            "MinStack", "push", "push", "push", "getMin", "pop", "top", "getMin",
        ];
        let inps = [[0], [-2], [0], [-3], [0], [0], [0], [0]];
        let refs = [0, 0, 0, 0, -3, 0, 0, -2];
        let mut obj = MinStack::new();
        for (i, &cmd) in cmds.iter().enumerate() {
            match cmd {
                "MinStack" => {
                    // println!("MinStack:");
                }
                "push" => {
                    // println!("push: {}", inps[i][0]);
                    obj.push(inps[i][0]);
                }
                "getMin" => {
                    let minim = obj.get_min();
                    // println!("get_min: {}", minim);
                    assert_eq!(minim, refs[i]);
                }
                "pop" => {
                    obj.pop();
                    // println!("pop:");
                }
                "top" => {
                    let val = obj.top();
                    // println!("top: {}", val);
                    assert_eq!(val, refs[i]);
                }
                _ => {
                    // println!("Unknown command!")
                }
            }
        }
    }
    #[test]
    fn ex2() {
        #[rustfmt::skip]
        let cmds = ["MinStack","push","push","push","top","pop","getMin","pop","getMin","pop","push","top","getMin","push","top","getMin","pop","getMin"];
        #[rustfmt::skip]
        let inps = [[0],[2147483646],[2147483646],[2147483647],[0],[0],[0],[0],[0],[0],[2147483647],[0],[0],[-2147483648],[0],[0],[0],[0]];
        #[rustfmt::skip]
        let refs = [0,0,0,0,2147483647,0,2147483646,0,2147483646,0,0,2147483647,2147483647,0,-2147483648,-2147483648,0,2147483647];
        // let cmds = [
        //     "MinStack", "push", "push", "push", "getMin", "pop", "top", "getMin",
        // ];
        // let inps = [0, -2, 0, -3, 0, 0, 0, 0];
        // let refs = [0, 0, 0, 0, -3, 0, 0, -2];
        let mut obj = MinStack::new();
        for (i, &cmd) in cmds.iter().enumerate() {
            match cmd {
                "MinStack" => {
                    // println!("MinStack:");
                }
                "push" => {
                    // println!("push: {}", inps[i][0]);
                    obj.push(inps[i][0]);
                }
                "getMin" => {
                    let minim = obj.get_min();
                    // println!("get_min: {}", minim);
                    assert_eq!(minim, refs[i]);
                }
                "pop" => {
                    obj.pop();
                    // println!("pop:");
                }
                "top" => {
                    let val = obj.top();
                    // println!("top: {}", val);
                    assert_eq!(val, refs[i]);
                }
                _ => {
                    // println!("Unknown command!")
                }
            }
        }
    }
}
