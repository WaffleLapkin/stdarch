#[macro_export]
macro_rules! detect_feature {
    ($feature:tt, $feature_lit:tt) => {
        $crate::detect_feature!($feature, $feature_lit : $feature_lit)
    };
    ($feature:tt, $feature_lit:tt : $($target_feature_lit:tt),*) => {
        $(cfg!(target_feature = $target_feature_lit) ||)*
            $crate::detect::__is_feature_detected::$feature()
    };
}

#[allow(unused)]
macro_rules! features {
    (
      @TARGET: $target:ident;
      @CFG: $cfg:meta;
      @MACRO_NAME: $macro_name:ident;
      @MACRO_ATTRS: $(#[$macro_attrs:meta])*
      $(@BIND_FEATURE_NAME: $bind_feature:tt; $feature_impl:tt; )*
      $(@NO_RUNTIME_DETECTION: $nort_feature:tt; )*
      $(@FEATURE: #[$stability_attr:meta] $feature:ident: $feature_lit:tt;
          $(implied by target_features: [$($target_feature_lit:tt),*];)?
          $(#[$feature_comment:meta])*)*
    ) => {
        #[macro_export]
        $(#[$macro_attrs])*
        #[allow_internal_unstable(stdsimd_internal, stdsimd)]
        #[cfg($cfg)]
        #[doc(cfg($cfg))]
        macro_rules! $macro_name {
            $(
                ($feature_lit) => {
                    $crate::detect_feature!($feature, $feature_lit $(: $($target_feature_lit),*)?)
                };
            )*
            $(
                ($bind_feature) => { $crate::$macro_name!($feature_impl) };
            )*
            $(
                ($nort_feature) => {
                    compile_error!(
                        concat!(
                            stringify!($nort_feature),
                            " feature cannot be detected at run-time"
                        )
                    )
                };
            )*
            ($t:tt,) => {
                    $crate::$macro_name!($t);
            };
            ($t:tt) => {
                compile_error!(
                    concat!(
                        concat!("unknown ", stringify!($target)),
                        concat!(" target feature: ", $t)
                    )
                )
            };
        }

        $(#[$macro_attrs])*
        #[macro_export]
        #[cfg(not($cfg))]
        #[doc(cfg($cfg))]
        macro_rules! $macro_name {
            $(
                ($feature_lit) => {
                    compile_error!(
                        concat!(
                            r#"This macro cannot be used on the current target.
                            You can prevent it from being used in other architectures by
                            guarding it behind a cfg("#,
                            stringify!($cfg),
                            ")."
                        )
                    )
                };
            )*
            $(
                ($bind_feature) => { $crate::$macro_name!($feature_impl) };
            )*
            $(
                ($nort_feature) => {
                    compile_error!(
                        concat!(
                            stringify!($nort_feature),
                            " feature cannot be detected at run-time"
                        )
                    )
                };
            )*
            ($t:tt,) => {
                    $crate::$macro_name!($t);
            };
            ($t:tt) => {
                compile_error!(
                    concat!(
                        concat!("unknown ", stringify!($target)),
                        concat!(" target feature: ", $t)
                    )
                )
            };
        }

        /// Each variant denotes a position in a bitset for a particular feature.
        ///
        /// PLEASE: do not use this, it is an implementation detail subject
        /// to change.
        #[doc(hidden)]
        #[allow(non_camel_case_types)]
        #[derive(Copy, Clone)]
        #[repr(u8)]
        #[unstable(feature = "stdsimd_internal", issue = "none")]
        #[cfg($cfg)]
        pub(crate) enum Feature {
            $(
                $(#[$feature_comment])*
                $feature,
            )*

            // Do not add variants after last:
            _last
        }

        #[cfg($cfg)]
        impl Feature {
            pub(crate) fn to_str(self) -> &'static str {
                match self {
                    $(Feature::$feature => $feature_lit,)*
                    Feature::_last => unreachable!(),
                }
            }
            #[cfg(feature = "std_detect_env_override")]
            pub(crate) fn from_str(s: &str) -> Result<Feature, ()> {
                match s {
                    $($feature_lit => Ok(Feature::$feature),)*
                    _ => Err(())
                }
            }
        }

        /// Each function performs run-time feature detection for a single
        /// feature. This allow us to use stability attributes on a per feature
        /// basis.
        ///
        /// PLEASE: do not use this, it is an implementation detail subject
        /// to change.
        #[doc(hidden)]
        #[cfg($cfg)]
        pub mod __is_feature_detected {
            $(

                /// PLEASE: do not use this, it is an implementation detail
                /// subject to change.
                #[inline]
                #[doc(hidden)]
                #[$stability_attr]
                pub fn $feature() -> bool {
                    $crate::detect::check_for($crate::detect::Feature::$feature)
                }
            )*
        }
    };
}
