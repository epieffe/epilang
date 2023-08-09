use crate::intermediate::constant::Constant;
use crate::operations::OperationError;
use std::ops::Not;

impl Not for Constant {
    type Output = Constant;

    fn not(self) -> Constant {
        Constant::Bool(!self.as_bool())
    }
}

pub fn conjunction(value_1: Constant, value_2: Constant) -> Result<Constant, OperationError> {
    match value_1 {
        Constant::Int(_) => error_other!(Int, Conj, value_2),
        Constant::Float(_) => error_other!(Float, Conj, value_2),
        Constant::String(_) => error_other!(String, Conj, value_2),
        Constant::Bool(v1) => match value_2 {
            Constant::Int(_) => error!(Bool, Conj, Int),
            Constant::Float(_) => error!(Bool, Conj, Float),
            Constant::String(_) => error!(Bool, Conj, String),
            Constant::Bool(v2) => Ok(Constant::Bool(v1 && v2)),
        },
    }
}

pub fn disjunction(value_1: Constant, value_2: Constant) -> Result<Constant, OperationError> {
    match value_1 {
        Constant::Int(_) => error_other!(Int, Disj, value_2),
        Constant::Float(_) => error_other!(Float, Disj, value_2),
        Constant::String(_) => error_other!(String, Disj, value_2),
        Constant::Bool(v1) => match value_2 {
            Constant::Int(_) => error!(Bool, Disj, Int),
            Constant::Float(_) => error!(Bool, Disj, Float),
            Constant::String(_) => error!(Bool, Disj, String),
            Constant::Bool(v2) => Ok(Constant::Bool(v1 || v2)),
        },
    }
}

pub fn equals(value_1: Constant, value_2: Constant) -> Result<Constant, OperationError> {
    match value_1 {
        Constant::Int(v1) => match value_2 {
            Constant::Int(v2) => Ok(Constant::Bool(v1 == v2)),
            other => error_other!(Int, Equals, other),
        },
        Constant::Float(v1) => match value_2 {
            Constant::Float(v2) => Ok(Constant::Bool(v1 == v2)),
            other => error_other!(Int, Equals, other),
        },
        Constant::String(v1) => match value_2 {
            Constant::String(v2) => Ok(Constant::Bool(v1 == v2)),
            other => error_other!(String, Equals, other),
        },
        Constant::Bool(v1) => match value_2 {
            Constant::Bool(v2) => Ok(Constant::Bool(v1 == v2)),
            other => error_other!(Bool, Equals, other),
        },
    }
}

pub fn not_equals(value_1: Constant, value_2: Constant) -> Result<Constant, OperationError> {
    match value_1 {
        Constant::Int(v1) => match value_2 {
            Constant::Int(v2) => Ok(Constant::Bool(v1 != v2)),
            other => error_other!(Int, Equals, other),
        },
        Constant::Float(v1) => match value_2 {
            Constant::Float(v2) => Ok(Constant::Bool(v1 != v2)),
            other => error_other!(Int, Equals, other),
        },
        Constant::String(v1) => match value_2 {
            Constant::String(v2) => Ok(Constant::Bool(v1 != v2)),
            other => error_other!(String, Equals, other),
        },
        Constant::Bool(v1) => match value_2 {
            Constant::Bool(v2) => Ok(Constant::Bool(v1 != v2)),
            other => error_other!(Bool, Equals, other),
        },
    }
}

pub fn greater(value_1: Constant, value_2: Constant) -> Result<Constant, OperationError> {
    match value_1 {
        Constant::Int(v1) => match value_2 {
            Constant::Int(v2) => Ok(Constant::Bool(v1 > v2)),
            other => error_other!(Int, Greater, other),
        },
        Constant::Float(v1) => match value_2 {
            Constant::Float(v2) => Ok(Constant::Bool(v1 > v2)),
            other => error_other!(Int, Greater, other),
        },
        Constant::String(v1) => match value_2 {
            Constant::String(v2) => Ok(Constant::Bool(v1 > v2)),
            other => error_other!(String, Greater, other),
        },
        Constant::Bool(_) => error_other!(Bool, Greater, value_2),
    }
}

pub fn greater_equals(value_1: Constant, value_2: Constant) -> Result<Constant, OperationError> {
    match value_1 {
        Constant::Int(v1) => match value_2 {
            Constant::Int(v2) => Ok(Constant::Bool(v1 >= v2)),
            other => error_other!(Int, GreaterEquals, other),
        },
        Constant::Float(v1) => match value_2 {
            Constant::Float(v2) => Ok(Constant::Bool(v1 >= v2)),
            other => error_other!(Int, GreaterEquals, other),
        },
        Constant::String(v1) => match value_2 {
            Constant::String(v2) => Ok(Constant::Bool(v1 >= v2)),
            other => error_other!(String, GreaterEquals, other),
        },
        Constant::Bool(_) => error_other!(Bool, GreaterEquals, value_2),
    }
}

pub fn lower(value_1: Constant, value_2: Constant) -> Result<Constant, OperationError> {
    match value_1 {
        Constant::Int(v1) => match value_2 {
            Constant::Int(v2) => Ok(Constant::Bool(v1 < v2)),
            other => error_other!(Int, Lower, other),
        },
        Constant::Float(v1) => match value_2 {
            Constant::Float(v2) => Ok(Constant::Bool(v1 < v2)),
            other => error_other!(Int, Lower, other),
        },
        Constant::String(v1) => match value_2 {
            Constant::String(v2) => Ok(Constant::Bool(v1 < v2)),
            other => error_other!(String, Lower, other),
        },
        Constant::Bool(_) => error_other!(Bool, Lower, value_2),
    }
}

pub fn lower_equals(value_1: Constant, value_2: Constant) -> Result<Constant, OperationError> {
    match value_1 {
        Constant::Int(v1) => match value_2 {
            Constant::Int(v2) => Ok(Constant::Bool(v1 <= v2)),
            other => error_other!(Int, LowerEquals, other),
        },
        Constant::Float(v1) => match value_2 {
            Constant::Float(v2) => Ok(Constant::Bool(v1 <= v2)),
            other => error_other!(Int, LowerEquals, other),
        },
        Constant::String(v1) => match value_2 {
            Constant::String(v2) => Ok(Constant::Bool(v1 <= v2)),
            other => error_other!(String, LowerEquals, other),
        },
        Constant::Bool(_) => error_other!(Bool, LowerEquals, value_2),
    }
}
