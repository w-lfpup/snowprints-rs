// The point is to distribute ids across logical volume shards evenly
//     - reset sequence every MS to 0 to remain sortable
//     - increase logical volume sequence by 1 every MS
//     - return err if available logical volume ids have been used

// This assumes sequences + logical volume ids occur in the same ms
// https://instagram-engineering.com/sharding-ids-at-instagram-1cf5a71e5a5c

#[cfg(test)]
mod test;

use std::time::SystemTime;

const SEQUENCE_BIT_LEN: u64 = 10;
const SEQUENCE_BIT_MASK: u64 = (1 << SEQUENCE_BIT_LEN) - 1;
const MAX_SEQUENCES: u64 = u32::pow(2, SEQUENCE_BIT_LEN as u32) as u64;
const LOGICAL_VOLUME_BIT_LEN: u64 = 13;
const LOGICAL_VOLUME_BIT_MASK: u64 = ((1 << LOGICAL_VOLUME_BIT_LEN) - 1) << SEQUENCE_BIT_LEN;
const MAX_LOGICAL_VOLUMES: u64 = u32::pow(2, LOGICAL_VOLUME_BIT_LEN as u32) as u64;

// core functionality of snowprints
pub fn compose(ms_timestamp: u64, logical_volume: u64, ticket_id: u64) -> u64 {
    ms_timestamp << (LOGICAL_VOLUME_BIT_LEN + SEQUENCE_BIT_LEN)
        | logical_volume << SEQUENCE_BIT_LEN
        | ticket_id
}

pub fn decompose(snowprint: u64) -> (u64, u64, u64) {
    let time = snowprint >> (LOGICAL_VOLUME_BIT_LEN + SEQUENCE_BIT_LEN);
    let logical_volume = (snowprint & LOGICAL_VOLUME_BIT_MASK) >> SEQUENCE_BIT_LEN;
    let ticket_id = snowprint & SEQUENCE_BIT_MASK;

    (time, logical_volume, ticket_id)
}

// utility for rotating logical volumes and sequences
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Error {
    LogicalVolumeModuloIsZero,
    ExceededAvailableLogicalVolumes,
    FailedToParseOriginSystemTime,
    ExceededAvailableSequences,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Settings {
    pub origin_system_time: SystemTime,
    pub logical_volume_base: u64,
    pub logical_volume_length: u64,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct State {
    pub duration_ms: u64,
    pub sequence: u64,
    pub logical_volume: u64,
    pub prev_logical_volume: u64,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Snowprint {
    settings: Settings,
    state: State,
}

impl Snowprint {
    pub fn new(settings: Settings) -> Result<Snowprint, Error> {
        if let Err(err) = check_settings(&settings) {
            return Err(err);
        }

        let duration_ms = match SystemTime::now().duration_since(settings.origin_system_time) {
            Ok(duration) => duration.as_millis() as u64,
            _ => return Err(Error::FailedToParseOriginSystemTime),
        };

        Ok(Snowprint {
            settings: settings,
            state: State {
                duration_ms: duration_ms,
                sequence: 0,
                logical_volume: 0,
                prev_logical_volume: 0,
            },
        })
    }

    pub fn compose(&mut self) -> Result<u64, Error> {
        let duration_ms =
            get_most_recent_duration_ms(self.settings.origin_system_time, self.state.duration_ms);
        compose_from_settings_and_state(&self.settings, &mut self.state, duration_ms)
    }
}

fn check_settings(settings: &Settings) -> Result<(), Error> {
    if settings.logical_volume_length == 0 {
        return Err(Error::LogicalVolumeModuloIsZero);
    }
    if MAX_LOGICAL_VOLUMES < (settings.logical_volume_base + settings.logical_volume_length) {
        return Err(Error::ExceededAvailableLogicalVolumes);
    }

    Ok(())
}

fn get_most_recent_duration_ms(origin_system_time: SystemTime, duration_ms: u64) -> u64 {
    match SystemTime::now().duration_since(origin_system_time) {
        // check time didn't go backward
        Ok(duration) => {
            let dur_ms = duration.as_millis() as u64;
            match duration_ms < dur_ms {
                true => dur_ms,
                _ => duration_ms,
            }
        }
        // yikes! time went backwards so use the most recent step
        _ => duration_ms,
    }
}

fn compose_from_settings_and_state(
    settings: &Settings,
    state: &mut State,
    duration_ms: u64,
) -> Result<u64, Error> {
    match state.duration_ms < duration_ms {
        true => modify_state_time_changed(state, settings.logical_volume_length, duration_ms),
        _ => {
            if let Err(err) =
                modify_state_time_did_not_change(state, settings.logical_volume_length)
            {
                return Err(err);
            };
        }
    }

    Ok(compose(
        duration_ms,
        settings.logical_volume_base + state.logical_volume,
        state.sequence,
    ))
}

fn modify_state_time_changed(state: &mut State, logical_volume_length: u64, duration_ms: u64) {
    state.duration_ms = duration_ms;
    state.sequence = 0;
    state.prev_logical_volume = state.logical_volume;
    state.logical_volume = (state.logical_volume + 1) % logical_volume_length;
}

fn modify_state_time_did_not_change(
    state: &mut State,
    logical_volume_length: u64,
) -> Result<(), Error> {
    state.sequence += 1;
    if state.sequence < MAX_SEQUENCES {
        return Ok(());
    }

    state.sequence = 0;

    let next_logical_volume = (state.logical_volume + 1) % logical_volume_length;
    if state.prev_logical_volume == next_logical_volume {
        // cycled through all sequences on all available logical shards
        return Err(Error::ExceededAvailableSequences);
    }

    state.logical_volume = next_logical_volume;
    Ok(())
}
