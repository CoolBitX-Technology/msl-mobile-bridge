use cardano_message_signing::LabelKind;
use cardano_message_signing::SigContext;
use cardano_message_signing::SignedMessageKind;
use cardano_message_signing::builders::AlgorithmId;
use cardano_message_signing::builders::CurveType;
use cardano_message_signing::builders::ECKey;
use cardano_message_signing::builders::KeyOperation;
use cardano_message_signing::builders::KeyType;
use cardano_message_signing::cbor::CBORSpecialType;
use cardano_message_signing::cbor::CBORValueKind;

use crate::panic::Result;

pub trait ToPrimitive {
    fn to_i32(&self) -> i32;
}

pub trait ToEnum<T> {
    fn to_enum(&self) -> Result<T>;
}
impl ToPrimitive for AlgorithmId {
    fn to_i32(&self) -> i32 {
        match self {
            AlgorithmId::EdDSA => 0,
            AlgorithmId::ChaCha20Poly1305 => 1,
        }
    }
}

impl ToEnum<AlgorithmId> for i32 {
    fn to_enum(&self) -> Result<AlgorithmId> {
        match self {
            0 => Ok(AlgorithmId::EdDSA),
            1 => Ok(AlgorithmId::ChaCha20Poly1305),
            _ => Err("Invalid value for AlgorithmId".into()),
        }
    }
}

impl ToPrimitive for CBORSpecialType {
    fn to_i32(&self) -> i32 {
        match self {
            CBORSpecialType::Bool => 0,
            CBORSpecialType::Float => 1,
            CBORSpecialType::Unassigned => 2,
            CBORSpecialType::Break => 3,
            CBORSpecialType::Undefined => 4,
            CBORSpecialType::Null => 5,
        }
    }
}

impl ToEnum<CBORSpecialType> for i32 {
    fn to_enum(&self) -> Result<CBORSpecialType> {
        match self {
            0 => Ok(CBORSpecialType::Bool),
            1 => Ok(CBORSpecialType::Float),
            2 => Ok(CBORSpecialType::Unassigned),
            3 => Ok(CBORSpecialType::Break),
            4 => Ok(CBORSpecialType::Undefined),
            5 => Ok(CBORSpecialType::Null),
            _ => Err("Invalid value for CBORSpecialType".into()),
        }
    }
}

impl ToPrimitive for CBORValueKind {
    fn to_i32(&self) -> i32 {
        match self {
            CBORValueKind::Int => 0,
            CBORValueKind::Bytes => 1,
            CBORValueKind::Text => 2,
            CBORValueKind::Array => 3,
            CBORValueKind::Object => 4,
            CBORValueKind::TaggedCBOR => 5,
            CBORValueKind::Special => 6,
        }
    }
}

impl ToEnum<CBORValueKind> for i32 {
    fn to_enum(&self) -> Result<CBORValueKind> {
        match self {
            0 => Ok(CBORValueKind::Int),
            1 => Ok(CBORValueKind::Bytes),
            2 => Ok(CBORValueKind::Text),
            3 => Ok(CBORValueKind::Array),
            4 => Ok(CBORValueKind::Object),
            5 => Ok(CBORValueKind::TaggedCBOR),
            6 => Ok(CBORValueKind::Special),
            _ => Err("Invalid value for CBORValueKind".into()),
        }
    }
}

impl ToPrimitive for CurveType {
    fn to_i32(&self) -> i32 {
        match self {
            CurveType::P256 => 0,
            CurveType::P384 => 1,
            CurveType::P521 => 2,
            CurveType::X25519 => 3,
            CurveType::X448 => 4,
            CurveType::Ed25519 => 5,
            CurveType::Ed448 => 6,
        }
    }
}

impl ToEnum<CurveType> for i32 {
    fn to_enum(&self) -> Result<CurveType> {
        match self {
            0 => Ok(CurveType::P256),
            1 => Ok(CurveType::P384),
            2 => Ok(CurveType::P521),
            3 => Ok(CurveType::X25519),
            4 => Ok(CurveType::X448),
            5 => Ok(CurveType::Ed25519),
            6 => Ok(CurveType::Ed448),
            _ => Err("Invalid value for CurveType".into()),
        }
    }
}

impl ToPrimitive for ECKey {
    fn to_i32(&self) -> i32 {
        match self {
            ECKey::CRV => 0,
            ECKey::X => 1,
            ECKey::Y => 2,
            ECKey::D => 3,
        }
    }
}

impl ToEnum<ECKey> for i32 {
    fn to_enum(&self) -> Result<ECKey> {
        match self {
            0 => Ok(ECKey::CRV),
            1 => Ok(ECKey::X),
            2 => Ok(ECKey::Y),
            3 => Ok(ECKey::D),
            _ => Err("Invalid value for ECKey".into()),
        }
    }
}

impl ToPrimitive for KeyOperation {
    fn to_i32(&self) -> i32 {
        match self {
            KeyOperation::Sign => 0,
            KeyOperation::Verify => 1,
            KeyOperation::Encrypt => 2,
            KeyOperation::Decrypt => 3,
            KeyOperation::WrapKey => 4,
            KeyOperation::UnwrapKey => 5,
            KeyOperation::DeriveKey => 6,
            KeyOperation::DeriveBits => 7,
        }
    }
}

impl ToEnum<KeyOperation> for i32 {
    fn to_enum(&self) -> Result<KeyOperation> {
        match self {
            0 => Ok(KeyOperation::Sign),
            1 => Ok(KeyOperation::Verify),
            2 => Ok(KeyOperation::Encrypt),
            3 => Ok(KeyOperation::Decrypt),
            4 => Ok(KeyOperation::WrapKey),
            5 => Ok(KeyOperation::UnwrapKey),
            6 => Ok(KeyOperation::DeriveKey),
            7 => Ok(KeyOperation::DeriveBits),
            _ => Err("Invalid value for KeyOperation".into()),
        }
    }
}

impl ToPrimitive for KeyType {
    fn to_i32(&self) -> i32 {
        match self {
            KeyType::OKP => 0,
            KeyType::EC2 => 1,
            KeyType::Symmetric => 2,
        }
    }
}

impl ToEnum<KeyType> for i32 {
    fn to_enum(&self) -> Result<KeyType> {
        match self {
            0 => Ok(KeyType::OKP),
            1 => Ok(KeyType::EC2),
            2 => Ok(KeyType::Symmetric),
            _ => Err("Invalid value for KeyType".into()),
        }
    }
}

impl ToPrimitive for LabelKind {
    fn to_i32(&self) -> i32 {
        match self {
            LabelKind::Int => 0,
            LabelKind::Text => 1,
        }
    }
}

impl ToEnum<LabelKind> for i32 {
    fn to_enum(&self) -> Result<LabelKind> {
        match self {
            0 => Ok(LabelKind::Int),
            1 => Ok(LabelKind::Text),
            _ => Err("Invalid value for LabelKind".into()),
        }
    }
}

impl ToPrimitive for SigContext {
    fn to_i32(&self) -> i32 {
        match self {
            SigContext::Signature => 0,
            SigContext::Signature1 => 1,
            SigContext::CounterSignature => 2,
        }
    }
}

impl ToEnum<SigContext> for i32 {
    fn to_enum(&self) -> Result<SigContext> {
        match self {
            0 => Ok(SigContext::Signature),
            1 => Ok(SigContext::Signature1),
            2 => Ok(SigContext::CounterSignature),
            _ => Err("Invalid value for SigContext".into()),
        }
    }
}

impl ToPrimitive for SignedMessageKind {
    fn to_i32(&self) -> i32 {
        match self {
            SignedMessageKind::COSESIGN => 0,
            SignedMessageKind::COSESIGN1 => 1,
        }
    }
}

impl ToEnum<SignedMessageKind> for i32 {
    fn to_enum(&self) -> Result<SignedMessageKind> {
        match self {
            0 => Ok(SignedMessageKind::COSESIGN),
            1 => Ok(SignedMessageKind::COSESIGN1),
            _ => Err("Invalid value for SignedMessageKind".into()),
        }
    }
}

