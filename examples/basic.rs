use config::prelude::*;
use options::{DefaultFactory, DefaultMonitor, Monitor, Ref};
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
    let factory = DefaultFactory::new(
        vec![Ref::new(move |_: &str, o: &mut PositionOptions| {
            config.bind_unchecked(o)
        })],
        Vec::new(),
        Vec::new(),
    );
    let monitor = DefaultMonitor::from(factory);
    let model = TestModel::new(monitor.get()?);

    println!("{}", model.get());
    Ok(())
}
