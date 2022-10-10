use crate::{cli::Args, config::ContainerOpts, errors::Errcode};

pub struct Container {
    config: ContainerOpts,
}

impl Container {
    pub fn new(args: Args) -> Result<Container, Errcode> {
        let config = ContainerOpts::new(args.command, args.uid, args.mount_dir)?;

        Ok(Container { config })
    }

    pub fn create(&mut self) -> Result<(), Errcode> {
        log::debug!("Creating container");
        Ok(())
    }

    pub fn clean_exit(&mut self) -> Result<(), Errcode> {
        log::debug!("Cleaning container");
        Ok(())
    }
}
