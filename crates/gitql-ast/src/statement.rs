use std::any::Any;
use std::collections::HashMap;

use crate::expression::Expression;
use crate::types::DataType;

pub enum StatementKind {
    Select,
    Where,
    Having,
    Limit,
    Offset,
    OrderBy,
    GroupBy,
    AggregateFunction,
}

pub trait Statement {
    fn get_statement_kind(&self) -> StatementKind;
    fn as_any(&self) -> &dyn Any;
}

pub struct GQLQuery {
    pub statements: HashMap<String, Box<dyn Statement>>,
    pub select_aggregations_only: bool,
    pub hidden_selections: Vec<String>,
}

pub struct SelectStatement {
    pub table_name: String,
    pub fields: Vec<String>,
    pub alias_table: HashMap<String, String>,
}

impl Statement for SelectStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_statement_kind(&self) -> StatementKind {
        StatementKind::Select
    }
}

pub struct WhereStatement {
    pub condition: Box<dyn Expression>,
}

impl Statement for WhereStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_statement_kind(&self) -> StatementKind {
        StatementKind::Where
    }
}

pub struct HavingStatement {
    pub condition: Box<dyn Expression>,
}

impl Statement for HavingStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_statement_kind(&self) -> StatementKind {
        StatementKind::Having
    }
}

pub struct LimitStatement {
    pub count: usize,
}

impl Statement for LimitStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_statement_kind(&self) -> StatementKind {
        StatementKind::Limit
    }
}

pub struct OffsetStatement {
    pub count: usize,
}

impl Statement for OffsetStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_statement_kind(&self) -> StatementKind {
        StatementKind::Offset
    }
}

pub struct OrderByStatement {
    pub field_name: String,
    pub is_ascending: bool,
    pub field_type: DataType,
}

impl Statement for OrderByStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_statement_kind(&self) -> StatementKind {
        StatementKind::OrderBy
    }
}

pub struct GroupByStatement {
    pub field_name: String,
}

impl Statement for GroupByStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_statement_kind(&self) -> StatementKind {
        StatementKind::GroupBy
    }
}

pub struct AggregateFunction {
    pub function_name: String,
    pub argument: String,
}

pub struct AggregationFunctionsStatement {
    pub aggregations: HashMap<String, AggregateFunction>,
}

impl Statement for AggregationFunctionsStatement {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn get_statement_kind(&self) -> StatementKind {
        StatementKind::AggregateFunction
    }
}