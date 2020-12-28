use std::collections::HashMap;

use crate::{
    lexer::{Instruction, Lexer},
    value::Value,
};

pub struct Vm<'a> {
    stack: Vec<Value>,
    lexer: Lexer<'a>,
}

impl<'a> Vm<'a> {
    pub fn new(input: &'a [u8]) -> Self {
        Self {
            stack: Vec::new(),
            lexer: Lexer::new(input),
        }
    }

    fn map_top(&mut self, f: impl Fn(&mut Value)) {
        let len = self.stack.len();
        if let Some(v) = self.stack.get_mut(len - 1) {
            f(v)
        }
    }

    fn reduce_top_two(&mut self, f: impl Fn(&mut Value, Value)) {
        if let Some(v2) = self.stack.pop() {
            self.map_top(|v1| f(v1, v2.clone()));
        }
    }

    fn reduce_top_three(&mut self, f: impl Fn(&mut Value, Value, Value)) {
        if let Some(p2) = self.stack.pop() {
            if let Some(p1) = self.stack.pop() {
                self.map_top(|v| f(v, p1.clone(), p2.clone()));
            }
        }
    }

    pub fn run(&mut self) -> Option<Value> {
        for instruction in self.lexer.clone() {
            if let Some(instruction) = instruction {
                match instruction {
                    Instruction::Inew => {
                        self.stack.push(Value::Int(0));
                    }
                    Instruction::Iinc => self.map_top(|x| {
                        if let Value::Int(i) = x {
                            *i += 1
                        }
                    }),
                    Instruction::Ishl => self.map_top(|x| {
                        if let Value::Int(i) = x {
                            *i <<= 1
                        }
                    }),
                    Instruction::Iadd => self.reduce_top_two(|x, y| {
                        if let (Value::Int(i1), Value::Int(i2)) = (x, y) {
                            *i1 += i2
                        }
                    }),
                    Instruction::Ineg => self.map_top(|x| {
                        if let Value::Int(i) = x {
                            *i = -*i;
                        }
                    }),
                    Instruction::Isht => self.reduce_top_two(|x, y| {
                        if let (Value::Int(i), Value::Int(y)) = (x, y) {
                            *i <<= y;
                        }
                    }),
                    Instruction::Itof => self.map_top(|x| {
                        if let Value::Int(i) = x {
                            let b = i.to_le_bytes();
                            let f = f64::from_le_bytes(b);
                            *x = Value::Float(f)
                        }
                    }),
                    Instruction::Itou => self.map_top(|x| {
                        if let Value::Int(i) = x {
                            let b = i.to_le_bytes();
                            let u = u64::from_le_bytes(b);
                            *x = Value::Uint(u)
                        }
                    }),
                    Instruction::Finf => self.stack.push(Value::Float(f64::INFINITY)),
                    Instruction::Fnan => self.stack.push(Value::Float(f64::NAN)),
                    Instruction::Fneg => self.map_top(|x| {
                        if let Value::Float(f) = x {
                            *f = -*f;
                        }
                    }),
                    Instruction::Snew => self.stack.push(Value::String(Vec::new())),
                    Instruction::Sadd => self.reduce_top_two(|x, y| {
                        if let (Value::String(s), Value::Int(i)) = (x, y) {
                            let b = i.to_le_bytes();
                            s.push(b[0]);
                        }
                    }),
                    Instruction::Onew => self.stack.push(Value::Object(HashMap::new())),
                    Instruction::Oadd => self.reduce_top_three(|x, y, z| {
                        if let Value::String(s) = y {
                            if let Value::Object(o) = x {
                                o.insert(s, z);
                            }
                        }
                    }),
                    Instruction::Anew => self.stack.push(Value::Array(Vec::new())),
                    Instruction::Aadd => self.reduce_top_two(|x, y| {
                        if let Value::Array(a) = x {
                            a.push(y);
                        }
                    }),
                    Instruction::Bnew => self.stack.push(Value::Bool(false)),
                    Instruction::Bneg => self.map_top(|x| {
                        if let Value::Bool(b) = x {
                            *b = !*b;
                        }
                    }),
                    Instruction::Nnew => self.stack.push(Value::Nil),
                    Instruction::Gdup => {
                        if let Some(v) = self.stack.pop() {
                            self.stack.push(v.clone());
                            self.stack.push(v);
                        }
                    }
                    Instruction::Gpop => {
                        self.stack.pop();
                    }
                    Instruction::Gswp => {
                        if let Some(x) = self.stack.pop() {
                            if let Some(y) = self.stack.pop() {
                                self.stack.push(y);
                                self.stack.push(x);
                                continue;
                            }
                        }
                        panic!("Insufficient elements on the stack");
                    }
                }
            }
        }

        self.stack.pop()
    }
}
