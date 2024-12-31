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
- [Name](blocks/name.md)
- [Shiny](blocks/shiny.md)
- [Powder Limit](blocks/powderlimit.md)
- [Powders](blocks/powders)
- [Identifications](blocks/identifications.md)
- [Custom Identifications](blocks/identifications.md)
- [Reroll](blocks/reroll.md)



# Current issues
Crafteds are not implemented