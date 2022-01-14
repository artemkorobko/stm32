pub trait CommandExecutor {
    fn exec(&self) -> anyhow::Result<()>;
}
