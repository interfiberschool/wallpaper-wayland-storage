pub struct CommandManager {
    commands: HashMap<String, Box<dyn RapdCommand>>,
}

pub trait RapdCommand {
    fn execute(&self, msg: ) -> RapdCommandResponse;
}