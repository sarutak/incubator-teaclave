use crate::teaclave_frontend_service_proto as proto;
use anyhow::anyhow;
use anyhow::{Error, Result};
use core::convert::TryInto;
use teaclave_types::TeaclaveFileCryptoInfo;
use url::Url;

pub use proto::TeaclaveFrontend;
pub use proto::TeaclaveFrontendClient;
pub use proto::TeaclaveFrontendRequest;
pub use proto::TeaclaveFrontendResponse;

#[derive(Debug)]
pub struct RegisterInputFileRequest {
    pub url: Url,
    pub hash: std::string::String,
    pub crypto_info: TeaclaveFileCryptoInfo,
}

#[derive(Debug)]
pub struct RegisterInputFileResponse {
    pub data_id: std::string::String,
}

#[derive(Debug)]
pub struct RegisterOutputFileRequest {
    pub url: Url,
    pub crypto_info: TeaclaveFileCryptoInfo,
}

#[derive(Debug)]
pub struct RegisterOutputFileResponse {
    pub data_id: std::string::String,
}

impl std::convert::TryFrom<proto::RegisterInputFileRequest> for RegisterInputFileRequest {
    type Error = Error;

    fn try_from(proto: proto::RegisterInputFileRequest) -> Result<Self> {
        let ret = Self {
            url: Url::parse(&proto.url)?,
            hash: proto.hash,
            crypto_info: proto
                .crypto_info
                .ok_or_else(|| anyhow!("missing crypto_info"))?
                .try_into()?,
        };

        Ok(ret)
    }
}

impl From<RegisterInputFileRequest> for proto::RegisterInputFileRequest {
    fn from(request: RegisterInputFileRequest) -> Self {
        Self {
            url: request.url.into_string(),
            hash: request.hash,
            crypto_info: Some(request.crypto_info.into()),
        }
    }
}

impl std::convert::TryFrom<proto::RegisterInputFileResponse> for RegisterInputFileResponse {
    type Error = Error;

    fn try_from(proto: proto::RegisterInputFileResponse) -> Result<Self> {
        Ok(Self {
            data_id: proto.data_id,
        })
    }
}

impl From<RegisterInputFileResponse> for proto::RegisterInputFileResponse {
    fn from(request: RegisterInputFileResponse) -> Self {
        Self {
            data_id: request.data_id,
        }
    }
}

impl std::convert::TryFrom<proto::RegisterOutputFileRequest> for RegisterOutputFileRequest {
    type Error = Error;

    fn try_from(proto: proto::RegisterOutputFileRequest) -> Result<Self> {
        let ret = Self {
            url: Url::parse(&proto.url)?,
            crypto_info: proto
                .crypto_info
                .ok_or_else(|| anyhow!("missing crypto_info"))?
                .try_into()?,
        };

        Ok(ret)
    }
}

impl From<RegisterOutputFileRequest> for proto::RegisterOutputFileRequest {
    fn from(request: RegisterOutputFileRequest) -> Self {
        Self {
            url: request.url.into_string(),
            crypto_info: Some(request.crypto_info.into()),
        }
    }
}

impl std::convert::TryFrom<proto::RegisterOutputFileResponse> for RegisterOutputFileResponse {
    type Error = Error;

    fn try_from(proto: proto::RegisterOutputFileResponse) -> Result<Self> {
        Ok(Self {
            data_id: proto.data_id,
        })
    }
}

impl From<RegisterOutputFileResponse> for proto::RegisterOutputFileResponse {
    fn from(request: RegisterOutputFileResponse) -> Self {
        Self {
            data_id: request.data_id,
        }
    }
}