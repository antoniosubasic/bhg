[![Version](https://img.shields.io/crates/v/bhg)](https://crates.io/crates/bhg)
[![Downloads](https://img.shields.io/crates/d/bhg)](https://crates.io/crates/bhg)

a simple _Block Header Generator_

# Download

```bash
cargo install bhg
```

# Setup

1. Create the `bhg` directory in the `.config` directory: `mkdir -p ~/.config/bhg`
1. Inside the directory create a
    - `variables.json`, which contains constants that can be used as variables in the block-header
    - `base.<file extension>` - for each file type
        - `bhg output.c` will use the `~/.config/bhg/base.c` as a base file
        - `bhg output.rs` will use the `~/.config/bhg/base.rs` as a base file

# Usage

## Command

```bash
bhg <output file> <description>
```

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
| timestampMS  |       the current timestamp in milliseconds       |
| timestampISO |        the current timestamp in ISO format        |

## Example

```bash
bhg output.c This is a description
```

### variables file

```json
{
    "name": "John Doe",
    "email": "john.doe@gmail.com"
}
```

### base.c file - lines prefixed with `->` will be centered

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

### output.c file - generated in the current working directory

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
