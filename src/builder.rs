pub trait Builder<T, E> {
    /// fn with_t(&mut self, t: T) -> &mut Self {
    fn build(&mut self) -> Result<T, E>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::BorrowMut;

    #[derive(Default, Debug, PartialEq)]
    struct Person {
        pub name: String,
        pub birthdate: String,
        pub favorite_things: Vec<String>,
    }

    #[derive(Debug, PartialEq)]
    enum PersonBuildingError {
        RequiresName,
        RequiresBirthdate,
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
        fn build(&mut self) -> Result<Person, Vec<PersonBuildingError>> {
            let name_opt: Option<String> = self.name.take();
            let birthdate_opt: Option<String> = self.birthdate.take();
            let favorite_things_opt: Option<Vec<String>> = self.favorite_things.take();

            use PersonBuildingError::{RequiresName, RequiresBirthdate, RequiresFavoriteThingsList};

            match (name_opt, birthdate_opt, favorite_things_opt) {
                (None, None, None) => Err(vec![RequiresName, RequiresBirthdate, RequiresFavoriteThingsList]),
                (None, None, Some(_)) => Err(vec![RequiresName, RequiresBirthdate]),
                (None, Some(_), None) => Err(vec![RequiresName, RequiresFavoriteThingsList]),
                (None, Some(_), Some(_)) => Err(vec![RequiresName]),
                (Some(_), None, None) => Err(vec![RequiresBirthdate, RequiresFavoriteThingsList]),
                (Some(_), None, Some(_)) => Err(vec![RequiresBirthdate]),
                (Some(_), Some(_), None) => Err(vec![RequiresFavoriteThingsList]),
                (Some(name), Some(birthdate), Some(favorite_things)) => {
                    Ok(Person {
                        name,
                        birthdate,
                        favorite_things,
                    })
                },
            }
        }
    }

    #[test]
    fn builds_a_new_person() {
        let person_result = PersonBuilder::new()
            .with_name(String::from("Evangivaldo"))
            .with_birthdate(String::from("25/12/1988"))
            .with_favorite_thing(String::from("Games"))
            .with_favorite_thing(String::from("Traveling"))
            .build();
        
            
        assert_eq!(person_result.is_ok(), true);
        let person = person_result.unwrap();
        assert_eq!(person.name, "Evangivaldo");
        assert_eq!(person.birthdate, "25/12/1988");
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
        let has_error_requires_birthdate = errors.contains(&PersonBuildingError::RequiresBirthdate);
        let has_error_requires_favorite_things = errors.contains(&PersonBuildingError::RequiresFavoriteThingsList);
        assert_eq!(has_error_requires_name, true);
        assert_eq!(has_error_requires_birthdate, true);
        assert_eq!(has_error_requires_favorite_things, true);
    }

    #[test]
    fn fails_building_person_without_name() {
        let person_result = PersonBuilder::new()
            .with_birthdate(String::from("25/12/1988"))
            .with_favorite_thing(String::from("Games"))
            .build();
            
        assert_eq!(person_result.is_err(), true);
        let errors = person_result.unwrap_err();
        let has_error_requires_name = errors.contains(&PersonBuildingError::RequiresName);
        let has_error_requires_birthdate = errors.contains(&PersonBuildingError::RequiresBirthdate);
        let has_error_requires_favorite_things = errors.contains(&PersonBuildingError::RequiresFavoriteThingsList);
        assert_eq!(has_error_requires_name, true);
        assert_eq!(has_error_requires_birthdate, false);
        assert_eq!(has_error_requires_favorite_things, false);
    }

    #[test]
    fn fails_building_person_without_birthdate() {
        let person_result = PersonBuilder::new()
            .with_name(String::from("Evangivaldo"))
            .with_favorite_thing(String::from("Games"))
            .build();
            
        assert_eq!(person_result.is_err(), true);
        let errors = person_result.unwrap_err();
        let has_error_requires_name = errors.contains(&PersonBuildingError::RequiresName);
        let has_error_requires_birthdate = errors.contains(&PersonBuildingError::RequiresBirthdate);
        let has_error_requires_favorite_things = errors.contains(&PersonBuildingError::RequiresFavoriteThingsList);
        assert_eq!(has_error_requires_name, false);
        assert_eq!(has_error_requires_birthdate, true);
        assert_eq!(has_error_requires_favorite_things, false);
    }

    #[test]
    fn fails_building_person_without_favorite_things() {
        let person_result = PersonBuilder::new()
            .with_name(String::from("Evangivaldo"))
            .with_birthdate(String::from("25/12/1988"))
            .build();
            
        assert_eq!(person_result.is_err(), true);
        let errors = person_result.unwrap_err();
        let has_error_requires_name = errors.contains(&PersonBuildingError::RequiresName);
        let has_error_requires_birthdate = errors.contains(&PersonBuildingError::RequiresBirthdate);
        let has_error_requires_favorite_things = errors.contains(&PersonBuildingError::RequiresFavoriteThingsList);
        assert_eq!(has_error_requires_name, false);
        assert_eq!(has_error_requires_birthdate, false);
        assert_eq!(has_error_requires_favorite_things, true);
    }

    #[test]
    fn fails_building_person_without_name_and_birthdate() {
        let person_result = PersonBuilder::new()
            .with_favorite_thing(String::from("Games"))
            .build();
            
        assert_eq!(person_result.is_err(), true);
        let errors = person_result.unwrap_err();
        let has_error_requires_name = errors.contains(&PersonBuildingError::RequiresName);
        let has_error_requires_birthdate = errors.contains(&PersonBuildingError::RequiresBirthdate);
        let has_error_requires_favorite_things = errors.contains(&PersonBuildingError::RequiresFavoriteThingsList);
        assert_eq!(has_error_requires_name, true);
        assert_eq!(has_error_requires_birthdate, true);
        assert_eq!(has_error_requires_favorite_things, false);
    }

    #[test]
    fn fails_building_person_without_name_and_favorite_things() {
        let person_result = PersonBuilder::new()
            .with_birthdate(String::from("25/12/1988"))
            .build();
            
        assert_eq!(person_result.is_err(), true);
        let errors = person_result.unwrap_err();
        let has_error_requires_name = errors.contains(&PersonBuildingError::RequiresName);
        let has_error_requires_birthdate = errors.contains(&PersonBuildingError::RequiresBirthdate);
        let has_error_requires_favorite_things = errors.contains(&PersonBuildingError::RequiresFavoriteThingsList);
        assert_eq!(has_error_requires_name, true);
        assert_eq!(has_error_requires_birthdate, false);
        assert_eq!(has_error_requires_favorite_things, true);
    }

    #[test]
    fn fails_building_person_without_birthdate_and_favorite_things() {
        let person_result = PersonBuilder::new()
            .with_name(String::from("Evangivaldo"))
            .build();
            
        assert_eq!(person_result.is_err(), true);
        let errors = person_result.unwrap_err();
        let has_error_requires_name = errors.contains(&PersonBuildingError::RequiresName);
        let has_error_requires_birthdate = errors.contains(&PersonBuildingError::RequiresBirthdate);
        let has_error_requires_favorite_things = errors.contains(&PersonBuildingError::RequiresFavoriteThingsList);
        assert_eq!(has_error_requires_name, false);
        assert_eq!(has_error_requires_birthdate, true);
        assert_eq!(has_error_requires_favorite_things, true);
    }
}