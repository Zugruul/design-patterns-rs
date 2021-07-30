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
        Protection
    }

    trait EquipableItem {
        fn get_usage(&self) -> Usage;
        
        fn as_any(&self) -> &dyn Any;
    }
    
    #[derive(Debug, PartialEq)]
    enum Item {
        Bow,
        Axe,
        Armor
    }
    
    struct Axe;
    
    impl EquipableItem for Axe {
        fn get_usage(&self) -> Usage {
            Usage::MeleeWeapon
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }
    
    struct Bow;

    impl EquipableItem for Bow {
        fn get_usage(&self) -> Usage {
            Usage::RangedWeapon
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    struct Armor;

    impl EquipableItem for Armor {
        fn get_usage(&self) -> Usage {
            Usage::Protection
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }
    
    struct ItemFactory;
    impl Factory for ItemFactory {
        type Kind = Item;
        type Output = Box<dyn EquipableItem>;
  
        fn create(kind: Self::Kind) -> Self::Output {
            match kind {
                Item::Bow => Box::new(Bow {}),
                Item::Axe => Box::new(Axe {}),
                Item::Armor => Box::new(Armor {}),
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
    fn it_creates_armor() {
        let item = ItemFactory::create(Item::Armor);
        let possibly_an_armor = item.as_any().downcast_ref::<Armor>();

        assert_eq!(item.get_usage(), Usage::Protection);
        assert_eq!(possibly_an_armor.is_some(), true)
    }
}
