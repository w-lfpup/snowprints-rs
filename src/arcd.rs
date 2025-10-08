use crate::snowprints::{Error, Params, Snowprints};
use std::sync::{Arc, Mutex};

pub fn from_params(params: Params) -> Result<Arc<Mutex<Snowprints>>, Error> {
    let snowprints = match Snowprints::from_params(params) {
        Ok(snowprints) => snowprints,
        Err(e) => return Err(e),
    };

    Ok(Arc::new(Mutex::new(snowprints)))
}

pub fn create_id(snowprints_acrd: &Arc<Mutex<Snowprints>>) -> Result<u64, String> {
    let mut snowprints = match snowprints_acrd.lock() {
        Ok(snowprints) => snowprints,
        Err(e) => return Err(e.to_string()),
    };

    match snowprints.create_id() {
        Ok(snowprint) => Ok(snowprint),
        Err(_e) => return Err("failed to create an id".to_string()),
    }
}

pub fn get_timestamp(snowprints_acrd: &Arc<Mutex<Snowprints>>) -> Result<u64, String> {
    let snowprints = match snowprints_acrd.lock() {
        Ok(snowprints) => snowprints,
        Err(e) => return Err(e.to_string()),
    };

    Ok(snowprints.get_timestamp())
}

pub fn get_bit_shifted_timestamp(
    snowprints_acrd: &Arc<Mutex<Snowprints>>,
    offset: u64,
) -> Result<u64, String> {
    let snowprints = match snowprints_acrd.lock() {
        Ok(snowprints) => snowprints,
        Err(e) => return Err(e.to_string()),
    };

    Ok(snowprints.get_bit_shifted_timestamp(offset))
}
