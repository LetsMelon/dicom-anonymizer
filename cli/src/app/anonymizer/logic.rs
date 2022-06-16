use anonymizer_lib::Anonymizer;
use anyhow::Result;
use clap::ArgMatches;

use crate::app::anonymizer::values::AnonymizerValues;
use crate::app::types::Matcher;

pub fn logic(matches: ArgMatches) -> Result<()> {
    let matches =
        AnonymizerValues::match_args(matches.subcommand_matches("anonymizer").unwrap().clone())?;

    let mut obj = Anonymizer::from_file(&matches.input.to_string_lossy())?;
    obj.meta(matches.match_trait()?);

    obj.anonymize();

    match (matches.output, matches.dry_run) {
        (_, true) => (),
        (None, false) => (),
        (Some(path), false) => {
            obj.save(path.to_string_lossy().as_ref())?;
        }
    }
    Ok(())
}
