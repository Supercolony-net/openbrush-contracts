#[macro_export]
macro_rules! define_getters{
    ($name_ref:ident, $name_mut:ident, $type:ty)=>{
        fn $name_ref(&self) -> & $type;

        fn $name_mut(&mut self) -> &mut $type;
    }
}

#[macro_export]
macro_rules! iml_getters {
    ($field:ident, $name_ref:ident, $name_mut:ident, $type:ty) => {
        #[cfg(not(feature = "ink-as-dependency"))]
        fn $name_ref(&self) -> &$type {
            &self.$field
        }
        #[cfg(feature = "ink-as-dependency")]
        fn $name_ref(&self) -> &$type {
            unimplemented!()
        }
        #[cfg(not(feature = "ink-as-dependency"))]
        fn $name_mut(&mut self) -> &mut $type {
            &mut self.$field
        }
        #[cfg(feature = "ink-as-dependency")]
        fn $name_mut(&mut self) -> &mut $type {
            unimplemented!()
        }
    };
}

#[macro_export]
macro_rules! assert_ok {
    ( $x:expr $(,)? ) => {
        let is = $x;
        match is {
            Ok(_) => (),
            _ => assert!(false, "Expected Ok(_). Got {:#?}", is),
        }
    };
    ( $x:expr, $y:expr $(,)? ) => {
        assert_eq!($x, Ok($y));
    };
}

#[macro_export]
macro_rules! assert_err {
    ( $x:expr , $y:expr $(,)? ) => {
        assert_eq!($x, Err($y.into()));
    };
}