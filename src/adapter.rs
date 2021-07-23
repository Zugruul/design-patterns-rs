#[cfg(test)]
mod test {
    trait ArrowShooter {
        fn has_arrows_in_quiver(&self) -> bool;
        fn draw_arrow(&mut self) -> bool;
        fn can_shoot(&self) -> bool;
        fn shoot(&mut self) -> bool;
    }
    
    struct Bow {
        arrows_left_in_quiver: u32,
        ready: bool
    }
    
    impl ArrowShooter for Bow {
        fn has_arrows_in_quiver(&self) -> bool {
            self.arrows_left_in_quiver > 0
        }
    
        fn draw_arrow(&mut self) -> bool {
            if self.ready && !self.has_arrows_in_quiver() {
                return false
            }
            
            self.arrows_left_in_quiver -= 1;
            self.ready = true;
            true
        }

        fn can_shoot(&self) -> bool {
            self.ready
        }
    
        fn shoot(&mut self) -> bool {
            if !self.can_shoot() {
                return false
            }
    
            self.ready = false;
            true
        }
    }
    
    trait BoltShooter {
        fn has_bolts_stowed(&self) -> bool;
        fn load_bolt(&mut self) -> bool;
        fn can_shoot(&self) -> bool;
        fn shoot(&mut self) -> bool;
    }
    
    struct Crossbow {
        ammunition_stowed: u32,
        loaded: bool
    }
    
    impl BoltShooter for Crossbow {
        fn has_bolts_stowed(&self) -> bool {
            self.ammunition_stowed > 0
        }
    
        fn load_bolt(&mut self) -> bool {
            if self.loaded && !self.has_bolts_stowed() {
                return false
            }
            
            self.ammunition_stowed -= 1;
            self.loaded = true;
            true
        }

        
        fn can_shoot(&self) -> bool {
            self.loaded
        }

        fn shoot(&mut self) -> bool {
            if !self.can_shoot() {
                return false
            }
    
            self.loaded = false;
            true
        }

    }
    
    trait RangedWeapon {
        fn has_ammunition_left(&self) -> bool;
        fn reload(&mut self) -> bool;
        fn can_shoot(&self) -> bool;
        fn shoot(&mut self) -> bool;
    }
    
    enum RangedWeaponAdapter<'a> {
        Bow(&'a mut Bow),
        Crossbow(&'a mut Crossbow)
    }
    
    impl<'a> RangedWeapon for RangedWeaponAdapter<'a> {
        fn has_ammunition_left(&self) -> bool {
            match self {
                RangedWeaponAdapter::Bow(bow) => bow.has_arrows_in_quiver(),
                RangedWeaponAdapter::Crossbow(crossbow) => crossbow.has_bolts_stowed(),
            }
        }
    
        fn reload(&mut self) -> bool {
            match self {
                RangedWeaponAdapter::Bow(bow) => bow.draw_arrow(),
                RangedWeaponAdapter::Crossbow(crossbow) => crossbow.load_bolt(),
            }
        }
    
        fn shoot(&mut self) -> bool {
            match self {
                RangedWeaponAdapter::Bow(bow) => bow.shoot(),
                RangedWeaponAdapter::Crossbow(crossbow) => crossbow.shoot(),
            }
        }

        fn can_shoot(&self) -> bool {
            match self {
                RangedWeaponAdapter::Bow(bow) => bow.can_shoot(),
                RangedWeaponAdapter::Crossbow(crossbow) => crossbow.can_shoot(),
            }
        }
    }

    #[test]
    fn adapter_works_for_bow() {
        let mut bow = Bow {
            arrows_left_in_quiver: 1,
            ready: false
        };

        assert_eq!(bow.has_arrows_in_quiver(), true);
        assert_eq!(bow.can_shoot(), false);

        let mut adapter = RangedWeaponAdapter::Bow(&mut bow);

        assert_eq!(adapter.has_ammunition_left(), true);
        assert_eq!(adapter.can_shoot(), false);
        assert_eq!(adapter.reload(), true);
        assert_eq!(adapter.has_ammunition_left(), false);
        assert_eq!(adapter.can_shoot(), true);
        assert_eq!(adapter.shoot(), true);
        assert_eq!(adapter.has_ammunition_left(), false);
        assert_eq!(adapter.can_shoot(), false);
        assert_eq!(bow.has_arrows_in_quiver(), false);
        assert_eq!(bow.can_shoot(), false);
    }

    #[test]
    fn adapter_works_for_crossbow() {
        let mut crossbow = Crossbow {
            ammunition_stowed: 1,
            loaded: false
        };

        assert_eq!(crossbow.has_bolts_stowed(), true);
        assert_eq!(crossbow.can_shoot(), false);

        let mut adapter = RangedWeaponAdapter::Crossbow(&mut crossbow);

        assert_eq!(adapter.has_ammunition_left(), true);
        assert_eq!(adapter.can_shoot(), false);
        assert_eq!(adapter.reload(), true);
        assert_eq!(adapter.has_ammunition_left(), false);
        assert_eq!(adapter.can_shoot(), true);
        assert_eq!(adapter.shoot(), true);
        assert_eq!(adapter.has_ammunition_left(), false);
        assert_eq!(adapter.can_shoot(), false);
        assert_eq!(crossbow.has_bolts_stowed(), false);
        assert_eq!(crossbow.can_shoot(), false);
    }
}