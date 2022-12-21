pub struct DaemonCommandManager {
    commands: HashMap<String, Box<dyn DaemonCommand>>,
}

pub trait RapdCommand {
    fn execute(&self, msg: DaemonCommand);
}