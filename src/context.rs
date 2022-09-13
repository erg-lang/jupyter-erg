use std::collections::HashMap;
use std::time::Duration;

use crate::errors::Error;

// Outputs from an EvalContext. This is a separate struct since users may want
// destructure this and pass its components to separate threads.
pub struct EvalContextOutputs {
    pub stdout: crossbeam_channel::Receiver<String>,
    pub stderr: crossbeam_channel::Receiver<String>,
}

#[derive(Default, Debug)]
pub struct EvalOutputs {
    pub content_by_mime_type: HashMap<String, String>,
    pub timing: Option<Duration>,
    // pub phases: Vec<PhaseDetails>,
}

impl EvalOutputs {
    pub fn new() -> EvalOutputs {
        EvalOutputs {
            content_by_mime_type: HashMap::new(),
            timing: None,
            // phases: Vec::new(),
        }
    }

    pub fn text_html(text: String, html: String) -> EvalOutputs {
        let mut out = EvalOutputs::new();
        out.content_by_mime_type
            .insert("text/plain".to_owned(), text);
        out.content_by_mime_type
            .insert("text/html".to_owned(), html);
        out
    }

    pub fn is_empty(&self) -> bool {
        self.content_by_mime_type.is_empty()
    }

    pub fn get(&self, mime_type: &str) -> Option<&str> {
        self.content_by_mime_type.get(mime_type).map(String::as_str)
    }

    pub fn merge(&mut self, other: EvalOutputs) {
        for (mime_type, content) in other.content_by_mime_type {
            self.content_by_mime_type
                .entry(mime_type)
                .or_default()
                .push_str(&content);
        }
    }
}

fn text_plain(content: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    map.insert("text/plain".to_owned(), content.to_owned());
    map
}

pub struct CommandContext {

}

impl CommandContext {
    pub fn new() -> Result<Self, Error> {
        /*let (stdout_sender, stdout_receiver) = crossbeam_channel::unbounded();
        let (stderr_sender, stderr_receiver) = crossbeam_channel::unbounded();
        let outputs = EvalContextOutputs {
            stdout: stdout_receiver,
            stderr: stderr_receiver,
        };*/
        Ok(Self {})
    }

    pub fn execute(
        &mut self,
        to_run: &str,
    ) -> Result<EvalOutputs, Error> {
        todo!()
    }
}
