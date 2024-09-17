
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Ident(u64),
    Generic(Box<Type>, Vec<Type>),
}
