// @Name: Using Soft Indexing
// @Description: Parameterized rule set using soft indexing
RuleSet: Name(first, last)
* name[+].given = "{first}"
* name[=].family = "{last}"

Instance: MrSmith
InstanceOf: Patient
// some rules
* insert Name(Robert, Smith)
* insert Name(Rob, Smith)
* insert Name(Bob, Smythe)

/* When the rule set is expanded and soft indices are resolved, this is equivalent to:

Instance: MrSmith
InstanceOf: Patient
* name[0].given = "Robert"
* name[0].family = "Smith"
* name[1].given = "Rob"
* name[1].family = "Smith"
* name[2].given = "Bob"
* name[2].family = "Smythe"
*/