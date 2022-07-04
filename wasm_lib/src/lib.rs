use anonymizer_lib::{Anonymizer, AnonymizerMeta};
use dicom_object::from_reader;
use js_sys::Uint8Array;
use once_cell::sync::OnceCell;
use std::io::Cursor;
use wasm_bindgen::prelude::*;

static ANONYMIZER: OnceCell<Anonymizer> = OnceCell::new();

fn get_mut_anonymizer() -> Result<Anonymizer, JsValue> {
    let mut any = Anonymizer::new().unwrap();
    ANONYMIZER.get().unwrap().clone_into(&mut any);
    Ok(any)
}

#[wasm_bindgen]
pub fn init_anonymize(data: Uint8Array) -> Result<(), JsValue> {
    let file = Cursor::new(data.to_vec());

    let obj = from_reader(file).unwrap_throw();
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

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn pass() {
        assert_eq!(1, 1);
    }
}
