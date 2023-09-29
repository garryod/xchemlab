use futures_util::task::{FutureObj, Spawn, SpawnError};
use tokio::runtime::Handle;

#[derive(Debug)]
pub struct Spawner(Handle);

impl Spawner {
    pub fn new() -> Self {
        Self(Handle::current())
    }
}

impl Spawn for Spawner {
    fn spawn_obj(&self, future: FutureObj<'static, ()>) -> Result<(), SpawnError> {
        self.0.spawn(future);
        Ok(())
    }
}
