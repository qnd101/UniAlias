The `math_unicode` dataset includes ~2500 unicode characters that can be represented in Latex. 
It was created based on [ViktorQvarfordt/unicode-latex](https://github.com/ViktorQvarfordt/unicode-latex).

## Alias Rules
Each alias is derived from its corresponding Latex command following these rules.
- All backslashes(`\`) are removed
- All `math` prefixes are removed 
- Brackets are replaced with underscores(`_`)

For example, the Latex command of ℂ is `\mathbb{C}`. The derived alias is `bb_C`.

## Tips
When you want to know an alias of a symbol, one way is to search for its Latex command, then apply the rules above. 
Alternatively, you can open `unicode_math.csv` directly and search for the symbol you want. 
(I recommend opening it with a text editor. Excel may produce some unicode encoding issues.)

## Overview of `math_unicode`
`math_unicode` mainly covers symbols that appear in mathematics and science. It also includes a surprisingly wide range of non math-related characters.

1. Math Symbols
± (`pm`) 
× (`times`)
≥ (`ge`)
∴ (`therefore`) 
⟺ (`iff`) 
⊕ (`oplus`) 
∂ (`partial`) 
∏ (`prod`)
And many more...

</br>

2. Arrows
This dataset supports a vast range of arrows. 
← (`leftarrow`)
⇄ (`rightleftarrows`)
↺ (`acwopencirclearrow`)
↲ (`Ldsh`)
↝ (`rightwavearrow`)
⬷ (`twoheadleftdbkarrow`)
⬳ (`longleftsquigglearrow`)

<br/>

3. Multiline Brackets
One can create a matrix using 
⎡ (`lbrackuend`), 
⎢ (`lbrackextender`)
⎣ (`lbracklend`)
⎤ (`rbrackuend`), ...
```
Ex.
⎡ 1 2 3 4 ⎤
⎢ 5 6 7 8 ⎥
⎣ 9 0 1 2 ⎦
```

<br/>

4. Subscripts & Superscripts
Note that sub/superscripts are not fully supported by Unicode. Some alphabets do not have a corresponding sub/superscript character. 
₀ (`_0`)
⁹ (`^9`)
⁺ (`^+`)
ⁿ (`^n`)
⁽ (`^(` )

<br/>

5. Greek Alphabets
α (`alpha`)
π (`pi`)
Ω (`omega`)
Ξ (`xi`)

<br/>

6. Variant Letters
φ (`varphi`)
ε (`varepsilon`)
ℏ (`hslash`)
ℇ (`eulerconst`)

<br/>

7. Styled Letters (alphabets, greek alphabets, and numbers)

| Glyph | Code         |
|-------|--------------|
| 𝐀     | `bf_A`       |
| 𝐵     | `it_B`       |
| 𝑪     | `bm_C`       |
| 𝒟     | `cal_D`      |
| 𝓔     | `bm_cal_E`   |
| 𝔉     | `frak_F`     |
| 𝔾     | `bb_G`       |
| 𝕳     | `bm_frack_H` |
| 𝖨     | `ss_I`       |
| 𝗝     | `bm_ss_J`    |
| 𝘒     | `ssit_K`     |
| 𝙇     | `bm_ssit_L`  |
| 𝙼     | `tt_M`       |

<br/>

8. Miscellaneous Symbols 
¥ (`yen`)
♠ (`spadesuit`) 
♭ (`flat`)
§ (`section`)
▒ (`blockhalfshaded`)
♂ (`male`)
✓ (`checkmark`)
ð (`eth`)

<br/>

9. Shapes
▪ (`smblksquare`)
◾(`mdsmblksquare`)
◼ (`mdblksquare`)
■ (`mdlgblksquare`)
⬛ (`lgblksquare`)
⬜ (`lgwhtsquare`)
▥ (`squarevfill`)
⟐ (`diamondcdot`)
⬟ (`pentagonblack`)
▷ (`triangleright`)
⭐ (`medwhitestar`)
