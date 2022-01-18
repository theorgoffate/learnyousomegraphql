#[derive(GraphQLObject)]
#[graphql(description="A notebook to store ideas in")]
pub struct Notebook {
    id: String,
    name: String,
    location: String
}

impl Notebook {
    pub fn default() -> Self {
        Notebook{id: String::new(), name: String::new(), location: String::new()}
    }
}

#[derive(GraphQLInputObject)]
#[graphql(description="A form for a new Notebook")]
pub struct NewNotebook {
    name: String,
    location: String
}