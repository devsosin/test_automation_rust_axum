use serde::{Deserialize, Serialize};

use crate::domain::category::entity::{BaseCategory, SubCategory};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewBaseCategory {
    type_id: i16,
    book_id: i32,
    is_record: bool,
    is_income: bool,
    name: String,
    color: String,
}

impl NewBaseCategory {
    pub fn new(
        type_id: i16,
        book_id: i32,
        is_record: bool,
        is_income: bool,
        name: String,
        color: String,
    ) -> Self {
        Self {
            type_id,
            book_id,
            is_record,
            is_income,
            name,
            color,
        }
    }

    pub fn to_entity(&self) -> BaseCategory {
        BaseCategory::new(
            self.type_id,
            self.book_id,
            self.is_record,
            self.is_income,
            self.name.to_string(),
            self.color.to_string(),
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NewSubCategory {
    base_id: i16,
    name: String,
}

impl NewSubCategory {
    pub fn new(base_id: i16, name: String) -> Self {
        Self { base_id, name }
    }

    pub fn to_entity(&self) -> SubCategory {
        SubCategory::new(self.base_id, self.name.to_string())
    }
}
