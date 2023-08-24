// @generated
impl serde::Serialize for BlockHeaderResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.block.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.execution.BlockHeaderResponse", len)?;
        if let Some(v) = self.block.as_ref() {
            struct_ser.serialize_field("block", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BlockHeaderResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "block",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Block,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "block" => Ok(GeneratedField::Block),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BlockHeaderResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.execution.BlockHeaderResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BlockHeaderResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut block__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Block => {
                            if block__.is_some() {
                                return Err(serde::de::Error::duplicate_field("block"));
                            }
                            block__ = map.next_value()?;
                        }
                    }
                }
                Ok(BlockHeaderResponse {
                    block: block__,
                })
            }
        }
        deserializer.deserialize_struct("flow.execution.BlockHeaderResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for EventEncodingVersion {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::JsonCdcV0 => "JSON_CDC_V0",
            Self::CcfV0 => "CCF_V0",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for EventEncodingVersion {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "JSON_CDC_V0",
            "CCF_V0",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = EventEncodingVersion;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(EventEncodingVersion::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(EventEncodingVersion::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "JSON_CDC_V0" => Ok(EventEncodingVersion::JsonCdcV0),
                    "CCF_V0" => Ok(EventEncodingVersion::CcfV0),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ExecuteScriptAtBlockIdRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.block_id.is_empty() {
            len += 1;
        }
        if !self.script.is_empty() {
            len += 1;
        }
        if !self.arguments.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.execution.ExecuteScriptAtBlockIDRequest", len)?;
        if !self.block_id.is_empty() {
            struct_ser.serialize_field("blockId", pbjson::private::base64::encode(&self.block_id).as_str())?;
        }
        if !self.script.is_empty() {
            struct_ser.serialize_field("script", pbjson::private::base64::encode(&self.script).as_str())?;
        }
        if !self.arguments.is_empty() {
            struct_ser.serialize_field("arguments", &self.arguments.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExecuteScriptAtBlockIdRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "block_id",
            "blockId",
            "script",
            "arguments",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BlockId,
            Script,
            Arguments,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "blockId" | "block_id" => Ok(GeneratedField::BlockId),
                            "script" => Ok(GeneratedField::Script),
                            "arguments" => Ok(GeneratedField::Arguments),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExecuteScriptAtBlockIdRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.execution.ExecuteScriptAtBlockIDRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExecuteScriptAtBlockIdRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut block_id__ = None;
                let mut script__ = None;
                let mut arguments__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BlockId => {
                            if block_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockId"));
                            }
                            block_id__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Script => {
                            if script__.is_some() {
                                return Err(serde::de::Error::duplicate_field("script"));
                            }
                            script__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Arguments => {
                            if arguments__.is_some() {
                                return Err(serde::de::Error::duplicate_field("arguments"));
                            }
                            arguments__ = 
                                Some(map.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(ExecuteScriptAtBlockIdRequest {
                    block_id: block_id__.unwrap_or_default(),
                    script: script__.unwrap_or_default(),
                    arguments: arguments__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.execution.ExecuteScriptAtBlockIDRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExecuteScriptAtBlockIdResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.execution.ExecuteScriptAtBlockIDResponse", len)?;
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", pbjson::private::base64::encode(&self.value).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExecuteScriptAtBlockIdResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExecuteScriptAtBlockIdResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.execution.ExecuteScriptAtBlockIDResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExecuteScriptAtBlockIdResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ExecuteScriptAtBlockIdResponse {
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.execution.ExecuteScriptAtBlockIDResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetAccountAtBlockIdRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.block_id.is_empty() {
            len += 1;
        }
        if !self.address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.execution.GetAccountAtBlockIDRequest", len)?;
        if !self.block_id.is_empty() {
            struct_ser.serialize_field("blockId", pbjson::private::base64::encode(&self.block_id).as_str())?;
        }
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", pbjson::private::base64::encode(&self.address).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAccountAtBlockIdRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "block_id",
            "blockId",
            "address",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BlockId,
            Address,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "blockId" | "block_id" => Ok(GeneratedField::BlockId),
                            "address" => Ok(GeneratedField::Address),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetAccountAtBlockIdRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.execution.GetAccountAtBlockIDRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetAccountAtBlockIdRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut block_id__ = None;
                let mut address__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BlockId => {
                            if block_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockId"));
                            }
                            block_id__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(GetAccountAtBlockIdRequest {
                    block_id: block_id__.unwrap_or_default(),
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.execution.GetAccountAtBlockIDRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetAccountAtBlockIdResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.account.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.execution.GetAccountAtBlockIDResponse", len)?;
        if let Some(v) = self.account.as_ref() {
            struct_ser.serialize_field("account", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetAccountAtBlockIdResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "account",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Account,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "account" => Ok(GeneratedField::Account),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetAccountAtBlockIdResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.execution.GetAccountAtBlockIDResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetAccountAtBlockIdResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut account__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Account => {
                            if account__.is_some() {
                                return Err(serde::de::Error::duplicate_field("account"));
                            }
                            account__ = map.next_value()?;
                        }
                    }
                }
                Ok(GetAccountAtBlockIdResponse {
                    account: account__,
                })
            }
        }
        deserializer.deserialize_struct("flow.execution.GetAccountAtBlockIDResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetBlockHeaderByIdRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.execution.GetBlockHeaderByIDRequest", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", pbjson::private::base64::encode(&self.id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetBlockHeaderByIdRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "id" => Ok(GeneratedField::Id),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetBlockHeaderByIdRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.execution.GetBlockHeaderByIDRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetBlockHeaderByIdRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(GetBlockHeaderByIdRequest {
                    id: id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.execution.GetBlockHeaderByIDRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetEventsForBlockIDsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.block_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.execution.GetEventsForBlockIDsRequest", len)?;
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.block_ids.is_empty() {
            struct_ser.serialize_field("blockIds", &self.block_ids.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetEventsForBlockIDsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "block_ids",
            "blockIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            BlockIds,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "type" => Ok(GeneratedField::Type),
                            "blockIds" | "block_ids" => Ok(GeneratedField::BlockIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetEventsForBlockIDsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.execution.GetEventsForBlockIDsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetEventsForBlockIDsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut block_ids__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map.next_value()?);
                        }
                        GeneratedField::BlockIds => {
                            if block_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockIds"));
                            }
                            block_ids__ = 
                                Some(map.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(GetEventsForBlockIDsRequest {
                    r#type: r#type__.unwrap_or_default(),
                    block_ids: block_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.execution.GetEventsForBlockIDsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetEventsForBlockIDsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.results.is_empty() {
            len += 1;
        }
        if self.event_encoding_version != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.execution.GetEventsForBlockIDsResponse", len)?;
        if !self.results.is_empty() {
            struct_ser.serialize_field("results", &self.results)?;
        }
        if self.event_encoding_version != 0 {
            let v = EventEncodingVersion::from_i32(self.event_encoding_version)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.event_encoding_version)))?;
            struct_ser.serialize_field("eventEncodingVersion", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetEventsForBlockIDsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "results",
            "event_encoding_version",
            "eventEncodingVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Results,
            EventEncodingVersion,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "results" => Ok(GeneratedField::Results),
                            "eventEncodingVersion" | "event_encoding_version" => Ok(GeneratedField::EventEncodingVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetEventsForBlockIDsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.execution.GetEventsForBlockIDsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetEventsForBlockIDsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut results__ = None;
                let mut event_encoding_version__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Results => {
                            if results__.is_some() {
                                return Err(serde::de::Error::duplicate_field("results"));
                            }
                            results__ = Some(map.next_value()?);
                        }
                        GeneratedField::EventEncodingVersion => {
                            if event_encoding_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventEncodingVersion"));
                            }
                            event_encoding_version__ = Some(map.next_value::<EventEncodingVersion>()? as i32);
                        }
                    }
                }
                Ok(GetEventsForBlockIDsResponse {
                    results: results__.unwrap_or_default(),
                    event_encoding_version: event_encoding_version__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.execution.GetEventsForBlockIDsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for get_events_for_block_i_ds_response::Result {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.block_id.is_empty() {
            len += 1;
        }
        if self.block_height != 0 {
            len += 1;
        }
        if !self.events.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.execution.GetEventsForBlockIDsResponse.Result", len)?;
        if !self.block_id.is_empty() {
            struct_ser.serialize_field("blockId", pbjson::private::base64::encode(&self.block_id).as_str())?;
        }
        if self.block_height != 0 {
            struct_ser.serialize_field("blockHeight", ToString::to_string(&self.block_height).as_str())?;
        }
        if !self.events.is_empty() {
            struct_ser.serialize_field("events", &self.events)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for get_events_for_block_i_ds_response::Result {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "block_id",
            "blockId",
            "block_height",
            "blockHeight",
            "events",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BlockId,
            BlockHeight,
            Events,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "blockId" | "block_id" => Ok(GeneratedField::BlockId),
                            "blockHeight" | "block_height" => Ok(GeneratedField::BlockHeight),
                            "events" => Ok(GeneratedField::Events),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = get_events_for_block_i_ds_response::Result;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.execution.GetEventsForBlockIDsResponse.Result")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<get_events_for_block_i_ds_response::Result, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut block_id__ = None;
                let mut block_height__ = None;
                let mut events__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BlockId => {
                            if block_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockId"));
                            }
                            block_id__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BlockHeight => {
                            if block_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockHeight"));
                            }
                            block_height__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Events => {
                            if events__.is_some() {
                                return Err(serde::de::Error::duplicate_field("events"));
                            }
                            events__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(get_events_for_block_i_ds_response::Result {
                    block_id: block_id__.unwrap_or_default(),
                    block_height: block_height__.unwrap_or_default(),
                    events: events__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.execution.GetEventsForBlockIDsResponse.Result", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetLatestBlockHeaderRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.is_sealed {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.execution.GetLatestBlockHeaderRequest", len)?;
        if self.is_sealed {
            struct_ser.serialize_field("isSealed", &self.is_sealed)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetLatestBlockHeaderRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "is_sealed",
            "isSealed",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IsSealed,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "isSealed" | "is_sealed" => Ok(GeneratedField::IsSealed),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetLatestBlockHeaderRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.execution.GetLatestBlockHeaderRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetLatestBlockHeaderRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut is_sealed__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::IsSealed => {
                            if is_sealed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isSealed"));
                            }
                            is_sealed__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetLatestBlockHeaderRequest {
                    is_sealed: is_sealed__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.execution.GetLatestBlockHeaderRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRegisterAtBlockIdRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.block_id.is_empty() {
            len += 1;
        }
        if !self.register_owner.is_empty() {
            len += 1;
        }
        if !self.register_key.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.execution.GetRegisterAtBlockIDRequest", len)?;
        if !self.block_id.is_empty() {
            struct_ser.serialize_field("blockId", pbjson::private::base64::encode(&self.block_id).as_str())?;
        }
        if !self.register_owner.is_empty() {
            struct_ser.serialize_field("registerOwner", pbjson::private::base64::encode(&self.register_owner).as_str())?;
        }
        if !self.register_key.is_empty() {
            struct_ser.serialize_field("registerKey", pbjson::private::base64::encode(&self.register_key).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRegisterAtBlockIdRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "block_id",
            "blockId",
            "register_owner",
            "registerOwner",
            "register_key",
            "registerKey",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BlockId,
            RegisterOwner,
            RegisterKey,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "blockId" | "block_id" => Ok(GeneratedField::BlockId),
                            "registerOwner" | "register_owner" => Ok(GeneratedField::RegisterOwner),
                            "registerKey" | "register_key" => Ok(GeneratedField::RegisterKey),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetRegisterAtBlockIdRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.execution.GetRegisterAtBlockIDRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetRegisterAtBlockIdRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut block_id__ = None;
                let mut register_owner__ = None;
                let mut register_key__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BlockId => {
                            if block_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockId"));
                            }
                            block_id__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RegisterOwner => {
                            if register_owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("registerOwner"));
                            }
                            register_owner__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::RegisterKey => {
                            if register_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("registerKey"));
                            }
                            register_key__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(GetRegisterAtBlockIdRequest {
                    block_id: block_id__.unwrap_or_default(),
                    register_owner: register_owner__.unwrap_or_default(),
                    register_key: register_key__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.execution.GetRegisterAtBlockIDRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRegisterAtBlockIdResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.execution.GetRegisterAtBlockIDResponse", len)?;
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", pbjson::private::base64::encode(&self.value).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRegisterAtBlockIdResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Value,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "value" => Ok(GeneratedField::Value),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetRegisterAtBlockIdResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.execution.GetRegisterAtBlockIDResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetRegisterAtBlockIdResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(GetRegisterAtBlockIdResponse {
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.execution.GetRegisterAtBlockIDResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetTransactionByIndexRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.block_id.is_empty() {
            len += 1;
        }
        if self.index != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.execution.GetTransactionByIndexRequest", len)?;
        if !self.block_id.is_empty() {
            struct_ser.serialize_field("blockId", pbjson::private::base64::encode(&self.block_id).as_str())?;
        }
        if self.index != 0 {
            struct_ser.serialize_field("index", &self.index)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTransactionByIndexRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "block_id",
            "blockId",
            "index",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BlockId,
            Index,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "blockId" | "block_id" => Ok(GeneratedField::BlockId),
                            "index" => Ok(GeneratedField::Index),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetTransactionByIndexRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.execution.GetTransactionByIndexRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetTransactionByIndexRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut block_id__ = None;
                let mut index__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BlockId => {
                            if block_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockId"));
                            }
                            block_id__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Index => {
                            if index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            index__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(GetTransactionByIndexRequest {
                    block_id: block_id__.unwrap_or_default(),
                    index: index__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.execution.GetTransactionByIndexRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetTransactionResultRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.block_id.is_empty() {
            len += 1;
        }
        if !self.transaction_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.execution.GetTransactionResultRequest", len)?;
        if !self.block_id.is_empty() {
            struct_ser.serialize_field("blockId", pbjson::private::base64::encode(&self.block_id).as_str())?;
        }
        if !self.transaction_id.is_empty() {
            struct_ser.serialize_field("transactionId", pbjson::private::base64::encode(&self.transaction_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTransactionResultRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "block_id",
            "blockId",
            "transaction_id",
            "transactionId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BlockId,
            TransactionId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "blockId" | "block_id" => Ok(GeneratedField::BlockId),
                            "transactionId" | "transaction_id" => Ok(GeneratedField::TransactionId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetTransactionResultRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.execution.GetTransactionResultRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetTransactionResultRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut block_id__ = None;
                let mut transaction_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BlockId => {
                            if block_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockId"));
                            }
                            block_id__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TransactionId => {
                            if transaction_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transactionId"));
                            }
                            transaction_id__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(GetTransactionResultRequest {
                    block_id: block_id__.unwrap_or_default(),
                    transaction_id: transaction_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.execution.GetTransactionResultRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetTransactionResultResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.status_code != 0 {
            len += 1;
        }
        if !self.error_message.is_empty() {
            len += 1;
        }
        if !self.events.is_empty() {
            len += 1;
        }
        if self.event_encoding_version != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.execution.GetTransactionResultResponse", len)?;
        if self.status_code != 0 {
            struct_ser.serialize_field("statusCode", &self.status_code)?;
        }
        if !self.error_message.is_empty() {
            struct_ser.serialize_field("errorMessage", &self.error_message)?;
        }
        if !self.events.is_empty() {
            struct_ser.serialize_field("events", &self.events)?;
        }
        if self.event_encoding_version != 0 {
            let v = EventEncodingVersion::from_i32(self.event_encoding_version)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.event_encoding_version)))?;
            struct_ser.serialize_field("eventEncodingVersion", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTransactionResultResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "status_code",
            "statusCode",
            "error_message",
            "errorMessage",
            "events",
            "event_encoding_version",
            "eventEncodingVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            StatusCode,
            ErrorMessage,
            Events,
            EventEncodingVersion,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "statusCode" | "status_code" => Ok(GeneratedField::StatusCode),
                            "errorMessage" | "error_message" => Ok(GeneratedField::ErrorMessage),
                            "events" => Ok(GeneratedField::Events),
                            "eventEncodingVersion" | "event_encoding_version" => Ok(GeneratedField::EventEncodingVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetTransactionResultResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.execution.GetTransactionResultResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetTransactionResultResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut status_code__ = None;
                let mut error_message__ = None;
                let mut events__ = None;
                let mut event_encoding_version__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::StatusCode => {
                            if status_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("statusCode"));
                            }
                            status_code__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ErrorMessage => {
                            if error_message__.is_some() {
                                return Err(serde::de::Error::duplicate_field("errorMessage"));
                            }
                            error_message__ = Some(map.next_value()?);
                        }
                        GeneratedField::Events => {
                            if events__.is_some() {
                                return Err(serde::de::Error::duplicate_field("events"));
                            }
                            events__ = Some(map.next_value()?);
                        }
                        GeneratedField::EventEncodingVersion => {
                            if event_encoding_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventEncodingVersion"));
                            }
                            event_encoding_version__ = Some(map.next_value::<EventEncodingVersion>()? as i32);
                        }
                    }
                }
                Ok(GetTransactionResultResponse {
                    status_code: status_code__.unwrap_or_default(),
                    error_message: error_message__.unwrap_or_default(),
                    events: events__.unwrap_or_default(),
                    event_encoding_version: event_encoding_version__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.execution.GetTransactionResultResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetTransactionResultsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.transaction_results.is_empty() {
            len += 1;
        }
        if self.event_encoding_version != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.execution.GetTransactionResultsResponse", len)?;
        if !self.transaction_results.is_empty() {
            struct_ser.serialize_field("transactionResults", &self.transaction_results)?;
        }
        if self.event_encoding_version != 0 {
            let v = EventEncodingVersion::from_i32(self.event_encoding_version)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.event_encoding_version)))?;
            struct_ser.serialize_field("eventEncodingVersion", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTransactionResultsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "transaction_results",
            "transactionResults",
            "event_encoding_version",
            "eventEncodingVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TransactionResults,
            EventEncodingVersion,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "transactionResults" | "transaction_results" => Ok(GeneratedField::TransactionResults),
                            "eventEncodingVersion" | "event_encoding_version" => Ok(GeneratedField::EventEncodingVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetTransactionResultsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.execution.GetTransactionResultsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetTransactionResultsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut transaction_results__ = None;
                let mut event_encoding_version__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TransactionResults => {
                            if transaction_results__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transactionResults"));
                            }
                            transaction_results__ = Some(map.next_value()?);
                        }
                        GeneratedField::EventEncodingVersion => {
                            if event_encoding_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventEncodingVersion"));
                            }
                            event_encoding_version__ = Some(map.next_value::<EventEncodingVersion>()? as i32);
                        }
                    }
                }
                Ok(GetTransactionResultsResponse {
                    transaction_results: transaction_results__.unwrap_or_default(),
                    event_encoding_version: event_encoding_version__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.execution.GetTransactionResultsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetTransactionsByBlockIdRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.block_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.execution.GetTransactionsByBlockIDRequest", len)?;
        if !self.block_id.is_empty() {
            struct_ser.serialize_field("blockId", pbjson::private::base64::encode(&self.block_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetTransactionsByBlockIdRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "block_id",
            "blockId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BlockId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "blockId" | "block_id" => Ok(GeneratedField::BlockId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetTransactionsByBlockIdRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.execution.GetTransactionsByBlockIDRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetTransactionsByBlockIdRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut block_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BlockId => {
                            if block_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockId"));
                            }
                            block_id__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(GetTransactionsByBlockIdRequest {
                    block_id: block_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.execution.GetTransactionsByBlockIDRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PingRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("flow.execution.PingRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PingRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PingRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.execution.PingRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PingRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PingRequest {
                })
            }
        }
        deserializer.deserialize_struct("flow.execution.PingRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PingResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer.serialize_struct("flow.execution.PingResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PingResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                            Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PingResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.execution.PingResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PingResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(PingResponse {
                })
            }
        }
        deserializer.deserialize_struct("flow.execution.PingResponse", FIELDS, GeneratedVisitor)
    }
}
