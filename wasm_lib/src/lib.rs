use anonymizer_lib::{Anonymizer, AnonymizerMeta};
use dicom_object::from_reader;
use once_cell::sync::OnceCell;
use wasm_bindgen::prelude::*;
use web_sys::console;

static ANONYMIZER: OnceCell<Anonymizer> = OnceCell::new();

fn get_mut_anonymizer() -> Result<Anonymizer, JsValue> {
    let mut any = Anonymizer::new().unwrap();
    ANONYMIZER.get().unwrap().clone_into(&mut any);
    Ok(any)
}

#[wasm_bindgen]
pub fn init_anonymize(fr: &[u8]) -> Result<(), JsValue> {
    let obj = from_reader(fr).unwrap_throw();
    let any = Anonymizer::from_object(obj).unwrap_throw();
    ANONYMIZER.set(any).unwrap_throw();

    Ok(())
}

#[wasm_bindgen]
pub fn anonymize(config: &JsValue) -> Result<(), JsValue> {
    let config: AnonymizerMeta = config.into_serde().unwrap_throw();

    let mut any = get_mut_anonymizer().unwrap_throw();

    any.meta(config);
    any.anonymize();
    ANONYMIZER.set(any).unwrap_throw();

    Ok(())
}
