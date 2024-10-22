use crate::schema::*;
use crate::validation::*;
use ::std::str::FromStr;
use ::yaserde::Visitor;

#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
impl ::yaserde::YaDeserialize for Cue {
    #[allow(unused_variables)]
    fn deserialize<R: ::std::io::Read>(
        reader: &mut ::yaserde::de::Deserializer<R>,
    ) -> ::std::result::Result<Self, ::std::string::String> {
        let (named_element, struct_namespace) =
            if let ::yaserde::__xml::reader::XmlEvent::StartElement { name, .. } =
                reader.peek()?.to_owned()
            {
                (name.local_name.to_owned(), name.namespace.clone())
            } else {
                (
                    ::std::string::String::from("ern:Cue"),
                    ::std::option::Option::None,
                )
            };
        let start_depth = reader.depth();

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: start to parse {:?}",
            stringify!(Cue),
            start_depth,
            named_element
        );

        if reader.depth() == 0 {
            if let Some(namespace) = struct_namespace {
                match namespace.as_str() {
                    "http://ddex.net/xml/ern/43" => {}

                    bad_namespace => {
                        let msg = ::std::format!(
                            "bad namespace for {}, found {}",
                            named_element,
                            bad_namespace
                        );
                        return Err(msg);
                    }
                }
            }
        }
        #[allow(unused_mut)]
        let mut __cue_use_type_value = None;
        #[allow(unused_mut)]
        let mut __cue_theme_type_value = None;
        #[allow(unused_mut)]
        let mut __cue_vocal_type_value = None;
        #[allow(unused_mut)]
        let mut __cue_visual_perception_type_value = None;
        #[allow(unused_mut)]
        let mut __cue_origin_value = None;
        #[allow(unused_mut)]
        let mut __display_title_texts_value: ::std::vec::Vec<DisplayTitleText> = vec![];
        #[allow(unused_mut)]
        let mut __display_titles_value: ::std::vec::Vec<DisplayTitle> = vec![];
        #[allow(unused_mut)]
        let mut __additional_titles_value: ::std::vec::Vec<AdditionalTitle> = vec![];
        #[allow(unused_mut)]
        let mut __contributors_value: ::std::vec::Vec<Contributor> = vec![];
        #[allow(unused_mut)]
        let mut __is_dance_value = None;
        #[allow(unused_mut)]
        let mut __has_musical_content_value = None;
        #[allow(unused_mut)]
        let mut __p_lines_value: ::std::vec::Vec<PLine> = vec![];
        #[allow(unused_mut)]
        let mut __c_lines_value: ::std::vec::Vec<CLine> = vec![];
        #[allow(unused_mut)]
        let mut __start_time_value = None;
        #[allow(unused_mut)]
        let mut __duration_value = None;
        #[allow(unused_mut)]
        let mut __end_time_value = None;
        #[allow(unused_mut)]
        let mut __resource_id_value = None;
        #[allow(unused_mut)]
        let mut __work_id_value = None;
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_IsDance_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_IsDance_ {
            type Value = bool;
            fn visit_bool(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                bool::from_str(match v {
                    "1" => "true",
                    "0" => "false",
                    _ => v,
                })
                .map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_HasMusicalContent_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_HasMusicalContent_ {
            type Value = bool;
            fn visit_bool(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                bool::from_str(match v {
                    "1" => "true",
                    "0" => "false",
                    _ => v,
                })
                .map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_StartTime_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_StartTime_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_Duration_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_Duration_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_EndTime_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_EndTime_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        let mut depth = 0;
        loop {
            let event = reader.peek()?.to_owned();

            ::yaserde::__derive_trace!(
                "Struct {} @ {}: matching {:?}",
                stringify!(Cue),
                start_depth,
                event
            );

            match event {
                ::yaserde::__xml::reader::XmlEvent::StartElement {
                    ref name,
                    ref attributes,
                    ..
                } => {
                    if depth == 0 && name.local_name == "ern:Cue" {
                        let event = reader.next_event()?;
                    } else {
                        match name.local_name.as_str() {
                            "CueUseType" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <CueUseType as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __cue_use_type_value = ::std::option::Option::Some(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "CueThemeType" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <CueThemeType as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __cue_theme_type_value = ::std::option::Option::Some(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "CueVocalType" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <CueVocalType as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __cue_vocal_type_value = ::std::option::Option::Some(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "CueVisualPerceptionType" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <CueVisualPerceptionType as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __cue_visual_perception_type_value =
                                        ::std::option::Option::Some(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "CueOrigin" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <CueOrigin as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __cue_origin_value = ::std::option::Option::Some(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "DisplayTitleText" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <DisplayTitleText as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __display_title_texts_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "DisplayTitle" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <DisplayTitle as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __display_titles_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "AdditionalTitle" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <AdditionalTitle as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __additional_titles_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "Contributor" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <Contributor as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __contributors_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "IsDance" => {
                                let visitor = __Visitor_IsDance_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result = reader.read_inner_value::<bool, _>(|reader| {
                                    if let ::std::result::Result::Ok(
                                        ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                    ) = reader.peek()
                                    {
                                        let val = visitor.visit_bool(&s);
                                        let _event = reader.next_event()?;
                                        val
                                    } else {
                                        ::std::result::Result::Err(::std::format!(
                                            "unable to parse content for {}",
                                            "IsDance"
                                        ))
                                    }
                                });
                                if let ::std::result::Result::Ok(value) = result {
                                    __is_dance_value = ::std::option::Option::Some(value)
                                }
                            }
                            "HasMusicalContent" => {
                                let visitor = __Visitor_HasMusicalContent_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result = reader.read_inner_value::<bool, _>(|reader| {
                                    if let ::std::result::Result::Ok(
                                        ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                    ) = reader.peek()
                                    {
                                        let val = visitor.visit_bool(&s);
                                        let _event = reader.next_event()?;
                                        val
                                    } else {
                                        ::std::result::Result::Err(::std::format!(
                                            "unable to parse content for {}",
                                            "HasMusicalContent"
                                        ))
                                    }
                                });
                                if let ::std::result::Result::Ok(value) = result {
                                    __has_musical_content_value = ::std::option::Option::Some(value)
                                }
                            }
                            "PLine" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <PLine as ::yaserde::YaDeserialize>::deserialize(reader)?;
                                    __p_lines_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "CLine" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <CLine as ::yaserde::YaDeserialize>::deserialize(reader)?;
                                    __c_lines_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "StartTime" => {
                                let visitor = __Visitor_StartTime_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "StartTime"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __start_time_value = ::std::option::Option::Some(value)
                                }
                            }
                            "Duration" => {
                                let visitor = __Visitor_Duration_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "Duration"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __duration_value = ::std::option::Option::Some(value)
                                }
                            }
                            "EndTime" => {
                                let visitor = __Visitor_EndTime_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "EndTime"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __end_time_value = ::std::option::Option::Some(value)
                                }
                            }
                            "ResourceId" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <ResourceId as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __resource_id_value = ::std::option::Option::Some(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "WorkId" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <MusicalWorkId as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __work_id_value = ::std::option::Option::Some(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            _ => {
                                let event = reader.next_event()?;
                                if depth > 0 {
                                    reader.skip_element(|event| {})?;
                                }
                            }
                        }
                    }
                    if depth == 0 {}
                    depth += 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndElement { ref name } => {
                    if name.local_name == named_element && reader.depth() == start_depth + 1 {
                        break;
                    }
                    let event = reader.next_event()?;
                    depth -= 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndDocument => {
                    if false {
                        break;
                    }
                }
                ::yaserde::__xml::reader::XmlEvent::Characters(ref text_content) => {
                    let event = reader.next_event()?;
                }
                event => {
                    return ::std::result::Result::Err(::std::format!("unknown event {:?}", event));
                }
            }
        }

        if let (Some(_), Some(_)) = (&__resource_id_value, &__work_id_value) {
            return Err("Both ResourceId & WorkId cannot be present".to_string());
        }

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: success",
            stringify!(Cue),
            start_depth
        );

        ::std::result::Result::Ok(Cue {
            cue_use_type: __cue_use_type_value,
            cue_theme_type: __cue_theme_type_value,
            cue_vocal_type: __cue_vocal_type_value,
            cue_visual_perception_type: __cue_visual_perception_type_value,
            cue_origin: __cue_origin_value,
            display_title_texts: __display_title_texts_value,
            display_titles: __display_titles_value,
            additional_titles: __additional_titles_value,
            contributors: __contributors_value,
            is_dance: __is_dance_value,
            has_musical_content: __has_musical_content_value,
            p_lines: __p_lines_value,
            c_lines: __c_lines_value,
            start_time: __start_time_value,
            duration: __duration_value,
            end_time: __end_time_value,
            resource_id: __resource_id_value,
            work_id: __work_id_value,
        })
    }
}

#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
impl ::yaserde::YaDeserialize for DealTerms {
    #[allow(unused_variables)]
    fn deserialize<R: ::std::io::Read>(
        reader: &mut ::yaserde::de::Deserializer<R>,
    ) -> ::std::result::Result<Self, ::std::string::String> {
        let (named_element, struct_namespace) =
            if let ::yaserde::__xml::reader::XmlEvent::StartElement { name, .. } =
                reader.peek()?.to_owned()
            {
                (name.local_name.to_owned(), name.namespace.clone())
            } else {
                (
                    ::std::string::String::from("ern:DealTerms"),
                    ::std::option::Option::None,
                )
            };
        let start_depth = reader.depth();
        ::yaserde::__derive_debug!(
            "Struct {} @ {}: start to parse {:?}",
            stringify!(DealTerms),
            start_depth,
            named_element
        );

        if reader.depth() == 0 {
            if let Some(namespace) = struct_namespace {
                match namespace.as_str() {
                    "http://ddex.net/xml/ern/43" => {}

                    bad_namespace => {
                        let msg = ::std::format!(
                            "bad namespace for {}, found {}",
                            named_element,
                            bad_namespace
                        );
                        return Err(msg);
                    }
                }
            }
        }
        #[allow(unused_mut)]
        let mut __territory_codes_value: ::std::vec::Vec<CurrentTerritoryCode> = vec![];
        #[allow(unused_mut)]
        let mut __excluded_territory_codes_value: ::std::vec::Vec<CurrentTerritoryCode> = vec![];
        #[allow(unused_mut)]
        let mut __distribution_channels_value: ::std::vec::Vec<Dsp> = vec![];
        #[allow(unused_mut)]
        let mut __excluded_distribution_channels_value: ::std::vec::Vec<Dsp> = vec![];
        #[allow(unused_mut)]
        let mut __validity_periods_value: ::std::vec::Vec<PeriodWithStartDate> = vec![];
        #[allow(unused_mut)]
        let mut __commercial_model_types_value: ::std::vec::Vec<CommercialModelType> = vec![];
        #[allow(unused_mut)]
        let mut __use_types_value: ::std::vec::Vec<DiscoverableUseType> = vec![];
        #[allow(unused_mut)]
        let mut __user_interface_types_value: ::std::vec::Vec<UserInterfaceType> = vec![];
        #[allow(unused_mut)]
        let mut __carrier_types_value: ::std::vec::Vec<CarrierType> = vec![];
        #[allow(unused_mut)]
        let mut __technical_instantiation_value = None;
        #[allow(unused_mut)]
        let mut __number_of_usages_value = None;
        #[allow(unused_mut)]
        let mut __rights_claim_policys_value: ::std::vec::Vec<RightsClaimPolicy> = vec![];
        #[allow(unused_mut)]
        let mut __price_informations_value: ::std::vec::Vec<PriceInformationWithType> = vec![];
        #[allow(unused_mut)]
        let mut __is_pre_order_deal_value = None;
        #[allow(unused_mut)]
        let mut __instant_gratification_resource_list_value = None;
        #[allow(unused_mut)]
        let mut __physical_returns_value = None;
        #[allow(unused_mut)]
        let mut __number_of_products_per_carton_value = None;
        #[allow(unused_mut)]
        let mut __is_promotional_value = None;
        #[allow(unused_mut)]
        let mut __promotional_code_value = None;
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_NumberOfUsages_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_NumberOfUsages_ {
            type Value = i32;
            fn visit_i32(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                i32::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_IsPreOrderDeal_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_IsPreOrderDeal_ {
            type Value = bool;
            fn visit_bool(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                bool::from_str(match v {
                    "1" => "true",
                    "0" => "false",
                    _ => v,
                })
                .map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_NumberOfProductsPerCarton_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_NumberOfProductsPerCarton_ {
            type Value = i32;
            fn visit_i32(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                i32::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_IsPromotional_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_IsPromotional_ {
            type Value = bool;
            fn visit_bool(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                bool::from_str(match v {
                    "1" => "true",
                    "0" => "false",
                    _ => v,
                })
                .map_err(|e| e.to_string())
            }
        }
        let mut depth = 0;
        loop {
            let event = reader.peek()?.to_owned();

            ::yaserde::__derive_trace!(
                "Struct {} @ {}: matching {:?}",
                stringify!(DealTerms),
                start_depth,
                event
            );

            match event {
                ::yaserde::__xml::reader::XmlEvent::StartElement {
                    ref name,
                    ref attributes,
                    ..
                } => {
                    if depth == 0 && name.local_name == "ern:DealTerms" {
                        let event = reader.next_event()?;
                    } else {
                        match name.local_name.as_str() {
                            "TerritoryCode" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <CurrentTerritoryCode as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __territory_codes_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "ExcludedTerritoryCode" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <CurrentTerritoryCode as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __excluded_territory_codes_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "DistributionChannel" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <Dsp as ::yaserde::YaDeserialize>::deserialize(reader)?;
                                    __distribution_channels_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "ExcludedDistributionChannel" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <Dsp as ::yaserde::YaDeserialize>::deserialize(reader)?;
                                    __excluded_distribution_channels_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "ValidityPeriod" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <PeriodWithStartDate as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __validity_periods_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "CommercialModelType" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <CommercialModelType as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __commercial_model_types_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "UseType" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <DiscoverableUseType as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __use_types_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "UserInterfaceType" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <UserInterfaceType as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __user_interface_types_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "CarrierType" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <CarrierType as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __carrier_types_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "TechnicalInstantiation" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <DealTermsTechnicalInstantiation as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __technical_instantiation_value =
                                        ::std::option::Option::Some(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "NumberOfUsages" => {
                                let visitor = __Visitor_NumberOfUsages_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result = reader.read_inner_value::<i32, _>(|reader| {
                                    if let ::std::result::Result::Ok(
                                        ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                    ) = reader.peek()
                                    {
                                        let val = visitor.visit_i32(&s);
                                        let _event = reader.next_event()?;
                                        val
                                    } else {
                                        ::std::result::Result::Err(::std::format!(
                                            "unable to parse content for {}",
                                            "NumberOfUsages"
                                        ))
                                    }
                                });
                                if let ::std::result::Result::Ok(value) = result {
                                    __number_of_usages_value = ::std::option::Option::Some(value)
                                }
                            }
                            "RightsClaimPolicy" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <RightsClaimPolicy as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __rights_claim_policys_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "PriceInformation" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <PriceInformationWithType as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __price_informations_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "IsPreOrderDeal" => {
                                let visitor = __Visitor_IsPreOrderDeal_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result = reader.read_inner_value::<bool, _>(|reader| {
                                    if let ::std::result::Result::Ok(
                                        ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                    ) = reader.peek()
                                    {
                                        let val = visitor.visit_bool(&s);
                                        let _event = reader.next_event()?;
                                        val
                                    } else {
                                        ::std::result::Result::Err(::std::format!(
                                            "unable to parse content for {}",
                                            "IsPreOrderDeal"
                                        ))
                                    }
                                });
                                if let ::std::result::Result::Ok(value) = result {
                                    __is_pre_order_deal_value = ::std::option::Option::Some(value)
                                }
                            }
                            "InstantGratificationResourceList" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <DealResourceReferenceList as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __instant_gratification_resource_list_value =
                                        ::std::option::Option::Some(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "PhysicalReturns" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <PhysicalReturns as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __physical_returns_value = ::std::option::Option::Some(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "NumberOfProductsPerCarton" => {
                                let visitor = __Visitor_NumberOfProductsPerCarton_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result = reader.read_inner_value::<i32, _>(|reader| {
                                    if let ::std::result::Result::Ok(
                                        ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                    ) = reader.peek()
                                    {
                                        let val = visitor.visit_i32(&s);
                                        let _event = reader.next_event()?;
                                        val
                                    } else {
                                        ::std::result::Result::Err(::std::format!(
                                            "unable to parse content for {}",
                                            "NumberOfProductsPerCarton"
                                        ))
                                    }
                                });
                                if let ::std::result::Result::Ok(value) = result {
                                    __number_of_products_per_carton_value =
                                        ::std::option::Option::Some(value)
                                }
                            }
                            "IsPromotional" => {
                                let visitor = __Visitor_IsPromotional_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result = reader.read_inner_value::<bool, _>(|reader| {
                                    if let ::std::result::Result::Ok(
                                        ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                    ) = reader.peek()
                                    {
                                        let val = visitor.visit_bool(&s);
                                        let _event = reader.next_event()?;
                                        val
                                    } else {
                                        ::std::result::Result::Err(::std::format!(
                                            "unable to parse content for {}",
                                            "IsPromotional"
                                        ))
                                    }
                                });
                                if let ::std::result::Result::Ok(value) = result {
                                    __is_promotional_value = ::std::option::Option::Some(value)
                                }
                            }
                            "PromotionalCode" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <PromotionalCode as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __promotional_code_value = ::std::option::Option::Some(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            _ => {
                                let event = reader.next_event()?;
                                if depth > 0 {
                                    reader.skip_element(|event| {})?;
                                }
                            }
                        }
                    }
                    if depth == 0 {}
                    depth += 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndElement { ref name } => {
                    if name.local_name == named_element && reader.depth() == start_depth + 1 {
                        break;
                    }
                    let event = reader.next_event()?;
                    depth -= 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndDocument => {
                    if false {
                        break;
                    }
                }
                ::yaserde::__xml::reader::XmlEvent::Characters(ref text_content) => {
                    let event = reader.next_event()?;
                }
                event => {
                    return ::std::result::Result::Err(::std::format!("unknown event {:?}", event));
                }
            }
        }

        if __territory_codes_value.len() > 0 && __excluded_territory_codes_value.len() > 0 {
            return Err("TerritoryCodes can't be with ExcludedTerritoryCodes".to_string());
        }

        if __distribution_channels_value.len() > 0
            && __excluded_distribution_channels_value.len() > 0
        {
            return Err(
                "DistributionChannels can't be with ExcludedDistributionChannels".to_string(),
            );
        }

        if let (Some(_), Some(_)) = (&__is_promotional_value, &__promotional_code_value) {
            return Err("PromotionalCode can't be with IsPromotional".to_string());
        }

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: success",
            stringify!(DealTerms),
            start_depth
        );

        ::std::result::Result::Ok(DealTerms {
            territory_codes: __territory_codes_value,
            excluded_territory_codes: __excluded_territory_codes_value,
            distribution_channels: __distribution_channels_value,
            excluded_distribution_channels: __excluded_distribution_channels_value,
            validity_periods: __validity_periods_value,
            commercial_model_types: __commercial_model_types_value,
            use_types: __use_types_value,
            user_interface_types: __user_interface_types_value,
            carrier_types: __carrier_types_value,
            technical_instantiation: __technical_instantiation_value,
            number_of_usages: __number_of_usages_value,
            rights_claim_policys: __rights_claim_policys_value,
            price_informations: __price_informations_value,
            is_pre_order_deal: __is_pre_order_deal_value,
            instant_gratification_resource_list: __instant_gratification_resource_list_value,
            physical_returns: __physical_returns_value,
            number_of_products_per_carton: __number_of_products_per_carton_value,
            is_promotional: __is_promotional_value,
            promotional_code: __promotional_code_value,
        })
    }
}

#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
impl ::yaserde::YaDeserialize for DetailedResourceContributor {
    #[allow(unused_variables)]
    fn deserialize<R: ::std::io::Read>(
        reader: &mut ::yaserde::de::Deserializer<R>,
    ) -> ::std::result::Result<Self, ::std::string::String> {
        let (named_element, struct_namespace) =
            if let ::yaserde::__xml::reader::XmlEvent::StartElement { name, .. } =
                reader.peek()?.to_owned()
            {
                (name.local_name.to_owned(), name.namespace.clone())
            } else {
                (
                    ::std::string::String::from("ern:DetailedResourceContributor"),
                    ::std::option::Option::None,
                )
            };
        let start_depth = reader.depth();

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: start to parse {:?}",
            stringify!(DetailedResourceContributor),
            start_depth,
            named_element
        );

        if reader.depth() == 0 {
            if let Some(namespace) = struct_namespace {
                match namespace.as_str() {
                    "http://ddex.net/xml/ern/43" => {}

                    bad_namespace => {
                        let msg = ::std::format!(
                            "bad namespace for {}, found {}",
                            named_element,
                            bad_namespace
                        );
                        return Err(msg);
                    }
                }
            }
        }
        #[allow(unused_mut)]
        let mut __roles_value: ::std::vec::Vec<ContributorRole> = vec![];
        #[allow(unused_mut)]
        let mut __instrument_types_value: ::std::vec::Vec<InstrumentType> = vec![];
        #[allow(unused_mut)]
        let mut __has_made_featured_contribution_value = None;
        #[allow(unused_mut)]
        let mut __has_made_contracted_contribution_value = None;
        #[allow(unused_mut)]
        let mut __display_creditss_value: ::std::vec::Vec<DisplayCredits> = vec![];
        #[allow(unused_mut)]
        let mut __sequence_number_value = None;
        #[allow(unused_mut)]
        let mut __parties_names_value: ::std::vec::Vec<PartyName> = vec![];
        #[allow(unused_mut)]
        let mut __parties_ids_value: ::std::vec::Vec<DetailedPartyId> = vec![];
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_HasMadeFeaturedContribution_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_HasMadeFeaturedContribution_ {
            type Value = bool;
            fn visit_bool(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                bool::from_str(match v {
                    "1" => "true",
                    "0" => "false",
                    _ => v,
                })
                .map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_HasMadeContractedContribution_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_HasMadeContractedContribution_ {
            type Value = bool;
            fn visit_bool(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                bool::from_str(match v {
                    "1" => "true",
                    "0" => "false",
                    _ => v,
                })
                .map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_Attribute_SequenceNumber_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_Attribute_SequenceNumber_ {
            type Value = i32;
            fn visit_i32(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                i32::from_str(v).map_err(|e| e.to_string())
            }
        }
        let mut depth = 0;
        loop {
            let event = reader.peek()?.to_owned();

            ::yaserde::__derive_trace!(
                "Struct {} @ {}: matching {:?}",
                stringify!(DetailedResourceContributor),
                start_depth,
                event
            );

            match event {
                ::yaserde::__xml::reader::XmlEvent::StartElement {
                    ref name,
                    ref attributes,
                    ..
                } => {
                    if depth == 0 && name.local_name == "ern:DetailedResourceContributor" {
                        let event = reader.next_event()?;
                    } else {
                        match name.local_name.as_str() {
                            "Role" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <ContributorRole as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __roles_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "InstrumentType" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <InstrumentType as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __instrument_types_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "HasMadeFeaturedContribution" => {
                                let visitor = __Visitor_HasMadeFeaturedContribution_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result = reader.read_inner_value::<bool, _>(|reader| {
                                    if let ::std::result::Result::Ok(
                                        ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                    ) = reader.peek()
                                    {
                                        let val = visitor.visit_bool(&s);
                                        let _event = reader.next_event()?;
                                        val
                                    } else {
                                        ::std::result::Result::Err(::std::format!(
                                            "unable to parse content for {}",
                                            "HasMadeFeaturedContribution"
                                        ))
                                    }
                                });
                                if let ::std::result::Result::Ok(value) = result {
                                    __has_made_featured_contribution_value =
                                        ::std::option::Option::Some(value)
                                }
                            }
                            "HasMadeContractedContribution" => {
                                let visitor = __Visitor_HasMadeContractedContribution_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result = reader.read_inner_value::<bool, _>(|reader| {
                                    if let ::std::result::Result::Ok(
                                        ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                    ) = reader.peek()
                                    {
                                        let val = visitor.visit_bool(&s);
                                        let _event = reader.next_event()?;
                                        val
                                    } else {
                                        ::std::result::Result::Err(::std::format!(
                                            "unable to parse content for {}",
                                            "HasMadeContractedContribution"
                                        ))
                                    }
                                });
                                if let ::std::result::Result::Ok(value) = result {
                                    __has_made_contracted_contribution_value =
                                        ::std::option::Option::Some(value)
                                }
                            }
                            "DisplayCredits" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <DisplayCredits as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __display_creditss_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "PartyName" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <PartyName as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __parties_names_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "PartyId" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <DetailedPartyId as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __parties_ids_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            _ => {
                                let event = reader.next_event()?;
                                if depth > 0 {
                                    reader.skip_element(|event| {})?;
                                }
                            }
                        }
                    }
                    if depth == 0 {
                        for attr in attributes {
                            if attr.name.local_name == "SequenceNumber" {
                                let visitor = __Visitor_Attribute_SequenceNumber_ {};
                                let value = visitor.visit_i32(&attr.value)?;
                                __sequence_number_value = ::std::option::Option::Some(value);
                            }
                        }
                    }
                    depth += 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndElement { ref name } => {
                    if name.local_name == named_element && reader.depth() == start_depth + 1 {
                        break;
                    }
                    let event = reader.next_event()?;
                    depth -= 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndDocument => {
                    if false {
                        break;
                    }
                }
                ::yaserde::__xml::reader::XmlEvent::Characters(ref text_content) => {
                    let event = reader.next_event()?;
                }
                event => {
                    return ::std::result::Result::Err(::std::format!("unknown event {:?}", event));
                }
            }
        }

        if __parties_names_value.len() == 0 && __parties_ids_value.len() == 0 {
            return Err("At least one PartyName and/or PartyId is required".to_string());
        }

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: success",
            stringify!(DetailedResourceContributor),
            start_depth
        );

        ::std::result::Result::Ok(DetailedResourceContributor {
            roles: __roles_value,
            instrument_types: __instrument_types_value,
            has_made_featured_contribution: __has_made_featured_contribution_value,
            has_made_contracted_contribution: __has_made_contracted_contribution_value,
            display_creditss: __display_creditss_value,
            sequence_number: __sequence_number_value,
            parties_names: __parties_names_value,
            parties_ids: __parties_ids_value,
        })
    }
}

#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
impl ::yaserde::YaDeserialize for Fingerprint {
    #[allow(unused_variables)]
    fn deserialize<R: ::std::io::Read>(
        reader: &mut ::yaserde::de::Deserializer<R>,
    ) -> ::std::result::Result<Self, ::std::string::String> {
        let (named_element, struct_namespace) =
            if let ::yaserde::__xml::reader::XmlEvent::StartElement { name, .. } =
                reader.peek()?.to_owned()
            {
                (name.local_name.to_owned(), name.namespace.clone())
            } else {
                (
                    ::std::string::String::from("ern:Fingerprint"),
                    ::std::option::Option::None,
                )
            };
        let start_depth = reader.depth();

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: start to parse {:?}",
            stringify!(Fingerprint),
            start_depth,
            named_element
        );

        if reader.depth() == 0 {
            if let Some(namespace) = struct_namespace {
                match namespace.as_str() {
                    "http://ddex.net/xml/ern/43" => {}

                    bad_namespace => {
                        let msg = ::std::format!(
                            "bad namespace for {}, found {}",
                            named_element,
                            bad_namespace
                        );
                        return Err(msg);
                    }
                }
            }
        }
        #[allow(unused_mut)]
        let mut __algorithm_value: Option<FingerprintAlgorithmType> = None;
        #[allow(unused_mut)]
        let mut __version_value = None;
        #[allow(unused_mut)]
        let mut __parameter_value = None;
        #[allow(unused_mut)]
        let mut __file_value = None;
        #[allow(unused_mut)]
        let mut __data_type_value = None;
        #[allow(unused_mut)]
        let mut __fingerprint_value_value = None;
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_Version_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_Version_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_Parameter_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_Parameter_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_DataType_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_DataType_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_FingerprintValue_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_FingerprintValue_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        let mut depth = 0;
        loop {
            let event = reader.peek()?.to_owned();

            ::yaserde::__derive_trace!(
                "Struct {} @ {}: matching {:?}",
                stringify!(Fingerprint),
                start_depth,
                event
            );

            match event {
                ::yaserde::__xml::reader::XmlEvent::StartElement {
                    ref name,
                    ref attributes,
                    ..
                } => {
                    if depth == 0 && name.local_name == "ern:Fingerprint" {
                        let event = reader.next_event()?;
                    } else {
                        match name.local_name.as_str() {
                            "Algorithm" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <FingerprintAlgorithmType as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __algorithm_value = ::std::option::Option::Some(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "Version" => {
                                let visitor = __Visitor_Version_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "Version"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __version_value = ::std::option::Option::Some(value)
                                }
                            }
                            "Parameter" => {
                                let visitor = __Visitor_Parameter_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "Parameter"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __parameter_value = ::std::option::Option::Some(value)
                                }
                            }
                            "File" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <File as ::yaserde::YaDeserialize>::deserialize(reader)?;
                                    __file_value = ::std::option::Option::Some(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "DataType" => {
                                let visitor = __Visitor_DataType_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "DataType"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __data_type_value = ::std::option::Option::Some(value)
                                }
                            }
                            "FingerprintValue" => {
                                let visitor = __Visitor_FingerprintValue_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "FingerprintValue"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __fingerprint_value_value = ::std::option::Option::Some(value)
                                }
                            }
                            _ => {
                                let event = reader.next_event()?;
                                if depth > 0 {
                                    reader.skip_element(|event| {})?;
                                }
                            }
                        }
                    }
                    if depth == 0 {}
                    depth += 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndElement { ref name } => {
                    if name.local_name == named_element && reader.depth() == start_depth + 1 {
                        break;
                    }
                    let event = reader.next_event()?;
                    depth -= 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndDocument => {
                    if false {
                        break;
                    }
                }
                ::yaserde::__xml::reader::XmlEvent::Characters(ref text_content) => {
                    let event = reader.next_event()?;
                }
                event => {
                    return ::std::result::Result::Err(::std::format!("unknown event {:?}", event));
                }
            }
        }

        match (
            &__file_value,
            &__data_type_value,
            &__fingerprint_value_value,
        ) {
            (Some(_), None, None) => {}
            (None, Some(_), Some(_)) => {}
            _ => Err("Provide File or DataType and FingerprintValue".to_string())?,
        }

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: success",
            stringify!(Fingerprint),
            start_depth
        );

        ::std::result::Result::Ok(Fingerprint {
            algorithm: __algorithm_value
                .ok_or_else(|| "algorithm is a required field of Fingerprint".to_string())?,
            version: __version_value,
            parameter: __parameter_value,
            file: __file_value,
            data_type: match &__data_type_value {
                Some(val) => {
                    if AvsBinaryDataTypeValidator::validate(&val) {
                        __data_type_value
                    } else {
                        Err(format!("invalid value in attribute {}", "DataType"))?
                    }
                }
                None => __data_type_value,
            },
            fingerprint_value: __fingerprint_value_value,
        })
    }
}

#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
impl ::yaserde::YaDeserialize for Party {
    #[allow(unused_variables)]
    fn deserialize<R: ::std::io::Read>(
        reader: &mut ::yaserde::de::Deserializer<R>,
    ) -> ::std::result::Result<Self, ::std::string::String> {
        let (named_element, struct_namespace) =
            if let ::yaserde::__xml::reader::XmlEvent::StartElement { name, .. } =
                reader.peek()?.to_owned()
            {
                (name.local_name.to_owned(), name.namespace.clone())
            } else {
                (
                    ::std::string::String::from("ern:Party"),
                    ::std::option::Option::None,
                )
            };
        let start_depth = reader.depth();

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: start to parse {:?}",
            stringify!(Party),
            start_depth,
            named_element
        );

        if reader.depth() == 0 {
            if let Some(namespace) = struct_namespace {
                match namespace.as_str() {
                    "http://ddex.net/xml/ern/43" => {}

                    bad_namespace => {
                        let msg = ::std::format!(
                            "bad namespace for {}, found {}",
                            named_element,
                            bad_namespace
                        );
                        return Err(msg);
                    }
                }
            }
        }
        #[allow(unused_mut)]
        let mut __party_reference_value: Option<::std::string::String> = None;
        #[allow(unused_mut)]
        let mut __affiliations_value: ::std::vec::Vec<Affiliation> = vec![];
        #[allow(unused_mut)]
        let mut __related_partys_value: ::std::vec::Vec<RelatedParty> = vec![];
        #[allow(unused_mut)]
        let mut __artist_profile_pages_value: ::std::vec::Vec<::std::string::String> = vec![];
        #[allow(unused_mut)]
        let mut __parties_names_value: ::std::vec::Vec<PartyNameWithTerritory> = vec![];
        #[allow(unused_mut)]
        let mut __parties_ids_value: ::std::vec::Vec<DetailedPartyId> = vec![];
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_PartyReference_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_PartyReference_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_ArtistProfilePage_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_ArtistProfilePage_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        let mut depth = 0;
        loop {
            let event = reader.peek()?.to_owned();

            ::yaserde::__derive_trace!(
                "Struct {} @ {}: matching {:?}",
                stringify!(Party),
                start_depth,
                event
            );

            match event {
                ::yaserde::__xml::reader::XmlEvent::StartElement {
                    ref name,
                    ref attributes,
                    ..
                } => {
                    if depth == 0 && name.local_name == "ern:Party" {
                        let event = reader.next_event()?;
                    } else {
                        match name.local_name.as_str() {
                            "PartyReference" => {
                                let visitor = __Visitor_PartyReference_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "PartyReference"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __party_reference_value = ::std::option::Option::Some(value)
                                }
                            }
                            "Affiliation" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <Affiliation as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __affiliations_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "RelatedParty" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <RelatedParty as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __related_partys_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "ArtistProfilePage" => {
                                let visitor = __Visitor_ArtistProfilePage_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "ArtistProfilePage"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __artist_profile_pages_value.push(value)
                                }
                            }
                            "PartyName" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <PartyNameWithTerritory as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __parties_names_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "PartyId" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <DetailedPartyId as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __parties_ids_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            _ => {
                                let event = reader.next_event()?;
                                if depth > 0 {
                                    reader.skip_element(|event| {})?;
                                }
                            }
                        }
                    }
                    if depth == 0 {}
                    depth += 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndElement { ref name } => {
                    if name.local_name == named_element && reader.depth() == start_depth + 1 {
                        break;
                    }
                    let event = reader.next_event()?;
                    depth -= 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndDocument => {
                    if false {
                        break;
                    }
                }
                ::yaserde::__xml::reader::XmlEvent::Characters(ref text_content) => {
                    let event = reader.next_event()?;
                }
                event => {
                    return ::std::result::Result::Err(::std::format!("unknown event {:?}", event));
                }
            }
        }

        if __parties_names_value.len() == 0 && __parties_ids_value.len() == 0 {
            return Err("At least one PartyName and/or PartyId is required".to_string());
        }

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: success",
            stringify!(Party),
            start_depth
        );

        ::std::result::Result::Ok(Party {
            party_reference: match __party_reference_value {
                Some(val) => {
                    if PartyReferenceValidator::validate(&val) {
                        val
                    } else {
                        Err("invalid value in field Party".to_string())?
                    }
                }
                None => Err("party_reference is a required field of Party".to_string())?,
            },
            affiliations: __affiliations_value,
            related_partys: __related_partys_value,
            artist_profile_pages: __artist_profile_pages_value,
            parties_names: __parties_names_value,
            parties_ids: __parties_ids_value,
        })
    }
}

#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
impl ::yaserde::YaDeserialize for RelatedResource {
    #[allow(unused_variables)]
    fn deserialize<R: ::std::io::Read>(
        reader: &mut ::yaserde::de::Deserializer<R>,
    ) -> ::std::result::Result<Self, ::std::string::String> {
        let (named_element, struct_namespace) =
            if let ::yaserde::__xml::reader::XmlEvent::StartElement { name, .. } =
                reader.peek()?.to_owned()
            {
                (name.local_name.to_owned(), name.namespace.clone())
            } else {
                (
                    ::std::string::String::from("ern:RelatedResource"),
                    ::std::option::Option::None,
                )
            };
        let start_depth = reader.depth();

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: start to parse {:?}",
            stringify!(RelatedResource),
            start_depth,
            named_element
        );

        if reader.depth() == 0 {
            if let Some(namespace) = struct_namespace {
                match namespace.as_str() {
                    "http://ddex.net/xml/ern/43" => {}

                    bad_namespace => {
                        let msg = ::std::format!(
                            "bad namespace for {}, found {}",
                            named_element,
                            bad_namespace
                        );
                        return Err(msg);
                    }
                }
            }
        }
        #[allow(unused_mut)]
        let mut __resource_relationship_type_value: Option<::std::string::String> = None;
        #[allow(unused_mut)]
        let mut __timings_value: ::std::vec::Vec<Timing> = vec![];
        #[allow(unused_mut)]
        let mut __resource_related_resource_reference_value = None;
        #[allow(unused_mut)]
        let mut __resource_id_value = None;
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_ResourceRelationshipType_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_ResourceRelationshipType_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_ResourceRelatedResourceReference_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_ResourceRelatedResourceReference_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        let mut depth = 0;
        loop {
            let event = reader.peek()?.to_owned();

            ::yaserde::__derive_trace!(
                "Struct {} @ {}: matching {:?}",
                stringify!(RelatedResource),
                start_depth,
                event
            );

            match event {
                ::yaserde::__xml::reader::XmlEvent::StartElement {
                    ref name,
                    ref attributes,
                    ..
                } => {
                    if depth == 0 && name.local_name == "ern:RelatedResource" {
                        let event = reader.next_event()?;
                    } else {
                        match name.local_name.as_str() {
                            "ResourceRelationshipType" => {
                                let visitor = __Visitor_ResourceRelationshipType_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "ResourceRelationshipType"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __resource_relationship_type_value =
                                        ::std::option::Option::Some(value)
                                }
                            }
                            "Timing" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <Timing as ::yaserde::YaDeserialize>::deserialize(reader)?;
                                    __timings_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "ResourceRelatedResourceReference" => {
                                let visitor = __Visitor_ResourceRelatedResourceReference_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "ResourceRelatedResourceReference"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __resource_related_resource_reference_value =
                                        ::std::option::Option::Some(value)
                                }
                            }
                            "ResourceId" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <ResourceId as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __resource_id_value = ::std::option::Option::Some(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            _ => {
                                let event = reader.next_event()?;
                                if depth > 0 {
                                    reader.skip_element(|event| {})?;
                                }
                            }
                        }
                    }
                    if depth == 0 {}
                    depth += 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndElement { ref name } => {
                    if name.local_name == named_element && reader.depth() == start_depth + 1 {
                        break;
                    }
                    let event = reader.next_event()?;
                    depth -= 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndDocument => {
                    if false {
                        break;
                    }
                }
                ::yaserde::__xml::reader::XmlEvent::Characters(ref text_content) => {
                    let event = reader.next_event()?;
                }
                event => {
                    return ::std::result::Result::Err(::std::format!("unknown event {:?}", event));
                }
            }
        }

        match (
            &__resource_related_resource_reference_value,
            &__resource_id_value,
        ) {
            (Some(_), None) | (None, Some(_)) => {}
            _ => Err(
                "RelatedResource: Provide ResourceRelatedResourceReference or ResourceId"
                    .to_string(),
            )?,
        }

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: success",
            stringify!(RelatedResource),
            start_depth
        );

        ::std::result::Result::Ok(RelatedResource {
            resource_relationship_type: match __resource_relationship_type_value {
                Some(val) => {
                    if AvsResourceRelationshipTypeValidator::validate(&val) {
                        val
                    } else {
                        Err("invalid value in field RelatedResource".to_string())?
                    }
                }
                None => Err(
                    "resource_relationship_type is a required field of RelatedResource".to_string(),
                )?,
            },
            timings: __timings_value,
            resource_related_resource_reference: match &__resource_related_resource_reference_value
            {
                Some(val) => {
                    if ResourceRelatedResourceReferenceValidator::validate(&val) {
                        __resource_related_resource_reference_value
                    } else {
                        Err(format!(
                            "invalid value in attribute {}",
                            "ResourceRelatedResourceReference"
                        ))?
                    }
                }
                None => __resource_related_resource_reference_value,
            },
            resource_id: __resource_id_value,
        })
    }
}

#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
impl ::yaserde::YaDeserialize for Release {
    #[allow(unused_variables)]
    fn deserialize<R: ::std::io::Read>(
        reader: &mut ::yaserde::de::Deserializer<R>,
    ) -> ::std::result::Result<Self, ::std::string::String> {
        let (named_element, struct_namespace) =
            if let ::yaserde::__xml::reader::XmlEvent::StartElement { name, .. } =
                reader.peek()?.to_owned()
            {
                (name.local_name.to_owned(), name.namespace.clone())
            } else {
                (
                    ::std::string::String::from("ern:Release"),
                    ::std::option::Option::None,
                )
            };
        let start_depth = reader.depth();

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: start to parse {:?}",
            stringify!(Release),
            start_depth,
            named_element
        );

        if reader.depth() == 0 {
            if let Some(namespace) = struct_namespace {
                match namespace.as_str() {
                    "http://ddex.net/xml/ern/43" => {}

                    bad_namespace => {
                        let msg = ::std::format!(
                            "bad namespace for {}, found {}",
                            named_element,
                            bad_namespace
                        );
                        return Err(msg);
                    }
                }
            }
        }
        #[allow(unused_mut)]
        let mut __release_reference_value: Option<::std::string::String> = None;
        #[allow(unused_mut)]
        let mut __release_types_value: ::std::vec::Vec<ReleaseTypeForReleaseNotification> = vec![];
        #[allow(unused_mut)]
        let mut __release_id_value: Option<ReleaseId> = None;
        #[allow(unused_mut)]
        let mut __display_title_texts_value: ::std::vec::Vec<DisplayTitleText> = vec![];
        #[allow(unused_mut)]
        let mut __display_titles_value: ::std::vec::Vec<DisplayTitle> = vec![];
        #[allow(unused_mut)]
        let mut __additional_titles_value: ::std::vec::Vec<AdditionalTitle> = vec![];
        #[allow(unused_mut)]
        let mut __version_types_value: ::std::vec::Vec<VersionType> = vec![];
        #[allow(unused_mut)]
        let mut __display_artist_names_value: ::std::vec::Vec<
            DisplayArtistNameWithDefault,
        > = vec![];
        #[allow(unused_mut)]
        let mut __display_artists_value: ::std::vec::Vec<DisplayArtist> = vec![];
        #[allow(unused_mut)]
        let mut __release_label_references_value: ::std::vec::Vec<
            ReleaseLabelReferenceWithParty,
        > = vec![];
        #[allow(unused_mut)]
        let mut __administrating_record_companys_value: ::std::vec::Vec<
            AdministratingRecordCompanyWithReference,
        > = vec![];
        #[allow(unused_mut)]
        let mut __p_lines_value: ::std::vec::Vec<PLineWithDefault> = vec![];
        #[allow(unused_mut)]
        let mut __c_lines_value: ::std::vec::Vec<CLineWithDefault> = vec![];
        #[allow(unused_mut)]
        let mut __courtesy_lines_value: ::std::vec::Vec<CourtesyLineWithDefault> = vec![];
        #[allow(unused_mut)]
        let mut __duration_value = None;
        #[allow(unused_mut)]
        let mut __genres_value: ::std::vec::Vec<GenreWithTerritory> = vec![];
        #[allow(unused_mut)]
        let mut __release_dates_value: ::std::vec::Vec<EventDateWithDefault> = vec![];
        #[allow(unused_mut)]
        let mut __original_release_dates_value: ::std::vec::Vec<EventDateWithDefault> = vec![];
        #[allow(unused_mut)]
        let mut __release_visibility_references_value: ::std::vec::Vec<
            ::std::string::String,
        > = vec![];
        #[allow(unused_mut)]
        let mut __parental_warning_types_value: ::std::vec::Vec<
            ParentalWarningTypeWithTerritory,
        > = vec![];
        #[allow(unused_mut)]
        let mut __av_ratings_value: ::std::vec::Vec<AvRating> = vec![];
        #[allow(unused_mut)]
        let mut __related_releases_value: ::std::vec::Vec<RelatedRelease> = vec![];
        #[allow(unused_mut)]
        let mut __related_resources_value: ::std::vec::Vec<RelatedResource> = vec![];
        #[allow(unused_mut)]
        let mut __resource_group_value: Option<ResourceGroup> = None;
        #[allow(unused_mut)]
        let mut __external_resource_links_value: ::std::vec::Vec<ExternalResourceLink> = vec![];
        #[allow(unused_mut)]
        let mut __target_url_value = None;
        #[allow(unused_mut)]
        let mut __keywordss_value: ::std::vec::Vec<KeywordsWithTerritory> = vec![];
        #[allow(unused_mut)]
        let mut __synopsiss_value: ::std::vec::Vec<SynopsisWithTerritory> = vec![];
        #[allow(unused_mut)]
        let mut __ragas_value: ::std::vec::Vec<Raga> = vec![];
        #[allow(unused_mut)]
        let mut __talas_value: ::std::vec::Vec<Tala> = vec![];
        #[allow(unused_mut)]
        let mut __deitys_value: ::std::vec::Vec<Deity> = vec![];
        #[allow(unused_mut)]
        let mut __hi_res_music_description_value = None;
        #[allow(unused_mut)]
        let mut __is_soundtrack_value = None;
        #[allow(unused_mut)]
        let mut __is_hi_res_music_value = None;
        #[allow(unused_mut)]
        let mut __marketing_comments_value: ::std::vec::Vec<MarketingComment> = vec![];
        #[allow(unused_mut)]
        let mut __language_and_script_code_value = None;
        #[allow(unused_mut)]
        let mut __is_single_artist_compilation_value = None;
        #[allow(unused_mut)]
        let mut __is_multi_artist_compilation_value = None;
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_ReleaseReference_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_ReleaseReference_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_Duration_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_Duration_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_ReleaseVisibilityReference_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_ReleaseVisibilityReference_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_TargetUrl_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_TargetUrl_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_HiResMusicDescription_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_HiResMusicDescription_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_IsSoundtrack_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_IsSoundtrack_ {
            type Value = bool;
            fn visit_bool(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                bool::from_str(match v {
                    "1" => "true",
                    "0" => "false",
                    _ => v,
                })
                .map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_IsHiResMusic_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_IsHiResMusic_ {
            type Value = bool;
            fn visit_bool(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                bool::from_str(match v {
                    "1" => "true",
                    "0" => "false",
                    _ => v,
                })
                .map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_Attribute_LanguageAndScriptCode_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_Attribute_LanguageAndScriptCode_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_IsSingleArtistCompilation_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_IsSingleArtistCompilation_ {
            type Value = bool;
            fn visit_bool(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                bool::from_str(match v {
                    "1" => "true",
                    "0" => "false",
                    _ => v,
                })
                .map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_IsMultiArtistCompilation_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_IsMultiArtistCompilation_ {
            type Value = bool;
            fn visit_bool(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                bool::from_str(match v {
                    "1" => "true",
                    "0" => "false",
                    _ => v,
                })
                .map_err(|e| e.to_string())
            }
        }
        let mut depth = 0;
        loop {
            let event = reader.peek()?.to_owned();

            ::yaserde::__derive_trace!(
                "Struct {} @ {}: matching {:?}",
                stringify!(Release),
                start_depth,
                event
            );
            
            match event {
                ::yaserde::__xml::reader::XmlEvent::StartElement {
                    ref name,
                    ref attributes,
                    ..
                } => {
                    if depth == 0 && name.local_name == "ern:Release" {
                        let event = reader.next_event()?;
                    } else {
                        match name.local_name.as_str() {
                            "ReleaseReference" => {
                                let visitor = __Visitor_ReleaseReference_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "ReleaseReference"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __release_reference_value = ::std::option::Option::Some(value)
                                }
                            }
                            "ReleaseType" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <ReleaseTypeForReleaseNotification as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __release_types_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "ReleaseId" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <ReleaseId as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __release_id_value = ::std::option::Option::Some(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "DisplayTitleText" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <DisplayTitleText as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __display_title_texts_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "DisplayTitle" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <DisplayTitle as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __display_titles_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "AdditionalTitle" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <AdditionalTitle as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __additional_titles_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "VersionType" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <VersionType as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __version_types_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "DisplayArtistName" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <DisplayArtistNameWithDefault as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __display_artist_names_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "DisplayArtist" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <DisplayArtist as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __display_artists_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "ReleaseLabelReference" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <ReleaseLabelReferenceWithParty as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __release_label_references_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "AdministratingRecordCompany" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <AdministratingRecordCompanyWithReference as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __administrating_record_companys_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "PLine" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <PLineWithDefault as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __p_lines_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "CLine" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <CLineWithDefault as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __c_lines_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "CourtesyLine" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <CourtesyLineWithDefault as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __courtesy_lines_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "Duration" => {
                                let visitor = __Visitor_Duration_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "Duration"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __duration_value = ::std::option::Option::Some(value)
                                }
                            }
                            "Genre" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <GenreWithTerritory as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __genres_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "ReleaseDate" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <EventDateWithDefault as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __release_dates_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "OriginalReleaseDate" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <EventDateWithDefault as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __original_release_dates_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "ReleaseVisibilityReference" => {
                                let visitor = __Visitor_ReleaseVisibilityReference_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "ReleaseVisibilityReference"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __release_visibility_references_value.push(value)
                                }
                            }
                            "ParentalWarningType" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <ParentalWarningTypeWithTerritory as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __parental_warning_types_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "AvRating" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <AvRating as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __av_ratings_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "RelatedRelease" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <RelatedRelease as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __related_releases_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "RelatedResource" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <RelatedResource as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __related_resources_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "ResourceGroup" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <ResourceGroup as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __resource_group_value = ::std::option::Option::Some(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "ExternalResourceLink" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <ExternalResourceLink as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __external_resource_links_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "TargetURL" => {
                                let visitor = __Visitor_TargetUrl_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "TargetURL"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __target_url_value = ::std::option::Option::Some(value)
                                }
                            }
                            "Keywords" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <KeywordsWithTerritory as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __keywordss_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "Synopsis" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <SynopsisWithTerritory as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __synopsiss_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "Raga" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <Raga as ::yaserde::YaDeserialize>::deserialize(reader)?;
                                    __ragas_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "Tala" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <Tala as ::yaserde::YaDeserialize>::deserialize(reader)?;
                                    __talas_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "Deity" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <Deity as ::yaserde::YaDeserialize>::deserialize(reader)?;
                                    __deitys_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "HiResMusicDescription" => {
                                let visitor = __Visitor_HiResMusicDescription_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "HiResMusicDescription"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __hi_res_music_description_value =
                                        ::std::option::Option::Some(value)
                                }
                            }
                            "IsSoundtrack" => {
                                let visitor = __Visitor_IsSoundtrack_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result = reader.read_inner_value::<bool, _>(|reader| {
                                    if let ::std::result::Result::Ok(
                                        ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                    ) = reader.peek()
                                    {
                                        let val = visitor.visit_bool(&s);
                                        let _event = reader.next_event()?;
                                        val
                                    } else {
                                        ::std::result::Result::Err(::std::format!(
                                            "unable to parse content for {}",
                                            "IsSoundtrack"
                                        ))
                                    }
                                });
                                if let ::std::result::Result::Ok(value) = result {
                                    __is_soundtrack_value = ::std::option::Option::Some(value)
                                }
                            }
                            "IsHiResMusic" => {
                                let visitor = __Visitor_IsHiResMusic_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result = reader.read_inner_value::<bool, _>(|reader| {
                                    if let ::std::result::Result::Ok(
                                        ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                    ) = reader.peek()
                                    {
                                        let val = visitor.visit_bool(&s);
                                        let _event = reader.next_event()?;
                                        val
                                    } else {
                                        ::std::result::Result::Err(::std::format!(
                                            "unable to parse content for {}",
                                            "IsHiResMusic"
                                        ))
                                    }
                                });
                                if let ::std::result::Result::Ok(value) = result {
                                    __is_hi_res_music_value = ::std::option::Option::Some(value)
                                }
                            }
                            "MarketingComment" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <MarketingComment as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __marketing_comments_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "IsSingleArtistCompilation" => {
                                let visitor = __Visitor_IsSingleArtistCompilation_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result = reader.read_inner_value::<bool, _>(|reader| {
                                    if let ::std::result::Result::Ok(
                                        ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                    ) = reader.peek()
                                    {
                                        let val = visitor.visit_bool(&s);
                                        let _event = reader.next_event()?;
                                        val
                                    } else {
                                        ::std::result::Result::Err(::std::format!(
                                            "unable to parse content for {}",
                                            "IsSingleArtistCompilation"
                                        ))
                                    }
                                });
                                if let ::std::result::Result::Ok(value) = result {
                                    __is_single_artist_compilation_value =
                                        ::std::option::Option::Some(value)
                                }
                            }
                            "IsMultiArtistCompilation" => {
                                let visitor = __Visitor_IsMultiArtistCompilation_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result = reader.read_inner_value::<bool, _>(|reader| {
                                    if let ::std::result::Result::Ok(
                                        ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                    ) = reader.peek()
                                    {
                                        let val = visitor.visit_bool(&s);
                                        let _event = reader.next_event()?;
                                        val
                                    } else {
                                        ::std::result::Result::Err(::std::format!(
                                            "unable to parse content for {}",
                                            "IsMultiArtistCompilation"
                                        ))
                                    }
                                });
                                if let ::std::result::Result::Ok(value) = result {
                                    __is_multi_artist_compilation_value =
                                        ::std::option::Option::Some(value)
                                }
                            }
                            _ => {
                                let event = reader.next_event()?;
                                if depth > 0 {
                                    reader.skip_element(|event| {})?;
                                }
                            }
                        }
                    }
                    if depth == 0 {
                        for attr in attributes {
                            if attr.name.local_name == "LanguageAndScriptCode" {
                                let visitor = __Visitor_Attribute_LanguageAndScriptCode_ {};
                                let value = visitor.visit_str(&attr.value)?;
                                __language_and_script_code_value =
                                    ::std::option::Option::Some(value);
                            }
                        }
                    }
                    depth += 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndElement { ref name } => {
                    if name.local_name == named_element && reader.depth() == start_depth + 1 {
                        break;
                    }
                    let event = reader.next_event()?;
                    depth -= 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndDocument => {
                    if false {
                        break;
                    }
                }
                ::yaserde::__xml::reader::XmlEvent::Characters(ref text_content) => {
                    let event = reader.next_event()?;
                }
                event => {
                    return ::std::result::Result::Err(::std::format!("unknown event {:?}", event));
                }
            }
        }

        if __is_single_artist_compilation_value.is_some()
            && __is_multi_artist_compilation_value.is_some()
        {
            Err(
                "Release: Select IsSingleArtistCompilation or IsMultiArtistCompilationValue"
                    .to_string(),
            )?
        }

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: success",
            stringify!(Release),
            start_depth
        );

        ::std::result::Result::Ok(Release {
            release_reference: match __release_reference_value {
                Some(val) => {
                    if ReleaseReferenceValidator::validate(&val) {
                        val
                    } else {
                        Err("invalid value in field Release".to_string())?
                    }
                }
                None => Err("release_reference is a required field of Release".to_string())?,
            },
            release_types: __release_types_value,
            release_id: __release_id_value
                .ok_or_else(|| "release_id is a required field of Release".to_string())?,
            display_title_texts: __display_title_texts_value,
            display_titles: __display_titles_value,
            additional_titles: __additional_titles_value,
            version_types: __version_types_value,
            display_artist_names: __display_artist_names_value,
            display_artists: __display_artists_value,
            release_label_references: __release_label_references_value,
            administrating_record_companys: __administrating_record_companys_value,
            p_lines: __p_lines_value,
            c_lines: __c_lines_value,
            courtesy_lines: __courtesy_lines_value,
            duration: __duration_value,
            genres: __genres_value,
            release_dates: __release_dates_value,
            original_release_dates: __original_release_dates_value,
            release_visibility_references: if __release_visibility_references_value.len() > 0 {
                let is_valid = &__release_visibility_references_value
                    .iter()
                    .all(|val| ReleaseVisibilityReferenceValidator::validate(&val));
                if *is_valid {
                    __release_visibility_references_value
                } else {
                    Err(format!(
                        "invalid value in field {}",
                        "ReleaseVisibilityReference"
                    ))?
                }
            } else {
                __release_visibility_references_value
            },
            parental_warning_types: __parental_warning_types_value,
            av_ratings: __av_ratings_value,
            related_releases: __related_releases_value,
            related_resources: __related_resources_value,
            resource_group: __resource_group_value
                .ok_or_else(|| "resource_group is a required field of Release".to_string())?,
            external_resource_links: __external_resource_links_value,
            target_url: __target_url_value,
            keywordss: __keywordss_value,
            synopsiss: __synopsiss_value,
            ragas: __ragas_value,
            talas: __talas_value,
            deitys: __deitys_value,
            hi_res_music_description: __hi_res_music_description_value,
            is_soundtrack: __is_soundtrack_value,
            is_hi_res_music: __is_hi_res_music_value,
            marketing_comments: __marketing_comments_value,
            language_and_script_code: match &__language_and_script_code_value {
                Some(val) => {
                    if LanguageAndScriptCodeValidator::validate(&val) {
                        __language_and_script_code_value
                    } else {
                        Err(format!(
                            "invalid value in attribute {}",
                            "LanguageAndScriptCode"
                        ))?
                    }
                }
                None => __language_and_script_code_value,
            },
            is_single_artist_compilation: __is_single_artist_compilation_value,
            is_multi_artist_compilation: __is_multi_artist_compilation_value,
        })
    }
}

#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
impl ::yaserde::YaDeserialize for ReleaseVisibility {
    #[allow(unused_variables)]
    fn deserialize<R: ::std::io::Read>(
        reader: &mut ::yaserde::de::Deserializer<R>,
    ) -> ::std::result::Result<Self, ::std::string::String> {
        let (named_element, struct_namespace) =
            if let ::yaserde::__xml::reader::XmlEvent::StartElement { name, .. } =
                reader.peek()?.to_owned()
            {
                (name.local_name.to_owned(), name.namespace.clone())
            } else {
                (
                    ::std::string::String::from("ern:ReleaseVisibility"),
                    ::std::option::Option::None,
                )
            };
        let start_depth = reader.depth();

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: start to parse {:?}",
            stringify!(ReleaseVisibility),
            start_depth,
            named_element
        );

        if reader.depth() == 0 {
            if let Some(namespace) = struct_namespace {
                match namespace.as_str() {
                    "http://ddex.net/xml/ern/43" => {}

                    bad_namespace => {
                        let msg = ::std::format!(
                            "bad namespace for {}, found {}",
                            named_element,
                            bad_namespace
                        );
                        return Err(msg);
                    }
                }
            }
        }
        #[allow(unused_mut)]
        let mut __visibility_reference_value: Option<::std::string::String> = None;
        #[allow(unused_mut)]
        let mut __release_display_start_date_time_value = None;
        #[allow(unused_mut)]
        let mut __cover_art_preview_start_date_time_value = None;
        #[allow(unused_mut)]
        let mut __full_track_listing_preview_start_date_time_value = None;
        #[allow(unused_mut)]
        let mut __clip_preview_start_date_time_value = None;
        #[allow(unused_mut)]
        let mut __do_not_display_dates_value = None;
        #[allow(unused_mut)]
        let mut __territory_codes_value: ::std::vec::Vec<::std::string::String> = vec![];
        #[allow(unused_mut)]
        let mut __excluded_territory_codes_value: ::std::vec::Vec<::std::string::String> = vec![];
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_VisibilityReference_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_VisibilityReference_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_ReleaseDisplayStartDateTime_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_ReleaseDisplayStartDateTime_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_CoverArtPreviewStartDateTime_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_CoverArtPreviewStartDateTime_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_FullTrackListingPreviewStartDateTime_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_FullTrackListingPreviewStartDateTime_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_ClipPreviewStartDateTime_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_ClipPreviewStartDateTime_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_Attribute_DoNotDisplayDates_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_Attribute_DoNotDisplayDates_ {
            type Value = bool;
            fn visit_bool(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                bool::from_str(match v {
                    "1" => "true",
                    "0" => "false",
                    _ => v,
                })
                .map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_TerritoryCode_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_TerritoryCode_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_ExcludedTerritoryCode_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_ExcludedTerritoryCode_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        let mut depth = 0;
        loop {
            let event = reader.peek()?.to_owned();

            ::yaserde::__derive_trace!(
                "Struct {} @ {}: matching {:?}",
                stringify!(ReleaseVisibility),
                start_depth,
                event
            );

            match event {
                ::yaserde::__xml::reader::XmlEvent::StartElement {
                    ref name,
                    ref attributes,
                    ..
                } => {
                    if depth == 0 && name.local_name == "ern:ReleaseVisibility" {
                        let event = reader.next_event()?;
                    } else {
                        match name.local_name.as_str() {
                            "VisibilityReference" => {
                                let visitor = __Visitor_VisibilityReference_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "VisibilityReference"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __visibility_reference_value =
                                        ::std::option::Option::Some(value)
                                }
                            }
                            "ReleaseDisplayStartDateTime" => {
                                let visitor = __Visitor_ReleaseDisplayStartDateTime_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "ReleaseDisplayStartDateTime"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __release_display_start_date_time_value =
                                        ::std::option::Option::Some(value)
                                }
                            }
                            "CoverArtPreviewStartDateTime" => {
                                let visitor = __Visitor_CoverArtPreviewStartDateTime_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "CoverArtPreviewStartDateTime"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __cover_art_preview_start_date_time_value =
                                        ::std::option::Option::Some(value)
                                }
                            }
                            "FullTrackListingPreviewStartDateTime" => {
                                let visitor = __Visitor_FullTrackListingPreviewStartDateTime_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "FullTrackListingPreviewStartDateTime"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __full_track_listing_preview_start_date_time_value =
                                        ::std::option::Option::Some(value)
                                }
                            }
                            "ClipPreviewStartDateTime" => {
                                let visitor = __Visitor_ClipPreviewStartDateTime_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "ClipPreviewStartDateTime"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __clip_preview_start_date_time_value =
                                        ::std::option::Option::Some(value)
                                }
                            }
                            "TerritoryCode" => {
                                let visitor = __Visitor_TerritoryCode_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "TerritoryCode"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __territory_codes_value.push(value)
                                }
                            }
                            "ExcludedTerritoryCode" => {
                                let visitor = __Visitor_ExcludedTerritoryCode_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "ExcludedTerritoryCode"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __excluded_territory_codes_value.push(value)
                                }
                            }
                            _ => {
                                let event = reader.next_event()?;
                                if depth > 0 {
                                    reader.skip_element(|event| {})?;
                                }
                            }
                        }
                    }
                    if depth == 0 {
                        for attr in attributes {
                            if attr.name.local_name == "DoNotDisplayDates" {
                                let visitor = __Visitor_Attribute_DoNotDisplayDates_ {};
                                let value = visitor.visit_bool(&attr.value)?;
                                __do_not_display_dates_value = ::std::option::Option::Some(value);
                            }
                        }
                    }
                    depth += 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndElement { ref name } => {
                    if name.local_name == named_element && reader.depth() == start_depth + 1 {
                        break;
                    }
                    let event = reader.next_event()?;
                    depth -= 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndDocument => {
                    if false {
                        break;
                    }
                }
                ::yaserde::__xml::reader::XmlEvent::Characters(ref text_content) => {
                    let event = reader.next_event()?;
                }
                event => {
                    return ::std::result::Result::Err(::std::format!("unknown event {:?}", event));
                }
            }
        }

        if __territory_codes_value.len() > 0 && __excluded_territory_codes_value.len() > 0 {
            Err("TerritoryCodes and ExcludedTerritoryCodes are exclusive".to_string())?
        }

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: success",
            stringify!(ReleaseVisibility),
            start_depth
        );

        ::std::result::Result::Ok(ReleaseVisibility {
            visibility_reference: match __visibility_reference_value {
                Some(val) => {
                    if VisibilityReferenceValidator::validate(&val) {
                        val
                    } else {
                        Err("invalid value in field ReleaseVisibility".to_string())?
                    }
                }
                None => Err(
                    "visibility_reference is a required field of ReleaseVisibility".to_string(),
                )?,
            },
            release_display_start_date_time: __release_display_start_date_time_value,
            cover_art_preview_start_date_time: __cover_art_preview_start_date_time_value,
            full_track_listing_preview_start_date_time:
                __full_track_listing_preview_start_date_time_value,
            clip_preview_start_date_time: __clip_preview_start_date_time_value,
            do_not_display_dates: __do_not_display_dates_value,
            territory_codes: if __territory_codes_value.len() > 0 {
                let is_valid = &__territory_codes_value
                    .iter()
                    .all(|val| AvsCurrentTerritoryCodeValidator::validate(&val));
                if *is_valid {
                    __territory_codes_value
                } else {
                    Err(format!("invalid value in field {}", "TerritoryCode"))?
                }
            } else {
                __territory_codes_value
            },
            excluded_territory_codes: if __excluded_territory_codes_value.len() > 0 {
                let is_valid = &__excluded_territory_codes_value
                    .iter()
                    .all(|val| AvsCurrentTerritoryCodeValidator::validate(&val));
                if *is_valid {
                    __excluded_territory_codes_value
                } else {
                    Err(format!(
                        "invalid value in field {}",
                        "ExcludedTerritoryCode"
                    ))?
                }
            } else {
                __excluded_territory_codes_value
            },
        })
    }
}

// RSG
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
impl ::yaserde::YaDeserialize for ResourceGroup {
    #[allow(unused_variables)]
    fn deserialize<R: ::std::io::Read>(
        reader: &mut ::yaserde::de::Deserializer<R>,
    ) -> ::std::result::Result<Self, ::std::string::String> {
        let (named_element, struct_namespace) =
            if let ::yaserde::__xml::reader::XmlEvent::StartElement { name, .. } =
                reader.peek()?.to_owned()
            {
                (name.local_name.to_owned(), name.namespace.clone())
            } else {
                (
                    ::std::string::String::from("ern:ResourceGroup"),
                    ::std::option::Option::None,
                )
            };
        let start_depth = reader.depth();

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: start to parse {:?}",
            stringify!(ResourceGroup),
            start_depth,
            named_element
        );

        if reader.depth() == 0 {
            if let Some(namespace) = struct_namespace {
                match namespace.as_str() {
                    "http://ddex.net/xml/ern/43" => {}

                    bad_namespace => {
                        let msg = ::std::format!(
                            "bad namespace for {}, found {}",
                            named_element,
                            bad_namespace
                        );
                        return Err(msg);
                    }
                }
            }
        }
        #[allow(unused_mut)]
        let mut __display_title_texts_value: ::std::vec::Vec<DisplayTitleText> = vec![];
        #[allow(unused_mut)]
        let mut __display_titles_value: ::std::vec::Vec<DisplayTitle> = vec![];
        #[allow(unused_mut)]
        let mut __additional_titles_value: ::std::vec::Vec<AdditionalTitle> = vec![];
        #[allow(unused_mut)]
        let mut __sequence_number_value = None;
        #[allow(unused_mut)]
        let mut __display_artists_value: ::std::vec::Vec<DisplayArtist> = vec![];
        #[allow(unused_mut)]
        let mut __carrier_types_value: ::std::vec::Vec<CarrierType> = vec![];
        #[allow(unused_mut)]
        let mut __duration_value = None;
        #[allow(unused_mut)]
        let mut __resource_groups_value: ::std::vec::Vec<ResourceSubGroup> = vec![];
        #[allow(unused_mut)]
        let mut __resource_group_content_items_value: ::std::vec::Vec<
            ResourceGroupContentItem,
        > = vec![];
        #[allow(unused_mut)]
        let mut __linked_release_resource_references_value: ::std::vec::Vec<
            LinkedReleaseResourceReference,
        > = vec![];
        #[allow(unused_mut)]
        let mut __no_display_sequence_value = None;
        #[allow(unused_mut)]
        let mut __display_sequence_value = None;
        #[allow(unused_mut)]
        let mut __resource_group_release_reference_value = None;
        #[allow(unused_mut)]
        let mut __release_id_value = None;
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_SequenceNumber_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_SequenceNumber_ {
            type Value = i32;
            fn visit_i32(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                i32::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_Duration_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_Duration_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_NoDisplaySequence_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_NoDisplaySequence_ {
            type Value = bool;
            fn visit_bool(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                bool::from_str(match v {
                    "1" => "true",
                    "0" => "false",
                    _ => v,
                })
                .map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_DisplaySequence_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_DisplaySequence_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_ResourceGroupReleaseReference_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_ResourceGroupReleaseReference_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        let mut depth = 0;

        loop {
            let event = reader.peek()?.to_owned();

            ::yaserde::__derive_trace!(
                "Struct {} @ {}: matching {:?}",
                stringify!(ResourceGroup),
                start_depth,
                event
            );
        
            match event {
                ::yaserde::__xml::reader::XmlEvent::StartElement {
                    ref name,
                    ref attributes,
                    ..
                } => {
                    if depth == 0 && name.local_name == "ResourceGroup" {
                        let event = reader.next_event()?;
                    } else {
                        match name.local_name.as_str() {
                            "DisplayTitleText" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <DisplayTitleText as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __display_title_texts_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "DisplayTitle" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <DisplayTitle as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __display_titles_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "AdditionalTitle" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <AdditionalTitle as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __additional_titles_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "SequenceNumber" => {
                                let visitor = __Visitor_SequenceNumber_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result = reader.read_inner_value::<i32, _>(|reader| {
                                    if let ::std::result::Result::Ok(
                                        ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                    ) = reader.peek()
                                    {
                                        let val = visitor.visit_i32(&s);
                                        let _event = reader.next_event()?;
                                        val
                                    } else {
                                        ::std::result::Result::Err(::std::format!(
                                            "unable to parse content for {}",
                                            "SequenceNumber"
                                        ))
                                    }
                                });
                                if let ::std::result::Result::Ok(value) = result {
                                    __sequence_number_value = ::std::option::Option::Some(value)
                                }
                            }
                            "DisplayArtist" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <DisplayArtist as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __display_artists_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "CarrierType" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <CarrierType as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __carrier_types_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "Duration" => {
                                let visitor = __Visitor_Duration_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "Duration"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __duration_value = ::std::option::Option::Some(value)
                                }
                            }
                            "ResourceGroup" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                   
                                        let value =  <ResourceSubGroup as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                        __resource_groups_value.push(value);
                                   
                                    let _event = reader.next_event()?;
                                }
                            }
                            "ResourceGroupContentItem" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <ResourceGroupContentItem as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __resource_group_content_items_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "LinkedReleaseResourceReference" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <LinkedReleaseResourceReference as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __linked_release_resource_references_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "NoDisplaySequence" => {
                                let visitor = __Visitor_NoDisplaySequence_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result = reader.read_inner_value::<bool, _>(|reader| {
                                    if let ::std::result::Result::Ok(
                                        ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                    ) = reader.peek()
                                    {
                                        let val = visitor.visit_bool(&s);
                                        let _event = reader.next_event()?;
                                        val
                                    } else {
                                        ::std::result::Result::Err(::std::format!(
                                            "unable to parse content for {}",
                                            "NoDisplaySequence"
                                        ))
                                    }
                                });
                                if let ::std::result::Result::Ok(value) = result {
                                    __no_display_sequence_value = ::std::option::Option::Some(value)
                                }
                            }
                            "DisplaySequence" => {
                                let visitor = __Visitor_DisplaySequence_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "DisplaySequence"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __display_sequence_value = ::std::option::Option::Some(value)
                                }
                            }
                            "ResourceGroupReleaseReference" => {
                                let visitor = __Visitor_ResourceGroupReleaseReference_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "ResourceGroupReleaseReference"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __resource_group_release_reference_value =
                                        ::std::option::Option::Some(value)
                                }
                            }
                            "ReleaseId" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <ReleaseId as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __release_id_value = ::std::option::Option::Some(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            _ => {
                                let event = reader.next_event()?;
                                if depth > 0 {
                                    reader.skip_element(|event| {})?;
                                }
                            }
                        }
                    }
                    if depth == 0 {}
                    depth += 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndElement { ref name } => {
                    if name.local_name == named_element && reader.depth() == start_depth + 1 {
                        break;
                    }
                    let event = reader.next_event()?;
                    depth -= 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndDocument => {
                    if false {
                        break;
                    }
                }
                ::yaserde::__xml::reader::XmlEvent::Characters(ref text_content) => {
                    let event = reader.next_event()?;
                }
                event => {
                    return ::std::result::Result::Err(::std::format!("unknown event {:?}", event));
                }
            }
        }

        if __no_display_sequence_value.is_some() && __display_sequence_value.is_some() {
            Err("ResourceGroup: DisplaySequence and NoDisplaySequence are exclusive".to_string())?
        }

        if __resource_group_release_reference_value.is_some() && __release_id_value.is_some() {
            Err(
                "ResourceGroup: ResourceGroupReleaseReference and ReleaseId are exclusive"
                    .to_string(),
            )?
        }

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: success",
            stringify!(ResourceGroup),
            start_depth
        );

        ::std::result::Result::Ok(ResourceGroup {
            display_title_texts: __display_title_texts_value,
            display_titles: __display_titles_value,
            additional_titles: __additional_titles_value,
            sequence_number: __sequence_number_value,
            display_artists: __display_artists_value,
            carrier_types: __carrier_types_value,
            duration: __duration_value,
            resource_groups: __resource_groups_value,
            resource_group_content_items: __resource_group_content_items_value,
            linked_release_resource_references: __linked_release_resource_references_value,
            no_display_sequence: __no_display_sequence_value,
            display_sequence: __display_sequence_value,
            resource_group_release_reference: match &__resource_group_release_reference_value {
                Some(val) => {
                    if ResourceGroupReleaseReferenceValidator::validate(&val) {
                        __resource_group_release_reference_value
                    } else {
                        Err(format!(
                            "invalid value in attribute {}",
                            "ResourceGroupReleaseReference"
                        ))?
                    }
                }
                None => __resource_group_release_reference_value,
            },
            release_id: __release_id_value,
        })
    }
}

#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
impl ::yaserde::YaDeserialize for ResourceGroupContentItem {
    #[allow(unused_variables)]
    fn deserialize<R: ::std::io::Read>(
        reader: &mut ::yaserde::de::Deserializer<R>,
    ) -> ::std::result::Result<Self, ::std::string::String> {
        let (named_element, struct_namespace) =
            if let ::yaserde::__xml::reader::XmlEvent::StartElement { name, .. } =
                reader.peek()?.to_owned()
            {
                (name.local_name.to_owned(), name.namespace.clone())
            } else {
                (
                    ::std::string::String::from("ern:ResourceGroupContentItem"),
                    ::std::option::Option::None,
                )
            };
        let start_depth = reader.depth();

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: start to parse {:?}",
            stringify!(ResourceGroupContentItem),
            start_depth,
            named_element
        );

        if reader.depth() == 0 {
            if let Some(namespace) = struct_namespace {
                match namespace.as_str() {
                    "http://ddex.net/xml/ern/43" => {}

                    bad_namespace => {
                        let msg = ::std::format!(
                            "bad namespace for {}, found {}",
                            named_element,
                            bad_namespace
                        );
                        return Err(msg);
                    }
                }
            }
        }
        #[allow(unused_mut)]
        let mut __sequence_number_value = None;
        #[allow(unused_mut)]
        let mut __release_resource_reference_value: Option<::std::string::String> = None;
        #[allow(unused_mut)]
        let mut __linked_release_resource_references_value: ::std::vec::Vec<
            LinkedReleaseResourceReference,
        > = vec![];
        #[allow(unused_mut)]
        let mut __is_bonus_resource_value = None;
        #[allow(unused_mut)]
        let mut __is_instant_gratification_resource_value = None;
        #[allow(unused_mut)]
        let mut __is_pre_order_incentive_resource_value = None;
        #[allow(unused_mut)]
        let mut __no_display_sequence_value = None;
        #[allow(unused_mut)]
        let mut __display_sequence_value = None;
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_SequenceNumber_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_SequenceNumber_ {
            type Value = i32;
            fn visit_i32(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                i32::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_ReleaseResourceReference_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_ReleaseResourceReference_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_IsBonusResource_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_IsBonusResource_ {
            type Value = bool;
            fn visit_bool(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                bool::from_str(match v {
                    "1" => "true",
                    "0" => "false",
                    _ => v,
                })
                .map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_IsInstantGratificationResource_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_IsInstantGratificationResource_ {
            type Value = bool;
            fn visit_bool(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                bool::from_str(match v {
                    "1" => "true",
                    "0" => "false",
                    _ => v,
                })
                .map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_IsPreOrderIncentiveResource_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_IsPreOrderIncentiveResource_ {
            type Value = bool;
            fn visit_bool(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                bool::from_str(match v {
                    "1" => "true",
                    "0" => "false",
                    _ => v,
                })
                .map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_NoDisplaySequence_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_NoDisplaySequence_ {
            type Value = bool;
            fn visit_bool(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                bool::from_str(match v {
                    "1" => "true",
                    "0" => "false",
                    _ => v,
                })
                .map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_DisplaySequence_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_DisplaySequence_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        let mut depth = 0;
        loop {
            let event = reader.peek()?.to_owned();

            ::yaserde::__derive_trace!(
                "Struct {} @ {}: matching {:?}",
                stringify!(ResourceGroupContentItem),
                start_depth,
                event
            );

            match event {
                ::yaserde::__xml::reader::XmlEvent::StartElement {
                    ref name,
                    ref attributes,
                    ..
                } => {
                    if depth == 0 && name.local_name == "ern:ResourceGroupContentItem" {
                        let event = reader.next_event()?;
                    } else {
                        match name.local_name.as_str() {
                            "SequenceNumber" => {
                                let visitor = __Visitor_SequenceNumber_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result = reader.read_inner_value::<i32, _>(|reader| {
                                    if let ::std::result::Result::Ok(
                                        ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                    ) = reader.peek()
                                    {
                                        let val = visitor.visit_i32(&s);
                                        let _event = reader.next_event()?;
                                        val
                                    } else {
                                        ::std::result::Result::Err(::std::format!(
                                            "unable to parse content for {}",
                                            "SequenceNumber"
                                        ))
                                    }
                                });
                                if let ::std::result::Result::Ok(value) = result {
                                    __sequence_number_value = ::std::option::Option::Some(value)
                                }
                            }
                            "ReleaseResourceReference" => {
                                let visitor = __Visitor_ReleaseResourceReference_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "ReleaseResourceReference"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __release_resource_reference_value =
                                        ::std::option::Option::Some(value)
                                }
                            }
                            "LinkedReleaseResourceReference" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <LinkedReleaseResourceReference as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __linked_release_resource_references_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "IsBonusResource" => {
                                let visitor = __Visitor_IsBonusResource_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result = reader.read_inner_value::<bool, _>(|reader| {
                                    if let ::std::result::Result::Ok(
                                        ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                    ) = reader.peek()
                                    {
                                        let val = visitor.visit_bool(&s);
                                        let _event = reader.next_event()?;
                                        val
                                    } else {
                                        ::std::result::Result::Err(::std::format!(
                                            "unable to parse content for {}",
                                            "IsBonusResource"
                                        ))
                                    }
                                });
                                if let ::std::result::Result::Ok(value) = result {
                                    __is_bonus_resource_value = ::std::option::Option::Some(value)
                                }
                            }
                            "IsInstantGratificationResource" => {
                                let visitor = __Visitor_IsInstantGratificationResource_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result = reader.read_inner_value::<bool, _>(|reader| {
                                    if let ::std::result::Result::Ok(
                                        ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                    ) = reader.peek()
                                    {
                                        let val = visitor.visit_bool(&s);
                                        let _event = reader.next_event()?;
                                        val
                                    } else {
                                        ::std::result::Result::Err(::std::format!(
                                            "unable to parse content for {}",
                                            "IsInstantGratificationResource"
                                        ))
                                    }
                                });
                                if let ::std::result::Result::Ok(value) = result {
                                    __is_instant_gratification_resource_value =
                                        ::std::option::Option::Some(value)
                                }
                            }
                            "IsPreOrderIncentiveResource" => {
                                let visitor = __Visitor_IsPreOrderIncentiveResource_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result = reader.read_inner_value::<bool, _>(|reader| {
                                    if let ::std::result::Result::Ok(
                                        ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                    ) = reader.peek()
                                    {
                                        let val = visitor.visit_bool(&s);
                                        let _event = reader.next_event()?;
                                        val
                                    } else {
                                        ::std::result::Result::Err(::std::format!(
                                            "unable to parse content for {}",
                                            "IsPreOrderIncentiveResource"
                                        ))
                                    }
                                });
                                if let ::std::result::Result::Ok(value) = result {
                                    __is_pre_order_incentive_resource_value =
                                        ::std::option::Option::Some(value)
                                }
                            }
                            "NoDisplaySequence" => {
                                let visitor = __Visitor_NoDisplaySequence_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result = reader.read_inner_value::<bool, _>(|reader| {
                                    if let ::std::result::Result::Ok(
                                        ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                    ) = reader.peek()
                                    {
                                        let val = visitor.visit_bool(&s);
                                        let _event = reader.next_event()?;
                                        val
                                    } else {
                                        ::std::result::Result::Err(::std::format!(
                                            "unable to parse content for {}",
                                            "NoDisplaySequence"
                                        ))
                                    }
                                });
                                if let ::std::result::Result::Ok(value) = result {
                                    __no_display_sequence_value = ::std::option::Option::Some(value)
                                }
                            }
                            "DisplaySequence" => {
                                let visitor = __Visitor_DisplaySequence_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "DisplaySequence"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __display_sequence_value = ::std::option::Option::Some(value)
                                }
                            }
                            _ => {
                                let event = reader.next_event()?;
                                if depth > 0 {
                                    reader.skip_element(|event| {})?;
                                }
                            }
                        }
                    }
                    if depth == 0 {}
                    depth += 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndElement { ref name } => {
                    if name.local_name == named_element && reader.depth() == start_depth + 1 {
                        break;
                    }
                
                    let event = reader.next_event()?;
                    depth -= 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndDocument => {
                    if false {
                        break;
                    }
                }
                ::yaserde::__xml::reader::XmlEvent::Characters(ref text_content) => {
                    let event = reader.next_event()?;
                }
                event => {
                    return ::std::result::Result::Err(::std::format!("unknown event {:?}", event));
                }
            }
        }

        if __no_display_sequence_value.is_some() && __display_sequence_value.is_some() {
            Err(
                "ResourceGroupContentItem: DisplaySequence and NoDisplaySequence are exclusive"
                    .to_string(),
            )?
        }

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: success",
            stringify!(ResourceGroupContentItem),
            start_depth
        );

        ::std::result::Result::Ok(ResourceGroupContentItem {
            sequence_number: __sequence_number_value,
            release_resource_reference: match __release_resource_reference_value {
                Some(val) => {
                    if ReleaseResourceReferenceValidator::validate(&val) {
                        val
                    } else {
                        Err("invalid value in field ResourceGroupContentItem".to_string())?
                    }
                }
                None => Err(
                    "release_resource_reference is a required field of ResourceGroupContentItem"
                        .to_string(),
                )?,
            },
            linked_release_resource_references: __linked_release_resource_references_value,
            is_bonus_resource: __is_bonus_resource_value,
            is_instant_gratification_resource: __is_instant_gratification_resource_value,
            is_pre_order_incentive_resource: __is_pre_order_incentive_resource_value,
            no_display_sequence: __no_display_sequence_value,
            display_sequence: __display_sequence_value,
        })
    }
}

#[allow(non_upper_case_globals,unused_attributes,unused_qualifications)]
impl ::yaserde::YaDeserialize for ResourceRightsController {
    #[allow(unused_variables)]
    fn deserialize<R: ::std::io::Read>(reader: &mut ::yaserde::de::Deserializer<R> ,) ->  ::std::result::Result<Self, ::std::string::String>{
        let (named_element,struct_namespace) = if let::yaserde::__xml::reader::XmlEvent::StartElement {
            name, ..
        } = reader.peek()? .to_owned(){
            (name.local_name.to_owned(),name.namespace.clone())
        } else {
            (::std::string::String::from("ern:ResourceRightsController"), ::std::option::Option::None)
        };
        let start_depth = reader.depth();

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: start to parse {:?}",
            stringify!(ResourceRightsController),
            start_depth,
            named_element
        );

        if reader.depth()==0 {
            if let Some(namespace) = struct_namespace {
                match namespace.as_str(){
                    "http://ddex.net/xml/ern/43" => {}
                    
                    bad_namespace => {
                        let msg =  ::std::format!("bad namespace for {}, found {}",named_element,bad_namespace);
                        return Err(msg);
                    }
                
                    }
            }
        }
        #[allow(unused_mut)]
        let mut __rights_controller_party_reference_value:Option:: < ::std::string::String>  = None;
        #[allow(unused_mut)]
        let mut __rights_control_types_value: ::std::vec::Vec< ::std::string::String>  = vec![];
        #[allow(unused_mut)]
        let mut __delegated_usage_rightss_value: ::std::vec::Vec<DelegatedUsageRights>  = vec![];
        #[allow(unused_mut)]
        let mut __sequence_number_value = None;
        #[allow(unused_mut)]
        let mut __right_share_unknown_value = None;
        #[allow(unused_mut)]
        let mut __right_share_percentage_value = None;
        #[allow(non_snake_case,non_camel_case_types)]
        struct __Visitor_RightsControllerPartyReference_;
        
        impl <'de> ::yaserde::Visitor<'de>for __Visitor_RightsControllerPartyReference_ {
            type Value =  ::std::string::String;
            fn visit_str(self,v: &str,) ->  ::std::result::Result<Self::Value, ::std::string::String>{
                ::std::string::String::from_str(v).map_err(|e|e.to_string())
            }
        
            }
        #[allow(non_snake_case,non_camel_case_types)]
        struct __Visitor_RightsControlType_;
        
        impl <'de> ::yaserde::Visitor<'de>for __Visitor_RightsControlType_ {
            type Value =  ::std::string::String;
            fn visit_str(self,v: &str,) ->  ::std::result::Result<Self::Value, ::std::string::String>{
                ::std::string::String::from_str(v).map_err(|e|e.to_string())
            }
        
            }
        #[allow(non_snake_case,non_camel_case_types)]
        struct __Visitor_Attribute_SequenceNumber_;
        
        impl <'de> ::yaserde::Visitor<'de>for __Visitor_Attribute_SequenceNumber_ {
            type Value = i32;
            fn visit_i32(self,v: &str,) ->  ::std::result::Result<Self::Value, ::std::string::String>{
                i32::from_str(v).map_err(|e|e.to_string())
            }
        
            }
        #[allow(non_snake_case,non_camel_case_types)]
        struct __Visitor_RightShareUnknown_;
        
        impl <'de> ::yaserde::Visitor<'de>for __Visitor_RightShareUnknown_ {
            type Value = bool;
            fn visit_bool(self,v: &str,) ->  ::std::result::Result<Self::Value, ::std::string::String>{
                bool::from_str(match v {
                    "1" => "true",
                    "0" => "false",
                    _ => v,
                
                    }).map_err(|e|e.to_string())
            }
        
            }
        #[allow(non_snake_case,non_camel_case_types)]
        struct __Visitor_RightSharePercentage_;
        
        impl <'de> ::yaserde::Visitor<'de>for __Visitor_RightSharePercentage_ {
            type Value =  ::std::string::String;
            fn visit_str(self,v: &str,) ->  ::std::result::Result<Self::Value, ::std::string::String>{
                ::std::string::String::from_str(v).map_err(|e|e.to_string())
            }
        
            }
        let mut depth = 0;
        loop {
            let event = reader.peek()? .to_owned();
            match event {
                ::yaserde::__xml::reader::XmlEvent::StartElement {
                    ref name,ref attributes, ..
                } => {
                    if depth==0 && name.local_name=="ern:ResourceRightsController" {
                        let event = reader.next_event()? ;
                    } else {
                        match name.local_name.as_str(){
                            "RightsControllerPartyReference" => {
                                let visitor = __Visitor_RightsControllerPartyReference_{}
                                ;
                                if let Some(namespace) = name.namespace.as_ref(){
                                    match namespace.as_str(){
                                        "http://ddex.net/xml/ern/43" => {}
                                        
                                        bad_namespace => {
                                            let msg =  ::std::format!("bad namespace for {}, found {}",name.local_name.as_str(),bad_namespace);
                                            return Err(msg);
                                        }
                                    
                                        }
                                }let result = reader.read_inner_value:: < ::std::string::String,_>(|reader|{
                                    if let::std::result::Result::Ok(::yaserde::__xml::reader::XmlEvent::Characters(s)) = reader.peek(){
                                        let val = visitor.visit_str(&s);
                                        let _event = reader.next_event()? ;
                                        val
                                    }else {
                                        ::std::result::Result::Err(::std::format!("unable to parse content for {}","RightsControllerPartyReference"))
                                    }
                                });
                                if let::std::result::Result::Ok(value) = result {
                                    __rights_controller_party_reference_value =  ::std::option::Option::Some(value)
                                }
                            }
                            "RightsControlType" => {
                                let visitor = __Visitor_RightsControlType_{}
                                ;
                                if let Some(namespace) = name.namespace.as_ref(){
                                    match namespace.as_str(){
                                        "http://ddex.net/xml/ern/43" => {}
                                        
                                        bad_namespace => {
                                            let msg =  ::std::format!("bad namespace for {}, found {}",name.local_name.as_str(),bad_namespace);
                                            return Err(msg);
                                        }
                                    
                                        }
                                }let result = reader.read_inner_value:: < ::std::string::String,_>(|reader|{
                                    if let::std::result::Result::Ok(::yaserde::__xml::reader::XmlEvent::Characters(s)) = reader.peek(){
                                        let val = visitor.visit_str(&s);
                                        let _event = reader.next_event()? ;
                                        val
                                    }else {
                                        ::std::result::Result::Err(::std::format!("unable to parse content for {}","RightsControlType"))
                                    }
                                });
                                if let::std::result::Result::Ok(value) = result {
                                    __rights_control_types_value.push(value)
                                }
                            }
                            "DelegatedUsageRights" => {
                                if depth==0 {
                                    let _root = reader.next_event();
                                }if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek(){
                                    let value =  <DelegatedUsageRights as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __delegated_usage_rightss_value.push(value);
                                    let _event = reader.next_event()? ;
                                }
                            }
                            "RightShareUnknown" => {
                                let visitor = __Visitor_RightShareUnknown_{}
                                ;
                                if let Some(namespace) = name.namespace.as_ref(){
                                    match namespace.as_str(){
                                        "http://ddex.net/xml/ern/43" => {}
                                        
                                        bad_namespace => {
                                            let msg =  ::std::format!("bad namespace for {}, found {}",name.local_name.as_str(),bad_namespace);
                                            return Err(msg);
                                        }
                                    
                                        }
                                }let result = reader.read_inner_value:: <bool,_>(|reader|{
                                    if let::std::result::Result::Ok(::yaserde::__xml::reader::XmlEvent::Characters(s)) = reader.peek(){
                                        let val = visitor.visit_bool(&s);
                                        let _event = reader.next_event()? ;
                                        val
                                    }else {
                                        ::std::result::Result::Err(::std::format!("unable to parse content for {}","RightShareUnknown"))
                                    }
                                });
                                if let::std::result::Result::Ok(value) = result {
                                    __right_share_unknown_value =  ::std::option::Option::Some(value)
                                }
                            }
                            "RightSharePercentage" => {
                                let visitor = __Visitor_RightSharePercentage_{}
                                ;
                                if let Some(namespace) = name.namespace.as_ref(){
                                    match namespace.as_str(){
                                        "http://ddex.net/xml/ern/43" => {}
                                        
                                        bad_namespace => {
                                            let msg =  ::std::format!("bad namespace for {}, found {}",name.local_name.as_str(),bad_namespace);
                                            return Err(msg);
                                        }
                                    
                                        }
                                }let result = reader.read_inner_value:: < ::std::string::String,_>(|reader|{
                                    if let::std::result::Result::Ok(::yaserde::__xml::reader::XmlEvent::Characters(s)) = reader.peek(){
                                        let val = visitor.visit_str(&s);
                                        let _event = reader.next_event()? ;
                                        val
                                    }else {
                                        ::std::result::Result::Err(::std::format!("unable to parse content for {}","RightSharePercentage"))
                                    }
                                });
                                if let::std::result::Result::Ok(value) = result {
                                    __right_share_percentage_value =  ::std::option::Option::Some(value)
                                }
                            }
                            _ => {
                                let event = reader.next_event()? ;
                                if depth>0 {
                                    reader.skip_element(|event|{}
                                    )? ;
                                }
                            }
                        
                        }
                    } 
                    if depth==0 {
                        for attr in attributes {
                            if attr.name.local_name=="SequenceNumber" {
                                let visitor = __Visitor_Attribute_SequenceNumber_{}
                                ;
                                let value = visitor.visit_i32(&attr.value)? ;
                                __sequence_number_value =  ::std::option::Option::Some(value);
                            }
                        }
                    }
                    depth+=1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndElement {
                    ref name
                } => {
                    if name.local_name==named_element&&reader.depth()==start_depth+1 {
                        break;
                    }let event = reader.next_event()? ;
                    depth-=1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndDocument => {
                    if false {
                        break;
                    }
                }
                ::yaserde::__xml::reader::XmlEvent::Characters(ref text_content) => {
                    let event = reader.next_event()? ;
                }
                event => {
                    return::std::result::Result::Err(::std::format!("unknown event {:?}",event));
                }
            
                }
        }

        if __right_share_unknown_value.is_some() && __right_share_percentage_value.is_some() {
            Err("ResourceRightsController: RightShareUnknown and RightSharePercentage are exclusive".to_string())?
        }

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: success",
            stringify!(ResourceRightsController),
            start_depth
        );

        ::std::result::Result::Ok(ResourceRightsController {
            rights_controller_party_reference:match __rights_controller_party_reference_value {
                Some(val) => {
                    if RightsControllerPartyReferenceValidator::validate(&val){
                        val
                    }else {
                        Err("invalid value in field ResourceRightsController".to_string())?
                    }
                }
                None => Err("rights_controller_party_reference is a required field of ResourceRightsController".to_string())? ,
            
                },rights_control_types:if __rights_control_types_value.len()>0 {
                let is_valid =  &__rights_control_types_value.iter().all(|val|AvsRightsControllerRoleValidator::validate(&val));
                if*is_valid {
                    __rights_control_types_value
                }else {
                    Err(format!("invalid value in field {}","RightsControlType"))?
                }
            }else {
                __rights_control_types_value
            },
            delegated_usage_rightss:__delegated_usage_rightss_value,
            sequence_number:__sequence_number_value,
            right_share_unknown:__right_share_unknown_value,
            right_share_percentage:__right_share_percentage_value,
        })
    }
}

// RSSG
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
impl ::yaserde::YaDeserialize for ResourceSubGroup {
    #[allow(unused_variables)]
    fn deserialize<R: ::std::io::Read>(
        reader: &mut ::yaserde::de::Deserializer<R>,
    ) -> ::std::result::Result<Self, ::std::string::String> {
        let (named_element, struct_namespace) =
            if let ::yaserde::__xml::reader::XmlEvent::StartElement { name, .. } =
                reader.peek()?.to_owned()
            {
                (name.local_name.to_owned(), name.namespace.clone())
            } else {
                (
                    ::std::string::String::from("ern:ResourceSubGroup"),
                    ::std::option::Option::None,
                )
            };
        let start_depth = reader.depth();

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: start to parse {:?}",
            stringify!(ResourceSubGroup),
            start_depth,
            named_element
        );
        
        if reader.depth() == 0 {
            if let Some(namespace) = struct_namespace {
                match namespace.as_str() {
                    "http://ddex.net/xml/ern/43" => {}

                    bad_namespace => {
                        let msg = ::std::format!(
                            "bad namespace for {}, found {}",
                            named_element,
                            bad_namespace
                        );
                        return Err(msg);
                    }
                }
            }
        }
        #[allow(unused_mut)]
        let mut __display_title_texts_value: ::std::vec::Vec<DisplayTitleText> = vec![];
        #[allow(unused_mut)]
        let mut __display_titles_value: ::std::vec::Vec<DisplayTitle> = vec![];
        #[allow(unused_mut)]
        let mut __additional_titles_value: ::std::vec::Vec<AdditionalTitle> = vec![];
        #[allow(unused_mut)]
        let mut __sequence_number_value = None;
        #[allow(unused_mut)]
        let mut __display_artists_value: ::std::vec::Vec<DisplayArtist> = vec![];
        #[allow(unused_mut)]
        let mut __carrier_types_value: ::std::vec::Vec<CarrierType> = vec![];
        #[allow(unused_mut)]
        let mut __duration_value = None;
        #[allow(unused_mut)]
        let mut __resource_groups_value: ::std::vec::Vec<ResourceSubGroup> = vec![];
        #[allow(unused_mut)]
        let mut __resource_group_content_items_value: ::std::vec::Vec<
            ResourceGroupContentItem,
        > = vec![];
        #[allow(unused_mut)]
        let mut __linked_release_resource_references_value: ::std::vec::Vec<
            LinkedReleaseResourceReference,
        > = vec![];
        #[allow(unused_mut)]
        let mut __resource_group_type_value: Option<::std::string::String> = None;
        #[allow(unused_mut)]
        let mut __no_display_sequence_value = None;
        #[allow(unused_mut)]
        let mut __display_sequence_value = None;
        #[allow(unused_mut)]
        let mut __resource_group_release_reference_value = None;
        #[allow(unused_mut)]
        let mut __release_id_value = None;
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_SequenceNumber_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_SequenceNumber_ {
            type Value = i32;
            fn visit_i32(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                i32::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_Duration_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_Duration_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_Attribute_ResourceGroupType_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_Attribute_ResourceGroupType_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_NoDisplaySequence_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_NoDisplaySequence_ {
            type Value = bool;
            fn visit_bool(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                bool::from_str(match v {
                    "1" => "true",
                    "0" => "false",
                    _ => v,
                })
                .map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_DisplaySequence_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_DisplaySequence_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_ResourceGroupReleaseReference_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_ResourceGroupReleaseReference_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        let mut depth = 0;

        loop {
            let event = reader.peek()?.to_owned();

            ::yaserde::__derive_trace!(
                "Struct {} @ {}: matching {:?}",
                stringify!(ResourceSubGroup),
                start_depth,
                event
            );

            match event {
                ::yaserde::__xml::reader::XmlEvent::StartElement {
                    ref name,
                    ref attributes,
                    ..
                } => {
                    if depth == 0 && name.local_name == "ResourceGroup" {
                        let event = reader.next_event()?;
                    } else {
                        match name.local_name.as_str() {
                            "DisplayTitleText" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <DisplayTitleText as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __display_title_texts_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "DisplayTitle" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <DisplayTitle as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __display_titles_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "AdditionalTitle" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <AdditionalTitle as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __additional_titles_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "SequenceNumber" => {
                                let visitor = __Visitor_SequenceNumber_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result = reader.read_inner_value::<i32, _>(|reader| {
                                    if let ::std::result::Result::Ok(
                                        ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                    ) = reader.peek()
                                    {
                                        let val = visitor.visit_i32(&s);
                                        let _event = reader.next_event()?;
                                        val
                                    } else {
                                        ::std::result::Result::Err(::std::format!(
                                            "unable to parse content for {}",
                                            "SequenceNumber"
                                        ))
                                    }
                                });
                                if let ::std::result::Result::Ok(value) = result {
                                    __sequence_number_value = ::std::option::Option::Some(value)
                                }
                            }
                            "DisplayArtist" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <DisplayArtist as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __display_artists_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "CarrierType" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <CarrierType as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __carrier_types_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "Duration" => {
                                let visitor = __Visitor_Duration_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result = reader
                                    .read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "Duration"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __duration_value = ::std::option::Option::Some(value)
                                }
                            }
                            "ResourceGroup" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                  
                                        let value =  <ResourceSubGroup as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                        __resource_groups_value.push(value);
                             

                                    let _event = reader.next_event()?;
                                }
                            }
                            "ResourceGroupContentItem" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <ResourceGroupContentItem as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __resource_group_content_items_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "LinkedReleaseResourceReference" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <LinkedReleaseResourceReference as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __linked_release_resource_references_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "NoDisplaySequence" => {
                                let visitor = __Visitor_NoDisplaySequence_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result = reader.read_inner_value::<bool, _>(|reader| {
                                    if let ::std::result::Result::Ok(
                                        ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                    ) = reader.peek()
                                    {
                                        let val = visitor.visit_bool(&s);
                                        let _event = reader.next_event()?;
                                        val
                                    } else {
                                        ::std::result::Result::Err(::std::format!(
                                            "unable to parse content for {}",
                                            "NoDisplaySequence"
                                        ))
                                    }
                                });
                                if let ::std::result::Result::Ok(value) = result {
                                    __no_display_sequence_value =
                                        ::std::option::Option::Some(value)
                                }
                            }
                            "DisplaySequence" => {
                                let visitor = __Visitor_DisplaySequence_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result = reader
                                    .read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "DisplaySequence"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __display_sequence_value =
                                        ::std::option::Option::Some(value)
                                }
                            }
                            "ResourceGroupReleaseReference" => {
                                let visitor = __Visitor_ResourceGroupReleaseReference_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result = reader
                                    .read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "ResourceGroupReleaseReference"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __resource_group_release_reference_value =
                                        ::std::option::Option::Some(value)
                                }
                            }
                            "ReleaseId" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <ReleaseId as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __release_id_value = ::std::option::Option::Some(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            _ => {
                                let event = reader.next_event()?;
                                if depth > 0 {
                                    reader.skip_element(|event| {})?;
                                }
                            }
                        }
                    }
                    if depth == 0 {
                        for attr in attributes {
                            if attr.name.local_name == "ResourceGroupType" {
                                __resource_group_type_value = Some(attr.value.to_owned());
                            }
                        }
                    }
                    depth += 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndElement { ref name } => {
                    if name.local_name == named_element && reader.depth() == start_depth + 1 {
                        break;
                    }
                    let event = reader.next_event()?;
                    depth -= 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndDocument => {
                    if false {
                        break;
                    }
                }
                ::yaserde::__xml::reader::XmlEvent::Characters(ref text_content) => {
                    let event = reader.next_event()?;
                }
                event => {
                    return ::std::result::Result::Err(::std::format!(
                        "unknown event {:?}",
                        event
                    ));
                }
            }
        }

        if __no_display_sequence_value.is_some() && __display_sequence_value.is_some() {
            Err("ResourceSubGroup: DisplaySequence and NoDisplaySequence are exclusive".to_string())?
        }

        if __resource_group_release_reference_value.is_some() && __release_id_value.is_some() {
            Err("ResourceSubGroup: ResourceGroupReleaseReference and ReleaseId are exclusive".to_string())?
        }
        
        ::yaserde::__derive_debug!(
            "Struct {} @ {}: success",
            stringify!(ResourceSubGroup),
            start_depth
        );

        ::std::result::Result::Ok(ResourceSubGroup {
            display_title_texts: __display_title_texts_value,
            display_titles: __display_titles_value,
            additional_titles: __additional_titles_value,
            sequence_number: __sequence_number_value,
            display_artists: __display_artists_value,
            carrier_types: __carrier_types_value,
            duration: __duration_value,
            resource_groups: __resource_groups_value,
            resource_group_content_items: __resource_group_content_items_value,
            linked_release_resource_references: __linked_release_resource_references_value,
            resource_group_type: match __resource_group_type_value {
                Some(val) => {
                    if AvsResourceGroupTypeValidator::validate(&val) {
                        val
                    } else {
                        Err("invalid value in field ResourceSubGroup".to_string())?
                    }
                }
                None => Err(
                    "resource_group_type is a required field of ResourceSubGroup".to_string(),
                )?,
            },
            no_display_sequence: __no_display_sequence_value,
            display_sequence: __display_sequence_value,
            resource_group_release_reference: match &__resource_group_release_reference_value {
                Some(val) => {
                    if ResourceGroupReleaseReferenceValidator::validate(&val) {
                        __resource_group_release_reference_value
                    } else {
                        Err(format!(
                            "invalid value in attribute {}",
                            "ResourceGroupReleaseReference"
                        ))?
                    }
                }
                None => __resource_group_release_reference_value,
            },
            release_id: __release_id_value,
        })
    }
}

#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
impl ::yaserde::YaDeserialize for Segment {
    #[allow(unused_variables)]
    fn deserialize<R: ::std::io::Read>(
        reader: &mut ::yaserde::de::Deserializer<R>,
    ) -> ::std::result::Result<Self, ::std::string::String> {
        let (named_element, struct_namespace) =
            if let ::yaserde::__xml::reader::XmlEvent::StartElement { name, .. } =
                reader.peek()?.to_owned()
            {
                (name.local_name.to_owned(), name.namespace.clone())
            } else {
                (
                    ::std::string::String::from("ern:Segment"),
                    ::std::option::Option::None,
                )
            };
        let start_depth = reader.depth();

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: start to parse {:?}",
            stringify!(Segment),
            start_depth,
            named_element
        );

        if reader.depth() == 0 {
            if let Some(namespace) = struct_namespace {
                match namespace.as_str() {
                    "http://ddex.net/xml/ern/43" => {}

                    bad_namespace => {
                        let msg = ::std::format!(
                            "bad namespace for {}, found {}",
                            named_element,
                            bad_namespace
                        );
                        return Err(msg);
                    }
                }
            }
        }
        #[allow(unused_mut)]
        let mut __start_time_value: Option<::std::string::String> = None;
        #[allow(unused_mut)]
        let mut __duration_value = None;
        #[allow(unused_mut)]
        let mut __end_time_value = None;
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_StartTime_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_StartTime_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_Duration_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_Duration_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_EndTime_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_EndTime_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        let mut depth = 0;
        loop {
            let event = reader.peek()?.to_owned();

            ::yaserde::__derive_trace!(
                "Struct {} @ {}: matching {:?}",
                stringify!(Segment),
                start_depth,
                event
            );

            match event {
                ::yaserde::__xml::reader::XmlEvent::StartElement {
                    ref name,
                    ref attributes,
                    ..
                } => {
                    if depth == 0 && name.local_name == "ern:Segment" {
                        let event = reader.next_event()?;
                    } else {
                        match name.local_name.as_str() {
                            "StartTime" => {
                                let visitor = __Visitor_StartTime_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "StartTime"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __start_time_value = ::std::option::Option::Some(value)
                                }
                            }
                            "Duration" => {
                                let visitor = __Visitor_Duration_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "Duration"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __duration_value = ::std::option::Option::Some(value)
                                }
                            }
                            "EndTime" => {
                                let visitor = __Visitor_EndTime_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "EndTime"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __end_time_value = ::std::option::Option::Some(value)
                                }
                            }
                            _ => {
                                let event = reader.next_event()?;
                                if depth > 0 {
                                    reader.skip_element(|event| {})?;
                                }
                            }
                        }
                    }
                    if depth == 0 {}
                    depth += 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndElement { ref name } => {
                    if name.local_name == named_element && reader.depth() == start_depth + 1 {
                        break;
                    }
                    let event = reader.next_event()?;
                    depth -= 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndDocument => {
                    if false {
                        break;
                    }
                }
                ::yaserde::__xml::reader::XmlEvent::Characters(ref text_content) => {
                    let event = reader.next_event()?;
                }
                event => {
                    return ::std::result::Result::Err(::std::format!("unknown event {:?}", event));
                }
            }
        }

        match (&__duration_value, &__end_time_value) {
            (Some(_), None) | (None, Some(_)) => {}
            _ => Err("Segment: Provide Duration or EndTime".to_string())?,
        }

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: success",
            stringify!(Segment),
            start_depth
        );

        ::std::result::Result::Ok(Segment {
            start_time: __start_time_value
                .ok_or_else(|| "start_time is a required field of Segment".to_string())?,
            duration: __duration_value,
            end_time: __end_time_value,
        })
    }
}

#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
impl ::yaserde::YaDeserialize for ServiceException {
    #[allow(unused_variables)]
    fn deserialize<R: ::std::io::Read>(
        reader: &mut ::yaserde::de::Deserializer<R>,
    ) -> ::std::result::Result<Self, ::std::string::String> {
        let (named_element, struct_namespace) =
            if let ::yaserde::__xml::reader::XmlEvent::StartElement { name, .. } =
                reader.peek()?.to_owned()
            {
                (name.local_name.to_owned(), name.namespace.clone())
            } else {
                (
                    ::std::string::String::from("ern:ServiceException"),
                    ::std::option::Option::None,
                )
            };
        let start_depth = reader.depth();

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: start to parse {:?}",
            stringify!(ServiceException),
            start_depth,
            named_element
        );

        if reader.depth() == 0 {
            if let Some(namespace) = struct_namespace {
                match namespace.as_str() {
                    "http://ddex.net/xml/ern/43" => {}

                    bad_namespace => {
                        let msg = ::std::format!(
                            "bad namespace for {}, found {}",
                            named_element,
                            bad_namespace
                        );
                        return Err(msg);
                    }
                }
            }
        }
        #[allow(unused_mut)]
        let mut __trading_name_value = None;
        #[allow(unused_mut)]
        let mut __urls_value: ::std::vec::Vec<::std::string::String> = vec![];
        #[allow(unused_mut)]
        let mut __channels_value: ::std::vec::Vec<Channel> = vec![];
        #[allow(unused_mut)]
        let mut __parties_names_value: ::std::vec::Vec<PartyName> = vec![];
        #[allow(unused_mut)]
        let mut __parties_ids_value: ::std::vec::Vec<DetailedPartyId> = vec![];
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_Url_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_Url_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        let mut depth = 0;
        loop {
            let event = reader.peek()?.to_owned();

            ::yaserde::__derive_trace!(
                "Struct {} @ {}: matching {:?}",
                stringify!(ServiceException),
                start_depth,
                event
            );

            match event {
                ::yaserde::__xml::reader::XmlEvent::StartElement {
                    ref name,
                    ref attributes,
                    ..
                } => {
                    if depth == 0 && name.local_name == "ern:ServiceException" {
                        let event = reader.next_event()?;
                    } else {
                        match name.local_name.as_str() {
                            "TradingName" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <Name as ::yaserde::YaDeserialize>::deserialize(reader)?;
                                    __trading_name_value = ::std::option::Option::Some(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "URL" => {
                                let visitor = __Visitor_Url_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "URL"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __urls_value.push(value)
                                }
                            }
                            "Channel" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <Channel as ::yaserde::YaDeserialize>::deserialize(reader)?;
                                    __channels_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "PartyName" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <PartyName as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __parties_names_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "PartyId" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <DetailedPartyId as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __parties_ids_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            _ => {
                                let event = reader.next_event()?;
                                if depth > 0 {
                                    reader.skip_element(|event| {})?;
                                }
                            }
                        }
                    }
                    if depth == 0 {}
                    depth += 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndElement { ref name } => {
                    if name.local_name == named_element && reader.depth() == start_depth + 1 {
                        break;
                    }
                    let event = reader.next_event()?;
                    depth -= 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndDocument => {
                    if false {
                        break;
                    }
                }
                ::yaserde::__xml::reader::XmlEvent::Characters(ref text_content) => {
                    let event = reader.next_event()?;
                }
                event => {
                    return ::std::result::Result::Err(::std::format!("unknown event {:?}", event));
                }
            }
        }

        if __parties_names_value.len() == 0 && __parties_ids_value.len() == 0 {
            return Err("At least one PartyName and/or PartyId is required".to_string());
        }

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: success",
            stringify!(ServiceException),
            start_depth
        );

        ::std::result::Result::Ok(ServiceException {
            trading_name: __trading_name_value,
            urls: __urls_value,
            channels: __channels_value,
            parties_names: __parties_names_value,
            parties_ids: __parties_ids_value,
        })
    }
}

#[allow(non_upper_case_globals,unused_attributes,unused_qualifications)]
impl ::yaserde::YaDeserialize for TrackReleaseVisibility {
    #[allow(unused_variables)]
    fn deserialize<R: ::std::io::Read>(reader: &mut ::yaserde::de::Deserializer<R> ,) ->  ::std::result::Result<Self, ::std::string::String>{
        let (named_element,struct_namespace) = if let::yaserde::__xml::reader::XmlEvent::StartElement {
            name, ..
        } = reader.peek()? .to_owned(){
            (name.local_name.to_owned(),name.namespace.clone())
        } else {
            (::std::string::String::from("ern:TrackReleaseVisibility"), ::std::option::Option::None)
        };
        let start_depth = reader.depth();

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: start to parse {:?}",
            stringify!(TrackReleaseVisibility),
            start_depth,
            named_element
        );

        if reader.depth()==0 {
            if let Some(namespace) = struct_namespace {
                match namespace.as_str(){
                    "http://ddex.net/xml/ern/43" => {}
                    
                    bad_namespace => {
                        let msg =  ::std::format!("bad namespace for {}, found {}",named_element,bad_namespace);
                        return Err(msg);
                    }
                
                    }
            }
        }
        #[allow(unused_mut)]
        let mut __visibility_reference_value:Option:: < ::std::string::String>  = None;
        #[allow(unused_mut)]
        let mut __track_listing_preview_start_date_time_value:Option:: < ::std::string::String>  = None;
        #[allow(unused_mut)]
        let mut __clip_preview_start_date_time_value = None;
        #[allow(unused_mut)]
        let mut __do_not_display_dates_value = None;
        #[allow(unused_mut)]
        let mut __territory_codes_value: ::std::vec::Vec< ::std::string::String>  = vec![];
        #[allow(unused_mut)]
        let mut __excluded_territory_codes_value: ::std::vec::Vec< ::std::string::String>  = vec![];
        #[allow(non_snake_case,non_camel_case_types)]
        struct __Visitor_VisibilityReference_;
        
        impl <'de> ::yaserde::Visitor<'de>for __Visitor_VisibilityReference_ {
            type Value =  ::std::string::String;
            fn visit_str(self,v: &str,) ->  ::std::result::Result<Self::Value, ::std::string::String>{
                ::std::string::String::from_str(v).map_err(|e|e.to_string())
            }
        
            }
        #[allow(non_snake_case,non_camel_case_types)]
        struct __Visitor_TrackListingPreviewStartDateTime_;
        
        impl <'de> ::yaserde::Visitor<'de>for __Visitor_TrackListingPreviewStartDateTime_ {
            type Value =  ::std::string::String;
            fn visit_str(self,v: &str,) ->  ::std::result::Result<Self::Value, ::std::string::String>{
                ::std::string::String::from_str(v).map_err(|e|e.to_string())
            }
        
            }
        #[allow(non_snake_case,non_camel_case_types)]
        struct __Visitor_ClipPreviewStartDateTime_;
        
        impl <'de> ::yaserde::Visitor<'de>for __Visitor_ClipPreviewStartDateTime_ {
            type Value =  ::std::string::String;
            fn visit_str(self,v: &str,) ->  ::std::result::Result<Self::Value, ::std::string::String>{
                ::std::string::String::from_str(v).map_err(|e|e.to_string())
            }
        
            }
        #[allow(non_snake_case,non_camel_case_types)]
        struct __Visitor_Attribute_DoNotDisplayDates_;
        
        impl <'de> ::yaserde::Visitor<'de>for __Visitor_Attribute_DoNotDisplayDates_ {
            type Value = bool;
            fn visit_bool(self,v: &str,) ->  ::std::result::Result<Self::Value, ::std::string::String>{
                bool::from_str(match v {
                    "1" => "true",
                    "0" => "false",
                    _ => v,
                
                    }).map_err(|e|e.to_string())
            }
        
            }
        #[allow(non_snake_case,non_camel_case_types)]
        struct __Visitor_TerritoryCode_;
        
        impl <'de> ::yaserde::Visitor<'de>for __Visitor_TerritoryCode_ {
            type Value =  ::std::string::String;
            fn visit_str(self,v: &str,) ->  ::std::result::Result<Self::Value, ::std::string::String>{
                ::std::string::String::from_str(v).map_err(|e|e.to_string())
            }
        
            }
        #[allow(non_snake_case,non_camel_case_types)]
        struct __Visitor_ExcludedTerritoryCode_;
        
        impl <'de> ::yaserde::Visitor<'de>for __Visitor_ExcludedTerritoryCode_ {
            type Value =  ::std::string::String;
            fn visit_str(self,v: &str,) ->  ::std::result::Result<Self::Value, ::std::string::String>{
                ::std::string::String::from_str(v).map_err(|e|e.to_string())
            }
        
            }
        let mut depth = 0;
        loop {
            let event = reader.peek()? .to_owned();
            match event {
                ::yaserde::__xml::reader::XmlEvent::StartElement {
                    ref name,ref attributes, ..
                } => {
                    if depth==0&&name.local_name=="ern:TrackReleaseVisibility" {
                        let event = reader.next_event()? ;
                    } else {
                        match name.local_name.as_str() {
                            "VisibilityReference" => {
                                let visitor = __Visitor_VisibilityReference_{};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}
                                        
                                        bad_namespace => {
                                            let msg =  ::std::format!("bad namespace for {}, found {}",name.local_name.as_str(),bad_namespace);
                                            return Err(msg);
                                        }
                                    
                                    }
                                }
                                let result = reader.read_inner_value:: < ::std::string::String,_>(|reader|{
                                    if let::std::result::Result::Ok(::yaserde::__xml::reader::XmlEvent::Characters(s)) = reader.peek() {
                                        let val = visitor.visit_str(&s);
                                        let _event = reader.next_event()? ;
                                        val
                                    } else {
                                        ::std::result::Result::Err(::std::format!("unable to parse content for {}","VisibilityReference"))
                                    }
                                });
                                if let::std::result::Result::Ok(value) = result {
                                    __visibility_reference_value =  ::std::option::Option::Some(value)
                                }
                            }
                            "TrackListingPreviewStartDateTime" => {
                                let visitor = __Visitor_TrackListingPreviewStartDateTime_{};

                                if let Some(namespace) = name.namespace.as_ref(){
                                    match namespace.as_str(){
                                        "http://ddex.net/xml/ern/43" => {}
                                        
                                        bad_namespace => {
                                            let msg =  ::std::format!("bad namespace for {}, found {}",name.local_name.as_str(),bad_namespace);
                                            return Err(msg);
                                        }
                                    
                                        }
                                }let result = reader.read_inner_value:: < ::std::string::String,_>(|reader|{
                                    if let::std::result::Result::Ok(::yaserde::__xml::reader::XmlEvent::Characters(s)) = reader.peek(){
                                        let val = visitor.visit_str(&s);
                                        let _event = reader.next_event()? ;
                                        val
                                    }else {
                                        ::std::result::Result::Err(::std::format!("unable to parse content for {}","TrackListingPreviewStartDateTime"))
                                    }
                                });
                                if let::std::result::Result::Ok(value) = result {
                                    __track_listing_preview_start_date_time_value =  ::std::option::Option::Some(value)
                                }
                            }
                            "ClipPreviewStartDateTime" => {
                                let visitor = __Visitor_ClipPreviewStartDateTime_{}
                                ;
                                if let Some(namespace) = name.namespace.as_ref(){
                                    match namespace.as_str(){
                                        "http://ddex.net/xml/ern/43" => {}
                                        
                                        bad_namespace => {
                                            let msg =  ::std::format!("bad namespace for {}, found {}",name.local_name.as_str(),bad_namespace);
                                            return Err(msg);
                                        }
                                    
                                        }
                                }let result = reader.read_inner_value:: < ::std::string::String,_>(|reader|{
                                    if let::std::result::Result::Ok(::yaserde::__xml::reader::XmlEvent::Characters(s)) = reader.peek(){
                                        let val = visitor.visit_str(&s);
                                        let _event = reader.next_event()? ;
                                        val
                                    }else {
                                        ::std::result::Result::Err(::std::format!("unable to parse content for {}","ClipPreviewStartDateTime"))
                                    }
                                });
                                if let::std::result::Result::Ok(value) = result {
                                    __clip_preview_start_date_time_value =  ::std::option::Option::Some(value)
                                }
                            }
                            "TerritoryCode" => {
                                let visitor = __Visitor_TerritoryCode_{}
                                ;
                                if let Some(namespace) = name.namespace.as_ref(){
                                    match namespace.as_str(){
                                        "http://ddex.net/xml/ern/43" => {}
                                        
                                        bad_namespace => {
                                            let msg =  ::std::format!("bad namespace for {}, found {}",name.local_name.as_str(),bad_namespace);
                                            return Err(msg);
                                        }
                                    
                                        }
                                }let result = reader.read_inner_value:: < ::std::string::String,_>(|reader|{
                                    if let::std::result::Result::Ok(::yaserde::__xml::reader::XmlEvent::Characters(s)) = reader.peek(){
                                        let val = visitor.visit_str(&s);
                                        let _event = reader.next_event()? ;
                                        val
                                    }else {
                                        ::std::result::Result::Err(::std::format!("unable to parse content for {}","TerritoryCode"))
                                    }
                                });
                                if let::std::result::Result::Ok(value) = result {
                                    __territory_codes_value.push(value)
                                }
                            }
                            "ExcludedTerritoryCode" => {
                                let visitor = __Visitor_ExcludedTerritoryCode_{}
                                ;
                                if let Some(namespace) = name.namespace.as_ref(){
                                    match namespace.as_str(){
                                        "http://ddex.net/xml/ern/43" => {}
                                        
                                        bad_namespace => {
                                            let msg =  ::std::format!("bad namespace for {}, found {}",name.local_name.as_str(),bad_namespace);
                                            return Err(msg);
                                        }
                                    
                                        }
                                }let result = reader.read_inner_value:: < ::std::string::String,_>(|reader|{
                                    if let::std::result::Result::Ok(::yaserde::__xml::reader::XmlEvent::Characters(s)) = reader.peek(){
                                        let val = visitor.visit_str(&s);
                                        let _event = reader.next_event()? ;
                                        val
                                    }else {
                                        ::std::result::Result::Err(::std::format!("unable to parse content for {}","ExcludedTerritoryCode"))
                                    }
                                });
                                if let::std::result::Result::Ok(value) = result {
                                    __excluded_territory_codes_value.push(value)
                                }
                            }
                            _ => {
                                let event = reader.next_event()? ;
                                if depth>0 {
                                    reader.skip_element(|event|{}
                                    )? ;
                                }
                            }
                        
                            }
                    }if depth==0 {
                        for attr in attributes {
                            if attr.name.local_name=="DoNotDisplayDates" {
                                let visitor = __Visitor_Attribute_DoNotDisplayDates_{}
                                ;
                                let value = visitor.visit_bool(&attr.value)? ;
                                __do_not_display_dates_value =  ::std::option::Option::Some(value);
                            }
                        }
                    }depth+=1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndElement {
                    ref name
                } => {
                    if name.local_name==named_element&&reader.depth()==start_depth+1 {
                        break;
                    }let event = reader.next_event()? ;
                    depth-=1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndDocument => {
                    if false {
                        break;
                    }
                }
                ::yaserde::__xml::reader::XmlEvent::Characters(ref text_content) => {
                    let event = reader.next_event()? ;
                }
                event => {
                    return::std::result::Result::Err(::std::format!("unknown event {:?}",event));
                }
            
                }
        }

        if __territory_codes_value.len() > 0 && __excluded_territory_codes_value.len() > 0 {
            Err("TerritoryCodes and ExcludedTerritoryCodes are exclusive".to_string())?
        }

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: success",
            stringify!(TrackReleaseVisibility),
            start_depth
        );

        ::std::result::Result::Ok(TrackReleaseVisibility {
            visibility_reference:match __visibility_reference_value {
                Some(val) => {
                    if VisibilityReferenceValidator::validate(&val){
                        val
                    }else {
                        Err("invalid value in field TrackReleaseVisibility".to_string())?
                    }
                }
                None => Err("visibility_reference is a required field of TrackReleaseVisibility".to_string())? ,
            
                },track_listing_preview_start_date_time:__track_listing_preview_start_date_time_value.ok_or_else(||"track_listing_preview_start_date_time is a required field of TrackReleaseVisibility".to_string())? ,clip_preview_start_date_time:__clip_preview_start_date_time_value,do_not_display_dates:__do_not_display_dates_value,territory_codes:if __territory_codes_value.len()>0 {
                let is_valid =  &__territory_codes_value.iter().all(|val|AvsCurrentTerritoryCodeValidator::validate(&val));
                if*is_valid {
                    __territory_codes_value
                }else {
                    Err(format!("invalid value in field {}","TerritoryCode"))?
                }
            }else {
                __territory_codes_value
            },excluded_territory_codes:if __excluded_territory_codes_value.len()>0 {
                let is_valid =  &__excluded_territory_codes_value.iter().all(|val|AvsCurrentTerritoryCodeValidator::validate(&val));
                if*is_valid {
                    __excluded_territory_codes_value
                }else {
                    Err(format!("invalid value in field {}","ExcludedTerritoryCode"))?
                }
            }else {
                __excluded_territory_codes_value
            },
        })
    }
}

#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
impl ::yaserde::YaDeserialize for Video {
    #[allow(unused_variables)]
    fn deserialize<R: ::std::io::Read>(
        reader: &mut ::yaserde::de::Deserializer<R>,
    ) -> ::std::result::Result<Self, ::std::string::String> {
        let (named_element, struct_namespace) =
            if let ::yaserde::__xml::reader::XmlEvent::StartElement { name, .. } =
                reader.peek()?.to_owned()
            {
                (name.local_name.to_owned(), name.namespace.clone())
            } else {
                (
                    ::std::string::String::from("ern:Video"),
                    ::std::option::Option::None,
                )
            };
        let start_depth = reader.depth();

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: start to parse {:?}",
            stringify!(Video),
            start_depth,
            named_element
        );

        if reader.depth() == 0 {
            if let Some(namespace) = struct_namespace {
                match namespace.as_str() {
                    "http://ddex.net/xml/ern/43" => {}

                    bad_namespace => {
                        let msg = ::std::format!(
                            "bad namespace for {}, found {}",
                            named_element,
                            bad_namespace
                        );
                        return Err(msg);
                    }
                }
            }
        }
        #[allow(unused_mut)]
        let mut __resource_reference_value: Option<::std::string::String> = None;
        #[allow(unused_mut)]
        let mut __kind_value: Option<VideoType> = None;
        #[allow(unused_mut)]
        let mut __video_editions_value: ::std::vec::Vec<VideoEdition> = vec![];
        #[allow(unused_mut)]
        let mut __recording_formats_value: ::std::vec::Vec<RecordingFormat> = vec![];
        #[allow(unused_mut)]
        let mut __work_ids_value: ::std::vec::Vec<MusicalWorkId> = vec![];
        #[allow(unused_mut)]
        let mut __display_title_texts_value: ::std::vec::Vec<DisplayTitleText> = vec![];
        #[allow(unused_mut)]
        let mut __display_titles_value: ::std::vec::Vec<DisplayTitle> = vec![];
        #[allow(unused_mut)]
        let mut __additional_titles_value: ::std::vec::Vec<AdditionalTitle> = vec![];
        #[allow(unused_mut)]
        let mut __version_types_value: ::std::vec::Vec<VersionType> = vec![];
        #[allow(unused_mut)]
        let mut __display_artist_names_value: ::std::vec::Vec<
            DisplayArtistNameWithDefault,
        > = vec![];
        #[allow(unused_mut)]
        let mut __display_artists_value: ::std::vec::Vec<DisplayArtist> = vec![];
        #[allow(unused_mut)]
        let mut __contributors_value: ::std::vec::Vec<Contributor> = vec![];
        #[allow(unused_mut)]
        let mut __characters_value: ::std::vec::Vec<Character> = vec![];
        #[allow(unused_mut)]
        let mut __resource_rights_controllers_value: ::std::vec::Vec<
            ResourceRightsController,
        > = vec![];
        #[allow(unused_mut)]
        let mut __work_rights_controllers_value: ::std::vec::Vec<WorkRightsController> = vec![];
        #[allow(unused_mut)]
        let mut __courtesy_lines_value: ::std::vec::Vec<CourtesyLineWithDefault> = vec![];
        #[allow(unused_mut)]
        let mut __duration_value: Option<::std::string::String> = None;
        #[allow(unused_mut)]
        let mut __creation_date_value = None;
        #[allow(unused_mut)]
        let mut __mastered_date_value = None;
        #[allow(unused_mut)]
        let mut __remastered_dates_value: ::std::vec::Vec<EventDateWithoutFlags> = vec![];
        #[allow(unused_mut)]
        let mut __first_publication_dates_value: ::std::vec::Vec<
            FulfillmentDateWithTerritory,
        > = vec![];
        #[allow(unused_mut)]
        let mut __parental_warning_types_value: ::std::vec::Vec<
            ParentalWarningTypeWithTerritory,
        > = vec![];
        #[allow(unused_mut)]
        let mut __av_ratings_value: ::std::vec::Vec<AvRating> = vec![];
        #[allow(unused_mut)]
        let mut __related_releases_value: ::std::vec::Vec<RelatedRelease> = vec![];
        #[allow(unused_mut)]
        let mut __related_resources_value: ::std::vec::Vec<RelatedResource> = vec![];
        #[allow(unused_mut)]
        let mut __composite_musical_work_type_value = None;
        #[allow(unused_mut)]
        let mut __is_cover_value = None;
        #[allow(unused_mut)]
        let mut __has_vocal_performance_value = None;
        #[allow(unused_mut)]
        let mut __has_foreground_vocal_performance_value = None;
        #[allow(unused_mut)]
        let mut __is_instrumental_value = None;
        #[allow(unused_mut)]
        let mut __contains_hidden_content_value = None;
        #[allow(unused_mut)]
        let mut __is_remastered_value = None;
        #[allow(unused_mut)]
        let mut __display_creditss_value: ::std::vec::Vec<DisplayCredits> = vec![];
        #[allow(unused_mut)]
        let mut __language_of_performances_value: ::std::vec::Vec<Language> = vec![];
        #[allow(unused_mut)]
        let mut __language_of_dubbings_value: ::std::vec::Vec<::std::string::String> = vec![];
        #[allow(unused_mut)]
        let mut __sub_title_languages_value: ::std::vec::Vec<::std::string::String> = vec![];
        #[allow(unused_mut)]
        let mut __resource_contained_resource_reference_list_value = None;
        #[allow(unused_mut)]
        let mut __ragas_value: ::std::vec::Vec<::std::string::String> = vec![];
        #[allow(unused_mut)]
        let mut __talas_value: ::std::vec::Vec<::std::string::String> = vec![];
        #[allow(unused_mut)]
        let mut __deitys_value: ::std::vec::Vec<::std::string::String> = vec![];
        #[allow(unused_mut)]
        let mut __video_chapter_references_value: ::std::vec::Vec<::std::string::String> = vec![];
        #[allow(unused_mut)]
        let mut __language_and_script_code_value = None;
        #[allow(unused_mut)]
        let mut __is_supplemental_value = None;
        #[allow(unused_mut)]
        let mut __apply_classical_profile_variant_value = None;
        #[allow(unused_mut)]
        let mut __video_cue_sheet_references_value: ::std::vec::Vec<::std::string::String> = vec![];
        #[allow(unused_mut)]
        let mut __reason_for_cue_sheet_absence_value = None;
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_ResourceReference_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_ResourceReference_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_Duration_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_Duration_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_CompositeMusicalWorkType_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_CompositeMusicalWorkType_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_IsCover_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_IsCover_ {
            type Value = bool;
            fn visit_bool(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                bool::from_str(match v {
                    "1" => "true",
                    "0" => "false",
                    _ => v,
                })
                .map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_HasVocalPerformance_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_HasVocalPerformance_ {
            type Value = bool;
            fn visit_bool(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                bool::from_str(match v {
                    "1" => "true",
                    "0" => "false",
                    _ => v,
                })
                .map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_HasForegroundVocalPerformance_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_HasForegroundVocalPerformance_ {
            type Value = bool;
            fn visit_bool(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                bool::from_str(match v {
                    "1" => "true",
                    "0" => "false",
                    _ => v,
                })
                .map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_IsInstrumental_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_IsInstrumental_ {
            type Value = bool;
            fn visit_bool(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                bool::from_str(match v {
                    "1" => "true",
                    "0" => "false",
                    _ => v,
                })
                .map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_ContainsHiddenContent_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_ContainsHiddenContent_ {
            type Value = bool;
            fn visit_bool(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                bool::from_str(match v {
                    "1" => "true",
                    "0" => "false",
                    _ => v,
                })
                .map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_IsRemastered_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_IsRemastered_ {
            type Value = bool;
            fn visit_bool(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                bool::from_str(match v {
                    "1" => "true",
                    "0" => "false",
                    _ => v,
                })
                .map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_LanguageOfDubbing_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_LanguageOfDubbing_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_SubTitleLanguage_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_SubTitleLanguage_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_Raga_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_Raga_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_Tala_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_Tala_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_Deity_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_Deity_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_VideoChapterReference_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_VideoChapterReference_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_Attribute_LanguageAndScriptCode_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_Attribute_LanguageAndScriptCode_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_Attribute_IsSupplemental_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_Attribute_IsSupplemental_ {
            type Value = bool;
            fn visit_bool(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                bool::from_str(match v {
                    "1" => "true",
                    "0" => "false",
                    _ => v,
                })
                .map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_Attribute_ApplyClassicalProfileVariant_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_Attribute_ApplyClassicalProfileVariant_ {
            type Value = bool;
            fn visit_bool(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                bool::from_str(match v {
                    "1" => "true",
                    "0" => "false",
                    _ => v,
                })
                .map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_VideoCueSheetReference_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_VideoCueSheetReference_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        let mut depth = 0;
        loop {
            let event = reader.peek()?.to_owned();

            ::yaserde::__derive_trace!(
                "Struct {} @ {}: matching {:?}",
                stringify!(Video),
                start_depth,
                event
            );

            match event {
                ::yaserde::__xml::reader::XmlEvent::StartElement {
                    ref name,
                    ref attributes,
                    ..
                } => {
                    if depth == 0 && name.local_name == "ern:Video" {
                        let event = reader.next_event()?;
                    } else {
                        match name.local_name.as_str() {
                            "ResourceReference" => {
                                let visitor = __Visitor_ResourceReference_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "ResourceReference"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __resource_reference_value = ::std::option::Option::Some(value)
                                }
                            }
                            "Type" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <VideoType as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __kind_value = ::std::option::Option::Some(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "VideoEdition" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <VideoEdition as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __video_editions_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "RecordingFormat" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <RecordingFormat as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __recording_formats_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "WorkId" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <MusicalWorkId as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __work_ids_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "DisplayTitleText" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <DisplayTitleText as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __display_title_texts_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "DisplayTitle" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <DisplayTitle as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __display_titles_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "AdditionalTitle" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <AdditionalTitle as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __additional_titles_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "VersionType" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <VersionType as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __version_types_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "DisplayArtistName" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <DisplayArtistNameWithDefault as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __display_artist_names_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "DisplayArtist" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <DisplayArtist as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __display_artists_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "Contributor" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <Contributor as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __contributors_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "Character" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <Character as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __characters_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "ResourceRightsController" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <ResourceRightsController as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __resource_rights_controllers_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "WorkRightsController" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <WorkRightsController as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __work_rights_controllers_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "CourtesyLine" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <CourtesyLineWithDefault as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __courtesy_lines_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "Duration" => {
                                let visitor = __Visitor_Duration_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "Duration"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __duration_value = ::std::option::Option::Some(value)
                                }
                            }
                            "CreationDate" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <EventDateWithoutFlags as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __creation_date_value = ::std::option::Option::Some(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "MasteredDate" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <EventDateWithoutFlags as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __mastered_date_value = ::std::option::Option::Some(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "RemasteredDate" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <EventDateWithoutFlags as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __remastered_dates_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "FirstPublicationDate" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <FulfillmentDateWithTerritory as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __first_publication_dates_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "ParentalWarningType" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <ParentalWarningTypeWithTerritory as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __parental_warning_types_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "AvRating" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <AvRating as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __av_ratings_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "RelatedRelease" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <RelatedRelease as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __related_releases_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "RelatedResource" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <RelatedResource as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __related_resources_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "CompositeMusicalWorkType" => {
                                let visitor = __Visitor_CompositeMusicalWorkType_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "CompositeMusicalWorkType"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __composite_musical_work_type_value =
                                        ::std::option::Option::Some(value)
                                }
                            }
                            "IsCover" => {
                                let visitor = __Visitor_IsCover_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result = reader.read_inner_value::<bool, _>(|reader| {
                                    if let ::std::result::Result::Ok(
                                        ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                    ) = reader.peek()
                                    {
                                        let val = visitor.visit_bool(&s);
                                        let _event = reader.next_event()?;
                                        val
                                    } else {
                                        ::std::result::Result::Err(::std::format!(
                                            "unable to parse content for {}",
                                            "IsCover"
                                        ))
                                    }
                                });
                                if let ::std::result::Result::Ok(value) = result {
                                    __is_cover_value = ::std::option::Option::Some(value)
                                }
                            }
                            "HasVocalPerformance" => {
                                let visitor = __Visitor_HasVocalPerformance_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result = reader.read_inner_value::<bool, _>(|reader| {
                                    if let ::std::result::Result::Ok(
                                        ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                    ) = reader.peek()
                                    {
                                        let val = visitor.visit_bool(&s);
                                        let _event = reader.next_event()?;
                                        val
                                    } else {
                                        ::std::result::Result::Err(::std::format!(
                                            "unable to parse content for {}",
                                            "HasVocalPerformance"
                                        ))
                                    }
                                });
                                if let ::std::result::Result::Ok(value) = result {
                                    __has_vocal_performance_value =
                                        ::std::option::Option::Some(value)
                                }
                            }
                            "HasForegroundVocalPerformance" => {
                                let visitor = __Visitor_HasForegroundVocalPerformance_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result = reader.read_inner_value::<bool, _>(|reader| {
                                    if let ::std::result::Result::Ok(
                                        ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                    ) = reader.peek()
                                    {
                                        let val = visitor.visit_bool(&s);
                                        let _event = reader.next_event()?;
                                        val
                                    } else {
                                        ::std::result::Result::Err(::std::format!(
                                            "unable to parse content for {}",
                                            "HasForegroundVocalPerformance"
                                        ))
                                    }
                                });
                                if let ::std::result::Result::Ok(value) = result {
                                    __has_foreground_vocal_performance_value =
                                        ::std::option::Option::Some(value)
                                }
                            }
                            "IsInstrumental" => {
                                let visitor = __Visitor_IsInstrumental_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result = reader.read_inner_value::<bool, _>(|reader| {
                                    if let ::std::result::Result::Ok(
                                        ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                    ) = reader.peek()
                                    {
                                        let val = visitor.visit_bool(&s);
                                        let _event = reader.next_event()?;
                                        val
                                    } else {
                                        ::std::result::Result::Err(::std::format!(
                                            "unable to parse content for {}",
                                            "IsInstrumental"
                                        ))
                                    }
                                });
                                if let ::std::result::Result::Ok(value) = result {
                                    __is_instrumental_value = ::std::option::Option::Some(value)
                                }
                            }
                            "ContainsHiddenContent" => {
                                let visitor = __Visitor_ContainsHiddenContent_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result = reader.read_inner_value::<bool, _>(|reader| {
                                    if let ::std::result::Result::Ok(
                                        ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                    ) = reader.peek()
                                    {
                                        let val = visitor.visit_bool(&s);
                                        let _event = reader.next_event()?;
                                        val
                                    } else {
                                        ::std::result::Result::Err(::std::format!(
                                            "unable to parse content for {}",
                                            "ContainsHiddenContent"
                                        ))
                                    }
                                });
                                if let ::std::result::Result::Ok(value) = result {
                                    __contains_hidden_content_value =
                                        ::std::option::Option::Some(value)
                                }
                            }
                            "IsRemastered" => {
                                let visitor = __Visitor_IsRemastered_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result = reader.read_inner_value::<bool, _>(|reader| {
                                    if let ::std::result::Result::Ok(
                                        ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                    ) = reader.peek()
                                    {
                                        let val = visitor.visit_bool(&s);
                                        let _event = reader.next_event()?;
                                        val
                                    } else {
                                        ::std::result::Result::Err(::std::format!(
                                            "unable to parse content for {}",
                                            "IsRemastered"
                                        ))
                                    }
                                });
                                if let ::std::result::Result::Ok(value) = result {
                                    __is_remastered_value = ::std::option::Option::Some(value)
                                }
                            }
                            "DisplayCredits" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <DisplayCredits as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __display_creditss_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "LanguageOfPerformance" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <Language as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __language_of_performances_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "LanguageOfDubbing" => {
                                let visitor = __Visitor_LanguageOfDubbing_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "LanguageOfDubbing"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __language_of_dubbings_value.push(value)
                                }
                            }
                            "SubTitleLanguage" => {
                                let visitor = __Visitor_SubTitleLanguage_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "SubTitleLanguage"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __sub_title_languages_value.push(value)
                                }
                            }
                            "ResourceContainedResourceReferenceList" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <ResourceContainedResourceReferenceList as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __resource_contained_resource_reference_list_value =
                                        ::std::option::Option::Some(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "Raga" => {
                                let visitor = __Visitor_Raga_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "Raga"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __ragas_value.push(value)
                                }
                            }
                            "Tala" => {
                                let visitor = __Visitor_Tala_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "Tala"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __talas_value.push(value)
                                }
                            }
                            "Deity" => {
                                let visitor = __Visitor_Deity_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "Deity"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __deitys_value.push(value)
                                }
                            }
                            "VideoChapterReference" => {
                                let visitor = __Visitor_VideoChapterReference_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "VideoChapterReference"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __video_chapter_references_value.push(value)
                                }
                            }
                            "VideoCueSheetReference" => {
                                let visitor = __Visitor_VideoCueSheetReference_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "VideoCueSheetReference"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __video_cue_sheet_references_value.push(value)
                                }
                            }
                            "ReasonForCueSheetAbsence" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <Reason as ::yaserde::YaDeserialize>::deserialize(reader)?;
                                    __reason_for_cue_sheet_absence_value =
                                        ::std::option::Option::Some(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            _ => {
                                let event = reader.next_event()?;
                                if depth > 0 {
                                    reader.skip_element(|event| {})?;
                                }
                            }
                        }
                    }
                    if depth == 0 {
                        for attr in attributes {
                            if attr.name.local_name == "LanguageAndScriptCode" {
                                let visitor = __Visitor_Attribute_LanguageAndScriptCode_ {};
                                let value = visitor.visit_str(&attr.value)?;
                                __language_and_script_code_value =
                                    ::std::option::Option::Some(value);
                            }
                        }
                        for attr in attributes {
                            if attr.name.local_name == "IsSupplemental" {
                                let visitor = __Visitor_Attribute_IsSupplemental_ {};
                                let value = visitor.visit_bool(&attr.value)?;
                                __is_supplemental_value = ::std::option::Option::Some(value);
                            }
                        }
                        for attr in attributes {
                            if attr.name.local_name == "ApplyClassicalProfileVariant" {
                                let visitor = __Visitor_Attribute_ApplyClassicalProfileVariant_ {};
                                let value = visitor.visit_bool(&attr.value)?;
                                __apply_classical_profile_variant_value =
                                    ::std::option::Option::Some(value);
                            }
                        }
                    }
                    depth += 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndElement { ref name } => {
                    if name.local_name == named_element && reader.depth() == start_depth + 1 {
                        break;
                    }
                    let event = reader.next_event()?;
                    depth -= 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndDocument => {
                    if false {
                        break;
                    }
                }
                ::yaserde::__xml::reader::XmlEvent::Characters(ref text_content) => {
                    let event = reader.next_event()?;
                }
                event => {
                    return ::std::result::Result::Err(::std::format!("unknown event {:?}", event));
                }
            }
        }

        if __reason_for_cue_sheet_absence_value.is_some()
            && __video_cue_sheet_references_value.len() > 0
        {
            Err(
                "Video: VideoCueSheetReferences and ReasonForCueSheetAbsence are exclusive"
                    .to_string(),
            )?
        }

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: success",
            stringify!(Video),
            start_depth
        );

        ::std::result::Result::Ok(Video {
            resource_reference: match __resource_reference_value {
                Some(val) => {
                    if ResourceReferenceValidator::validate(&val) {
                        val
                    } else {
                        Err("invalid value in field Video".to_string())?
                    }
                }
                None => Err("resource_reference is a required field of Video".to_string())?,
            },
            kind: __kind_value.ok_or_else(|| "kind is a required field of Video".to_string())?,
            video_editions: __video_editions_value,
            recording_formats: __recording_formats_value,
            work_ids: __work_ids_value,
            display_title_texts: __display_title_texts_value,
            display_titles: __display_titles_value,
            additional_titles: __additional_titles_value,
            version_types: __version_types_value,
            display_artist_names: __display_artist_names_value,
            display_artists: __display_artists_value,
            contributors: __contributors_value,
            characters: __characters_value,
            resource_rights_controllers: __resource_rights_controllers_value,
            work_rights_controllers: __work_rights_controllers_value,
            courtesy_lines: __courtesy_lines_value,
            duration: __duration_value
                .ok_or_else(|| "duration is a required field of Video".to_string())?,
            creation_date: __creation_date_value,
            mastered_date: __mastered_date_value,
            remastered_dates: __remastered_dates_value,
            first_publication_dates: __first_publication_dates_value,
            parental_warning_types: __parental_warning_types_value,
            av_ratings: __av_ratings_value,
            related_releases: __related_releases_value,
            related_resources: __related_resources_value,
            composite_musical_work_type: match &__composite_musical_work_type_value {
                Some(val) => {
                    if AvsCompositeMusicalWorkTypeValidator::validate(&val) {
                        __composite_musical_work_type_value
                    } else {
                        Err(format!(
                            "invalid value in attribute {}",
                            "CompositeMusicalWorkType"
                        ))?
                    }
                }
                None => __composite_musical_work_type_value,
            },
            is_cover: __is_cover_value,
            has_vocal_performance: __has_vocal_performance_value,
            has_foreground_vocal_performance: __has_foreground_vocal_performance_value,
            is_instrumental: __is_instrumental_value,
            contains_hidden_content: __contains_hidden_content_value,
            is_remastered: __is_remastered_value,
            display_creditss: __display_creditss_value,
            language_of_performances: __language_of_performances_value,
            language_of_dubbings: if __language_of_dubbings_value.len() > 0 {
                let is_valid = &__language_of_dubbings_value
                    .iter()
                    .all(|val| LanguageOfDubbingValidator::validate(&val));
                if *is_valid {
                    __language_of_dubbings_value
                } else {
                    Err(format!("invalid value in field {}", "LanguageOfDubbing"))?
                }
            } else {
                __language_of_dubbings_value
            },
            sub_title_languages: if __sub_title_languages_value.len() > 0 {
                let is_valid = &__sub_title_languages_value
                    .iter()
                    .all(|val| SubTitleLanguageValidator::validate(&val));
                if *is_valid {
                    __sub_title_languages_value
                } else {
                    Err(format!("invalid value in field {}", "SubTitleLanguage"))?
                }
            } else {
                __sub_title_languages_value
            },
            resource_contained_resource_reference_list:
                __resource_contained_resource_reference_list_value,
            ragas: __ragas_value,
            talas: __talas_value,
            deitys: __deitys_value,
            video_chapter_references: if __video_chapter_references_value.len() > 0 {
                let is_valid = &__video_chapter_references_value
                    .iter()
                    .all(|val| VideoChapterReferenceValidator::validate(&val));
                if *is_valid {
                    __video_chapter_references_value
                } else {
                    Err(format!(
                        "invalid value in field {}",
                        "VideoChapterReference"
                    ))?
                }
            } else {
                __video_chapter_references_value
            },
            language_and_script_code: match &__language_and_script_code_value {
                Some(val) => {
                    if LanguageAndScriptCodeValidator::validate(&val) {
                        __language_and_script_code_value
                    } else {
                        Err(format!(
                            "invalid value in attribute {}",
                            "LanguageAndScriptCode"
                        ))?
                    }
                }
                None => __language_and_script_code_value,
            },
            is_supplemental: __is_supplemental_value,
            apply_classical_profile_variant: __apply_classical_profile_variant_value,
            video_cue_sheet_references: if __video_cue_sheet_references_value.len() > 0 {
                let is_valid = &__video_cue_sheet_references_value
                    .iter()
                    .all(|val| VideoCueSheetReferenceValidator::validate(&val));
                if *is_valid {
                    __video_cue_sheet_references_value
                } else {
                    Err(format!(
                        "invalid value in field {}",
                        "VideoCueSheetReference"
                    ))?
                }
            } else {
                __video_cue_sheet_references_value
            },
            reason_for_cue_sheet_absence: __reason_for_cue_sheet_absence_value,
        })
    }
}

#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
impl ::yaserde::YaDeserialize for WorkRightsController {
    #[allow(unused_variables)]
    fn deserialize<R: ::std::io::Read>(
        reader: &mut ::yaserde::de::Deserializer<R>,
    ) -> ::std::result::Result<Self, ::std::string::String> {
        let (named_element, struct_namespace) =
            if let ::yaserde::__xml::reader::XmlEvent::StartElement { name, .. } =
                reader.peek()?.to_owned()
            {
                (name.local_name.to_owned(), name.namespace.clone())
            } else {
                (
                    ::std::string::String::from("ern:WorkRightsController"),
                    ::std::option::Option::None,
                )
            };
        let start_depth = reader.depth();

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: start to parse {:?}",
            stringify!(WorkRightsController),
            start_depth,
            named_element
        );

        if reader.depth() == 0 {
            if let Some(namespace) = struct_namespace {
                match namespace.as_str() {
                    "http://ddex.net/xml/ern/43" => {}

                    bad_namespace => {
                        let msg = ::std::format!(
                            "bad namespace for {}, found {}",
                            named_element,
                            bad_namespace
                        );
                        return Err(msg);
                    }
                }
            }
        }
        #[allow(unused_mut)]
        let mut __rights_controller_party_reference_value: Option<::std::string::String> = None;
        #[allow(unused_mut)]
        let mut __rights_control_types_value: ::std::vec::Vec<::std::string::String> = vec![];
        #[allow(unused_mut)]
        let mut __rights_controller_type_value = None;
        #[allow(unused_mut)]
        let mut __territorys_value: ::std::vec::Vec<AllTerritoryCode> = vec![];
        #[allow(unused_mut)]
        let mut __start_date_value = None;
        #[allow(unused_mut)]
        let mut __end_date_value = None;
        #[allow(unused_mut)]
        let mut __right_share_unknown_value = None;
        #[allow(unused_mut)]
        let mut __right_share_percentage_value = None;
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_RightsControllerPartyReference_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_RightsControllerPartyReference_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_RightsControlType_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_RightsControlType_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_RightsControllerType_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_RightsControllerType_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_StartDate_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_StartDate_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_EndDate_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_EndDate_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_RightShareUnknown_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_RightShareUnknown_ {
            type Value = bool;
            fn visit_bool(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                bool::from_str(match v {
                    "1" => "true",
                    "0" => "false",
                    _ => v,
                })
                .map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_RightSharePercentage_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_RightSharePercentage_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        let mut depth = 0;
        loop {
            let event = reader.peek()?.to_owned();

            ::yaserde::__derive_trace!(
                "Struct {} @ {}: matching {:?}",
                stringify!(WorkRightsController),
                start_depth,
                event
            );

            match event {
                ::yaserde::__xml::reader::XmlEvent::StartElement {
                    ref name,
                    ref attributes,
                    ..
                } => {
                    if depth == 0 && name.local_name == "ern:WorkRightsController" {
                        let event = reader.next_event()?;
                    } else {
                        match name.local_name.as_str() {
                            "RightsControllerPartyReference" => {
                                let visitor = __Visitor_RightsControllerPartyReference_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "RightsControllerPartyReference"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __rights_controller_party_reference_value =
                                        ::std::option::Option::Some(value)
                                }
                            }
                            "RightsControlType" => {
                                let visitor = __Visitor_RightsControlType_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "RightsControlType"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __rights_control_types_value.push(value)
                                }
                            }
                            "RightsControllerType" => {
                                let visitor = __Visitor_RightsControllerType_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "RightsControllerType"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __rights_controller_type_value =
                                        ::std::option::Option::Some(value)
                                }
                            }
                            "Territory" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =  <AllTerritoryCode as ::yaserde::YaDeserialize> ::deserialize(reader)? ;
                                    __territorys_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "StartDate" => {
                                let visitor = __Visitor_StartDate_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "StartDate"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __start_date_value = ::std::option::Option::Some(value)
                                }
                            }
                            "EndDate" => {
                                let visitor = __Visitor_EndDate_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "EndDate"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __end_date_value = ::std::option::Option::Some(value)
                                }
                            }
                            "RightShareUnknown" => {
                                let visitor = __Visitor_RightShareUnknown_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result = reader.read_inner_value::<bool, _>(|reader| {
                                    if let ::std::result::Result::Ok(
                                        ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                    ) = reader.peek()
                                    {
                                        let val = visitor.visit_bool(&s);
                                        let _event = reader.next_event()?;
                                        val
                                    } else {
                                        ::std::result::Result::Err(::std::format!(
                                            "unable to parse content for {}",
                                            "RightShareUnknown"
                                        ))
                                    }
                                });
                                if let ::std::result::Result::Ok(value) = result {
                                    __right_share_unknown_value = ::std::option::Option::Some(value)
                                }
                            }
                            "RightSharePercentage" => {
                                let visitor = __Visitor_RightSharePercentage_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "RightSharePercentage"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __right_share_percentage_value =
                                        ::std::option::Option::Some(value)
                                }
                            }
                            _ => {
                                let event = reader.next_event()?;
                                if depth > 0 {
                                    reader.skip_element(|event| {})?;
                                }
                            }
                        }
                    }
                    if depth == 0 {}
                    depth += 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndElement { ref name } => {
                    if name.local_name == named_element && reader.depth() == start_depth + 1 {
                        break;
                    }
                    let event = reader.next_event()?;
                    depth -= 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndDocument => {
                    if false {
                        break;
                    }
                }
                ::yaserde::__xml::reader::XmlEvent::Characters(ref text_content) => {
                    let event = reader.next_event()?;
                }
                event => {
                    return ::std::result::Result::Err(::std::format!("unknown event {:?}", event));
                }
            }
        }

        if __right_share_unknown_value.is_some() && __right_share_percentage_value.is_some() {
            Err(
                "WorkRightController: RightShareUnknown and RightSharePercentage are exclusive"
                    .to_string(),
            )?
        }

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: success",
            stringify!(WorkRightsController),
            start_depth
        );

        ::std::result::Result::Ok(WorkRightsController {
            rights_controller_party_reference: match __rights_controller_party_reference_value {
                Some(val) => {
                    if RightsControllerPartyReferenceValidator::validate(&val) {
                        val
                    } else {
                        Err("invalid value in field WorkRightsController".to_string())?
                    }
                }
                None => Err(
                    "rights_controller_party_reference is a required field of WorkRightsController"
                        .to_string(),
                )?,
            },
            rights_control_types: if __rights_control_types_value.len() > 0 {
                let is_valid = &__rights_control_types_value
                    .iter()
                    .all(|val| AvsRightsControllerRoleValidator::validate(&val));
                if *is_valid {
                    __rights_control_types_value
                } else {
                    Err(format!("invalid value in field {}", "RightsControlType"))?
                }
            } else {
                __rights_control_types_value
            },
            rights_controller_type: match &__rights_controller_type_value {
                Some(val) => {
                    if AvsRightsControllerTypeValidator::validate(&val) {
                        __rights_controller_type_value
                    } else {
                        Err(format!(
                            "invalid value in attribute {}",
                            "RightsControllerType"
                        ))?
                    }
                }
                None => __rights_controller_type_value,
            },
            territorys: __territorys_value,
            start_date: match &__start_date_value {
                Some(val) => {
                    if DdexIsoDateValidator::validate(&val) {
                        __start_date_value
                    } else {
                        Err(format!("invalid value in attribute {}", "StartDate"))?
                    }
                }
                None => __start_date_value,
            },
            end_date: match &__end_date_value {
                Some(val) => {
                    if DdexIsoDateValidator::validate(&val) {
                        __end_date_value
                    } else {
                        Err(format!("invalid value in attribute {}", "EndDate"))?
                    }
                }
                None => __end_date_value,
            },
            right_share_unknown: __right_share_unknown_value,
            right_share_percentage: __right_share_percentage_value,
        })
    }
}

#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
impl ::yaserde::YaDeserialize for Affiliation {
    #[allow(unused_variables)]
    fn deserialize<R: ::std::io::Read>(
        reader: &mut ::yaserde::de::Deserializer<R>,
    ) -> ::std::result::Result<Self, ::std::string::String> {
        let (named_element, struct_namespace) =
            if let ::yaserde::__xml::reader::XmlEvent::StartElement { name, .. } =
                reader.peek()?.to_owned()
            {
                (name.local_name.to_owned(), name.namespace.clone())
            } else {
                (
                    ::std::string::String::from("ern:Affiliation"),
                    ::std::option::Option::None,
                )
            };
        let start_depth = reader.depth();

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: start to parse {:?}",
            stringify!(Affiliation),
            start_depth,
            named_element
        );

        if reader.depth() == 0 {
            if let Some(namespace) = struct_namespace {
                match namespace.as_str() {
                    "http://ddex.net/xml/ern/43" => {}

                    bad_namespace => {
                        let msg = ::std::format!(
                            "bad namespace for {}, found {}",
                            named_element,
                            bad_namespace
                        );
                        return Err(msg);
                    }
                }
            }
        }
        #[allow(unused_mut)]
        let mut __kind_value: Option<::std::string::String> = None;
        #[allow(unused_mut)]
        let mut __validity_period_value = None;
        #[allow(unused_mut)]
        let mut __rights_types_value: ::std::vec::Vec<RightsType> = vec![];
        #[allow(unused_mut)]
        let mut __percentage_of_rights_assignment_value = None;
        #[allow(unused_mut)]
        let mut __company_name_value = None;
        #[allow(unused_mut)]
        let mut __party_affiliate_reference_value = None;
        #[allow(unused_mut)]
        let mut __territory_codes_value: ::std::vec::Vec<::std::string::String> = vec![];
        #[allow(unused_mut)]
        let mut __excluded_territory_codes_value: ::std::vec::Vec<::std::string::String> = vec![];
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_Type_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_Type_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_PercentageOfRightsAssignment_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_PercentageOfRightsAssignment_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_CompanyName_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_CompanyName_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_PartyAffiliateReference_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_PartyAffiliateReference_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_TerritoryCode_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_TerritoryCode_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_ExcludedTerritoryCode_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_ExcludedTerritoryCode_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        let mut depth = 0;
        loop {
            let event = reader.peek()?.to_owned();

            ::yaserde::__derive_trace!(
                "Struct {} @ {}: matching {:?}",
                stringify!(Affiliation),
                start_depth,
                event
            );

            match event {
                ::yaserde::__xml::reader::XmlEvent::StartElement {
                    ref name,
                    ref attributes,
                    ..
                } => {
                    if depth == 0 && name.local_name == "ern:Affiliation" {
                        let event = reader.next_event()?;
                    } else {
                        match name.local_name.as_str() {
                            "Type" => {
                                let visitor = __Visitor_Type_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "Type"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __kind_value = ::std::option::Option::Some(value)
                                }
                            }
                            "ValidityPeriod" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <ValidityPeriod as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __validity_period_value = ::std::option::Option::Some(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "RightsType" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <RightsType as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __rights_types_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "PercentageOfRightsAssignment" => {
                                let visitor = __Visitor_PercentageOfRightsAssignment_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "PercentageOfRightsAssignment"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __percentage_of_rights_assignment_value =
                                        ::std::option::Option::Some(value)
                                }
                            }
                            "CompanyName" => {
                                let visitor = __Visitor_CompanyName_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "CompanyName"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __company_name_value = ::std::option::Option::Some(value)
                                }
                            }
                            "PartyAffiliateReference" => {
                                let visitor = __Visitor_PartyAffiliateReference_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "PartyAffiliateReference"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __party_affiliate_reference_value =
                                        ::std::option::Option::Some(value)
                                }
                            }
                            "TerritoryCode" => {
                                let visitor = __Visitor_TerritoryCode_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "TerritoryCode"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __territory_codes_value.push(value)
                                }
                            }
                            "ExcludedTerritoryCode" => {
                                let visitor = __Visitor_ExcludedTerritoryCode_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "ExcludedTerritoryCode"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __excluded_territory_codes_value.push(value)
                                }
                            }
                            _ => {
                                let event = reader.next_event()?;
                                if depth > 0 {
                                    reader.skip_element(|event| {})?;
                                }
                            }
                        }
                    }
                    if depth == 0 {}
                    depth += 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndElement { ref name } => {
                    if name.local_name == named_element && reader.depth() == start_depth + 1 {
                        break;
                    }
                    let event = reader.next_event()?;
                    depth -= 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndDocument => {
                    if false {
                        break;
                    }
                }
                ::yaserde::__xml::reader::XmlEvent::Characters(ref text_content) => {
                    let event = reader.next_event()?;
                }
                event => {
                    return ::std::result::Result::Err(::std::format!("unknown event {:?}", event));
                }
            }
        }

        match (&__company_name_value, &__party_affiliate_reference_value) {
            (Some(_), None) | (None, Some(_)) => {}
            _ => Err("Provide CompanyName or PartyAffiliateReference".to_string())?,
        }

        if __territory_codes_value.len() > 0 && __excluded_territory_codes_value.len() > 0 {
            Err("TerritoryCodes and ExcludedTerritoryCodes are exclusive".to_string())?
        }

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: success",
            stringify!(Affiliation),
            start_depth
        );

        ::std::result::Result::Ok(Affiliation {
            kind: match __kind_value {
                Some(val) => {
                    if AvsAffiliationTypeValidator::validate(&val) {
                        val
                    } else {
                        Err("invalid value in field Affiliation".to_string())?
                    }
                }
                None => Err("kind is a required field of Affiliation".to_string())?,
            },
            validity_period: __validity_period_value,
            rights_types: __rights_types_value,
            percentage_of_rights_assignment: __percentage_of_rights_assignment_value,
            company_name: __company_name_value,
            party_affiliate_reference: match &__party_affiliate_reference_value {
                Some(val) => {
                    if PartyAffiliateReferenceValidator::validate(&val) {
                        __party_affiliate_reference_value
                    } else {
                        Err(format!(
                            "invalid value in attribute {}",
                            "PartyAffiliateReference"
                        ))?
                    }
                }
                None => __party_affiliate_reference_value,
            },
            territory_codes: if __territory_codes_value.len() > 0 {
                let is_valid = &__territory_codes_value
                    .iter()
                    .all(|val| AvsCurrentTerritoryCodeValidator::validate(&val));
                if *is_valid {
                    __territory_codes_value
                } else {
                    Err(format!("invalid value in field {}", "TerritoryCode"))?
                }
            } else {
                __territory_codes_value
            },
            excluded_territory_codes: if __excluded_territory_codes_value.len() > 0 {
                let is_valid = &__excluded_territory_codes_value
                    .iter()
                    .all(|val| AvsCurrentTerritoryCodeValidator::validate(&val));
                if *is_valid {
                    __excluded_territory_codes_value
                } else {
                    Err(format!(
                        "invalid value in field {}",
                        "ExcludedTerritoryCode"
                    ))?
                }
            } else {
                __excluded_territory_codes_value
            },
        })
    }
}

#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
impl ::yaserde::YaDeserialize for Dsp {
    #[allow(unused_variables)]
    fn deserialize<R: ::std::io::Read>(
        reader: &mut ::yaserde::de::Deserializer<R>,
    ) -> ::std::result::Result<Self, ::std::string::String> {
        let (named_element, struct_namespace) =
            if let ::yaserde::__xml::reader::XmlEvent::StartElement { name, .. } =
                reader.peek()?.to_owned()
            {
                (name.local_name.to_owned(), name.namespace.clone())
            } else {
                (
                    ::std::string::String::from("ern:Dsp"),
                    ::std::option::Option::None,
                )
            };
        let start_depth = reader.depth();

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: start to parse {:?}",
            stringify!(Dsp),
            start_depth,
            named_element
        );

        if reader.depth() == 0 {
            if let Some(namespace) = struct_namespace {
                match namespace.as_str() {
                    "http://ddex.net/xml/ern/43" => {}

                    bad_namespace => {
                        let msg = ::std::format!(
                            "bad namespace for {}, found {}",
                            named_element,
                            bad_namespace
                        );
                        return Err(msg);
                    }
                }
            }
        }
        #[allow(unused_mut)]
        let mut __trading_name_value = None;
        #[allow(unused_mut)]
        let mut __urls_value: ::std::vec::Vec<::std::string::String> = vec![];
        #[allow(unused_mut)]
        let mut __parties_names_value: ::std::vec::Vec<PartyName> = vec![];
        #[allow(unused_mut)]
        let mut __parties_ids_value: ::std::vec::Vec<DetailedPartyId> = vec![];
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_Url_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_Url_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        let mut depth = 0;
        loop {
            let event = reader.peek()?.to_owned();

            ::yaserde::__derive_trace!(
                "Struct {} @ {}: matching {:?}",
                stringify!(Dsp),
                start_depth,
                event
            );

            match event {
                ::yaserde::__xml::reader::XmlEvent::StartElement {
                    ref name,
                    ref attributes,
                    ..
                } => {
                    if depth == 0 && name.local_name == "ern:Dsp" {
                        let event = reader.next_event()?;
                    } else {
                        match name.local_name.as_str() {
                            "TradingName" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <Name as ::yaserde::YaDeserialize>::deserialize(reader)?;
                                    __trading_name_value = ::std::option::Option::Some(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "URL" => {
                                let visitor = __Visitor_Url_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "URL"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __urls_value.push(value)
                                }
                            }
                            "PartyName" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <PartyName as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __parties_names_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            "PartyId" => {
                                if depth == 0 {
                                    let _root = reader.next_event();
                                }
                                if let Ok(::yaserde::__xml::reader::XmlEvent::StartElement {
                                    ..
                                }) = reader.peek()
                                {
                                    let value =
                                        <DetailedPartyId as ::yaserde::YaDeserialize>::deserialize(
                                            reader,
                                        )?;
                                    __parties_ids_value.push(value);
                                    let _event = reader.next_event()?;
                                }
                            }
                            _ => {
                                let event = reader.next_event()?;
                                if depth > 0 {
                                    reader.skip_element(|event| {})?;
                                }
                            }
                        }
                    }
                    if depth == 0 {}
                    depth += 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndElement { ref name } => {
                    if name.local_name == named_element && reader.depth() == start_depth + 1 {
                        break;
                    }
                    let event = reader.next_event()?;
                    depth -= 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndDocument => {
                    if false {
                        break;
                    }
                }
                ::yaserde::__xml::reader::XmlEvent::Characters(ref text_content) => {
                    let event = reader.next_event()?;
                }
                event => {
                    return ::std::result::Result::Err(::std::format!("unknown event {:?}", event));
                }
            }
        }

        if __parties_names_value.len() == 0 && __parties_ids_value.len() == 0 {
            return Err("At least one PartyName and/or PartyId is required".to_string());
        }

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: success",
            stringify!(Dsp),
            start_depth
        );

        ::std::result::Result::Ok(Dsp {
            trading_name: __trading_name_value,
            urls: __urls_value,
            parties_names: __parties_names_value,
            parties_ids: __parties_ids_value,
        })
    }
}

#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
impl ::yaserde::YaDeserialize for DisplayCredits {
    #[allow(unused_variables)]
    fn deserialize<R: ::std::io::Read>(
        reader: &mut ::yaserde::de::Deserializer<R>,
    ) -> ::std::result::Result<Self, ::std::string::String> {
        let (named_element, struct_namespace) =
            if let ::yaserde::__xml::reader::XmlEvent::StartElement { name, .. } =
                reader.peek()?.to_owned()
            {
                (name.local_name.to_owned(), name.namespace.clone())
            } else {
                (
                    ::std::string::String::from("ern:DisplayCredits"),
                    ::std::option::Option::None,
                )
            };
        let start_depth = reader.depth();

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: start to parse {:?}",
            stringify!(DisplayCredits),
            start_depth,
            named_element
        );

        if reader.depth() == 0 {
            if let Some(namespace) = struct_namespace {
                match namespace.as_str() {
                    "http://ddex.net/xml/ern/43" => {}

                    bad_namespace => {
                        let msg = ::std::format!(
                            "bad namespace for {}, found {}",
                            named_element,
                            bad_namespace
                        );
                        return Err(msg);
                    }
                }
            }
        }
        let mut __display_credit_text_value: Option<::std::string::String> = None;
        let mut __display_credit_parties_value: ::std::vec::Vec<::std::string::String> = vec![];
        let mut __names_used_in_display_credits_value: ::std::vec::Vec<::std::string::String> =
            vec![];
        let mut __language_and_script_code_value = None;
        let mut __applicable_territory_code_value = None;
        let mut __is_default_value = None;

        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_DisplayCreditText_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_DisplayCreditText_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_DisplayCreditParty_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_DisplayCreditParty_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_NameUsedInDisplayCredit_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_NameUsedInDisplayCredit_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_Attribute_LanguageAndScriptCode_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_Attribute_LanguageAndScriptCode_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_Attribute_ApplicableTerritoryCode_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_Attribute_ApplicableTerritoryCode_ {
            type Value = ::std::string::String;
            fn visit_str(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                ::std::string::String::from_str(v).map_err(|e| e.to_string())
            }
        }
        #[allow(non_snake_case, non_camel_case_types)]
        struct __Visitor_Attribute_IsDefault_;

        impl<'de> ::yaserde::Visitor<'de> for __Visitor_Attribute_IsDefault_ {
            type Value = bool;
            fn visit_bool(
                self,
                v: &str,
            ) -> ::std::result::Result<Self::Value, ::std::string::String> {
                bool::from_str(match v {
                    "1" => "true",
                    "0" => "false",
                    _ => v,
                })
                .map_err(|e| e.to_string())
            }
        }
        let mut depth = 0;

        loop {
            let event = reader.peek()?.to_owned();

            ::yaserde::__derive_trace!(
                "Struct {} @ {}: matching {:?}",
                stringify!(DisplayCredits),
                start_depth,
                event
            );

            match event {
                ::yaserde::__xml::reader::XmlEvent::StartElement {
                    ref name,
                    ref attributes,
                    ..
                } => {
                    if depth == 0 && name.local_name == "ern:DisplayCredits" {
                        let event = reader.next_event()?;
                    } else {
                        match name.local_name.as_str() {
                            "DisplayCreditText" => {
                                let visitor = __Visitor_DisplayCreditText_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "DisplayCreditText"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __display_credit_text_value = ::std::option::Option::Some(value)
                                }
                            }
                            "DisplayCreditParty" => {
                                let visitor = __Visitor_DisplayCreditParty_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }
                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "DisplayCreditParty"
                                            ))
                                        }
                                    });
                                if let ::std::result::Result::Ok(value) = result {
                                    __display_credit_parties_value.push(value)
                                }
                            }
                            "NameUsedInDisplayCredit" => {
                                let visitor = __Visitor_NameUsedInDisplayCredit_ {};
                                if let Some(namespace) = name.namespace.as_ref() {
                                    match namespace.as_str() {
                                        "http://ddex.net/xml/ern/43" => {}

                                        bad_namespace => {
                                            let msg = ::std::format!(
                                                "bad namespace for {}, found {}",
                                                name.local_name.as_str(),
                                                bad_namespace
                                            );
                                            return Err(msg);
                                        }
                                    }
                                }

                                let result =
                                    reader.read_inner_value::<::std::string::String, _>(|reader| {
                                        if let ::std::result::Result::Ok(
                                            ::yaserde::__xml::reader::XmlEvent::Characters(s),
                                        ) = reader.peek()
                                        {
                                            let val = visitor.visit_str(&s);
                                            let _event = reader.next_event()?;
                                            val
                                        } else {
                                            ::std::result::Result::Err(::std::format!(
                                                "unable to parse content for {}",
                                                "NameUsedInDisplayCredit"
                                            ))
                                        }
                                    });

                                if let ::std::result::Result::Ok(value) = result {
                                    __names_used_in_display_credits_value.push(value)
                                }
                            }
                            _ => {
                                let event = reader.next_event()?;
                                if depth > 0 {
                                    reader.skip_element(|event| {})?;
                                }
                            }
                        };
                    }

                    if depth == 0 {
                        for attr in attributes {
                            if attr.name.local_name == "LanguageAndScriptCode" {
                                let visitor = __Visitor_Attribute_LanguageAndScriptCode_ {};
                                let value = visitor.visit_str(&attr.value)?;
                                __language_and_script_code_value =
                                    ::std::option::Option::Some(value);
                            }
                        }
                        for attr in attributes {
                            if attr.name.local_name == "ApplicableTerritoryCode" {
                                let visitor = __Visitor_Attribute_ApplicableTerritoryCode_ {};
                                let value = visitor.visit_str(&attr.value)?;
                                __applicable_territory_code_value =
                                    ::std::option::Option::Some(value);
                            }
                        }
                        for attr in attributes {
                            if attr.name.local_name == "IsDefault" {
                                let visitor = __Visitor_Attribute_IsDefault_ {};
                                let value = visitor.visit_bool(&attr.value)?;
                                __is_default_value = ::std::option::Option::Some(value);
                            }
                        }
                    }
                    depth += 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndElement { ref name } => {
                    if name.local_name == named_element && reader.depth() == start_depth + 1 {
                        break;
                    }
                    let event = reader.next_event()?;
                    depth -= 1;
                }
                ::yaserde::__xml::reader::XmlEvent::EndDocument => {
                    if false {
                        break;
                    }
                }
                ::yaserde::__xml::reader::XmlEvent::Characters(ref text_content) => {
                    let event = reader.next_event()?;
                }
                event => {
                    return ::std::result::Result::Err(::std::format!("unknown event {:?}", event));
                }
            }
        }

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: success",
            stringify!(DisplayCredits),
            start_depth
        );

        ::std::result::Result::Ok(DisplayCredits {
            display_credit_text: __display_credit_text_value.ok_or_else(|| {
                "display_credit_text is a required field of DisplayCredits".to_string()
            })?,
            display_credit_parties: if __display_credit_parties_value.len() > 0 {
                let is_valid = &__display_credit_parties_value
                    .iter()
                    .all(|val| DisplayCreditPartyValidator::validate(&val));
                if *is_valid {
                    __display_credit_parties_value.to_owned()
                } else {
                    Err(format!("invalid value in field {}", "DisplayCreditParty"))?
                }
            } else {
                __display_credit_parties_value.to_owned()
            },
            names_used_in_display_credits: if __names_used_in_display_credits_value.len()
                > __display_credit_parties_value.len()
            {
                Err(format!(
                    "Not all NamesUsedInDisplayCredits have corresponding DisplayCreditParty"
                ))?
            } else {
                __names_used_in_display_credits_value
            },
            language_and_script_code: __language_and_script_code_value,
            applicable_territory_code: match &__applicable_territory_code_value {
                Some(val) => {
                    if AvsCurrentTerritoryCodeValidator::validate(&val) {
                        __applicable_territory_code_value
                    } else {
                        Err(format!(
                            "invalid value in attribute {}",
                            "ApplicableTerritoryCode"
                        ))?
                    }
                }
                None => __applicable_territory_code_value,
            },
            is_default: __is_default_value,
        })
    }
}

#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
impl ::yaserde::YaDeserialize for PeriodWithStartDate {
    #[allow(unused_variables)]
    fn deserialize<R: ::std::io::Read>(
        reader: &mut ::yaserde::de::Deserializer<R>,
    ) -> ::std::result::Result<Self, ::std::string::String> {
        let (named_element, enum_namespace) =
            if let ::yaserde::__xml::reader::XmlEvent::StartElement { name, .. } =
                reader.peek()?.to_owned()
            {
                (name.local_name.to_owned(), name.namespace.clone())
            } else {
                (
                    ::std::string::String::from("ern:PeriodWithStartDate"),
                    ::std::option::Option::None,
                )
            };
        let start_depth = reader.depth();

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: start to parse {:?}",
            stringify!(PeriodWithStartDate),
            start_depth,
            named_element
        );

        if let Some(namespace) = enum_namespace {
            match namespace.as_str() {
                "http://ddex.net/xml/ern/43" => {}

                bad_namespace => {
                    let msg = ::std::format!(
                        "bad namespace for {}, found {}",
                        named_element,
                        bad_namespace
                    );
                    return Err(msg);
                }
            }
        }

        let mut enum_value = None;
        let mut start_date = None;
        let mut end_date = None;
        let mut start_date_time = None;
        let mut end_date_time = None;

        loop {
            let event = reader.peek()?.to_owned();

            ::yaserde::__derive_trace!(
                "Struct {} @ {}: matching {:?}",
                stringify!(PeriodWithStartDate),
                start_depth,
                event
            );

            match event {
                        ::yaserde::__xml::reader::XmlEvent::StartElement {
                            ref name,
                            ref attributes,
                            ..
                        } => {
                            match name.local_name.as_str(){
                                "StartDate" => {
                                    match<EventDateWithCurrentTerritory as ::yaserde::YaDeserialize> ::deserialize(reader){
                                        Ok(value) => {
                                            start_date = Some(value);
                                            let _root = reader.next_event();
                                        },
                                        Err(msg) => {
                                            return Err(msg);
                                        },
                                    }
                                },
                                "EndDate" => {
                                    match<EventDateWithCurrentTerritory as ::yaserde::YaDeserialize> ::deserialize(reader){
                                        Ok(value) => {
                                            end_date = Some(value);
                                            let _root = reader.next_event();
                                        },
                                        Err(msg) => {
                                            return Err(msg);
                                        },
                                    }
                                },
                                "StartDateTime" => {
                                    match<EventDateTimeWithoutFlags as ::yaserde::YaDeserialize> ::deserialize(reader){
                                        Ok(value) => {
                                            start_date_time = Some(value);
                                            let _root = reader.next_event();
                                        },
                                        Err(msg) => {
                                            return Err(msg);
                                        },
                                    }
                                },
                                "EndDateTime" => {
                                    match<EventDateTimeWithoutFlags as ::yaserde::YaDeserialize> ::deserialize(reader){
                                        Ok(value) => {
                                            end_date_time = Some(value);
                                            let _root = reader.next_event();
                                        },
                                        Err(msg) => {
                                            return Err(msg);
                                        },
                                    }
                                },
                                _ => {
                                    let _root = reader.next_event();
                                }
                            }
                        }
                        ::yaserde::__xml::reader::XmlEvent::EndElement { ref name } => {
                            if name.local_name == named_element
                                && reader.depth() == start_depth + 1
                            {
                                break;
                            }
                            let _root = reader.next_event();
                        }
                        ::yaserde::__xml::reader::XmlEvent::Characters(ref text_content) => {
                            let _root = reader.next_event();
                        }
                        ::yaserde::__xml::reader::XmlEvent::EndDocument => {
                            if false {
                                break;
                            }
                            return ::std::result::Result::Err(::std::format!(
                                "End of document, missing some content ?"
                            ));
                        }
                        event => {
                            return ::std::result::Result::Err(::std::format!(
                                "unknown event {:?}",
                                event
                            ))
                        }
                    }
        }

        match (start_date, &end_date, start_date_time, &end_date_time) {
            (Some(sd), _, None, None) => {
                enum_value = Some(PeriodWithStartDate::EventDateWithCurrentTerritory(
                    EventDateWithCurrentTerritoryRequired {
                        start_date: sd,
                        end_date,
                    },
                ))
            }
            (None, None, Some(sdt), _) => {
                enum_value = Some(PeriodWithStartDate::EventDateTimeWithoutFlags(
                    EventDateTimeWithoutFlagsRequired {
                        start_date_time: sdt,
                        end_date_time,
                    },
                ))
            }
            _ => Err("PeriodStartDate: Invalid value".to_string())?,
        }

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: success",
            stringify!(PeriodStartDate),
            start_depth
        );

        ::std::result::Result::Ok(
            enum_value.ok_or_else(|| "Invalid format of PeriodStartDate".to_string())?,
        )
    }
}

#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
impl ::yaserde::YaDeserialize for PeriodWithoutFlags {
    #[allow(unused_variables)]
    fn deserialize<R: ::std::io::Read>(
        reader: &mut ::yaserde::de::Deserializer<R>,
    ) -> ::std::result::Result<Self, ::std::string::String> {
        let (named_element, enum_namespace) =
            if let ::yaserde::__xml::reader::XmlEvent::StartElement { name, .. } =
                reader.peek()?.to_owned()
            {
                (name.local_name.to_owned(), name.namespace.clone())
            } else {
                (
                    ::std::string::String::from("ern:PeriodWithoutFlags"),
                    ::std::option::Option::None,
                )
            };
        let start_depth = reader.depth();

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: start to parse {:?}",
            stringify!(PeriodWithoutFlags),
            start_depth,
            named_element
        );

        if let Some(namespace) = enum_namespace {
            match namespace.as_str() {
                "http://ddex.net/xml/ern/43" => {}

                bad_namespace => {
                    let msg = ::std::format!(
                        "bad namespace for {}, found {}",
                        named_element,
                        bad_namespace
                    );
                    return Err(msg);
                }
            }
        }

        let mut enum_value = None;
        let mut start_date = None;
        let mut end_date = None;
        let mut start_date_time = None;
        let mut end_date_time = None;

        loop {
            let event = reader.peek()?.to_owned();

            ::yaserde::__derive_trace!(
                "Struct {} @ {}: matching {:?}",
                stringify!(PeriodWithoutFlags),
                start_depth,
                event
            );

            match event {
                        ::yaserde::__xml::reader::XmlEvent::StartElement {
                            ref name,
                            ref attributes,
                            ..
                        } => {
                            match name.local_name.as_str(){
                                "StartDate" => {
                                    match<EventDateWithCurrentTerritory as ::yaserde::YaDeserialize> ::deserialize(reader){
                                        Ok(value) => {
                                            start_date = Some(value);
                                            let _root = reader.next_event();
                                        },
                                        Err(msg) => {
                                            return Err(msg);
                                        },
                                    }
                                },
                                "EndDate" => {
                                    match<EventDateWithCurrentTerritory as ::yaserde::YaDeserialize> ::deserialize(reader){
                                        Ok(value) => {
                                            end_date = Some(value);
                                            let _root = reader.next_event();
                                        },
                                        Err(msg) => {
                                            return Err(msg);
                                        },
                                    }
                                },
                                "StartDateTime" => {
                                    match<EventDateTimeWithoutFlags as ::yaserde::YaDeserialize> ::deserialize(reader){
                                        Ok(value) => {
                                            start_date_time = Some(value);
                                            let _root = reader.next_event();
                                        },
                                        Err(msg) => {
                                            return Err(msg);
                                        },
                                    }
                                },
                                "EndDateTime" => {
                                    match<EventDateTimeWithoutFlags as ::yaserde::YaDeserialize> ::deserialize(reader){
                                        Ok(value) => {
                                            end_date_time = Some(value);
                                            let _root = reader.next_event();
                                        },
                                        Err(msg) => {
                                            return Err(msg);
                                        },
                                    }
                                },
                                _ => {
                                    let _root = reader.next_event();
                                }
                            }
                        }
                        ::yaserde::__xml::reader::XmlEvent::EndElement { ref name } => {
                            if name.local_name == named_element
                                && reader.depth() == start_depth + 1
                            {
                                break;
                            }
                            let _root = reader.next_event();
                        }
                        ::yaserde::__xml::reader::XmlEvent::Characters(ref text_content) => {
                            let _root = reader.next_event();
                        }
                        ::yaserde::__xml::reader::XmlEvent::EndDocument => {
                            if false {
                                break;
                            }
                            return ::std::result::Result::Err(::std::format!(
                                "End of document, missing some content ?"
                            ));
                        }
                        event => {
                            return ::std::result::Result::Err(::std::format!(
                                "unknown event {:?}",
                                event
                            ))
                        }
                    }
        }

        match (&start_date, &end_date, &start_date_time, &end_date_time) {
            (_, _, None, None) => {
                enum_value = Some(PeriodWithoutFlags::EventDateWithCurrentTerritory(
                    EventDateWithCurrentTerritoryOptional {
                        start_date,
                        end_date,
                    },
                ))
            }
            (None, None, _, _) => {
                enum_value = Some(PeriodWithoutFlags::EventDateTimeWithoutFlags(
                    EventDateTimeWithoutFlagsOptional {
                        start_date_time,
                        end_date_time,
                    },
                ))
            }
            _ => Err("PeriodWithoutFlags: Invalid value".to_string())?,
        }

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: success",
            stringify!(PeriodWithoutFlags),
            start_depth
        );

        ::std::result::Result::Ok(
            enum_value.ok_or_else(|| "Illegal empty tag of PeriodWithoutFlags".to_string())?,
        )
    }
}

#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
impl ::yaserde::YaDeserialize for Period {
    #[allow(unused_variables)]
    fn deserialize<R: ::std::io::Read>(
        reader: &mut ::yaserde::de::Deserializer<R>,
    ) -> ::std::result::Result<Self, ::std::string::String> {
        let (named_element, enum_namespace) =
            if let ::yaserde::__xml::reader::XmlEvent::StartElement { name, .. } =
                reader.peek()?.to_owned()
            {
                (name.local_name.to_owned(), name.namespace.clone())
            } else {
                (
                    ::std::string::String::from("ern:Period"),
                    ::std::option::Option::None,
                )
            };
        let start_depth = reader.depth();

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: start to parse {:?}",
            stringify!(Period),
            start_depth,
            named_element
        );

        if let Some(namespace) = enum_namespace {
            match namespace.as_str() {
                "http://ddex.net/xml/ern/43" => {}

                bad_namespace => {
                    let msg = ::std::format!(
                        "bad namespace for {}, found {}",
                        named_element,
                        bad_namespace
                    );
                    return Err(msg);
                }
            }
        }

        let mut enum_value = None;
        let mut start_date = None;
        let mut end_date = None;
        let mut start_date_time = None;
        let mut end_date_time = None;

        loop {
            let event = reader.peek()?.to_owned();

            ::yaserde::__derive_trace!(
                "Struct {} @ {}: matching {:?}",
                stringify!(Period),
                start_depth,
                event
            );

            match event {
                ::yaserde::__xml::reader::XmlEvent::StartElement {
                    ref name,
                    ref attributes,
                    ..
                } => match name.local_name.as_str() {
                    "StartDate" => {
                        match <EventDate as ::yaserde::YaDeserialize>::deserialize(reader) {
                            Ok(value) => {
                                start_date = Some(value);
                                let _root = reader.next_event();
                            }
                            Err(msg) => {
                                return Err(msg);
                            }
                        }
                    }
                    "EndDate" => {
                        match <EventDate as ::yaserde::YaDeserialize>::deserialize(reader) {
                            Ok(value) => {
                                end_date = Some(value);
                                let _root = reader.next_event();
                            }
                            Err(msg) => {
                                return Err(msg);
                            }
                        }
                    }
                    "StartDateTime" => {
                        match <EventDateTime as ::yaserde::YaDeserialize>::deserialize(reader) {
                            Ok(value) => {
                                start_date_time = Some(value);
                                let _root = reader.next_event();
                            }
                            Err(msg) => {
                                return Err(msg);
                            }
                        }
                    }
                    "EndDateTime" => {
                        match <EventDateTime as ::yaserde::YaDeserialize>::deserialize(reader) {
                            Ok(value) => {
                                end_date_time = Some(value);
                                let _root = reader.next_event();
                            }
                            Err(msg) => {
                                return Err(msg);
                            }
                        }
                    }
                    _ => {
                        let _root = reader.next_event();
                    }
                },
                ::yaserde::__xml::reader::XmlEvent::EndElement { ref name } => {
                    if name.local_name == named_element && reader.depth() == start_depth + 1 {
                        break;
                    }
                    let _root = reader.next_event();
                }
                ::yaserde::__xml::reader::XmlEvent::Characters(ref text_content) => {
                    let _root = reader.next_event();
                }
                ::yaserde::__xml::reader::XmlEvent::EndDocument => {
                    if false {
                        break;
                    }
                    return ::std::result::Result::Err(::std::format!(
                        "End of document, missing some content ?"
                    ));
                }
                event => {
                    return ::std::result::Result::Err(::std::format!("unknown event {:?}", event))
                }
            }
        }

        match (&start_date, &end_date, &start_date_time, &end_date_time) {
            (_, _, None, None) => {
                enum_value = Some(Period::EventDate(EventDateOptional {
                    start_date,
                    end_date,
                }))
            }
            (None, None, _, _) => {
                enum_value = Some(Period::EventDateTime(EventDateTimeOptional {
                    start_date_time,
                    end_date_time,
                }))
            }
            _ => Err("Period: Invalid value".to_string())?,
        }

        ::yaserde::__derive_debug!(
            "Struct {} @ {}: success",
            stringify!(Period),
            start_depth
        );

        ::std::result::Result::Ok(
            enum_value.ok_or_else(|| "Illegal empty tag of Period".to_string())?,
        )
    }
}