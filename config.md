# Values Guide
## Name
**Name** is stored as a String. It must be a valid Wynncraft item for it to display as intended.
## Shiny ID
Inside the Shiny{} list, there will be two elements: "key" and "value".  
The potential keys for any shiny value are listed here: https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Data-Storage/shiny_stats.json  
The value is the shown amount.
## Item type
WIP. This will support crafteds in the future, but for now you must have this:
```
  "item_type": "Gear"
```

## Powders

### Powder Limit
Powder limit cannot be increased beyond 255. This is because the powder count is stored as a byte.  
Potential value range: 0<->255.

### Format
Inside the "powders":[  ] array, set it up in the following structure.  
**{ "type":"$TYPE", "tier":$TIER, "amount": $AMOUNT }**  
**TYPE** is stored as a single character.  
Potential Values: "E" "T" "W" "F" "A" .  These represent Earth, Thunder, Wind, Fire, Air.  
**TIER** is stored as a single digit integer.  
Potential value range: 1<->6.  
**AMOUNT** is stored as an integer. It is optional. If not provided it falls back to 1.  
Potential value range: 1<->255.
#### Other things about powder format:
Each value in the array must have a comma at the end except the last.  
The use of spaces is optional, as well as letter case for the $TYPE value.  
The keys ("type" "tier" "amount") must All be lowercase.  
If type is invalid, it will default back to being Thunder powder.
#### Powders Example (yes, this looks wacky to show that this is allowed)
```json
"powders": [
    { "type":"T", "tier":6, "amount":5 },
    {"type"  :"e","tier":1,"amount":5},
    {"type":  "F", "tier": 3,"amount":1},
    {     "type"     : "w"     ,  "tier":6     }
]
```
Note that the last powder block in array has no comma at the end.
### Which items can have powders?
Powders can only be encoded on an item that originally supported powders in the first place.  
Unfortunately you can't add powders to an item that didn't originally have them.  
e.g. can't put powder on depressing shears, as depressing weapons don't have powder slot.

## Identifications
### Format
Inside the "ids":[] array, set it up in the following structure.  
**{"id": "$ID","base": $BASE,"roll": $ROLL}**  
**ID** is stored as a string corresponding to the Wynntils internal ID string of any roll. See here for a list: https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Reference/id_keys.json .  
**BASE** is the base roll. The default base data is defined in (WARNING: EXTREMELY MASSIVE TEXT FILE) https://raw.githubusercontent.com/Wynntils/Static-Storage/main/Reference/gear.json .  
Beautify it then look for `ITEMNAME > identifications > IDENTIFICATIONNAME > raw` for default value. This base value defines the listed ID ranges.  
**ROLL** defines the actual rolled value for the Identification. The formula is `BASE * ROLL / 100`. Most values are in the range 30-130 due to how Wynncraft handles many Identifications as 30-130 percent of a base stat. Thus, if you are trying to find the ROLL value, try your desired roll **(NOT the roll percentage 1-100)** divided by the BASE then round it to the nearest integer. This value is optional, and you should only exclude it when it is a fixed value e.g. Skill Points.

## Rerolls
Optional single value, i8. Stores number of rerolls. If missing or is 0, rerolls are not encoded.  
Potential range: 0<->255.

### Other things about this
The format obeys the same json rules as the powders.  
If you are trying to get the 100% or 0% roll and the value you calculated is close to 130 or 30, change it to 130 or 30 because that is the real value.
However, if the value is negative it will be between 70%-130%. 

# Current issues
Crafteds are not implemented