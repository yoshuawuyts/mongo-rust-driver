#[cfg(test)]
mod test;

use bson::Document;

use super::Operation;
use crate::{
    cmap::{Command, CommandResponse, StreamDescription},
    error::{ErrorKind, Result},
    options::WriteConcern,
    selection_criteria::SelectionCriteria,
};

#[derive(Debug)]
pub(crate) struct RunCommand {
    db: String,
    command: Document,
    selection_criteria: Option<SelectionCriteria>,
    write_concern: Option<WriteConcern>,
}

impl RunCommand {
    pub(crate) fn new(
        db: String,
        command: Document,
        selection_criteria: Option<SelectionCriteria>,
    ) -> Result<Self> {
        let write_concern = command
            .get("writeConcern")
            .map(|doc| bson::from_bson::<WriteConcern>(doc.clone()))
            .transpose()?;

        Ok(Self {
            db,
            command,
            selection_criteria,
            write_concern,
        })
    }
}

impl Operation for RunCommand {
    type O = Document;

    // Since we can't actually specify a string statically here, we just put a descriptive string
    // that should fail loudly if accidentally passed to the server.
    const NAME: &'static str = "$genericRunCommand";

    fn build(&self, description: &StreamDescription) -> Result<Command> {
        let command_name =
            self.command
                .keys()
                .next()
                .cloned()
                .ok_or_else(|| ErrorKind::ArgumentError {
                    message: "an empty document cannot be passed to a run_command operation".into(),
                })?;

        Ok(Command::new(
            command_name,
            self.db.clone(),
            self.command.clone(),
        ))
    }

    fn handle_response(&self, response: CommandResponse) -> Result<Self::O> {
        Ok(response.raw_response)
    }

    fn selection_criteria(&self) -> Option<&SelectionCriteria> {
        self.selection_criteria.as_ref()
    }

    fn write_concern(&self) -> Option<&WriteConcern> {
        self.write_concern.as_ref()
    }

    fn handles_command_errors(&self) -> bool {
        true
    }
}
