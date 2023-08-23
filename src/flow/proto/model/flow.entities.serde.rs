// @generated
impl serde::Serialize for Account {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        if self.balance != 0 {
            len += 1;
        }
        if !self.code.is_empty() {
            len += 1;
        }
        if !self.keys.is_empty() {
            len += 1;
        }
        if !self.contracts.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.entities.Account", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", pbjson::private::base64::encode(&self.address).as_str())?;
        }
        if self.balance != 0 {
            struct_ser.serialize_field("balance", ToString::to_string(&self.balance).as_str())?;
        }
        if !self.code.is_empty() {
            struct_ser.serialize_field("code", pbjson::private::base64::encode(&self.code).as_str())?;
        }
        if !self.keys.is_empty() {
            struct_ser.serialize_field("keys", &self.keys)?;
        }
        if !self.contracts.is_empty() {
            let v: std::collections::HashMap<_, _> = self.contracts.iter()
                .map(|(k, v)| (k, pbjson::private::base64::encode(v))).collect();
            struct_ser.serialize_field("contracts", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Account {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "balance",
            "code",
            "keys",
            "contracts",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            Balance,
            Code,
            Keys,
            Contracts,
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
                            "address" => Ok(GeneratedField::Address),
                            "balance" => Ok(GeneratedField::Balance),
                            "code" => Ok(GeneratedField::Code),
                            "keys" => Ok(GeneratedField::Keys),
                            "contracts" => Ok(GeneratedField::Contracts),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Account;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.entities.Account")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Account, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut balance__ = None;
                let mut code__ = None;
                let mut keys__ = None;
                let mut contracts__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Balance => {
                            if balance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("balance"));
                            }
                            balance__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Code => {
                            if code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("code"));
                            }
                            code__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Keys => {
                            if keys__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keys"));
                            }
                            keys__ = Some(map.next_value()?);
                        }
                        GeneratedField::Contracts => {
                            if contracts__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contracts"));
                            }
                            contracts__ = Some(
                                map.next_value::<std::collections::HashMap<_, ::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|(k,v)| (k, v.0)).collect()
                            );
                        }
                    }
                }
                Ok(Account {
                    address: address__.unwrap_or_default(),
                    balance: balance__.unwrap_or_default(),
                    code: code__.unwrap_or_default(),
                    keys: keys__.unwrap_or_default(),
                    contracts: contracts__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.entities.Account", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AccountKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.index != 0 {
            len += 1;
        }
        if !self.public_key.is_empty() {
            len += 1;
        }
        if self.sign_algo != 0 {
            len += 1;
        }
        if self.hash_algo != 0 {
            len += 1;
        }
        if self.weight != 0 {
            len += 1;
        }
        if self.sequence_number != 0 {
            len += 1;
        }
        if self.revoked {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.entities.AccountKey", len)?;
        if self.index != 0 {
            struct_ser.serialize_field("index", &self.index)?;
        }
        if !self.public_key.is_empty() {
            struct_ser.serialize_field("publicKey", pbjson::private::base64::encode(&self.public_key).as_str())?;
        }
        if self.sign_algo != 0 {
            struct_ser.serialize_field("signAlgo", &self.sign_algo)?;
        }
        if self.hash_algo != 0 {
            struct_ser.serialize_field("hashAlgo", &self.hash_algo)?;
        }
        if self.weight != 0 {
            struct_ser.serialize_field("weight", &self.weight)?;
        }
        if self.sequence_number != 0 {
            struct_ser.serialize_field("sequenceNumber", &self.sequence_number)?;
        }
        if self.revoked {
            struct_ser.serialize_field("revoked", &self.revoked)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AccountKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "index",
            "public_key",
            "publicKey",
            "sign_algo",
            "signAlgo",
            "hash_algo",
            "hashAlgo",
            "weight",
            "sequence_number",
            "sequenceNumber",
            "revoked",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Index,
            PublicKey,
            SignAlgo,
            HashAlgo,
            Weight,
            SequenceNumber,
            Revoked,
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
                            "index" => Ok(GeneratedField::Index),
                            "publicKey" | "public_key" => Ok(GeneratedField::PublicKey),
                            "signAlgo" | "sign_algo" => Ok(GeneratedField::SignAlgo),
                            "hashAlgo" | "hash_algo" => Ok(GeneratedField::HashAlgo),
                            "weight" => Ok(GeneratedField::Weight),
                            "sequenceNumber" | "sequence_number" => Ok(GeneratedField::SequenceNumber),
                            "revoked" => Ok(GeneratedField::Revoked),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AccountKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.entities.AccountKey")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AccountKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut index__ = None;
                let mut public_key__ = None;
                let mut sign_algo__ = None;
                let mut hash_algo__ = None;
                let mut weight__ = None;
                let mut sequence_number__ = None;
                let mut revoked__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Index => {
                            if index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("index"));
                            }
                            index__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::PublicKey => {
                            if public_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("publicKey"));
                            }
                            public_key__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SignAlgo => {
                            if sign_algo__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signAlgo"));
                            }
                            sign_algo__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::HashAlgo => {
                            if hash_algo__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hashAlgo"));
                            }
                            hash_algo__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Weight => {
                            if weight__.is_some() {
                                return Err(serde::de::Error::duplicate_field("weight"));
                            }
                            weight__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SequenceNumber => {
                            if sequence_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequenceNumber"));
                            }
                            sequence_number__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Revoked => {
                            if revoked__.is_some() {
                                return Err(serde::de::Error::duplicate_field("revoked"));
                            }
                            revoked__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(AccountKey {
                    index: index__.unwrap_or_default(),
                    public_key: public_key__.unwrap_or_default(),
                    sign_algo: sign_algo__.unwrap_or_default(),
                    hash_algo: hash_algo__.unwrap_or_default(),
                    weight: weight__.unwrap_or_default(),
                    sequence_number: sequence_number__.unwrap_or_default(),
                    revoked: revoked__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.entities.AccountKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for AggregatedSignature {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.verifier_signatures.is_empty() {
            len += 1;
        }
        if !self.signer_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.entities.AggregatedSignature", len)?;
        if !self.verifier_signatures.is_empty() {
            struct_ser.serialize_field("verifierSignatures", &self.verifier_signatures.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        if !self.signer_ids.is_empty() {
            struct_ser.serialize_field("signerIds", &self.signer_ids.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AggregatedSignature {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "verifier_signatures",
            "verifierSignatures",
            "signer_ids",
            "signerIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VerifierSignatures,
            SignerIds,
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
                            "verifierSignatures" | "verifier_signatures" => Ok(GeneratedField::VerifierSignatures),
                            "signerIds" | "signer_ids" => Ok(GeneratedField::SignerIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AggregatedSignature;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.entities.AggregatedSignature")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AggregatedSignature, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut verifier_signatures__ = None;
                let mut signer_ids__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::VerifierSignatures => {
                            if verifier_signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verifierSignatures"));
                            }
                            verifier_signatures__ = 
                                Some(map.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::SignerIds => {
                            if signer_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signerIds"));
                            }
                            signer_ids__ = 
                                Some(map.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(AggregatedSignature {
                    verifier_signatures: verifier_signatures__.unwrap_or_default(),
                    signer_ids: signer_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.entities.AggregatedSignature", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Block {
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
        if !self.parent_id.is_empty() {
            len += 1;
        }
        if self.height != 0 {
            len += 1;
        }
        if self.timestamp.is_some() {
            len += 1;
        }
        if !self.collection_guarantees.is_empty() {
            len += 1;
        }
        if !self.block_seals.is_empty() {
            len += 1;
        }
        if !self.signatures.is_empty() {
            len += 1;
        }
        if !self.execution_receipt_meta_list.is_empty() {
            len += 1;
        }
        if !self.execution_result_list.is_empty() {
            len += 1;
        }
        if self.block_header.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.entities.Block", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", pbjson::private::base64::encode(&self.id).as_str())?;
        }
        if !self.parent_id.is_empty() {
            struct_ser.serialize_field("parentId", pbjson::private::base64::encode(&self.parent_id).as_str())?;
        }
        if self.height != 0 {
            struct_ser.serialize_field("height", ToString::to_string(&self.height).as_str())?;
        }
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        if !self.collection_guarantees.is_empty() {
            struct_ser.serialize_field("collectionGuarantees", &self.collection_guarantees)?;
        }
        if !self.block_seals.is_empty() {
            struct_ser.serialize_field("blockSeals", &self.block_seals)?;
        }
        if !self.signatures.is_empty() {
            struct_ser.serialize_field("signatures", &self.signatures.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        if !self.execution_receipt_meta_list.is_empty() {
            struct_ser.serialize_field("executionReceiptMetaList", &self.execution_receipt_meta_list)?;
        }
        if !self.execution_result_list.is_empty() {
            struct_ser.serialize_field("executionResultList", &self.execution_result_list)?;
        }
        if let Some(v) = self.block_header.as_ref() {
            struct_ser.serialize_field("blockHeader", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Block {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "parent_id",
            "parentId",
            "height",
            "timestamp",
            "collection_guarantees",
            "collectionGuarantees",
            "block_seals",
            "blockSeals",
            "signatures",
            "execution_receipt_metaList",
            "executionReceiptMetaList",
            "execution_result_list",
            "executionResultList",
            "block_header",
            "blockHeader",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            ParentId,
            Height,
            Timestamp,
            CollectionGuarantees,
            BlockSeals,
            Signatures,
            ExecutionReceiptMetaList,
            ExecutionResultList,
            BlockHeader,
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
                            "parentId" | "parent_id" => Ok(GeneratedField::ParentId),
                            "height" => Ok(GeneratedField::Height),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "collectionGuarantees" | "collection_guarantees" => Ok(GeneratedField::CollectionGuarantees),
                            "blockSeals" | "block_seals" => Ok(GeneratedField::BlockSeals),
                            "signatures" => Ok(GeneratedField::Signatures),
                            "executionReceiptMetaList" | "execution_receipt_metaList" => Ok(GeneratedField::ExecutionReceiptMetaList),
                            "executionResultList" | "execution_result_list" => Ok(GeneratedField::ExecutionResultList),
                            "blockHeader" | "block_header" => Ok(GeneratedField::BlockHeader),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Block;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.entities.Block")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Block, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut parent_id__ = None;
                let mut height__ = None;
                let mut timestamp__ = None;
                let mut collection_guarantees__ = None;
                let mut block_seals__ = None;
                let mut signatures__ = None;
                let mut execution_receipt_meta_list__ = None;
                let mut execution_result_list__ = None;
                let mut block_header__ = None;
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
                        GeneratedField::ParentId => {
                            if parent_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentId"));
                            }
                            parent_id__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map.next_value()?;
                        }
                        GeneratedField::CollectionGuarantees => {
                            if collection_guarantees__.is_some() {
                                return Err(serde::de::Error::duplicate_field("collectionGuarantees"));
                            }
                            collection_guarantees__ = Some(map.next_value()?);
                        }
                        GeneratedField::BlockSeals => {
                            if block_seals__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockSeals"));
                            }
                            block_seals__ = Some(map.next_value()?);
                        }
                        GeneratedField::Signatures => {
                            if signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signatures"));
                            }
                            signatures__ = 
                                Some(map.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::ExecutionReceiptMetaList => {
                            if execution_receipt_meta_list__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executionReceiptMetaList"));
                            }
                            execution_receipt_meta_list__ = Some(map.next_value()?);
                        }
                        GeneratedField::ExecutionResultList => {
                            if execution_result_list__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executionResultList"));
                            }
                            execution_result_list__ = Some(map.next_value()?);
                        }
                        GeneratedField::BlockHeader => {
                            if block_header__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockHeader"));
                            }
                            block_header__ = map.next_value()?;
                        }
                    }
                }
                Ok(Block {
                    id: id__.unwrap_or_default(),
                    parent_id: parent_id__.unwrap_or_default(),
                    height: height__.unwrap_or_default(),
                    timestamp: timestamp__,
                    collection_guarantees: collection_guarantees__.unwrap_or_default(),
                    block_seals: block_seals__.unwrap_or_default(),
                    signatures: signatures__.unwrap_or_default(),
                    execution_receipt_meta_list: execution_receipt_meta_list__.unwrap_or_default(),
                    execution_result_list: execution_result_list__.unwrap_or_default(),
                    block_header: block_header__,
                })
            }
        }
        deserializer.deserialize_struct("flow.entities.Block", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BlockExecutionData {
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
        if !self.chunk_execution_data.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.entities.BlockExecutionData", len)?;
        if !self.block_id.is_empty() {
            struct_ser.serialize_field("blockId", pbjson::private::base64::encode(&self.block_id).as_str())?;
        }
        if !self.chunk_execution_data.is_empty() {
            struct_ser.serialize_field("chunkExecutionData", &self.chunk_execution_data)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BlockExecutionData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "block_id",
            "blockId",
            "chunk_execution_data",
            "chunkExecutionData",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BlockId,
            ChunkExecutionData,
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
                            "chunkExecutionData" | "chunk_execution_data" => Ok(GeneratedField::ChunkExecutionData),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BlockExecutionData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.entities.BlockExecutionData")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BlockExecutionData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut block_id__ = None;
                let mut chunk_execution_data__ = None;
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
                        GeneratedField::ChunkExecutionData => {
                            if chunk_execution_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chunkExecutionData"));
                            }
                            chunk_execution_data__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(BlockExecutionData {
                    block_id: block_id__.unwrap_or_default(),
                    chunk_execution_data: chunk_execution_data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.entities.BlockExecutionData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BlockHeader {
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
        if !self.parent_id.is_empty() {
            len += 1;
        }
        if self.height != 0 {
            len += 1;
        }
        if self.timestamp.is_some() {
            len += 1;
        }
        if !self.payload_hash.is_empty() {
            len += 1;
        }
        if self.view != 0 {
            len += 1;
        }
        if !self.parent_voter_ids.is_empty() {
            len += 1;
        }
        if !self.parent_voter_sig_data.is_empty() {
            len += 1;
        }
        if !self.proposer_id.is_empty() {
            len += 1;
        }
        if !self.proposer_sig_data.is_empty() {
            len += 1;
        }
        if !self.chain_id.is_empty() {
            len += 1;
        }
        if !self.parent_voter_indices.is_empty() {
            len += 1;
        }
        if self.last_view_tc.is_some() {
            len += 1;
        }
        if self.parent_view != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.entities.BlockHeader", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", pbjson::private::base64::encode(&self.id).as_str())?;
        }
        if !self.parent_id.is_empty() {
            struct_ser.serialize_field("parentId", pbjson::private::base64::encode(&self.parent_id).as_str())?;
        }
        if self.height != 0 {
            struct_ser.serialize_field("height", ToString::to_string(&self.height).as_str())?;
        }
        if let Some(v) = self.timestamp.as_ref() {
            struct_ser.serialize_field("timestamp", v)?;
        }
        if !self.payload_hash.is_empty() {
            struct_ser.serialize_field("payloadHash", pbjson::private::base64::encode(&self.payload_hash).as_str())?;
        }
        if self.view != 0 {
            struct_ser.serialize_field("view", ToString::to_string(&self.view).as_str())?;
        }
        if !self.parent_voter_ids.is_empty() {
            struct_ser.serialize_field("parentVoterIds", &self.parent_voter_ids.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        if !self.parent_voter_sig_data.is_empty() {
            struct_ser.serialize_field("parentVoterSigData", pbjson::private::base64::encode(&self.parent_voter_sig_data).as_str())?;
        }
        if !self.proposer_id.is_empty() {
            struct_ser.serialize_field("proposerId", pbjson::private::base64::encode(&self.proposer_id).as_str())?;
        }
        if !self.proposer_sig_data.is_empty() {
            struct_ser.serialize_field("proposerSigData", pbjson::private::base64::encode(&self.proposer_sig_data).as_str())?;
        }
        if !self.chain_id.is_empty() {
            struct_ser.serialize_field("chainId", &self.chain_id)?;
        }
        if !self.parent_voter_indices.is_empty() {
            struct_ser.serialize_field("parentVoterIndices", pbjson::private::base64::encode(&self.parent_voter_indices).as_str())?;
        }
        if let Some(v) = self.last_view_tc.as_ref() {
            struct_ser.serialize_field("lastViewTc", v)?;
        }
        if self.parent_view != 0 {
            struct_ser.serialize_field("parentView", ToString::to_string(&self.parent_view).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BlockHeader {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "parent_id",
            "parentId",
            "height",
            "timestamp",
            "payload_hash",
            "payloadHash",
            "view",
            "parent_voter_ids",
            "parentVoterIds",
            "parent_voter_sig_data",
            "parentVoterSigData",
            "proposer_id",
            "proposerId",
            "proposer_sig_data",
            "proposerSigData",
            "chain_id",
            "chainId",
            "parent_voter_indices",
            "parentVoterIndices",
            "last_view_tc",
            "lastViewTc",
            "parent_view",
            "parentView",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            ParentId,
            Height,
            Timestamp,
            PayloadHash,
            View,
            ParentVoterIds,
            ParentVoterSigData,
            ProposerId,
            ProposerSigData,
            ChainId,
            ParentVoterIndices,
            LastViewTc,
            ParentView,
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
                            "parentId" | "parent_id" => Ok(GeneratedField::ParentId),
                            "height" => Ok(GeneratedField::Height),
                            "timestamp" => Ok(GeneratedField::Timestamp),
                            "payloadHash" | "payload_hash" => Ok(GeneratedField::PayloadHash),
                            "view" => Ok(GeneratedField::View),
                            "parentVoterIds" | "parent_voter_ids" => Ok(GeneratedField::ParentVoterIds),
                            "parentVoterSigData" | "parent_voter_sig_data" => Ok(GeneratedField::ParentVoterSigData),
                            "proposerId" | "proposer_id" => Ok(GeneratedField::ProposerId),
                            "proposerSigData" | "proposer_sig_data" => Ok(GeneratedField::ProposerSigData),
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            "parentVoterIndices" | "parent_voter_indices" => Ok(GeneratedField::ParentVoterIndices),
                            "lastViewTc" | "last_view_tc" => Ok(GeneratedField::LastViewTc),
                            "parentView" | "parent_view" => Ok(GeneratedField::ParentView),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BlockHeader;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.entities.BlockHeader")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BlockHeader, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut parent_id__ = None;
                let mut height__ = None;
                let mut timestamp__ = None;
                let mut payload_hash__ = None;
                let mut view__ = None;
                let mut parent_voter_ids__ = None;
                let mut parent_voter_sig_data__ = None;
                let mut proposer_id__ = None;
                let mut proposer_sig_data__ = None;
                let mut chain_id__ = None;
                let mut parent_voter_indices__ = None;
                let mut last_view_tc__ = None;
                let mut parent_view__ = None;
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
                        GeneratedField::ParentId => {
                            if parent_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentId"));
                            }
                            parent_id__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Height => {
                            if height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("height"));
                            }
                            height__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Timestamp => {
                            if timestamp__.is_some() {
                                return Err(serde::de::Error::duplicate_field("timestamp"));
                            }
                            timestamp__ = map.next_value()?;
                        }
                        GeneratedField::PayloadHash => {
                            if payload_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payloadHash"));
                            }
                            payload_hash__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::View => {
                            if view__.is_some() {
                                return Err(serde::de::Error::duplicate_field("view"));
                            }
                            view__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ParentVoterIds => {
                            if parent_voter_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentVoterIds"));
                            }
                            parent_voter_ids__ = 
                                Some(map.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::ParentVoterSigData => {
                            if parent_voter_sig_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentVoterSigData"));
                            }
                            parent_voter_sig_data__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ProposerId => {
                            if proposer_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proposerId"));
                            }
                            proposer_id__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ProposerSigData => {
                            if proposer_sig_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proposerSigData"));
                            }
                            proposer_sig_data__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::ParentVoterIndices => {
                            if parent_voter_indices__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentVoterIndices"));
                            }
                            parent_voter_indices__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::LastViewTc => {
                            if last_view_tc__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastViewTc"));
                            }
                            last_view_tc__ = map.next_value()?;
                        }
                        GeneratedField::ParentView => {
                            if parent_view__.is_some() {
                                return Err(serde::de::Error::duplicate_field("parentView"));
                            }
                            parent_view__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(BlockHeader {
                    id: id__.unwrap_or_default(),
                    parent_id: parent_id__.unwrap_or_default(),
                    height: height__.unwrap_or_default(),
                    timestamp: timestamp__,
                    payload_hash: payload_hash__.unwrap_or_default(),
                    view: view__.unwrap_or_default(),
                    parent_voter_ids: parent_voter_ids__.unwrap_or_default(),
                    parent_voter_sig_data: parent_voter_sig_data__.unwrap_or_default(),
                    proposer_id: proposer_id__.unwrap_or_default(),
                    proposer_sig_data: proposer_sig_data__.unwrap_or_default(),
                    chain_id: chain_id__.unwrap_or_default(),
                    parent_voter_indices: parent_voter_indices__.unwrap_or_default(),
                    last_view_tc: last_view_tc__,
                    parent_view: parent_view__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.entities.BlockHeader", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BlockSeal {
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
        if !self.execution_receipt_id.is_empty() {
            len += 1;
        }
        if !self.execution_receipt_signatures.is_empty() {
            len += 1;
        }
        if !self.result_approval_signatures.is_empty() {
            len += 1;
        }
        if !self.final_state.is_empty() {
            len += 1;
        }
        if !self.result_id.is_empty() {
            len += 1;
        }
        if !self.aggregated_approval_sigs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.entities.BlockSeal", len)?;
        if !self.block_id.is_empty() {
            struct_ser.serialize_field("blockId", pbjson::private::base64::encode(&self.block_id).as_str())?;
        }
        if !self.execution_receipt_id.is_empty() {
            struct_ser.serialize_field("executionReceiptId", pbjson::private::base64::encode(&self.execution_receipt_id).as_str())?;
        }
        if !self.execution_receipt_signatures.is_empty() {
            struct_ser.serialize_field("executionReceiptSignatures", &self.execution_receipt_signatures.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        if !self.result_approval_signatures.is_empty() {
            struct_ser.serialize_field("resultApprovalSignatures", &self.result_approval_signatures.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        if !self.final_state.is_empty() {
            struct_ser.serialize_field("finalState", pbjson::private::base64::encode(&self.final_state).as_str())?;
        }
        if !self.result_id.is_empty() {
            struct_ser.serialize_field("resultId", pbjson::private::base64::encode(&self.result_id).as_str())?;
        }
        if !self.aggregated_approval_sigs.is_empty() {
            struct_ser.serialize_field("aggregatedApprovalSigs", &self.aggregated_approval_sigs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for BlockSeal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "block_id",
            "blockId",
            "execution_receipt_id",
            "executionReceiptId",
            "execution_receipt_signatures",
            "executionReceiptSignatures",
            "result_approval_signatures",
            "resultApprovalSignatures",
            "final_state",
            "finalState",
            "result_id",
            "resultId",
            "aggregated_approval_sigs",
            "aggregatedApprovalSigs",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BlockId,
            ExecutionReceiptId,
            ExecutionReceiptSignatures,
            ResultApprovalSignatures,
            FinalState,
            ResultId,
            AggregatedApprovalSigs,
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
                            "executionReceiptId" | "execution_receipt_id" => Ok(GeneratedField::ExecutionReceiptId),
                            "executionReceiptSignatures" | "execution_receipt_signatures" => Ok(GeneratedField::ExecutionReceiptSignatures),
                            "resultApprovalSignatures" | "result_approval_signatures" => Ok(GeneratedField::ResultApprovalSignatures),
                            "finalState" | "final_state" => Ok(GeneratedField::FinalState),
                            "resultId" | "result_id" => Ok(GeneratedField::ResultId),
                            "aggregatedApprovalSigs" | "aggregated_approval_sigs" => Ok(GeneratedField::AggregatedApprovalSigs),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BlockSeal;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.entities.BlockSeal")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<BlockSeal, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut block_id__ = None;
                let mut execution_receipt_id__ = None;
                let mut execution_receipt_signatures__ = None;
                let mut result_approval_signatures__ = None;
                let mut final_state__ = None;
                let mut result_id__ = None;
                let mut aggregated_approval_sigs__ = None;
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
                        GeneratedField::ExecutionReceiptId => {
                            if execution_receipt_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executionReceiptId"));
                            }
                            execution_receipt_id__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ExecutionReceiptSignatures => {
                            if execution_receipt_signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executionReceiptSignatures"));
                            }
                            execution_receipt_signatures__ = 
                                Some(map.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::ResultApprovalSignatures => {
                            if result_approval_signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resultApprovalSignatures"));
                            }
                            result_approval_signatures__ = 
                                Some(map.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::FinalState => {
                            if final_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("finalState"));
                            }
                            final_state__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ResultId => {
                            if result_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resultId"));
                            }
                            result_id__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::AggregatedApprovalSigs => {
                            if aggregated_approval_sigs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("aggregatedApprovalSigs"));
                            }
                            aggregated_approval_sigs__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(BlockSeal {
                    block_id: block_id__.unwrap_or_default(),
                    execution_receipt_id: execution_receipt_id__.unwrap_or_default(),
                    execution_receipt_signatures: execution_receipt_signatures__.unwrap_or_default(),
                    result_approval_signatures: result_approval_signatures__.unwrap_or_default(),
                    final_state: final_state__.unwrap_or_default(),
                    result_id: result_id__.unwrap_or_default(),
                    aggregated_approval_sigs: aggregated_approval_sigs__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.entities.BlockSeal", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for BlockStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::BlockUnknown => "BLOCK_UNKNOWN",
            Self::BlockFinalized => "BLOCK_FINALIZED",
            Self::BlockSealed => "BLOCK_SEALED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for BlockStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "BLOCK_UNKNOWN",
            "BLOCK_FINALIZED",
            "BLOCK_SEALED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BlockStatus;

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
                    .and_then(BlockStatus::from_i32)
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
                    .and_then(BlockStatus::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "BLOCK_UNKNOWN" => Ok(BlockStatus::BlockUnknown),
                    "BLOCK_FINALIZED" => Ok(BlockStatus::BlockFinalized),
                    "BLOCK_SEALED" => Ok(BlockStatus::BlockSealed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Chunk {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.collection_index != 0 {
            len += 1;
        }
        if !self.start_state.is_empty() {
            len += 1;
        }
        if !self.event_collection.is_empty() {
            len += 1;
        }
        if !self.block_id.is_empty() {
            len += 1;
        }
        if self.total_computation_used != 0 {
            len += 1;
        }
        if self.number_of_transactions != 0 {
            len += 1;
        }
        if self.index != 0 {
            len += 1;
        }
        if !self.end_state.is_empty() {
            len += 1;
        }
        if !self.execution_data_id.is_empty() {
            len += 1;
        }
        if !self.state_delta_commitment.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.entities.Chunk", len)?;
        if self.collection_index != 0 {
            struct_ser.serialize_field("CollectionIndex", &self.collection_index)?;
        }
        if !self.start_state.is_empty() {
            struct_ser.serialize_field("startState", pbjson::private::base64::encode(&self.start_state).as_str())?;
        }
        if !self.event_collection.is_empty() {
            struct_ser.serialize_field("eventCollection", pbjson::private::base64::encode(&self.event_collection).as_str())?;
        }
        if !self.block_id.is_empty() {
            struct_ser.serialize_field("blockId", pbjson::private::base64::encode(&self.block_id).as_str())?;
        }
        if self.total_computation_used != 0 {
            struct_ser.serialize_field("totalComputationUsed", ToString::to_string(&self.total_computation_used).as_str())?;
        }
        if self.number_of_transactions != 0 {
            struct_ser.serialize_field("numberOfTransactions", &self.number_of_transactions)?;
        }
        if self.index != 0 {
            struct_ser.serialize_field("index", ToString::to_string(&self.index).as_str())?;
        }
        if !self.end_state.is_empty() {
            struct_ser.serialize_field("endState", pbjson::private::base64::encode(&self.end_state).as_str())?;
        }
        if !self.execution_data_id.is_empty() {
            struct_ser.serialize_field("executionDataId", pbjson::private::base64::encode(&self.execution_data_id).as_str())?;
        }
        if !self.state_delta_commitment.is_empty() {
            struct_ser.serialize_field("stateDeltaCommitment", pbjson::private::base64::encode(&self.state_delta_commitment).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Chunk {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "CollectionIndex",
            "start_state",
            "startState",
            "event_collection",
            "eventCollection",
            "block_id",
            "blockId",
            "total_computation_used",
            "totalComputationUsed",
            "number_of_transactions",
            "numberOfTransactions",
            "index",
            "end_state",
            "endState",
            "execution_data_id",
            "executionDataId",
            "state_delta_commitment",
            "stateDeltaCommitment",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CollectionIndex,
            StartState,
            EventCollection,
            BlockId,
            TotalComputationUsed,
            NumberOfTransactions,
            Index,
            EndState,
            ExecutionDataId,
            StateDeltaCommitment,
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
                            "CollectionIndex" => Ok(GeneratedField::CollectionIndex),
                            "startState" | "start_state" => Ok(GeneratedField::StartState),
                            "eventCollection" | "event_collection" => Ok(GeneratedField::EventCollection),
                            "blockId" | "block_id" => Ok(GeneratedField::BlockId),
                            "totalComputationUsed" | "total_computation_used" => Ok(GeneratedField::TotalComputationUsed),
                            "numberOfTransactions" | "number_of_transactions" => Ok(GeneratedField::NumberOfTransactions),
                            "index" => Ok(GeneratedField::Index),
                            "endState" | "end_state" => Ok(GeneratedField::EndState),
                            "executionDataId" | "execution_data_id" => Ok(GeneratedField::ExecutionDataId),
                            "stateDeltaCommitment" | "state_delta_commitment" => Ok(GeneratedField::StateDeltaCommitment),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Chunk;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.entities.Chunk")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Chunk, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut collection_index__ = None;
                let mut start_state__ = None;
                let mut event_collection__ = None;
                let mut block_id__ = None;
                let mut total_computation_used__ = None;
                let mut number_of_transactions__ = None;
                let mut index__ = None;
                let mut end_state__ = None;
                let mut execution_data_id__ = None;
                let mut state_delta_commitment__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CollectionIndex => {
                            if collection_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("CollectionIndex"));
                            }
                            collection_index__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::StartState => {
                            if start_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startState"));
                            }
                            start_state__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EventCollection => {
                            if event_collection__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventCollection"));
                            }
                            event_collection__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BlockId => {
                            if block_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockId"));
                            }
                            block_id__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TotalComputationUsed => {
                            if total_computation_used__.is_some() {
                                return Err(serde::de::Error::duplicate_field("totalComputationUsed"));
                            }
                            total_computation_used__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NumberOfTransactions => {
                            if number_of_transactions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("numberOfTransactions"));
                            }
                            number_of_transactions__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
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
                        GeneratedField::EndState => {
                            if end_state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("endState"));
                            }
                            end_state__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ExecutionDataId => {
                            if execution_data_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executionDataId"));
                            }
                            execution_data_id__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::StateDeltaCommitment => {
                            if state_delta_commitment__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stateDeltaCommitment"));
                            }
                            state_delta_commitment__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Chunk {
                    collection_index: collection_index__.unwrap_or_default(),
                    start_state: start_state__.unwrap_or_default(),
                    event_collection: event_collection__.unwrap_or_default(),
                    block_id: block_id__.unwrap_or_default(),
                    total_computation_used: total_computation_used__.unwrap_or_default(),
                    number_of_transactions: number_of_transactions__.unwrap_or_default(),
                    index: index__.unwrap_or_default(),
                    end_state: end_state__.unwrap_or_default(),
                    execution_data_id: execution_data_id__.unwrap_or_default(),
                    state_delta_commitment: state_delta_commitment__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.entities.Chunk", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ChunkExecutionData {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.collection.is_some() {
            len += 1;
        }
        if !self.events.is_empty() {
            len += 1;
        }
        if self.trie_update.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.entities.ChunkExecutionData", len)?;
        if let Some(v) = self.collection.as_ref() {
            struct_ser.serialize_field("collection", v)?;
        }
        if !self.events.is_empty() {
            struct_ser.serialize_field("events", &self.events)?;
        }
        if let Some(v) = self.trie_update.as_ref() {
            struct_ser.serialize_field("trieUpdate", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ChunkExecutionData {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "collection",
            "events",
            "trieUpdate",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Collection,
            Events,
            TrieUpdate,
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
                            "collection" => Ok(GeneratedField::Collection),
                            "events" => Ok(GeneratedField::Events),
                            "trieUpdate" => Ok(GeneratedField::TrieUpdate),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ChunkExecutionData;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.entities.ChunkExecutionData")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ChunkExecutionData, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut collection__ = None;
                let mut events__ = None;
                let mut trie_update__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Collection => {
                            if collection__.is_some() {
                                return Err(serde::de::Error::duplicate_field("collection"));
                            }
                            collection__ = map.next_value()?;
                        }
                        GeneratedField::Events => {
                            if events__.is_some() {
                                return Err(serde::de::Error::duplicate_field("events"));
                            }
                            events__ = Some(map.next_value()?);
                        }
                        GeneratedField::TrieUpdate => {
                            if trie_update__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trieUpdate"));
                            }
                            trie_update__ = map.next_value()?;
                        }
                    }
                }
                Ok(ChunkExecutionData {
                    collection: collection__,
                    events: events__.unwrap_or_default(),
                    trie_update: trie_update__,
                })
            }
        }
        deserializer.deserialize_struct("flow.entities.ChunkExecutionData", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Collection {
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
        if !self.transaction_ids.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.entities.Collection", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", pbjson::private::base64::encode(&self.id).as_str())?;
        }
        if !self.transaction_ids.is_empty() {
            struct_ser.serialize_field("transactionIds", &self.transaction_ids.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Collection {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "transaction_ids",
            "transactionIds",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            TransactionIds,
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
                            "transactionIds" | "transaction_ids" => Ok(GeneratedField::TransactionIds),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Collection;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.entities.Collection")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Collection, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut transaction_ids__ = None;
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
                        GeneratedField::TransactionIds => {
                            if transaction_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transactionIds"));
                            }
                            transaction_ids__ = 
                                Some(map.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                    }
                }
                Ok(Collection {
                    id: id__.unwrap_or_default(),
                    transaction_ids: transaction_ids__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.entities.Collection", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CollectionGuarantee {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.collection_id.is_empty() {
            len += 1;
        }
        if !self.signatures.is_empty() {
            len += 1;
        }
        if !self.reference_block_id.is_empty() {
            len += 1;
        }
        if !self.signature.is_empty() {
            len += 1;
        }
        if !self.signer_ids.is_empty() {
            len += 1;
        }
        if !self.signer_indices.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.entities.CollectionGuarantee", len)?;
        if !self.collection_id.is_empty() {
            struct_ser.serialize_field("collectionId", pbjson::private::base64::encode(&self.collection_id).as_str())?;
        }
        if !self.signatures.is_empty() {
            struct_ser.serialize_field("signatures", &self.signatures.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        if !self.reference_block_id.is_empty() {
            struct_ser.serialize_field("referenceBlockId", pbjson::private::base64::encode(&self.reference_block_id).as_str())?;
        }
        if !self.signature.is_empty() {
            struct_ser.serialize_field("signature", pbjson::private::base64::encode(&self.signature).as_str())?;
        }
        if !self.signer_ids.is_empty() {
            struct_ser.serialize_field("signerIds", &self.signer_ids.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        if !self.signer_indices.is_empty() {
            struct_ser.serialize_field("signerIndices", pbjson::private::base64::encode(&self.signer_indices).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CollectionGuarantee {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "collection_id",
            "collectionId",
            "signatures",
            "reference_block_id",
            "referenceBlockId",
            "signature",
            "signer_ids",
            "signerIds",
            "signer_indices",
            "signerIndices",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CollectionId,
            Signatures,
            ReferenceBlockId,
            Signature,
            SignerIds,
            SignerIndices,
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
                            "collectionId" | "collection_id" => Ok(GeneratedField::CollectionId),
                            "signatures" => Ok(GeneratedField::Signatures),
                            "referenceBlockId" | "reference_block_id" => Ok(GeneratedField::ReferenceBlockId),
                            "signature" => Ok(GeneratedField::Signature),
                            "signerIds" | "signer_ids" => Ok(GeneratedField::SignerIds),
                            "signerIndices" | "signer_indices" => Ok(GeneratedField::SignerIndices),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CollectionGuarantee;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.entities.CollectionGuarantee")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CollectionGuarantee, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut collection_id__ = None;
                let mut signatures__ = None;
                let mut reference_block_id__ = None;
                let mut signature__ = None;
                let mut signer_ids__ = None;
                let mut signer_indices__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CollectionId => {
                            if collection_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("collectionId"));
                            }
                            collection_id__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Signatures => {
                            if signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signatures"));
                            }
                            signatures__ = 
                                Some(map.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::ReferenceBlockId => {
                            if reference_block_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceBlockId"));
                            }
                            reference_block_id__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Signature => {
                            if signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signature"));
                            }
                            signature__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SignerIds => {
                            if signer_ids__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signerIds"));
                            }
                            signer_ids__ = 
                                Some(map.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::SignerIndices => {
                            if signer_indices__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signerIndices"));
                            }
                            signer_indices__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(CollectionGuarantee {
                    collection_id: collection_id__.unwrap_or_default(),
                    signatures: signatures__.unwrap_or_default(),
                    reference_block_id: reference_block_id__.unwrap_or_default(),
                    signature: signature__.unwrap_or_default(),
                    signer_ids: signer_ids__.unwrap_or_default(),
                    signer_indices: signer_indices__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.entities.CollectionGuarantee", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Event {
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
        if !self.transaction_id.is_empty() {
            len += 1;
        }
        if self.transaction_index != 0 {
            len += 1;
        }
        if self.event_index != 0 {
            len += 1;
        }
        if !self.payload.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.entities.Event", len)?;
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.transaction_id.is_empty() {
            struct_ser.serialize_field("transactionId", pbjson::private::base64::encode(&self.transaction_id).as_str())?;
        }
        if self.transaction_index != 0 {
            struct_ser.serialize_field("transactionIndex", &self.transaction_index)?;
        }
        if self.event_index != 0 {
            struct_ser.serialize_field("eventIndex", &self.event_index)?;
        }
        if !self.payload.is_empty() {
            struct_ser.serialize_field("payload", pbjson::private::base64::encode(&self.payload).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Event {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "transaction_id",
            "transactionId",
            "transaction_index",
            "transactionIndex",
            "event_index",
            "eventIndex",
            "payload",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            TransactionId,
            TransactionIndex,
            EventIndex,
            Payload,
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
                            "transactionId" | "transaction_id" => Ok(GeneratedField::TransactionId),
                            "transactionIndex" | "transaction_index" => Ok(GeneratedField::TransactionIndex),
                            "eventIndex" | "event_index" => Ok(GeneratedField::EventIndex),
                            "payload" => Ok(GeneratedField::Payload),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Event;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.entities.Event")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Event, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut transaction_id__ = None;
                let mut transaction_index__ = None;
                let mut event_index__ = None;
                let mut payload__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map.next_value()?);
                        }
                        GeneratedField::TransactionId => {
                            if transaction_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transactionId"));
                            }
                            transaction_id__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::TransactionIndex => {
                            if transaction_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transactionIndex"));
                            }
                            transaction_index__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::EventIndex => {
                            if event_index__.is_some() {
                                return Err(serde::de::Error::duplicate_field("eventIndex"));
                            }
                            event_index__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Payload => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payload"));
                            }
                            payload__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Event {
                    r#type: r#type__.unwrap_or_default(),
                    transaction_id: transaction_id__.unwrap_or_default(),
                    transaction_index: transaction_index__.unwrap_or_default(),
                    event_index: event_index__.unwrap_or_default(),
                    payload: payload__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.entities.Event", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExecutionDataCollection {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.transactions.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.entities.ExecutionDataCollection", len)?;
        if !self.transactions.is_empty() {
            struct_ser.serialize_field("transactions", &self.transactions)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExecutionDataCollection {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "transactions",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Transactions,
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
                            "transactions" => Ok(GeneratedField::Transactions),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExecutionDataCollection;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.entities.ExecutionDataCollection")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExecutionDataCollection, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut transactions__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Transactions => {
                            if transactions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("transactions"));
                            }
                            transactions__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ExecutionDataCollection {
                    transactions: transactions__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.entities.ExecutionDataCollection", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExecutionReceiptMeta {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.executor_id.is_empty() {
            len += 1;
        }
        if !self.result_id.is_empty() {
            len += 1;
        }
        if !self.spocks.is_empty() {
            len += 1;
        }
        if !self.executor_signature.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.entities.ExecutionReceiptMeta", len)?;
        if !self.executor_id.is_empty() {
            struct_ser.serialize_field("executorId", pbjson::private::base64::encode(&self.executor_id).as_str())?;
        }
        if !self.result_id.is_empty() {
            struct_ser.serialize_field("resultId", pbjson::private::base64::encode(&self.result_id).as_str())?;
        }
        if !self.spocks.is_empty() {
            struct_ser.serialize_field("spocks", &self.spocks.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        if !self.executor_signature.is_empty() {
            struct_ser.serialize_field("executorSignature", pbjson::private::base64::encode(&self.executor_signature).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExecutionReceiptMeta {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "executor_id",
            "executorId",
            "result_id",
            "resultId",
            "spocks",
            "executor_signature",
            "executorSignature",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExecutorId,
            ResultId,
            Spocks,
            ExecutorSignature,
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
                            "executorId" | "executor_id" => Ok(GeneratedField::ExecutorId),
                            "resultId" | "result_id" => Ok(GeneratedField::ResultId),
                            "spocks" => Ok(GeneratedField::Spocks),
                            "executorSignature" | "executor_signature" => Ok(GeneratedField::ExecutorSignature),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExecutionReceiptMeta;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.entities.ExecutionReceiptMeta")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExecutionReceiptMeta, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut executor_id__ = None;
                let mut result_id__ = None;
                let mut spocks__ = None;
                let mut executor_signature__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ExecutorId => {
                            if executor_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executorId"));
                            }
                            executor_id__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ResultId => {
                            if result_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resultId"));
                            }
                            result_id__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Spocks => {
                            if spocks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("spocks"));
                            }
                            spocks__ = 
                                Some(map.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::ExecutorSignature => {
                            if executor_signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executorSignature"));
                            }
                            executor_signature__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ExecutionReceiptMeta {
                    executor_id: executor_id__.unwrap_or_default(),
                    result_id: result_id__.unwrap_or_default(),
                    spocks: spocks__.unwrap_or_default(),
                    executor_signature: executor_signature__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.entities.ExecutionReceiptMeta", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExecutionResult {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.previous_result_id.is_empty() {
            len += 1;
        }
        if !self.block_id.is_empty() {
            len += 1;
        }
        if !self.chunks.is_empty() {
            len += 1;
        }
        if !self.service_events.is_empty() {
            len += 1;
        }
        if !self.execution_data_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.entities.ExecutionResult", len)?;
        if !self.previous_result_id.is_empty() {
            struct_ser.serialize_field("previousResultId", pbjson::private::base64::encode(&self.previous_result_id).as_str())?;
        }
        if !self.block_id.is_empty() {
            struct_ser.serialize_field("blockId", pbjson::private::base64::encode(&self.block_id).as_str())?;
        }
        if !self.chunks.is_empty() {
            struct_ser.serialize_field("chunks", &self.chunks)?;
        }
        if !self.service_events.is_empty() {
            struct_ser.serialize_field("serviceEvents", &self.service_events)?;
        }
        if !self.execution_data_id.is_empty() {
            struct_ser.serialize_field("executionDataId", pbjson::private::base64::encode(&self.execution_data_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExecutionResult {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "previous_result_id",
            "previousResultId",
            "block_id",
            "blockId",
            "chunks",
            "service_events",
            "serviceEvents",
            "execution_data_id",
            "executionDataId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PreviousResultId,
            BlockId,
            Chunks,
            ServiceEvents,
            ExecutionDataId,
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
                            "previousResultId" | "previous_result_id" => Ok(GeneratedField::PreviousResultId),
                            "blockId" | "block_id" => Ok(GeneratedField::BlockId),
                            "chunks" => Ok(GeneratedField::Chunks),
                            "serviceEvents" | "service_events" => Ok(GeneratedField::ServiceEvents),
                            "executionDataId" | "execution_data_id" => Ok(GeneratedField::ExecutionDataId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExecutionResult;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.entities.ExecutionResult")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExecutionResult, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut previous_result_id__ = None;
                let mut block_id__ = None;
                let mut chunks__ = None;
                let mut service_events__ = None;
                let mut execution_data_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PreviousResultId => {
                            if previous_result_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("previousResultId"));
                            }
                            previous_result_id__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BlockId => {
                            if block_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockId"));
                            }
                            block_id__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Chunks => {
                            if chunks__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chunks"));
                            }
                            chunks__ = Some(map.next_value()?);
                        }
                        GeneratedField::ServiceEvents => {
                            if service_events__.is_some() {
                                return Err(serde::de::Error::duplicate_field("serviceEvents"));
                            }
                            service_events__ = Some(map.next_value()?);
                        }
                        GeneratedField::ExecutionDataId => {
                            if execution_data_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("executionDataId"));
                            }
                            execution_data_id__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ExecutionResult {
                    previous_result_id: previous_result_id__.unwrap_or_default(),
                    block_id: block_id__.unwrap_or_default(),
                    chunks: chunks__.unwrap_or_default(),
                    service_events: service_events__.unwrap_or_default(),
                    execution_data_id: execution_data_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.entities.ExecutionResult", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for KeyPart {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type != 0 {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.entities.KeyPart", len)?;
        if self.r#type != 0 {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", pbjson::private::base64::encode(&self.value).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for KeyPart {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
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
                            "type" => Ok(GeneratedField::Type),
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
            type Value = KeyPart;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.entities.KeyPart")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<KeyPart, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
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
                Ok(KeyPart {
                    r#type: r#type__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.entities.KeyPart", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Metadata {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.latest_finalized_block_id.is_empty() {
            len += 1;
        }
        if self.latest_finalized_height != 0 {
            len += 1;
        }
        if !self.node_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.entities.Metadata", len)?;
        if !self.latest_finalized_block_id.is_empty() {
            struct_ser.serialize_field("latestFinalizedBlockId", pbjson::private::base64::encode(&self.latest_finalized_block_id).as_str())?;
        }
        if self.latest_finalized_height != 0 {
            struct_ser.serialize_field("latestFinalizedHeight", ToString::to_string(&self.latest_finalized_height).as_str())?;
        }
        if !self.node_id.is_empty() {
            struct_ser.serialize_field("nodeId", pbjson::private::base64::encode(&self.node_id).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Metadata {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "latest_finalized_block_id",
            "latestFinalizedBlockId",
            "latest_finalized_height",
            "latestFinalizedHeight",
            "node_id",
            "nodeId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LatestFinalizedBlockId,
            LatestFinalizedHeight,
            NodeId,
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
                            "latestFinalizedBlockId" | "latest_finalized_block_id" => Ok(GeneratedField::LatestFinalizedBlockId),
                            "latestFinalizedHeight" | "latest_finalized_height" => Ok(GeneratedField::LatestFinalizedHeight),
                            "nodeId" | "node_id" => Ok(GeneratedField::NodeId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Metadata;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.entities.Metadata")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Metadata, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut latest_finalized_block_id__ = None;
                let mut latest_finalized_height__ = None;
                let mut node_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::LatestFinalizedBlockId => {
                            if latest_finalized_block_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("latestFinalizedBlockId"));
                            }
                            latest_finalized_block_id__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::LatestFinalizedHeight => {
                            if latest_finalized_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("latestFinalizedHeight"));
                            }
                            latest_finalized_height__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::NodeId => {
                            if node_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nodeId"));
                            }
                            node_id__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(Metadata {
                    latest_finalized_block_id: latest_finalized_block_id__.unwrap_or_default(),
                    latest_finalized_height: latest_finalized_height__.unwrap_or_default(),
                    node_id: node_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.entities.Metadata", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for NodeVersionInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.semver.is_empty() {
            len += 1;
        }
        if !self.commit.is_empty() {
            len += 1;
        }
        if !self.spork_id.is_empty() {
            len += 1;
        }
        if self.protocol_version != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.entities.NodeVersionInfo", len)?;
        if !self.semver.is_empty() {
            struct_ser.serialize_field("semver", &self.semver)?;
        }
        if !self.commit.is_empty() {
            struct_ser.serialize_field("commit", &self.commit)?;
        }
        if !self.spork_id.is_empty() {
            struct_ser.serialize_field("sporkId", pbjson::private::base64::encode(&self.spork_id).as_str())?;
        }
        if self.protocol_version != 0 {
            struct_ser.serialize_field("protocolVersion", ToString::to_string(&self.protocol_version).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NodeVersionInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "semver",
            "commit",
            "spork_id",
            "sporkId",
            "protocol_version",
            "protocolVersion",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Semver,
            Commit,
            SporkId,
            ProtocolVersion,
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
                            "semver" => Ok(GeneratedField::Semver),
                            "commit" => Ok(GeneratedField::Commit),
                            "sporkId" | "spork_id" => Ok(GeneratedField::SporkId),
                            "protocolVersion" | "protocol_version" => Ok(GeneratedField::ProtocolVersion),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NodeVersionInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.entities.NodeVersionInfo")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<NodeVersionInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut semver__ = None;
                let mut commit__ = None;
                let mut spork_id__ = None;
                let mut protocol_version__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Semver => {
                            if semver__.is_some() {
                                return Err(serde::de::Error::duplicate_field("semver"));
                            }
                            semver__ = Some(map.next_value()?);
                        }
                        GeneratedField::Commit => {
                            if commit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commit"));
                            }
                            commit__ = Some(map.next_value()?);
                        }
                        GeneratedField::SporkId => {
                            if spork_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sporkId"));
                            }
                            spork_id__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ProtocolVersion => {
                            if protocol_version__.is_some() {
                                return Err(serde::de::Error::duplicate_field("protocolVersion"));
                            }
                            protocol_version__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(NodeVersionInfo {
                    semver: semver__.unwrap_or_default(),
                    commit: commit__.unwrap_or_default(),
                    spork_id: spork_id__.unwrap_or_default(),
                    protocol_version: protocol_version__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.entities.NodeVersionInfo", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Payload {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key_part.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.entities.Payload", len)?;
        if !self.key_part.is_empty() {
            struct_ser.serialize_field("keyPart", &self.key_part)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", pbjson::private::base64::encode(&self.value).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Payload {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "keyPart",
            "value",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            KeyPart,
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
                            "keyPart" => Ok(GeneratedField::KeyPart),
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
            type Value = Payload;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.entities.Payload")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Payload, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key_part__ = None;
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::KeyPart => {
                            if key_part__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyPart"));
                            }
                            key_part__ = Some(map.next_value()?);
                        }
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
                Ok(Payload {
                    key_part: key_part__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.entities.Payload", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for QuorumCertificate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.view != 0 {
            len += 1;
        }
        if !self.block_id.is_empty() {
            len += 1;
        }
        if !self.signer_indices.is_empty() {
            len += 1;
        }
        if !self.sig_data.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.entities.QuorumCertificate", len)?;
        if self.view != 0 {
            struct_ser.serialize_field("view", ToString::to_string(&self.view).as_str())?;
        }
        if !self.block_id.is_empty() {
            struct_ser.serialize_field("blockId", pbjson::private::base64::encode(&self.block_id).as_str())?;
        }
        if !self.signer_indices.is_empty() {
            struct_ser.serialize_field("signerIndices", pbjson::private::base64::encode(&self.signer_indices).as_str())?;
        }
        if !self.sig_data.is_empty() {
            struct_ser.serialize_field("sigData", pbjson::private::base64::encode(&self.sig_data).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QuorumCertificate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "view",
            "block_id",
            "blockId",
            "signer_indices",
            "signerIndices",
            "sig_data",
            "sigData",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            View,
            BlockId,
            SignerIndices,
            SigData,
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
                            "view" => Ok(GeneratedField::View),
                            "blockId" | "block_id" => Ok(GeneratedField::BlockId),
                            "signerIndices" | "signer_indices" => Ok(GeneratedField::SignerIndices),
                            "sigData" | "sig_data" => Ok(GeneratedField::SigData),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QuorumCertificate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.entities.QuorumCertificate")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<QuorumCertificate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut view__ = None;
                let mut block_id__ = None;
                let mut signer_indices__ = None;
                let mut sig_data__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::View => {
                            if view__.is_some() {
                                return Err(serde::de::Error::duplicate_field("view"));
                            }
                            view__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::BlockId => {
                            if block_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("blockId"));
                            }
                            block_id__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SignerIndices => {
                            if signer_indices__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signerIndices"));
                            }
                            signer_indices__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SigData => {
                            if sig_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sigData"));
                            }
                            sig_data__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(QuorumCertificate {
                    view: view__.unwrap_or_default(),
                    block_id: block_id__.unwrap_or_default(),
                    signer_indices: signer_indices__.unwrap_or_default(),
                    sig_data: sig_data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.entities.QuorumCertificate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ServiceEvent {
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
        if !self.payload.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.entities.ServiceEvent", len)?;
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.payload.is_empty() {
            struct_ser.serialize_field("payload", pbjson::private::base64::encode(&self.payload).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ServiceEvent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "payload",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            Payload,
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
                            "payload" => Ok(GeneratedField::Payload),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ServiceEvent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.entities.ServiceEvent")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ServiceEvent, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut payload__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map.next_value()?);
                        }
                        GeneratedField::Payload => {
                            if payload__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payload"));
                            }
                            payload__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ServiceEvent {
                    r#type: r#type__.unwrap_or_default(),
                    payload: payload__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.entities.ServiceEvent", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TimeoutCertificate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.view != 0 {
            len += 1;
        }
        if !self.high_qc_views.is_empty() {
            len += 1;
        }
        if self.highest_qc.is_some() {
            len += 1;
        }
        if !self.signer_indices.is_empty() {
            len += 1;
        }
        if !self.sig_data.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.entities.TimeoutCertificate", len)?;
        if self.view != 0 {
            struct_ser.serialize_field("view", ToString::to_string(&self.view).as_str())?;
        }
        if !self.high_qc_views.is_empty() {
            struct_ser.serialize_field("highQcViews", &self.high_qc_views.iter().map(ToString::to_string).collect::<Vec<_>>())?;
        }
        if let Some(v) = self.highest_qc.as_ref() {
            struct_ser.serialize_field("highestQc", v)?;
        }
        if !self.signer_indices.is_empty() {
            struct_ser.serialize_field("signerIndices", pbjson::private::base64::encode(&self.signer_indices).as_str())?;
        }
        if !self.sig_data.is_empty() {
            struct_ser.serialize_field("sigData", pbjson::private::base64::encode(&self.sig_data).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TimeoutCertificate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "view",
            "high_qc_views",
            "highQcViews",
            "highest_qc",
            "highestQc",
            "signer_indices",
            "signerIndices",
            "sig_data",
            "sigData",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            View,
            HighQcViews,
            HighestQc,
            SignerIndices,
            SigData,
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
                            "view" => Ok(GeneratedField::View),
                            "highQcViews" | "high_qc_views" => Ok(GeneratedField::HighQcViews),
                            "highestQc" | "highest_qc" => Ok(GeneratedField::HighestQc),
                            "signerIndices" | "signer_indices" => Ok(GeneratedField::SignerIndices),
                            "sigData" | "sig_data" => Ok(GeneratedField::SigData),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TimeoutCertificate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.entities.TimeoutCertificate")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TimeoutCertificate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut view__ = None;
                let mut high_qc_views__ = None;
                let mut highest_qc__ = None;
                let mut signer_indices__ = None;
                let mut sig_data__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::View => {
                            if view__.is_some() {
                                return Err(serde::de::Error::duplicate_field("view"));
                            }
                            view__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::HighQcViews => {
                            if high_qc_views__.is_some() {
                                return Err(serde::de::Error::duplicate_field("highQcViews"));
                            }
                            high_qc_views__ = 
                                Some(map.next_value::<Vec<::pbjson::private::NumberDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::HighestQc => {
                            if highest_qc__.is_some() {
                                return Err(serde::de::Error::duplicate_field("highestQc"));
                            }
                            highest_qc__ = map.next_value()?;
                        }
                        GeneratedField::SignerIndices => {
                            if signer_indices__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signerIndices"));
                            }
                            signer_indices__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SigData => {
                            if sig_data__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sigData"));
                            }
                            sig_data__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(TimeoutCertificate {
                    view: view__.unwrap_or_default(),
                    high_qc_views: high_qc_views__.unwrap_or_default(),
                    highest_qc: highest_qc__,
                    signer_indices: signer_indices__.unwrap_or_default(),
                    sig_data: sig_data__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.entities.TimeoutCertificate", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Transaction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.script.is_empty() {
            len += 1;
        }
        if !self.arguments.is_empty() {
            len += 1;
        }
        if !self.reference_block_id.is_empty() {
            len += 1;
        }
        if self.gas_limit != 0 {
            len += 1;
        }
        if self.proposal_key.is_some() {
            len += 1;
        }
        if !self.payer.is_empty() {
            len += 1;
        }
        if !self.authorizers.is_empty() {
            len += 1;
        }
        if !self.payload_signatures.is_empty() {
            len += 1;
        }
        if !self.envelope_signatures.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.entities.Transaction", len)?;
        if !self.script.is_empty() {
            struct_ser.serialize_field("script", pbjson::private::base64::encode(&self.script).as_str())?;
        }
        if !self.arguments.is_empty() {
            struct_ser.serialize_field("arguments", &self.arguments.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        if !self.reference_block_id.is_empty() {
            struct_ser.serialize_field("referenceBlockId", pbjson::private::base64::encode(&self.reference_block_id).as_str())?;
        }
        if self.gas_limit != 0 {
            struct_ser.serialize_field("gasLimit", ToString::to_string(&self.gas_limit).as_str())?;
        }
        if let Some(v) = self.proposal_key.as_ref() {
            struct_ser.serialize_field("proposalKey", v)?;
        }
        if !self.payer.is_empty() {
            struct_ser.serialize_field("payer", pbjson::private::base64::encode(&self.payer).as_str())?;
        }
        if !self.authorizers.is_empty() {
            struct_ser.serialize_field("authorizers", &self.authorizers.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        if !self.payload_signatures.is_empty() {
            struct_ser.serialize_field("payloadSignatures", &self.payload_signatures)?;
        }
        if !self.envelope_signatures.is_empty() {
            struct_ser.serialize_field("envelopeSignatures", &self.envelope_signatures)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Transaction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "script",
            "arguments",
            "reference_block_id",
            "referenceBlockId",
            "gas_limit",
            "gasLimit",
            "proposal_key",
            "proposalKey",
            "payer",
            "authorizers",
            "payload_signatures",
            "payloadSignatures",
            "envelope_signatures",
            "envelopeSignatures",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Script,
            Arguments,
            ReferenceBlockId,
            GasLimit,
            ProposalKey,
            Payer,
            Authorizers,
            PayloadSignatures,
            EnvelopeSignatures,
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
                            "script" => Ok(GeneratedField::Script),
                            "arguments" => Ok(GeneratedField::Arguments),
                            "referenceBlockId" | "reference_block_id" => Ok(GeneratedField::ReferenceBlockId),
                            "gasLimit" | "gas_limit" => Ok(GeneratedField::GasLimit),
                            "proposalKey" | "proposal_key" => Ok(GeneratedField::ProposalKey),
                            "payer" => Ok(GeneratedField::Payer),
                            "authorizers" => Ok(GeneratedField::Authorizers),
                            "payloadSignatures" | "payload_signatures" => Ok(GeneratedField::PayloadSignatures),
                            "envelopeSignatures" | "envelope_signatures" => Ok(GeneratedField::EnvelopeSignatures),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Transaction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.entities.Transaction")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Transaction, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut script__ = None;
                let mut arguments__ = None;
                let mut reference_block_id__ = None;
                let mut gas_limit__ = None;
                let mut proposal_key__ = None;
                let mut payer__ = None;
                let mut authorizers__ = None;
                let mut payload_signatures__ = None;
                let mut envelope_signatures__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
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
                        GeneratedField::ReferenceBlockId => {
                            if reference_block_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceBlockId"));
                            }
                            reference_block_id__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::GasLimit => {
                            if gas_limit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gasLimit"));
                            }
                            gas_limit__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::ProposalKey => {
                            if proposal_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("proposalKey"));
                            }
                            proposal_key__ = map.next_value()?;
                        }
                        GeneratedField::Payer => {
                            if payer__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payer"));
                            }
                            payer__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Authorizers => {
                            if authorizers__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authorizers"));
                            }
                            authorizers__ = 
                                Some(map.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::PayloadSignatures => {
                            if payload_signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payloadSignatures"));
                            }
                            payload_signatures__ = Some(map.next_value()?);
                        }
                        GeneratedField::EnvelopeSignatures => {
                            if envelope_signatures__.is_some() {
                                return Err(serde::de::Error::duplicate_field("envelopeSignatures"));
                            }
                            envelope_signatures__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Transaction {
                    script: script__.unwrap_or_default(),
                    arguments: arguments__.unwrap_or_default(),
                    reference_block_id: reference_block_id__.unwrap_or_default(),
                    gas_limit: gas_limit__.unwrap_or_default(),
                    proposal_key: proposal_key__,
                    payer: payer__.unwrap_or_default(),
                    authorizers: authorizers__.unwrap_or_default(),
                    payload_signatures: payload_signatures__.unwrap_or_default(),
                    envelope_signatures: envelope_signatures__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.entities.Transaction", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for transaction::ProposalKey {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        if self.key_id != 0 {
            len += 1;
        }
        if self.sequence_number != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.entities.Transaction.ProposalKey", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", pbjson::private::base64::encode(&self.address).as_str())?;
        }
        if self.key_id != 0 {
            struct_ser.serialize_field("keyId", &self.key_id)?;
        }
        if self.sequence_number != 0 {
            struct_ser.serialize_field("sequenceNumber", ToString::to_string(&self.sequence_number).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for transaction::ProposalKey {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "key_id",
            "keyId",
            "sequence_number",
            "sequenceNumber",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            KeyId,
            SequenceNumber,
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
                            "address" => Ok(GeneratedField::Address),
                            "keyId" | "key_id" => Ok(GeneratedField::KeyId),
                            "sequenceNumber" | "sequence_number" => Ok(GeneratedField::SequenceNumber),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = transaction::ProposalKey;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.entities.Transaction.ProposalKey")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<transaction::ProposalKey, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut key_id__ = None;
                let mut sequence_number__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::KeyId => {
                            if key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyId"));
                            }
                            key_id__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::SequenceNumber => {
                            if sequence_number__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sequenceNumber"));
                            }
                            sequence_number__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(transaction::ProposalKey {
                    address: address__.unwrap_or_default(),
                    key_id: key_id__.unwrap_or_default(),
                    sequence_number: sequence_number__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.entities.Transaction.ProposalKey", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for transaction::Signature {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        if self.key_id != 0 {
            len += 1;
        }
        if !self.signature.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.entities.Transaction.Signature", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", pbjson::private::base64::encode(&self.address).as_str())?;
        }
        if self.key_id != 0 {
            struct_ser.serialize_field("keyId", &self.key_id)?;
        }
        if !self.signature.is_empty() {
            struct_ser.serialize_field("signature", pbjson::private::base64::encode(&self.signature).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for transaction::Signature {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "key_id",
            "keyId",
            "signature",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            KeyId,
            Signature,
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
                            "address" => Ok(GeneratedField::Address),
                            "keyId" | "key_id" => Ok(GeneratedField::KeyId),
                            "signature" => Ok(GeneratedField::Signature),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = transaction::Signature;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.entities.Transaction.Signature")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<transaction::Signature, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut key_id__ = None;
                let mut signature__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::KeyId => {
                            if key_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("keyId"));
                            }
                            key_id__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Signature => {
                            if signature__.is_some() {
                                return Err(serde::de::Error::duplicate_field("signature"));
                            }
                            signature__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(transaction::Signature {
                    address: address__.unwrap_or_default(),
                    key_id: key_id__.unwrap_or_default(),
                    signature: signature__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.entities.Transaction.Signature", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for TransactionStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "UNKNOWN",
            Self::Pending => "PENDING",
            Self::Finalized => "FINALIZED",
            Self::Executed => "EXECUTED",
            Self::Sealed => "SEALED",
            Self::Expired => "EXPIRED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for TransactionStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNKNOWN",
            "PENDING",
            "FINALIZED",
            "EXECUTED",
            "SEALED",
            "EXPIRED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TransactionStatus;

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
                    .and_then(TransactionStatus::from_i32)
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
                    .and_then(TransactionStatus::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNKNOWN" => Ok(TransactionStatus::Unknown),
                    "PENDING" => Ok(TransactionStatus::Pending),
                    "FINALIZED" => Ok(TransactionStatus::Finalized),
                    "EXECUTED" => Ok(TransactionStatus::Executed),
                    "SEALED" => Ok(TransactionStatus::Sealed),
                    "EXPIRED" => Ok(TransactionStatus::Expired),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for TrieUpdate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.root_hash.is_empty() {
            len += 1;
        }
        if !self.paths.is_empty() {
            len += 1;
        }
        if !self.payloads.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("flow.entities.TrieUpdate", len)?;
        if !self.root_hash.is_empty() {
            struct_ser.serialize_field("rootHash", pbjson::private::base64::encode(&self.root_hash).as_str())?;
        }
        if !self.paths.is_empty() {
            struct_ser.serialize_field("paths", &self.paths.iter().map(pbjson::private::base64::encode).collect::<Vec<_>>())?;
        }
        if !self.payloads.is_empty() {
            struct_ser.serialize_field("payloads", &self.payloads)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TrieUpdate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "root_hash",
            "rootHash",
            "paths",
            "payloads",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RootHash,
            Paths,
            Payloads,
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
                            "rootHash" | "root_hash" => Ok(GeneratedField::RootHash),
                            "paths" => Ok(GeneratedField::Paths),
                            "payloads" => Ok(GeneratedField::Payloads),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TrieUpdate;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct flow.entities.TrieUpdate")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TrieUpdate, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut root_hash__ = None;
                let mut paths__ = None;
                let mut payloads__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RootHash => {
                            if root_hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rootHash"));
                            }
                            root_hash__ = 
                                Some(map.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Paths => {
                            if paths__.is_some() {
                                return Err(serde::de::Error::duplicate_field("paths"));
                            }
                            paths__ = 
                                Some(map.next_value::<Vec<::pbjson::private::BytesDeserialize<_>>>()?
                                    .into_iter().map(|x| x.0).collect())
                            ;
                        }
                        GeneratedField::Payloads => {
                            if payloads__.is_some() {
                                return Err(serde::de::Error::duplicate_field("payloads"));
                            }
                            payloads__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(TrieUpdate {
                    root_hash: root_hash__.unwrap_or_default(),
                    paths: paths__.unwrap_or_default(),
                    payloads: payloads__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("flow.entities.TrieUpdate", FIELDS, GeneratedVisitor)
    }
}
