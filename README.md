# Notepad TabState parser
This is a parser for Windows 11 `TabState` artifact. This project is a library and a parser.
You can read more about the artifact structure in my blog: https://u0041.co/posts/articals/exploring-windows-artifacts-notepad-files/

## Install
You can install the parsers in three ways:
### Cargo
Run the following command:
```bash
cargo install notepad_parser
```

### Release
Download the latest precomiled version from release

### From source
Clone and build from source by executing the following commands:
```bash
git clone https://github.com/AbdulRhmanAlfaifi/notepad_parser
cd notepad_parser
cargo build --release
target\release\notepad_parser.exe
```

## Usage
The following is the help message for the parser:
```bash
Created By: AbdulRhman Alfaifi <aalfaifi@u0041.co>
Version: v0.1.0
Reference: https://u0041.co/posts/articals/exploring-windows-artifacts-notepad-files/

Notepad TabState file parser

Usage: notepad_parser.exe [OPTIONS] [FILE]

Arguments:
  [FILE]  Path the files to parse. Accepts glob. [default: C:\Users\*\AppData\Local\Packages\Microsoft.WindowsNotepad_8wekyb3d8bbwe\LocalState\TabState\????????-????-????-????-????????????.bin]

Options:
  -f, --output-format <FORMAT>  Specifiy the output format [default: jsonl] [possible values: jsonl, csv]
  -o, --output-path <FILE>      Specifiy the output file [default: stdout]
  -l, --log-level <LEVEL>       Level for logs [default: quiet] [possible values: trace, debug, info, error, quiet]
  -h, --help                    Print help
  -V, --version                 Print version
```

## Example output
### Doesn't Contains Unsaved Chunks
```json
{
  "tabstate_path": "C:\\Users\\u0041\\AppData\\Local\\Packages\\Microsoft.WindowsNotepad_8wekyb3d8bbwe\\LocalState\\TabState\\79f851b1-e2d3-45ad-82d4-b69c87c40eeb.bin",
  "seq_number": 0,
  "is_saved_file": true,
  "path_size": 25,
  "path": "C:\\Windows\\Temp\\u0041.txt",
  "file_size": 25,
  "encoding": "UTF8",
  "cr_type": "CRLF",
  "last_write_time": "2024-08-16T20:49:42Z",
  "file_hash": "0039C19E2071A4BD7D355CE381B218966A12016EA11FCACB34C3A3F0A6E5D385",
  "cursor_start": 25,
  "cursor_end": 25,
  "config_block": {
    "word_wrap": true,
    "rtl": false,
    "show_unicode": false,
    "version": 2,
    "unknown0": 1,
    "unknown1": 1
  },
  "file_content_size": 25,
  "file_content": "This is a test file saved",
  "contain_unsaved_data": false,
  "checksum": "A49DA5D2"
}
```
### Contain Unsaved Chunks
```json
{
  "seq_number": 0,
  "is_saved_file": true,
  "path_size": 24,
  "path": "C:\\Windows\\Temp\\test.txt",
  "file_size": 32,
  "encoding": "UTF8",
  "cr_type": "CRLF",
  "last_write_time": "2024-08-08T22:18:57Z",
  "file_hash": "C60D8FFBD2FF969A36BFFCA31F609E801E8E0B8DE41568E948DBEBAC1BD9B2E4",
  "cursor_start": 31,
  "cursor_end": 31,
  "config_block": {
    "word_wrap": true,
    "rtl": false,
    "show_unicode": false,
    "version": 2,
    "unknown0": 1,
    "unknown1": 1
  },
  "file_content_size": 31,
  "file_content": "File saved test\rFile saved test",
  "contain_unsaved_data": false,
  "checksum": "F44C93E7",
  "unsaved_chunks": [
    {
      "position": 31,
      "num_of_deletion": 0,
      "num_of_addition": 1,
      "data": "\r",
      "checksum": "90FEE334"
    },
    {
      "position": 32,
      "num_of_deletion": 0,
      "num_of_addition": 1,
      "data": "t",
      "checksum": "4D720EDC"
    },
    {
      "position": 33,
      "num_of_deletion": 0,
      "num_of_addition": 1,
      "data": "h",
      "checksum": "96657A31"
    },
    {
      "position": 34,
      "num_of_deletion": 0,
      "num_of_addition": 1,
      "data": "i",
      "checksum": "C8DE31A0"
    },
    {
      "position": 35,
      "num_of_deletion": 0,
      "num_of_addition": 1,
      "data": "s",
      "checksum": "4593E2CB"
    },
    {
      "position": 36,
      "num_of_deletion": 0,
      "num_of_addition": 1,
      "data": " ",
      "checksum": "6625304C"
    },
    {
      "position": 37,
      "num_of_deletion": 0,
      "num_of_addition": 1,
      "data": "a",
      "checksum": "B22767B8"
    },
    {
      "position": 38,
      "num_of_deletion": 0,
      "num_of_addition": 1,
      "data": " ",
      "checksum": "1CE5632C"
    },
    {
      "position": 38,
      "num_of_deletion": 1,
      "num_of_addition": 0,
      "checksum": "DA9AD201"
    },
    {
      "position": 37,
      "num_of_deletion": 1,
      "num_of_addition": 0,
      "checksum": "D8DC6C58"
    },
    {
      "position": 37,
      "num_of_deletion": 0,
      "num_of_addition": 1,
      "data": "i",
      "checksum": "7AFEEDB0"
    },
    {
      "position": 38,
      "num_of_deletion": 0,
      "num_of_addition": 1,
      "data": "s",
      "checksum": "8D736DBB"
    },
    {
      "position": 39,
      "num_of_deletion": 0,
      "num_of_addition": 1,
      "data": " ",
      "checksum": "21854A9C"
    },
    {
      "position": 40,
      "num_of_deletion": 0,
      "num_of_addition": 1,
      "data": "u",
      "checksum": "6419745C"
    },
    {
      "position": 41,
      "num_of_deletion": 0,
      "num_of_addition": 1,
      "data": "n",
      "checksum": "F04F9676"
    },
    {
      "position": 42,
      "num_of_deletion": 0,
      "num_of_addition": 1,
      "data": "s",
      "checksum": "488380BA"
    },
    {
      "position": 43,
      "num_of_deletion": 0,
      "num_of_addition": 1,
      "data": "a",
      "checksum": "0D17D9D9"
    },
    {
      "position": 44,
      "num_of_deletion": 0,
      "num_of_addition": 1,
      "data": "v",
      "checksum": "BAB4815F"
    },
    {
      "position": 45,
      "num_of_deletion": 0,
      "num_of_addition": 1,
      "data": "e",
      "checksum": "E63BE97D"
    },
    {
      "position": 46,
      "num_of_deletion": 0,
      "num_of_addition": 1,
      "data": "d",
      "checksum": "B880A2EC"
    }
  ],
  "unsaved_chunks_str": "[31]:\rthis a <DEL:38><DEL:37>is unsaved"
}
```