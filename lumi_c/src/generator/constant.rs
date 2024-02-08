use super::chunk::Chunk;

#[derive(Debug, Clone)]
pub struct ConstFunc {
    name: String,
    params: Vec<String>,
    chunk: Chunk,
}

impl ConstFunc {
    pub fn new(name: &str, params: Vec<String>, chunk: Chunk) -> Self {
        Self {
            name: name.to_owned(),
            params: params.to_vec(),
            chunk,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn params(&self) -> &Vec<String> {
        &self.params
    }

    pub fn chunk(&self) -> &Chunk {
        &self.chunk
    }
}

#[derive(Debug, Clone)]
pub enum Constant {
    Nil,
    Num(f64),
    Bool(bool),
    Str(String),
    Func(ConstFunc),
}

impl Constant {
    pub fn as_str(&self) -> String {
        if let Constant::Str(s) = self {
            s.clone()
        } else {
            panic!("Not a string")
        }
    }

    pub fn as_num(&self) -> f64 {
        if let Constant::Num(s) = self {
            *s
        } else {
            panic!("Not a string")
        }
    }

    pub fn as_function(&self) -> ConstFunc {
        if let Constant::Func(f) = self {
            f.clone()
        } else {
            panic!("Not a string")
        }
    }
}
