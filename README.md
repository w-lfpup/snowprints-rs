# Snowprints-rs

Create unique and sortable ids.

## How to use

### Settings

Define a `Settings` struct.

The `logical_volume_base` property defines where to begin logical volume rotations. The `logical_volume_length` property defines how many logical volumes will be rotated.

```rust
use snowprints::Settings;

// THIS VALUE CANNOT BE CHANGED FOR THE ENTIRE SOFTWARE / SYSTEM LIFETIME
const JANUARY_1ST_2024_AS_MS: u64 = 1704096000000;

let settings = Settings {
    origin_time_ms: JANUARY_1ST_2024_AS_MS,
    logical_volume_base: 0,
    logical_volume_length: 8192,
};
```

### Compose

In the example below, `Snowprints` start on `2024 Jan 1st` and rotate through logical volumes `0-8191`.

```rust
use snowprints::Snowprints;

let mut snowprinter = match Snowprints::new(settings) {
    Ok(snow) => snow,
    _ => return println!("Settings are not valid!"),
};

let snowprint = match snowprinter.compose() {
    Ok(sp) => sp,
    _ => return println!("Exhausted all available logical volumes and sequences for the current millisecond!"),
};
```

### Decompose

To get values from a `snowprint` use the `decompose` function.

```rust
use snowprints::decompose;

let (timestamp_ms, logical_volume, sequence) = decompose(snowprint);
```

## What is a snowprint?

A `snowprint` is a [snowflake id](https://en.wikipedia.org/wiki/Snowflake_ID) variant based on this [article](https://instagram-engineering.com/sharding-ids-at-instagram-1cf5a71e5a5c).

I called them snowprints because this library creates a "sequential trail" of snowflake IDs across all available logical volumes.

## License

`Snowprints` is released under the BSD 3-Clause License.
