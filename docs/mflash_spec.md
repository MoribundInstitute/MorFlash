# MFLASH Deck File Format Specification (v1)

**Extension:** `.mflash`  
**Role:** Portable MorFlash deck (cards + SRS + optional media)  
**Reference implementation:** `src/srs/mflash.rs`

---

## 1. Overview

A `.mflash` file is a **ZIP-based container** that packs:

- High-level deck metadata (`manifest.json`)
- Deck content and SRS state (`deck.sqlite`)
- Optional media files (`media/*`)

Design goals:

- Human-inspectable using standard tools (`unzip`, SQLite browser).
- Backwards-/forwards-friendly via explicit versioning.
- Captures both **static data** (cards) and **dynamic state** (review schedule).

---

## 2. Container Layout

A `.mflash` is a ZIP archive.

For **version 1**, the container **must** contain:

- `manifest.json` – JSON manifest describing the deck.
- `deck.sqlite` – SQLite database with deck, cards, review state.

It **may** contain:

- `media/` – directory of media files (images, audio, video, etc.).
- `thumbnail.png` – deck thumbnail (convention only in v1, not required).
- `theme.toml` – potential deck theme/config (not used by v1 core).

## Example `.mflash` Directory Tree

```text
my_deck.mflash
├── manifest.json
├── deck.sqlite
└── media/
    ├── 0001.png
    ├── 0002.mp3
    └── cover.webp

## 3. Manifest (manifest.json)

The manifest is a UTF-8 JSON file that allows tools to inspect decks
without opening the SQLite DB.

It corresponds to the Rust struct Manifest in src/srs/mflash.rs.

## 3.1 Fields
###Field	Type	Required	Description
format	string	yes	Container identifier. Must be "morflash.mflash" in v1.
version	integer	yes	Container version. For v1, this is 1.
deck_id	integer	yes	Primary key of the deck row in deck.sqlite.
name	string	yes	Human-readable deck name.
description	string	no	Optional deck description.
tags	array<string>	no	Tag list (e.g., ["vocabulary", "JLPT"]).
lang_front	string	no	Language code/name for front side.
lang_back	string	no	Language code/name for back side.
card_count	integer	yes	Number of cards in the deck.
created_at_utc	RFC3339 string	yes	Deck creation time (UTC).
updated_at_utc	RFC3339 string	yes	Last updated time (UTC).
has_thumbnail	bool	no	True if a thumbnail image is present.
has_deck_media	bool	no	True if deck-level media is included.
min_core_version	string	no	Minimum MorFlash core version needed to open this deck.
generator	string	no	Name/version of the generating tool.
3.2 Example Manifest
{
  "format": "morflash.mflash",
  "version": 1,
  "deck_id": 1,
  "name": "Sample Deck",
  "description": "Example MorFlash deck.",
  "tags": ["vocabulary", "demo"],
  "lang_front": "en",
  "lang_back": "en",
  "card_count": 42,
  "created_at_utc": "2025-12-01T12:34:56Z",
  "updated_at_utc": "2025-12-01T12:34:56Z",
  "has_thumbnail": false,
  "has_deck_media": false,
  "min_core_version": "0.1.0",
  "generator": "MorFlash Deck Builder 0.1.0"
}

4. Deck Database (deck.sqlite)

deck.sqlite is a SQLite 3 database that holds:

deck metadata

flashcard content

media metadata

SRS review state

The v1 schema is defined by SCHEMA_SQL in src/srs/mflash.rs.

4.1 Tables
4.1.1 meta

Generic key–value metadata.

Column	Type	Notes
key	TEXT PRIMARY KEY	
value	TEXT NOT NULL	

Recommended keys:

schema_version (e.g., "1")

created_at_utc

updated_at_utc

generator

4.1.2 deck

Single row describing the deck.

Column	Type	Notes
id	INTEGER PRIMARY KEY	
name	TEXT NOT NULL	
description	TEXT DEFAULT ''	
tags	TEXT DEFAULT ''	comma-separated tags
lang_front	TEXT DEFAULT ''	
lang_back	TEXT DEFAULT ''	
4.1.3 card

One row per flashcard.

Column	Type	Notes
id	INTEGER PRIMARY KEY	
deck_id	INTEGER NOT NULL	FK → deck.id
term	TEXT NOT NULL	
definition	TEXT NOT NULL	
example	TEXT DEFAULT ''	
notes	TEXT DEFAULT ''	
hyperlink	TEXT DEFAULT ''	
sort_order	INTEGER NOT NULL DEFAULT 0	
extra_json	TEXT DEFAULT ''	freeform extension
4.1.4 media

Maps media files to cards or to the deck as a whole.

Column	Type	Notes
id	INTEGER PRIMARY KEY	
file_name	TEXT NOT NULL	filename inside media/
kind	TEXT NOT NULL	"image", "audio", "video", etc.
mime_type	TEXT NOT NULL	
card_id	INTEGER	FK → card.id
deck_wide	INTEGER NOT NULL DEFAULT 0	1 = deck-level
alt_text	TEXT DEFAULT ''	
caption	TEXT DEFAULT ''	

Convention:

deck_wide = 1 and card_id IS NULL → deck-wide media

deck_wide = 0 and card_id NOT NULL → per-card media

4.1.5 review_state

Per-card spaced repetition properties.

Column	Type	Description
card_id	INTEGER PRIMARY KEY	FK → card.id
due_utc	TEXT NOT NULL	next review timestamp
interval_days	REAL NOT NULL	
ease_factor	REAL NOT NULL	
reps	INTEGER NOT NULL	
lapses	INTEGER NOT NULL	
last_review_utc	TEXT NOT NULL	last review timestamp
4.2 Indexes

idx_card_deck on card(deck_id, sort_order)

idx_media_card on media(card_id)

idx_media_deckwide on media(deck_wide)

idx_review_due on review_state(due_utc)

5. Media Files (media/)

All embedded media lives under the media/ directory in the ZIP.

Example paths:

media/0001.png

media/sfx01.ogg

The database stores only:

"0001.png"


Not:

"media/0001.png"


A client typically:

Queries the media table.

For each record: opens media/{file_name} inside the ZIP.

Uses mime_type and kind to decide how to render it.

6. Versioning & Compatibility

Three independent version indicators exist:

Container version: manifest.version (current: 1)

Container format tag: manifest.format = "morflash.mflash"

DB schema version: meta.schema_version (current: "1")

6.1 Container Version & Format Rules

The v1 implementation:

Rejects if format != "morflash.mflash".

Rejects if version != 1.

Future implementations may:

Accept older versions with shims.

Gracefully handle newer versions (e.g., warning + partial load).

6.2 Core Compatibility

manifest.min_core_version tells MorFlash whether the file is safe to load.
A UI may warn the user if the file requires a newer core version.

6.3 Schema Version

meta.schema_version allows the SQLite layout to evolve independent of ZIP layout.

7. Reference Implementation (Rust)
7.1 Exporting

export_deck_to_mflash(deck, cards, states, output_path):

Creates in-memory SQLite DB (create_empty_deck_db)

Writes deck, card, review_state tables (populate_deck_db)

Builds and serializes manifest.json

Copies DB to a temp disk file via SQLite backup API

Creates ZIP file with:

manifest.json

deck.sqlite

(future) media/*

Removes temp DB file

7.2 Importing

open_mflash(path):

Opens ZIP archive

Reads + parses manifest.json

Validates container format + version

Extracts deck.sqlite into temp file

Opens SQLite connection to that temp file

Scans archive for media/* and collects filenames

Returns MflashDeck with:

manifest

deck_path

media_files

conn

8. Future Extensions

Add thumbnail.png at top level

Add theme.toml for deck appearance

Add more tables (styling, extra sides, cloze, etc.)

Add fields such as:

author

license

homepage / source_url

difficulty metadata

Add optional compression hints for DB or media