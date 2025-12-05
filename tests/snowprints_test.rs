use snowprints::{Error, Params, Snowprints, compose, decompose};

const JANUARY_1ST_2024_AS_MS: u64 = 1704096000000;

#[test]
fn compose_and_decompose() {
    let time = 987654321;
    let logical_volume = 7890;
    let sequence = 956;

    let snowprint = compose(time, logical_volume, sequence);
    let (d_time, d_logical_volume, d_sequence) = decompose(snowprint);

    assert_eq!(time, d_time);
    assert_eq!(logical_volume, d_logical_volume);
    assert_eq!(sequence, d_sequence);
}

#[test]
fn compose_and_decompose_from_a_real_date() {
    let logical_volume = 7890;
    let sequence = 956;

    let snowprint = compose(JANUARY_1ST_2024_AS_MS, logical_volume, sequence);
    let (d_time, d_logical_volume, d_sequence) = decompose(snowprint);

    assert_eq!(JANUARY_1ST_2024_AS_MS, d_time);
    assert_eq!(logical_volume, d_logical_volume);
    assert_eq!(sequence, d_sequence);
}

#[test]
fn snowprint_struct_builds_and_returns_snowprint() {
    let params = Params {
        origin_time_ms: JANUARY_1ST_2024_AS_MS,
        logical_volume_base: 0,
        logical_volume_length: 8192,
    };

    let mut snowprints = match Snowprints::from_params(params) {
        Ok(snow) => snow,
        // error by comparing result to incorrect error
        Err(err) => return assert_eq!(Error::ExceededAvailableSequences, err),
    };

    let snowprint = snowprints.create_id();
    match snowprint {
        Ok(sp) => {
            let (_timestamp, logical_volume, sequence) = decompose(sp);

            assert_eq!(logical_volume, 0);
            assert_eq!(sequence, 1);
        }
        // error by comparing result to incorrect error
        Err(err) => assert_eq!(Error::ExceededAvailableLogicalVolumes, err),
    }
}
