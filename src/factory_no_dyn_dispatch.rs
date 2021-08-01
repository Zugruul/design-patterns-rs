#[cfg(test)]
mod tests {
    #[derive(Debug, PartialEq)]
    enum Usage {
        MeleeWeapon,
        RangedWeapon,
        Protection
    }

    trait EquipableItem {
        fn get_usage(&self) -> Usage;
    }

    struct Axe;
    
    impl EquipableItem for Axe {
        fn get_usage(&self) -> Usage {
            Usage::MeleeWeapon
        }
    }
    
    struct Bow;

    impl EquipableItem for Bow {
        fn get_usage(&self) -> Usage {
            Usage::RangedWeapon
        }
    }

    struct Armor;

    impl EquipableItem for Armor {
        fn get_usage(&self) -> Usage {
            Usage::Protection
        }
    }
    
    struct ItemFactory;

    impl ItemFactory {
        fn create_axe() -> Axe {
            Axe {}
        }

        fn create_bow() -> Bow {
            Bow {}
        }

        fn create_armor() -> Armor {
            Armor {}
        }
    }

    #[test]
    fn it_creates_bow() {
        let bow = ItemFactory::create_bow();

        assert_eq!(bow.get_usage(), Usage::RangedWeapon);
    }

    #[test]
    fn it_creates_axe() {
        let axe = ItemFactory::create_axe();

        assert_eq!(axe.get_usage(), Usage::MeleeWeapon);
    }

    #[test]
    fn it_creates_armor() {
        let armor = ItemFactory::create_armor();

        assert_eq!(armor.get_usage(), Usage::Protection);
    }
}
