# Powders
- This data block encodes for powders present on the item.
- Field name: `powders`
- Field type: `Array[type, tier, amount]`
- Field range: `0` to `255` powders total.
- ## Type
  - This is the type of powder that is present.
  - These values represent Earth, Thunder, Water, Fire, and Air.
  - Field name: `type`
  - Field type: `Char`
  - Field potential values: `E`,`T`,`W`,`F`,`A`
  - Field fallback value (if invalid): `T`
  - Example: `"type":"E"`
- ## Tier
  - This is the powder tier, although it really doesn't do anything as by default Wynntils only encodes to and accepts Tier 6.
  - Field name: `tier`
  - Field type: `Unsigned 8-bit Integer`
  - Field range: `1` to `6`
  - Example: `"tier":6`
- ## Amount (OPTIONAL)
  - For each entry in the `powders` array, this is the amount of powders encoded. 
  - It is optional, as it's just used to make encoding lots of the same powder easier.
  - For example, you could repeat `{"type":"E", "tier":6}` six times, or just do `{"type":"E", "tier":6, "amount":6}` and both would be the same.
  - Field name: `amount`
  - Field type: `Unsigned 8-bit Integer`
  - Field range: `1` to `255`
  - Field fallback value (if not provided): `1`
  - Example: `"amount":6`
## EXAMPLE of the Powders array
```
"powders": [
    {"type":"T", "tier":6, "amount":5},
    {"type":"e", "tier":1, "amount":5},
    {"type":"F", "tier":3, "amount":1},
    {"type":"w", "tier":6}
]
```
## Which items can have powders?
Powders can only be encoded on an item that originally supported powders in the first place.  
Unfortunately you can't add powders to an item that didn't originally have them.  
e.g. can't put powder on depressing shears, as depressing weapons don't have powder slot.
