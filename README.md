duckdb-warc
===========

duckdb-warc is a DuckDB extension for reading web archive files in WARC format. It is written in Rust and is based on the [experimental Rust extension template](https://github.com/duckdb/extension-template-rs).

## Example

The extension includes the table function `read_warc`, which can parse well-formed WARC files like so:

```sql
-- First, load the extension
LOAD 'duckdb_warc.duckdb_extension';

-- Retrieve all WARC header fields and body
SELECT * FROM read_warc('archive.warc');

-- Retrieve selected WARC header fields
SELECT record_id, content_length, target_uri, body
FROM read_warc('archive.warc');
```
