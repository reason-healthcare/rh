// @Name: HierarchicalCodes
// @Description: A code system that uses concept hierarchy (is-a relationships)

CodeSystem: AnimalCS
Id: animal-cs
Title: "Animal Code System"
Description: "A code system with hierarchical animal categories."

* #animal "Animal" "Any living creature"
  * #mammal "Mammal" "A warm-blooded vertebrate"
    * #dog "Dog" "Canis lupus familiaris"
    * #cat "Cat" "Felis catus"
  * #bird "Bird" "A feathered vertebrate"
    * #eagle "Eagle" "A large bird of prey"
