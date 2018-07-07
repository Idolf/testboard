pub(crate) static GLOBAL_UNIT_VALUE: () = ();

macro_rules! clock_source {
    ($(#[$attr:meta])* $name:ident) => {
        $(#[$attr])*
        pub struct $name<Frequency> {
            frequency: ::core::marker::PhantomData<Frequency>,
            non_send: ::core::marker::PhantomData<*mut ()>,
        }

        impl<Frequency: ::typenum::Unsigned> ::cmu::Clock for $name<Frequency> {
            const FREQUENCY: f64 = Frequency::U64 as f64;

            #[inline]
            fn finalize(self) -> &'static $name<Frequency> {
                assert_eq_size!(Self, ());
                ::core::mem::forget(self);
                let result: &'static () = &::cmu::macros::GLOBAL_UNIT_VALUE;
                unsafe { ::core::mem::transmute(result) }
            }
        }

        impl<Frequency> $name<Frequency> {
            #[allow(unused)]
            #[inline]
            unsafe fn transmute_state<NewFrequency>(self) -> $name<NewFrequency> {
                ::core::mem::forget(self);
                $name {
                    frequency: ::core::marker::PhantomData,
                    non_send: ::core::marker::PhantomData,
                }
            }
        }

        impl<Frequency> Drop for $name<Frequency> {
            #[inline]
            fn drop(&mut self) {
                self._disable();
            }
        }

        unsafe impl<Frequency> Sync for $name<Frequency> {}
    };
}

macro_rules! clock_switch {
    ($(#[$attr:meta])* $name:ident) => {
        $(#[$attr])*
        pub struct $name<'source, Source: 'source> {
            source: ::core::marker::PhantomData<&'source Source>,
            non_send: ::core::marker::PhantomData<*mut ()>,
        }

        impl<'source, Source: ::cmu::Clock> ::cmu::Clock for $name<'source, Source> {
            const FREQUENCY: f64 = Source::FREQUENCY;

            #[inline]
            fn finalize(self) -> &'static $name<'source, Source> {
                assert_eq_size!(Self, ());
                ::core::mem::forget(self);
                let result: &'static () = &::cmu::macros::GLOBAL_UNIT_VALUE;
                unsafe { ::core::mem::transmute(result) }
            }
        }

        impl<'source, Source> $name<'source, Source> {
            #[inline]
            unsafe fn transmute_state<'new_source, NewSource>(
                self,
            ) -> $name<'new_source, NewSource> {
                ::core::mem::forget(self);
                $name {
                    source: ::core::marker::PhantomData,
                    non_send: ::core::marker::PhantomData,
                }
            }
        }

        impl<'source, Source> Drop for $name<'source, Source> {
            #[inline]
            fn drop(&mut self) {
                self._disable();
            }
        }

        unsafe impl<'source, Source> Sync for $name<'source, Source> {}
    };
}

macro_rules! clock_switch_and_divide {
    ($(#[$attr:meta])* $name:ident) => {
        $(#[$attr])*
        pub struct $name<'source, Source: 'source, Division> {
            source: ::core::marker::PhantomData<&'source Source>,
            division: ::core::marker::PhantomData<Division>,
            non_send: ::core::marker::PhantomData<*mut ()>,
        }

        impl<'source, Source: ::cmu::Clock, Division: ::typenum::Unsigned> ::cmu::Clock
            for $name<'source, Source, Division>
        {
            const FREQUENCY: f64 = Source::FREQUENCY / (Division::U64 as f64);

            #[inline]
            fn finalize(self) -> &'static $name<'source, Source, Division> {
                assert_eq_size!(Self, ());
                ::core::mem::forget(self);
                let result: &'static () = &::cmu::macros::GLOBAL_UNIT_VALUE;
                unsafe { ::core::mem::transmute(result) }
            }
        }

        impl<'source, Source, Division> $name<'source, Source, Division> {
            #[inline]
            unsafe fn transmute_state<'new_source, NewSource, NewDivision>(
                self,
            ) -> $name<'new_source, NewSource, NewDivision> {
                ::core::mem::forget(self);
                $name {
                    source: ::core::marker::PhantomData,
                    division: ::core::marker::PhantomData,
                    non_send: ::core::marker::PhantomData,
                }
            }
        }

        impl<'source, Source, Division> Drop for $name<'source, Source, Division> {
            #[inline]
            fn drop(&mut self) {
                self._disable();
            }
        }

        unsafe impl<'source, Source, Division> Sync for $name<'source, Source, Division> {}
    };
}
