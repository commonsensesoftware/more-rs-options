use config::prelude::*;
use di::{injectable, Injectable, Ref, ServiceCollection};
use options::prelude::*;
use serde::Deserialize;
use std::error::Error;

#[derive(Default, Deserialize)]
pub struct PositionOptions {
    pub title: String,
    pub name: String,
}

pub struct TestModel {
    options: Ref<PositionOptions>,
}

#[injectable]
impl TestModel {
    pub fn new(options: Ref<PositionOptions>) -> Self {
        Self { options }
    }

    pub fn get(&self) -> String {
        format!("Title: {}\nName: {}", self.options.title, self.options.name)
    }
}

fn main() -> Result<(), Box<dyn Error + 'static>> {
    let config = config::builder()
        .add_in_memory(&[("Title", "Example"), ("Name", "Demo")])
        .build()?;
    let provider = ServiceCollection::new()
        .add(TestModel::transient())
        .apply_config::<PositionOptions>(config)
        .build_provider()?;
    let model = provider.get_required::<TestModel>();

    println!("{}", model.get());
    Ok(())
}
