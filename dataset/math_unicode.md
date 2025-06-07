# About math_unicode.csv
This dataset, in csv format, provides alias-to-unicode mappings for math symbols & operators.

This dataset is based on https://github.com/ViktorQvarfordt/unicode-latex, which provides a mapping between latex commands and unicode characters in JSON format. 

Each alias is derived from the corresponding latex command based on the following set of rules. These rules are automated in `math.ipynb`.

### Rules
- All backslashes are removed
- All "math" prefixes are removed (ex. mathbb → bb)
- Brackets are replaced by underscores (ex. mathbb{C} → bb_C)