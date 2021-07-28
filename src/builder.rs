pub trait Builder<T, E> {
    /// fn with_t(&mut self, t: T) -> &mut Self {
    fn build(self) -> Result<T, E>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::BorrowMut;

    #[derive(Debug, PartialEq)]
    struct Person {
        pub name: String,
        pub birthdate: Option<String>,
        pub favorite_things: Vec<String>,
    }

    #[derive(Debug, PartialEq)]
    enum PersonBuildingError {
        RequiresName,
        RequiresFavoriteThingsList,
    }

    #[derive(Debug)]
    struct PersonBuilder {
        name: Option<String>,
        birthdate: Option<String>,
        favorite_things: Option<Vec<String>>,
    }

    impl PersonBuilder {
        fn new() -> Self {
            Self {
                name: None,
                birthdate: None,
                favorite_things: None
            }
        }

        fn with_name(&mut self, name: String) -> &mut Self {
            self.name = Some(name);
            self
        }

        fn with_birthdate(&mut self, birthdate: String) -> &mut Self {
            self.birthdate = Some(birthdate);
            self
        }

        fn with_favorite_thing(&mut self, favorite_thing: String) -> &mut Self {
            match self.favorite_things.borrow_mut() {
                Some(favorite_things) => favorite_things.push(favorite_thing),
                None => self.favorite_things = Some(vec![favorite_thing])
            };
            self
        }
    }

    impl Builder<Person, Vec<PersonBuildingError>> for PersonBuilder {
        fn build(self) -> Result<Person, Vec<PersonBuildingError>> {
            use PersonBuildingError::{RequiresFavoriteThingsList, RequiresName};

            match (self.name, self.favorite_things) {
                (None, None) => Err(vec![
                    RequiresName, 
                    RequiresFavoriteThingsList
                ]),
                (Some(_), None) => Err(vec![
                    RequiresFavoriteThingsList
                ]),
                (None, Some(_)) => Err(vec![
                    RequiresName
                ]),
                (Some(name), Some(favorite_things)) => {
                    Ok(Person {
                        name,
                        birthdate: self.birthdate,
                        favorite_things,
                    })
                },
            }
        }
    }

    #[test]
    fn builds_a_person_with_all_fields() {
        let mut person_builder = PersonBuilder::new();

        &person_builder.with_name(String::from("Evangivaldo"))
            .with_birthdate(String::from("25/12/1988"))
            .with_favorite_thing(String::from("Games"))
            .with_favorite_thing(String::from("Traveling"));


        let person_result = person_builder.build();
        
            
        assert_eq!(person_result.is_ok(), true);
        let person = person_result.unwrap();
        assert_eq!(person.name, "Evangivaldo");
        assert_eq!(person.birthdate, Some(String::from("25/12/1988")));
        assert_eq!(person.favorite_things[0], "Games");
        assert_eq!(person.favorite_things[1], "Traveling");
    }

    #[test]
    fn fails_building_person_with_no_fields() {
        let person_result = PersonBuilder::new()
            .build();
        
        assert_eq!(person_result.is_err(), true);
        let errors = person_result.unwrap_err();
        let has_error_requires_name = errors.contains(&PersonBuildingError::RequiresName);
        let has_error_requires_favorite_things = errors.contains(&PersonBuildingError::RequiresFavoriteThingsList);
        assert_eq!(has_error_requires_name, true);
        assert_eq!(has_error_requires_favorite_things, true);
    }

    #[test]
    fn fails_building_person_without_name() {
        let mut person_builder = PersonBuilder::new();

        &person_builder
            .with_birthdate(String::from("25/12/1988"))
            .with_favorite_thing(String::from("Games"));

        let person_result = person_builder.build();
            
        assert_eq!(person_result.is_err(), true);
        assert_eq!(person_result.unwrap_err(), vec![PersonBuildingError::RequiresName]);
    }

    #[test]
    fn builds_person_without_birthdate() {
        let mut person_builder = PersonBuilder::new();

        &person_builder
            .with_name(String::from("Evangivaldo"))
            .with_favorite_thing(String::from("Games"))
            .with_favorite_thing(String::from("Traveling"));

        let person_result = person_builder.build();
        
            
        assert_eq!(person_result.is_ok(), true);
        let person = person_result.unwrap();
        assert_eq!(person.name, "Evangivaldo");
        assert_eq!(person.birthdate, None);
        assert_eq!(person.favorite_things[0], "Games");
        assert_eq!(person.favorite_things[1], "Traveling");
    }

    #[test]
    fn fails_building_person_without_favorite_things() {
        let mut person_builder = PersonBuilder::new();

        &person_builder
            .with_name(String::from("Evangivaldo"))
            .with_birthdate(String::from("25/12/1988"));

        let person_result = person_builder.build();

        assert_eq!(person_result.is_err(), true);
        assert_eq!(person_result.unwrap_err(), vec![PersonBuildingError::RequiresFavoriteThingsList]);
    }

    #[test]
    fn fails_building_person_without_name_and_birthdate() {
        let mut person_builder = PersonBuilder::new();

        &person_builder
            .with_favorite_thing(String::from("Games"));

        let person_result = person_builder.build();

        assert_eq!(person_result.is_err(), true);
        assert_eq!(person_result.unwrap_err(), vec![PersonBuildingError::RequiresName]);
    }

    #[test]
    fn fails_building_person_without_name_and_favorite_things() {
        let mut person_builder = PersonBuilder::new();

        &person_builder
            .with_birthdate(String::from("25/12/1988"));

        let person_result = person_builder.build();

        assert_eq!(person_result.is_err(), true);
        assert_eq!(person_result.unwrap_err(), vec![PersonBuildingError::RequiresName, PersonBuildingError::RequiresFavoriteThingsList]);
    }

    #[test]
    fn fails_building_person_without_birthdate_and_favorite_things() {
        let mut person_builder = PersonBuilder::new();

        &person_builder
            .with_name(String::from("Evangivaldo"));

        let person_result = person_builder.build();
            
        assert_eq!(person_result.is_err(), true);
        assert_eq!(person_result.unwrap_err(), vec![PersonBuildingError::RequiresFavoriteThingsList]);
    }
}
