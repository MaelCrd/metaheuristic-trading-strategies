use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct IndicatorCombination {
    pub id: i32,
    pub name: String,
    pub hidden: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompleteIndicatorCombination {
    pub id: i32,
    pub name: String,
    pub indicators_struct_names: Vec<String>,
    pub hidden: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIndicatorCombination {
    pub name: String,
    pub indicators_struct_names: Vec<String>,
}
