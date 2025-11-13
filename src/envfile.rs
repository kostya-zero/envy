use indexmap::IndexMap;

#[derive(Debug, Default)]
pub struct Envfile {
    variables: IndexMap<String, String>,
}

impl Envfile {
    pub fn new() -> Self {
        Self {
            variables: IndexMap::new(),
        }
    }
}
