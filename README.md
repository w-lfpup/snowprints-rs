# Snowprints-rs

Create unique and sortable ids.

## How to use

### Install

Snowprints-rs is available on [crates.io](https://crates.io/crates/snowprints/)

### Settings

Define a `Settings` struct.

The `logical_volume_base` property defines where to begin logical volume rotations. The `logical_volume_length` property defines how many logical volumes will be rotated.

```rust
use snowprints::Params;

// THIS VALUE CANNOT BE CHANGED FOR THE ENTIRE SOFTWARE / SYSTEM LIFETIME
const JANUARY_1ST_2024_AS_MS: u64 = 1704096000000;

let params = Params {
    origin_time_ms: JANUARY_1ST_2024_AS_MS,
    logical_volume_base: 0,
    logical_volume_length: 8192,
};
```

### Compose

In the example below, `Snowprints` start on `2024 Jan 1st` and rotate through logical volumes `0-8191`.

```rust
use snowprints::Snowprints;

let mut snowprints = match Snowprints::from(params) {
    Ok(snow) => snow,
    Err(e) => return println!("{}", e),
};

// create a unique id
let snowflake_id = match snowprints.create_id() {
    Ok(sp) => sp,
    Err(e) => return println!("{}", e),
};

// get the current timestamp
let timestamp = snowprints.get_timestamp();

// get a shifted timestamp (for searching / indexing);
let offset_ms = 5;
let bit_shifted_timestamp = snowprints.get_bit_shifted_timestamp(offset_ms);
```

### Decompose

To pul values from a `snowprint` use the `decompose` function.

```rust
use snowprints::decompose;

let (timestamp_ms, logical_volume, sequence) = decompose(snowprint);
```

## What is a snowprint?

A `snowprint` is a [snowflake id](https://en.wikipedia.org/wiki/Snowflake_ID) variant based on this [article](https://instagram-engineering.com/sharding-ids-at-instagram-1cf5a71e5a5c).

They're called snowprints because this library creates a "sequential trail" of snowflake IDs across all available logical volumes. This evenly distributes entries across a shardable real estate.

## License

`Snowprints` is released under the BSD 3-Clause License.
