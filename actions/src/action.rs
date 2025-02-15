use ghactions::prelude::*;

#[derive(Actions, Debug, Clone)]
#[action(
    // Name of the Action
    name = "My Action",
    // Description of the Action
    description = "My Action Description",
    // Path to the action.yml file
    path = "./action.yml",
    // Path to the Dockerfile
    image = "./Dockerfile",

)]
pub struct Action {
    /// GitHub Token
    #[input(description = "GitHub Token")]
    token: String,
}
