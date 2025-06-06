// Copyright 2025 RisingWave Labs
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use risingwave_common::types::Fields;
use risingwave_frontend_macro::system_catalog;

/// The view `pg_sequences` provides access to useful information about each sequence in the database.
/// Ref: [`https://www.postgresql.org/docs/current/view-pg-sequences.html`]
#[system_catalog(view, "pg_catalog.pg_sequences")]
#[derive(Fields)]
struct PgSequences {
    schemaname: String,
    sequencename: String,
    sequenceowner: String,
    increment_by: i64,
    last_value: i64,
    cycle: bool,
    start_value: i64,
    max_value: i64,
    min_value: i64,
}
