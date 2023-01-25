# Entity Component System

## Components

A component is a container of data, it is the only way to create a persistent state in Yonscript.

A component could refer to another component, or entity. 

## Entity

Think of entities as a container of components of data. 

```
player = new entity                     // create a new empty entity
player[Health] = Health { max: 100 }     // add a Health component to the player entity
remove player<Health>                   // remove the Health component from the entity
```

*Under the hood they are just indexes!*

```
enum DamageType:
    Physical
    Magical
    True

component Health: 
    max: int

component Armor:
    armor: float

event Damage:
    receiver: entity
    damage: int 
    type: DamageType.Physical

handle event Damage:
    filter: receiver has Health 
    handler: (event) => {
        damage = event.damage
        if event.receiver has Armor 
            damage = damage - (damage * event.receiver<Armor>.armor)
        event.receiver<Health>.hp - event.damage
    }
```



