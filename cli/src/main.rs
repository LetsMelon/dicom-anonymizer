use anonymizer_lib::Anonymizer;
use anyhow::Result;

mod args;
mod cli;
mod utils;
mod validator;

use crate::cli::App;
use crate::utils::match_args_into_trait;

fn main() -> Result<()> {
    let app = App::new();

    let mut obj = Anonymizer::from_file(&app.input.to_string_lossy())?;
    obj.meta(match_args_into_trait(&app)?);

    obj.anonymize();

    match (app.output, app.dry_run) {
        (_, true) => (),
        (None, false) => (),
        (Some(path), false) => {
            obj.save(path.to_string_lossy().as_ref())?;
        }
    }

    Ok(())
}
