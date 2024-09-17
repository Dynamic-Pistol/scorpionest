#[derive(Debug, Clone, Copy)]
pub enum DeclarationType {
    Mutable,
    Immutable,
}

#[derive(Debug, Clone, Copy)]
pub enum ParamType {
    Reference,
    Value,
    Input,
    Output,
    Invalid,
}

#[derive(Debug, Clone, Copy)]
pub enum ParamRestrictor {
    Mutable,
    Constant,
    Invalid,
}
