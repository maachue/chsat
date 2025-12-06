use anyhow::Result;

use crate::settings::build::BuiltSettings;

pub trait Exec {
    fn display(&self);
    fn apply(&self) -> Result<()>;
}

impl Exec for BuiltSettings {
    fn display(&self) {
        println!("{}.{} = {}", self.master, self.sub, self.value);
        println!("└── Write to: {:?}", self.path)
    }
    fn apply(&self) -> Result<()> {
        std::fs::create_dir_all(self.path.parent().unwrap())?;
        std::fs::write(&self.path, &self.value)?;
        Ok(())
    }
}

