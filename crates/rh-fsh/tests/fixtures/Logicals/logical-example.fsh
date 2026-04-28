// @Name: Logical Example
// @Description: An example of Logical resources and how to use them

// A 'Logical' is neither a resource nor a datatype. It can be thought of
// as a data class.
Logical: AnimalAbilities
* hasLungs 0..1 boolean "Does the animal have lungs?"
* canSwim 0..1 boolean "Can the animal swim?"

// A Logical can be used in place of a "complex datatype"
Logical: Animal
* name 1..1 string "The animal's name"
* age 0..1 decimal "The animal's age in years"
* traits 1..1 AnimalAbilities "The animal's abilities"

// A Logical can be a Parent (inheritance)
Logical: Pet
Parent: Animal
* isVaccinated 0..1 boolean "Is the pet vaccinated?"

// A Logical can be like a Resource, but you have to add an id element and the 
// #can-be-target Characteristic, so that References work.
Logical: PetAsResourceLikeObject
Parent: Animal
Characteristics: #can-be-target
* id 0..1 string "The id"
* isVaccinated 0..1 boolean "Is the pet vaccinated?"

// Example of Reference from one Logical to another
Logical: Aquarium
* members 0..* Reference(PetAsResourceLikeObject) "The pets inside the aquarium"

// Instance examples:
Instance: ExamplePetPhil
InstanceOf: PetAsResourceLikeObject
* name = "Phil"
* age = 2
* traits.canSwim = true

Instance: ExamplePetClaire
InstanceOf: PetAsResourceLikeObject
* name = "Claire"
* age = 3
* traits.canSwim = false

Instance: ExampleAquarium
InstanceOf: Aquarium
* members[0] = Reference(ExamplePetPhil)
* members[1] = Reference(ExamplePetClaire)
