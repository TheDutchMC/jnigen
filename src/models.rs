use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Input {
    pub packages: Vec<String>,
    pub classes: Vec<Class>,
    pub interfaces: Vec<Class>,
}

#[derive(Deserialize, Clone)]
pub struct Class {
    pub name: String,
    pub methods: Vec<Method>,
    pub implementing: Vec<String>,
}

#[derive(Deserialize, Clone)]
pub struct Method {
    pub name: String,
    #[serde(rename(deserialize = "returnType"))]
    pub return_type: Parameter,
    #[serde(rename(deserialize = "isStatic"))]
    pub is_static: bool,
    #[serde(rename(deserialize = "fromInterface"))]
    pub from_interface: Option<String>,
    pub parameters: Vec<Parameter>,
}

#[derive(Deserialize, Clone)]
pub struct Parameter {
    pub name: Option<String>,
    #[serde(rename(deserialize = "type"))]
    pub class: String,
    #[serde(rename(deserialize = "isArray"))]
    pub is_array: bool
}