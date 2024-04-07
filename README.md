bhg is a simple block-header-generator written in Rust

# Download

You can either download the source code and compile the program yourself:

1. [Download the Source Code](https://github.com/antonio-subasic/bhg/archive/refs/heads/main.zip)
1. Unzip the folder
1. Run `cargo build --release`
1. Copy the executable from `target/release/` to `/usr/local/bin/`: `sudo cp target/release/bhg /usr/local/bin/bhg`

Or you can download the precompiled binary:

1. [Download the binary](https://github.com/antonio-subasic/bhg/releases)
1. Move the binary to `/usr/local/bin/`: `sudo cp bhg /usr/local/bin/bhg`

# Setup

1. Create the `bhg` directory in the `.config` directory: `mkdir -p ~/.config/bhg`
1. Inside the directory create a
    - `variables.json`, which contains constants that can be used as variables in the block-header
    - `base.<file extension>`, foreach each file type. Lines you want to be centered prefix with "->" <br> e.g.
        - `bhg output.c` will access the `~/.config/bhg/base.c` file
        - `bhg output.cpp` will access the `~/.config/bhg/base.cpp` file

# Usage

## Command

`bhg <output file> <description>`

## Special Variables

|   Variable   |                   Value source                    |
| :----------: | :-----------------------------------------------: |
| description  |          passed as command line argument          |
|     year     |                 the current year                  |
|    month     |                 the current month                 |
|     day      |                  the current day                  |
|     hour     |                 the current hour                  |
|    minute    |                the current minute                 |
|    second    |                the current second                 |
| microsecond  |              the current microsecond              |
|   weekday    | the current weekday (monday = 1, ..., sunday = 7) |
| weekdayName  |             the current weekday name              |
|  monthName   |              the current month name               |
|  timestamp   |               the current timestamp               |
| timestampMs  |       the current timestamp in milliseconds       |
| timestampISO |        the current timestamp in ISO format        |

## Example

`bhg output.c This is a description`

### ~/.config/bhg/variables.json

```json
{
    "name": "John Doe",
    "email": "john.doe@gmail.com"
}
```

### ~/.config/bhg/base.c

```c
/*
-------------------------------------------------------------
->ABC Company
-------------------------------------------------------------
->{name}, {day}-{month}-{year}
-------------------------------------------------------------
->{email}, {description}
-------------------------------------------------------------
This line won't be centered {weekdayName}
-------------------------------------------------------------
*/
```

### output.c

```c
/*
-------------------------------------------------------------
                         ABC Company
-------------------------------------------------------------
                     John Doe, 31-12-2023
-------------------------------------------------------------
          john.doe@gmail.com, This is a description
-------------------------------------------------------------
This line won't be centered Sunday
-------------------------------------------------------------
*/
```
