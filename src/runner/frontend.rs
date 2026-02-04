use crate::runner::BuiltCommand;
use color_eyre::Result;

pub trait FrontEnd {
    fn dry_run(&self);
    fn execute(&self) -> Result<()>;
}

impl FrontEnd for BuiltCommand {
    fn dry_run(&self) {
        println!("{}", self)
    }

    fn execute(&self) -> Result<()> {
        use duct::cmd;
        if self.is_sudo {
            cmd(
                "sudo",
                std::iter::once(self.program.clone()).chain(self.args.clone()),
            )
            .run()?
        } else {
            cmd(&self.program, &self.args).run()?
        };

        Ok(())
    }
}
