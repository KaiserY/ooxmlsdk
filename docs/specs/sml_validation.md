# SpreadsheetML Data Validation — ooxmlsdk Clean-Room Spec

**Source authority:** ECMA-376 5th edition Part 1 §18.3.1.32 (dataValidations); ISO/IEC 29500:2016 Part 1; XSD `sml.xsd`.

---

## 1. Overview

Data validation constrains what users may enter into a cell. A worksheet
carries one `<x:dataValidations>` block listing zero or more
`<x:dataValidation>` rules. Each rule binds a validation type
(whole-number, list, date, custom formula, etc.) to a range and optionally
attaches an input prompt and an error alert.

---

## 2. Worksheet Markup

### Position in worksheet child order

```
<x:worksheet>
  …
  <x:conditionalFormatting/>*
  <x:dataValidations count="…">    ← here, after conditionalFormatting
  …
</x:worksheet>
```

### dataValidations block

```xml
<x:dataValidations count="3">
  <x:dataValidation type="whole" operator="between"
                    allowBlank="1" showInputMessage="1" showErrorAlert="1"
                    errorTitle="Out of range" error="Enter 1–100."
                    promptTitle="Hint" prompt="Whole number 1 to 100."
                    sqref="A1:A10">
    <x:formula1>1</x:formula1>
    <x:formula2>100</x:formula2>
  </x:dataValidation>
  <x:dataValidation type="list" allowBlank="1" showDropDown="0"
                    sqref="B1:B10">
    <x:formula1>"red,green,blue"</x:formula1>
  </x:dataValidation>
  <x:dataValidation type="custom" errorStyle="warning"
                    sqref="C1">
    <x:formula1>LEN(C1)&lt;=10</x:formula1>
  </x:dataValidation>
</x:dataValidations>
```

`<x:dataValidations count>` must equal the child count.

---

## 3. dataValidation attributes

| Attr | Type | Default | Meaning |
|------|------|---------|---------|
| `type` | enum | `none` | Validation kind (table below) |
| `operator` | enum | `between` | Comparison operator |
| `allowBlank` | bool | 0 | True ⇒ empty cell is acceptable |
| `showDropDown` | bool | 0 | **List type only**, *inverted*: 1 ⇒ hide dropdown arrow (yes, the attribute name lies — see §6) |
| `showInputMessage` | bool | 0 | True ⇒ display `prompt` when cell selected |
| `showErrorAlert` | bool | 0 | True ⇒ alert when value invalid |
| `errorStyle` | enum | `stop` | `stop` (block), `warning` (yes/no), `information` (ok) |
| `errorTitle` | string | — | Alert dialog title |
| `error` | string | — | Alert dialog body |
| `promptTitle` | string | — | Input-message title |
| `prompt` | string | — | Input-message body |
| `imeMode` | enum | `noControl` | IME mode for East Asian input |
| `sqref` | space-separated ranges | required | Cells the rule applies to |

### `type` values

`none`, `whole`, `decimal`, `list`, `date`, `time`, `textLength`,
`custom`.

### `operator` values (per `type`)

`between`, `notBetween`, `equal`, `notEqual`, `lessThan`,
`lessThanOrEqual`, `greaterThan`, `greaterThanOrEqual`. Ignored for
`type="list"` and `type="custom"` (those use `formula1` only).

### `formula1` / `formula2`

| Validation type | `formula1` | `formula2` |
|----------------|------------|------------|
| `whole` / `decimal` / `date` / `time` / `textLength` (between/notBetween) | low bound | high bound |
| same types (other operators) | the comparison value | absent |
| `list` | comma-separated literals quoted (`"a,b,c"`) **or** a range (`$D$1:$D$5`) | absent |
| `custom` | Boolean expression evaluated per cell | absent |

`formula1` is mandatory for all types except `none`.

---

## 4. SDK Access Pattern

```rust
let ws: &Worksheet = sheet1.root(&package);
if let Some(validations) = ws.data_validations.as_ref() {
  for dv in &validations.data_validation {
    match dv.r#type {
      DataValidationValues::List => {
        let f1 = dv.formula1.as_ref()?;
        // …
      }
      DataValidationValues::Whole => {
        let lo = dv.formula1.as_ref()?;
        let hi = dv.formula2.as_ref()?;
        // …
      }
      _ => {}
    }
  }
}
```

---

## 5. List-type details

Two source forms:

1. **Inline literal list.** Single CSV string, *quoted with `"..."` outside
   the formula tokens*. Comma is the only separator. Whitespace inside the
   list is taken literally.

   ```xml
   <x:formula1>"red,green,blue"</x:formula1>
   ```

2. **Range reference.** Absolute reference to a column or row. Excel
   accepts only absolute references (`$A$1:$A$5`); relative or named
   ranges work in modern versions but older readers reject them.

   ```xml
   <x:formula1>$Sheet2.$A$1:$A$5</x:formula1>
   ```

A list with more than 32,767 characters in the inline form is rejected by
Excel. Use a range reference for large lists.

---

## 6. Round-Trip Gotchas

1. **`showDropDown`'s sense is inverted.** `showDropDown="1"` *hides* the
   dropdown arrow; `showDropDown="0"` (the default) *shows* it. ECMA-376
   keeps the attribute name for historical reasons. Do not "normalise" it
   to a positive sense.

2. **`<x:dataValidations count="N">` must match.** Like
   `conditionalFormatting/dxfs`, the count attribute is mandatory and
   must equal the number of `<x:dataValidation>` children.

3. **`sqref` uses spaces, not commas.** Same rule as `conditionalFormatting`.

4. **`<x:formula1>` for `list` carries embedded quotes.** The formula
   text *itself* is `"red,green,blue"` — the outer quotes are part of
   the formula string and must be preserved through XML escape/unescape.

5. **`type="none"` is valid and neutralises a rule** without removing
   its `sqref`. Some consumers use it to clear a previously-applied
   validation while preserving the cell selection. Round-trip must
   preserve `type="none"` literally rather than dropping the rule.

6. **`<x:formula1>` and `<x:formula2>` may contain XML-special
   characters that need escaping.** `<`, `>`, `&` in custom formulas
   round-trip via standard XML escape. Re-emitting with raw operators
   produces malformed XML.

7. **`promptTitle`, `prompt`, `errorTitle`, `error` are limited to
   32-byte and 255-byte ranges respectively.** Strings longer than the
   limit are silently truncated by Excel; ooxmlsdk preserves them
   verbatim.

8. **`imeMode` defaults to `noControl`.** Round-trip must not insert a
   default `imeMode="noControl"` on every rule.

---

## 7. Fixture Plan

| ID | File | Coverage |
|----|------|----------|
| SML-DV-01 | `test-data/spreadsheet/data_validation.xlsx` | Three `<x:dataValidation>` rules in one block: `type="whole" operator="between"` with `<x:formula1>1</x:formula1><x:formula2>100</x:formula2>` plus `errorTitle/error` and `promptTitle/prompt`; `type="list"` with inline `"red,green,blue"` literal; `type="custom" errorStyle="warning"` with a Boolean formula; `<x:dataValidations count="3">` |
