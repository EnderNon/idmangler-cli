# Config Guide

## The required blocks
According to the [Wynntils Artemis encoding proposal Github Issue](https://github.com/Wynntils/Wynntils/issues/2246),  
the allowed blocks should look like this:

![An image of the original encoding proposal table, present on GitHub](images/encodingproposaltable.png)

However, it's inaccurate. This is what it SHOULD look like:

 ##### Key to Type Mapping Table  

| Key | Type                     | Required blocks                            | Optional blocks                                             |
|-----|--------------------------|--------------------------------------------|-------------------------------------------------------------|
| `0` | Gear Item                | Name                                       | Identifications, Powders, Shiny, Reroll                     |
| `1` | Tome Item                | Name                                       | Identifications, Reroll                                     |
| `2` | Charm Item               | Name                                       | Identifications, Reroll                                     |
| `3` | Crafted Gear Item        | Custom Gear Type, Durability, Requirements | NameAfter, Damage, Defense, Custom Identifications, Powders |
| `4` | Crafted Consumable Item  | Custom Consumable Type, Uses, Requirements | Effects, NameAfter, Custom Identifications                  |
| `5` | Crafted Item from Recipe | TODO                                       | TODO                                                        |


## Documentation for each block:
- [Item type](blocks/type.md)
- [Custom Gear Type (NOT IMPLEMENTED YET)](blocks/customgeartype.md)
- [Custom Consumable Type (NOT IMPLEMENTED YET)](blocks/customconsumtype.md)
- [Durability (NOT IMPLEMENTED YET)](blocks/durability.md)
- [Requirements (NOT IMPLEMENTED YET)](blocks/requirements.md)
- [Name](blocks/name.md)
- [NameAfter (NOT IMPLEMENTED YET)](blocks/nameafter.md)
- [Shiny](blocks/shiny.md)
- [Identifications](blocks/identifications.md)
- [Damage (NOT IMPLEMENTED YET)](blocks/damage.md)
- [Defense (NOT IMPLEMENTED YET)](blocks/defense.md)
- [Effects (NOT IMPLEMENTED YET)](blocks/effects.md)
- [Powders](blocks/powders.md)
- [Custom Identifications (NOT IMPLEMENTED YET)](blocks/identifications.md)
- [Reroll](blocks/reroll.md)



# Current issues
Crafteds are not implemented