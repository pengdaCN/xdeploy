use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("var type conflict [id: {id}]")]
    VarTypeConflict { id: String },
    #[error("var type non infer [id: {id}]")]
    VarTypeNonInfer{ id: String},
}