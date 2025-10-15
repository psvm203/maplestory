[![Crates.io Version](https://img.shields.io/crates/v/maplestory?link=https%3A%2F%2Fcrates.io%2Fcrates%2Fmaplestory)](https://crates.io/crates/maplestory)
[![docs.rs](https://img.shields.io/docsrs/maplestory?link=https%3A%2F%2Fdocs.rs%2Fmaplestory)](https://docs.rs/maplestory)

# maplestory
easy-to-use MapleStory API wrapper for Rust

## Regional Support

Currently, this crate is available on [KMS](https://maplestory.nexon.com/) and [MSEA](https://www.maplesea.com/).  \
Other regions like [GMS](https://www.nexon.com/maplestory/) will be supported once Nexon provides official APIs for those regions.

## Example

You can use other asynchronous crates, however, in this async example, we will use [Tokio](https://tokio.rs/).

```console
cargo add maplestory
cargo add tokio --features full
```

So your `Cargo.toml` would look like:

```toml
[dependencies]
maplestory = { version = "1.0.2" }
tokio = { version = "1.48.0", features = ["full"] }
```

Then, on your `main.rs`,

```rust
use maplestory::prelude::*;

#[tokio::main]
async fn main() {
    let api = MaplestoryApi::builder().region(Region::KMS).api_key("YOUR_API_KEY").build();
    let ocid = api.get_id("00월").await.unwrap().ocid;
    let character_level = api
        .get_character_basic(&ocid, None)
        .await
        .unwrap()
        .character_level;

    println!("{character_level}");
}
```

![level](./assets/level.png)

This example prints 281, my character level (which has probably increased since I wrote this README).

## MaplestoryApi
```rust
struct MaplestoryApi {
    region: Region,
    api_key: String,
    origin: String,
}
```

`MaplestoryApi` consists of 3 fields:
- `region`: Represents the region such as KMS, MSEA. Default is `Region::KMS`.
- `api_key`(required): Can be obtained from https://openapi.nexon.com/.
- `origin`: Default is "https://open.api.nexon.com". Or you can use any proxy server instead. For an example, visit [nexon-open-api-proxy](https://github.com/psvm203/nexon-open-api-proxy).

## Schemas

For schema descriptions, please refer to [KMS Docs](https://openapi.nexon.com/game/maplestory/) or [MSEA Docs](https://openapi.nexon.com/game/maplestorysea/).  \
When the official documentation differs from the actual API response structure, schemas are based on the actual API responses.

Since API responses are in JSON format, their values are nullable.  \
However, making all Rust fields optional would be terrible.  \
So instead, through multiple attempts, only some fields are set as optional.

Due to this, errors may occur when actual response values, which are null, are received as non-optional values.  \
In such cases, please report them through [Issues](https://github.com/psvm203/nexon-open-api-proxy/issues).

For example, the structure of the `CharacterBasic` schema is as follows:

```rust
struct CharacterBasic {
    date: Option<String>,
    character_name: String,
    world_name: String,
    character_gender: String,
    character_class: String,
    character_class_level: String,
    character_level: u32,
    character_exp: u64,
    character_exp_rate: String,
    character_guild_name: Option<String>,
    character_image: String,
    character_date_create: String,
    access_flag: String,
    liberation_quest_clear: String,
}
```

When user does not provide optional date parameter, `date` field becomes null.  \
When character not joined to a guild, `character_guild_name` field becomes null.  \
Other fields are not null in any case.

## API

|API|Description|KMS|MSEA|
|-|-|:-:|:-:|
|get_character_list|Retrieve list of characters in account|O|X|
|get_user_achievement|Retrieve user achievement information|O|X|
|get_id|Retrieve character identifier (ocid)|O|O|
|get_character_basic|Retrieve basic character information|O|O|
|get_character_popularity|Retrieve popularity information|O|O|
|get_character_stat|Retrieve comprehensive stats information|O|O|
|get_character_hyperstat|Retrieve Hyper Stat information|O|O|
|get_character_propensity|Retrieve traits information|O|O|
|get_character_ability|Retrieve Ability information|O|O|
|get_character_item_equipment|Retrieve equipped equipment information (excluding cash items)|O|O|
|get_character_cashitem_equipment|Retrieve equipped cash item information|O|O|
|get_character_symbol_equipment|Retrieve equipped symbol information|O|O|
|get_character_set_effect|Retrieve information about equipped set item effects|O|O|
|get_character_beauty_equipment|Retrieve equipped hair, face, and skin information|O|O|
|get_character_android_equipment|Retrieve equipped andriod information|O|O|
|get_character_pet_equipment|Retrieve equipped pet information|O|O|
|get_character_skill|Retrieve skill information|O|O|
|get_character_link_skill|Retrieve equipped Link Skill information|O|O|
|get_character_vmatrix|Retrieve V Matrix information|O|O|
|get_character_hexamatrix|Retrieve HEXA Node information|O|O|
|get_character_hexamatrix_stat|Retrieve HEXA stats information|O|O|
|get_character_dojang|Retrieve Mu Lung Dojo highest record information|O|O|
|get_character_other_stat|Retrieve stats information difficult to verify in other api|O|X|
|get_character_ring_exchange_skill_equipment|Retrieve ring exchange skill equipment|O|X|
|get_user_union|Retrieve Union information|O|O|
|get_user_union_raider|Retrieve Union Raider information|O|O|
|get_user_union_artifact|Retrieve Union Artifact information|O|O|
|get_user_union_champion|Retrieve Union Champion information|O|X|
|get_guild_id|Retrieve guild identifier (oguild_id) information|O|O|
|get_guild_basic|Retrieve basic information|O|O|
|get_ouid|Retrieve user identifier (ouid)|O|X|
|get_history_starforce|Retrieve results of Star Force|O|X|
|get_history_potential|Retrieve results of Potential Reset|O|X|
|get_history_cube|Retrieve results of Cube|O|X|
|get_ranking_overall|Retrieve Overall ranking information|O|X|
|get_ranking_union|Retrieve Union information|O|X|
|get_ranking_guild|Retrieve Guild ranking information|O|X|
|get_ranking_dojang|Retrieve Mu Lung Dojo ranking information|O|X|
|get_ranking_theseed|Retrieve Tower of Oz ranking information|O|X|
|get_ranking_achievement|Retrieve Achievement ranking information|O|X|
|get_notice|Retrieve recently registered Maplestory Generals|O|X|
|get_notice_detail|Retrieve Maplestory General details|O|X|
|get_notice_update|Retrieve recently registered Maplestory Updates|O|X|
|get_notice_update_detail|Retrieve Maplestory Update details|O|X|
|get_notice_event|Retrieve recently registered Maplestory Events|O|X|
|get_notice_event_detail|Retrieve Maplestory Event details|O|X|
|get_notice_cashshop|Retrieve recently registered Maplestory Sales|O|X|
|get_notice_cashshop_detail|Retrieve Maplestory Sale details|O|X|

## Errors

|Error code ([Official Docs](https://openapi.nexon.com/guide/request-api/))|Enum (`ApiError`)|Description|
|:-:|:-:|:-:|
|OPENAPI00001|InternalServerError|Internal server error|
|OPENAPI00002|UnauthorizedAccess|Unauthorized access|
|OPENAPI00003|InvalidIdentifier|Invalid identifier|
|OPENAPI00004|InvalidParameter|Missing or invalid parameter|
|OPENAPI00005|InvalidApiKey|Invalid API key|
|OPENAPI00006|InvalidPath|Invalid game or API path|
|OPENAPI00007|LimitExceeded|API call limit exceeded|
|OPENAPI00009|DataBeingPrepared|Data being prepared|
|OPENAPI00010|ServiceUnderMaintenance|Service under maintenance|
|OPENAPI00011|ApiUnderMaintenance|API under maintenance|
|-|SendRequestError|error while sending request|
|-|ParseError|response body is not in JSON format, or it cannot be properly deserialized|
|-|NotSupported|API is not supported for region|

## License
This project is licensed under the [MIT license](./LICENSE).
