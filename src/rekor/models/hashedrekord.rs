/*
 * Rekor
 *
 * Rekor is a cryptographically secure, immutable transparency log for signed software releases.
 *
 * The version of the OpenAPI document: 0.0.1
 *
 * Generated by: https://openapi-generator.tech
 */

use serde::{Deserialize, Serialize};

/// Hashedrekord : Hashed Rekord object

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Hashedrekord {
    #[serde(rename = "kind")]
    pub kind: String,
    #[serde(rename = "apiVersion")]
    pub api_version: String,
    #[serde(rename = "spec")]
    pub spec: Spec,
}

impl Hashedrekord {
    /// Hashed Rekord object
    pub fn new(kind: String, api_version: String, spec: Spec) -> Hashedrekord {
        Hashedrekord {
            kind,
            api_version,
            spec,
        }
    }
}

/// Stores the Signature and Data struct
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    signature: Signature,
    data: Data,
}

// Design a SPEC struct
impl Spec {
    pub fn new(signature: Signature, data: Data) -> Spec {
        Spec { signature, data }
    }
}

/// Stores the signature format, signature of the artifact and the PublicKey struct
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Signature {
    format: String,
    content: String,
    public_key: PublicKey,
}

impl Signature {
    pub fn new(format: String, content: String, public_key: PublicKey) -> Signature {
        Signature {
            format,
            content,
            public_key,
        }
    }
}

/// Stores the public key used to sign the artifact
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicKey {
    content: String,
}

impl PublicKey {
    pub fn new(content: String) -> PublicKey {
        PublicKey { content }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub hash: Hash,
}

impl Data {
    pub fn new(hash: Hash) -> Data {
        Data { hash }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[allow(non_camel_case_types)]
pub enum AlgorithmKind {
    sha256,
    sha1,
}

/// Stores the algorithm used to hash the artifact and the value of the hash
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hash {
    algorithm: AlgorithmKind,
    value: String,
}

impl Hash {
    pub fn new(algorithm: AlgorithmKind, value: String) -> Hash {
        Hash { algorithm, value }
    }
}
