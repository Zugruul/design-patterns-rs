pub trait Factory {
    type Kind;
    type Output;

    fn create(kind: Self::Kind) -> Self::Output;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::Any;

    #[derive(Debug, PartialEq)]
    enum Usage {
        MeleeWeapon,
        RangedWeapon,
        Consumable
    }

    trait UsableItem {
        fn get_usage(&self) -> Usage;
        
        fn as_any(&self) -> &dyn Any;
    }
    
    #[derive(Debug, PartialEq)]
    enum Item {
        Bow,
        Axe,
        Potion
    }
    
    struct Axe;
    
    impl UsableItem for Axe {
        fn get_usage(&self) -> Usage {
            Usage::MeleeWeapon
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }
    
    struct Bow;

    impl UsableItem for Bow {
        fn get_usage(&self) -> Usage {
            Usage::RangedWeapon
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    struct Potion;

    impl UsableItem for Potion {
        fn get_usage(&self) -> Usage {
            Usage::Consumable
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }
    
    struct ItemFactory;
    impl Factory for ItemFactory {
        type Kind = Item;
        type Output = Box<dyn UsableItem>;
  
        fn create(kind: Self::Kind) -> Self::Output {
            match kind {
                Item::Bow => Box::new(Bow {}),
                Item::Axe => Box::new(Axe {}),
                Item::Potion => Box::new(Potion {}),
            }
        }
    }

    #[test]
    fn it_creates_bow() {
        let item = ItemFactory::create(Item::Bow);
        let possibly_a_bow = item.as_any().downcast_ref::<Bow>();

        assert_eq!(item.get_usage(), Usage::RangedWeapon);
        assert_eq!(possibly_a_bow.is_some(), true)
    }

    #[test]
    fn it_creates_axe() {
        let item = ItemFactory::create(Item::Axe);
        let possibly_an_axe = item.as_any().downcast_ref::<Axe>();

        assert_eq!(item.get_usage(), Usage::MeleeWeapon);
        assert_eq!(possibly_an_axe.is_some(), true)
    }

    #[test]
    fn it_creates_potion() {
        let item = ItemFactory::create(Item::Potion);
        let possibly_a_potion = item.as_any().downcast_ref::<Potion>();

        assert_eq!(item.get_usage(), Usage::Consumable);
        assert_eq!(possibly_a_potion.is_some(), true)
    }
}
