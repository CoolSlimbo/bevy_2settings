#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use bevy::prelude::*;
use bevy_2settings::{BevyError, Settings, TomlFileConfiguration, WithConfigurator};
use serde::Serialize;
#[settings(styling(Hello, World))]
struct Test {
    /// Testing Description!
    ///
    /// NEW LINE!!
    #[settings(name = "Big Screen", default = test_system, predicate = pred)]
    fullscreen: bool,
    #[settings(name = "Nested Test", nested)]
    nested: NestedTest,
}
#[allow(warnings, clippy::all)]
#[serde(default)]
struct __bevy_2settings_partial_Test {
    fullscreen: ::std::option::Option<bool>,
    nested: __bevy_2settings_partial_NestedTest,
}
#[automatically_derived]
#[allow(warnings, clippy::all)]
impl ::core::fmt::Debug for __bevy_2settings_partial_Test {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "__bevy_2settings_partial_Test",
            "fullscreen",
            &self.fullscreen,
            "nested",
            &&self.nested,
        )
    }
}
#[automatically_derived]
#[allow(warnings, clippy::all)]
impl ::core::clone::Clone for __bevy_2settings_partial_Test {
    #[inline]
    fn clone(&self) -> __bevy_2settings_partial_Test {
        __bevy_2settings_partial_Test {
            fullscreen: ::core::clone::Clone::clone(&self.fullscreen),
            nested: ::core::clone::Clone::clone(&self.nested),
        }
    }
}
#[automatically_derived]
#[allow(warnings, clippy::all)]
impl ::core::default::Default for __bevy_2settings_partial_Test {
    #[inline]
    fn default() -> __bevy_2settings_partial_Test {
        __bevy_2settings_partial_Test {
            fullscreen: ::core::default::Default::default(),
            nested: ::core::default::Default::default(),
        }
    }
}
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for __bevy_2settings_partial_Test
    where
        __bevy_2settings_partial_Test: _serde::__private::Default,
    {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "fullscreen" => _serde::__private::Ok(__Field::__field0),
                        "nested" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"fullscreen" => _serde::__private::Ok(__Field::__field0),
                        b"nested" => _serde::__private::Ok(__Field::__field1),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de>
            where
                __bevy_2settings_partial_Test: _serde::__private::Default,
            {
                marker: _serde::__private::PhantomData<__bevy_2settings_partial_Test>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de>
            where
                __bevy_2settings_partial_Test: _serde::__private::Default,
            {
                type Value = __bevy_2settings_partial_Test;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct __bevy_2settings_partial_Test",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __default: Self::Value = _serde::__private::Default::default();
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        ::std::option::Option<bool>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => __default.fullscreen,
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        __bevy_2settings_partial_NestedTest,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => __default.nested,
                    };
                    _serde::__private::Ok(__bevy_2settings_partial_Test {
                        fullscreen: __field0,
                        nested: __field1,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<
                        ::std::option::Option<bool>,
                    > = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<
                        __bevy_2settings_partial_NestedTest,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "fullscreen",
                                        ),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        ::std::option::Option<bool>,
                                    >(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("nested"),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        __bevy_2settings_partial_NestedTest,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __default: Self::Value = _serde::__private::Default::default();
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => __default.fullscreen,
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => __default.nested,
                    };
                    _serde::__private::Ok(__bevy_2settings_partial_Test {
                        fullscreen: __field0,
                        nested: __field1,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["fullscreen", "nested"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "__bevy_2settings_partial_Test",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<
                        __bevy_2settings_partial_Test,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[allow(warnings, clippy::all)]
struct __bevy_2settings_view_Test {
    fullscreen: bool,
    nested: __bevy_2settings_view_NestedTest,
}
#[automatically_derived]
#[allow(warnings, clippy::all)]
impl ::core::fmt::Debug for __bevy_2settings_view_Test {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "__bevy_2settings_view_Test",
            "fullscreen",
            &self.fullscreen,
            "nested",
            &&self.nested,
        )
    }
}
#[automatically_derived]
#[allow(warnings, clippy::all)]
impl ::core::clone::Clone for __bevy_2settings_view_Test {
    #[inline]
    fn clone(&self) -> __bevy_2settings_view_Test {
        __bevy_2settings_view_Test {
            fullscreen: ::core::clone::Clone::clone(&self.fullscreen),
            nested: ::core::clone::Clone::clone(&self.nested),
        }
    }
}
impl ::bevy_2settings::Settings for Test {
    const META: ::bevy_2settings::Meta = ::bevy_2settings::Meta {
        fields: &[
            ::bevy_2settings::Field {
                name: "Big Screen",
                internal_name: "fullscreen",
                description: " Testing Description!\n\n NEW LINE!!",
                styling: &[],
                nested: false,
            },
            ::bevy_2settings::Field {
                name: "Nested Test",
                internal_name: "nested",
                description: "",
                styling: &[],
                nested: true,
            },
        ],
        self_stylings: &[
            ::bevy_2settings::Styling::Hello,
            ::bevy_2settings::Styling::World,
        ],
    };
    const INTERNAL_NAME: &'static str = "Test";
    type Partial = __bevy_2settings_partial_Test;
    type ViewNode = __bevy_2settings_view_Test;
    fn from_partial(
        partial: Self::Partial,
        world: &mut ::bevy_2settings::World,
    ) -> ::std::result::Result<Self, ::bevy_2settings::BevyError> {
        Ok(Self {
            fullscreen: match partial.fullscreen {
                Some(v) => v,
                None => {
                    ::bevy_2settings::RunSystemOnce::run_system_once(
                        &mut *world,
                        test_system,
                    )??
                }
            },
            nested: <NestedTest as ::bevy_2settings::Settings>::from_partial(
                partial.nested,
                &mut *world,
            )?,
        })
    }
    fn get_view_tree(
        &self,
        world: &mut ::bevy_2settings::World,
    ) -> ::std::result::Result<Self::ViewNode, ::bevy_2settings::BevyError> {
        Ok(Self::ViewNode {
            fullscreen: ::bevy_2settings::RunSystemOnce::run_system_once(
                &mut *world,
                pred,
            )??,
            nested: self.nested.get_view_tree(&mut *world)?,
        })
    }
}
impl bevy::ecs::resource::Resource for Test
where
    Self: Send + Sync + 'static,
{}
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for Test {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "Test",
                false as usize + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "fullscreen",
                &self.fullscreen,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "nested",
                &self.nested,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for Test {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "Test",
            "fullscreen",
            &self.fullscreen,
            "nested",
            &&self.nested,
        )
    }
}
struct NestedTest {
    #[settings(name = "No Screen")]
    anti_screen: bool,
}
#[allow(warnings, clippy::all)]
#[serde(default)]
struct __bevy_2settings_partial_NestedTest {
    anti_screen: ::std::option::Option<bool>,
}
#[automatically_derived]
#[allow(warnings, clippy::all)]
impl ::core::fmt::Debug for __bevy_2settings_partial_NestedTest {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "__bevy_2settings_partial_NestedTest",
            "anti_screen",
            &&self.anti_screen,
        )
    }
}
#[automatically_derived]
#[allow(warnings, clippy::all)]
impl ::core::clone::Clone for __bevy_2settings_partial_NestedTest {
    #[inline]
    fn clone(&self) -> __bevy_2settings_partial_NestedTest {
        __bevy_2settings_partial_NestedTest {
            anti_screen: ::core::clone::Clone::clone(&self.anti_screen),
        }
    }
}
#[automatically_derived]
#[allow(warnings, clippy::all)]
impl ::core::default::Default for __bevy_2settings_partial_NestedTest {
    #[inline]
    fn default() -> __bevy_2settings_partial_NestedTest {
        __bevy_2settings_partial_NestedTest {
            anti_screen: ::core::default::Default::default(),
        }
    }
}
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for __bevy_2settings_partial_NestedTest
    where
        __bevy_2settings_partial_NestedTest: _serde::__private::Default,
    {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "anti_screen" => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"anti_screen" => _serde::__private::Ok(__Field::__field0),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de>
            where
                __bevy_2settings_partial_NestedTest: _serde::__private::Default,
            {
                marker: _serde::__private::PhantomData<
                    __bevy_2settings_partial_NestedTest,
                >,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            #[automatically_derived]
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de>
            where
                __bevy_2settings_partial_NestedTest: _serde::__private::Default,
            {
                type Value = __bevy_2settings_partial_NestedTest;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct __bevy_2settings_partial_NestedTest",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __default: Self::Value = _serde::__private::Default::default();
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        ::std::option::Option<bool>,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => __default.anti_screen,
                    };
                    _serde::__private::Ok(__bevy_2settings_partial_NestedTest {
                        anti_screen: __field0,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<
                        ::std::option::Option<bool>,
                    > = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "anti_screen",
                                        ),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<
                                        ::std::option::Option<bool>,
                                    >(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __default: Self::Value = _serde::__private::Default::default();
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => __default.anti_screen,
                    };
                    _serde::__private::Ok(__bevy_2settings_partial_NestedTest {
                        anti_screen: __field0,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &["anti_screen"];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "__bevy_2settings_partial_NestedTest",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<
                        __bevy_2settings_partial_NestedTest,
                    >,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
#[allow(warnings, clippy::all)]
struct __bevy_2settings_view_NestedTest {
    anti_screen: bool,
}
#[automatically_derived]
#[allow(warnings, clippy::all)]
impl ::core::fmt::Debug for __bevy_2settings_view_NestedTest {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "__bevy_2settings_view_NestedTest",
            "anti_screen",
            &&self.anti_screen,
        )
    }
}
#[automatically_derived]
#[allow(warnings, clippy::all)]
impl ::core::clone::Clone for __bevy_2settings_view_NestedTest {
    #[inline]
    fn clone(&self) -> __bevy_2settings_view_NestedTest {
        __bevy_2settings_view_NestedTest {
            anti_screen: ::core::clone::Clone::clone(&self.anti_screen),
        }
    }
}
impl ::bevy_2settings::Settings for NestedTest {
    const META: ::bevy_2settings::Meta = ::bevy_2settings::Meta {
        fields: &[
            ::bevy_2settings::Field {
                name: "No Screen",
                internal_name: "anti_screen",
                description: "",
                styling: &[],
                nested: false,
            },
        ],
        self_stylings: &[],
    };
    const INTERNAL_NAME: &'static str = "NestedTest";
    type Partial = __bevy_2settings_partial_NestedTest;
    type ViewNode = __bevy_2settings_view_NestedTest;
    fn from_partial(
        partial: Self::Partial,
        world: &mut ::bevy_2settings::World,
    ) -> ::std::result::Result<Self, ::bevy_2settings::BevyError> {
        Ok(Self {
            anti_screen: match partial.anti_screen {
                Some(v) => v,
                None => <bool as Default>::default(),
            },
        })
    }
    fn get_view_tree(
        &self,
        world: &mut ::bevy_2settings::World,
    ) -> ::std::result::Result<Self::ViewNode, ::bevy_2settings::BevyError> {
        Ok(Self::ViewNode {
            anti_screen: true,
        })
    }
}
#[doc(hidden)]
#[allow(
    non_upper_case_globals,
    unused_attributes,
    unused_qualifications,
    clippy::absolute_paths,
)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for NestedTest {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "NestedTest",
                false as usize + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "anti_screen",
                &self.anti_screen,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[automatically_derived]
impl ::core::fmt::Debug for NestedTest {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field1_finish(
            f,
            "NestedTest",
            "anti_screen",
            &&self.anti_screen,
        )
    }
}
fn test_system() -> Result<bool, BevyError> {
    Ok(true)
}
fn pred() -> Result<bool, BevyError> {
    Ok(true)
}
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(Test::with_configurator(TomlFileConfiguration::default()))
        .add_systems(Startup, generate_ui)
        .add_systems(Update, (get_settings, space_pressed))
        .run();
}
fn get_settings(settings: Res<Test>) {
    {
        ::std::io::_print(format_args!("{0:?}\n", settings));
    };
}
fn space_pressed(keys: Res<ButtonInput<KeyCode>>, mut settings: ResMut<Test>) {
    if keys.just_pressed(KeyCode::Space) {
        settings.nested.anti_screen = !settings.nested.anti_screen;
    }
}
fn generate_ui(mut commands: Commands) {
    let root = commands.spawn(Node { ..Default::default() }).id();
}
