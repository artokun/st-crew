use std::{borrow::Cow, convert::Infallible};

use serde::{de::IgnoredAny, Deserialize, Serialize};
use utoipa::ToSchema;

use super::RpcCommand;

#[derive(Debug, Deserialize)]
pub(crate) struct RpcCallCommand {
    pub id: u64,
    pub command: Cow<'static, str>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct RpcCallInput<C>
where
    C: RpcCommand,
{
    pub input: C::Input,
}

#[derive(Debug, Serialize)]
pub(crate) struct RpcReply<C>
where
    C: RpcCommand,
{
    pub id: u64,
    pub output: C::Output,
}

#[derive(ToSchema)]
/// Indicates that there are no inputs for the command.
///
/// This is necessary because `()` is not handled the same as a `Option::None` during
/// deserialization. This type is used to indicate that the input for a command is
/// nothing, and that the command's `input` field should be omitted.
pub struct NoInput;

impl From<()> for NoInput {
    fn from(_: ()) -> Self {
        NoInput
    }
}

impl From<Infallible> for NoInput {
    fn from(_: Infallible) -> Self {
        NoInput
    }
}

impl<'de> Deserialize<'de> for NoInput {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match Option::<IgnoredAny>::deserialize(deserializer) {
            Ok(Some(_)) => return Err(serde::de::Error::custom("expected no value or null")),

            Ok(None) => {}

            Err(e) => return Err(e),
        }

        Ok(NoInput)
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::NoInput;

    #[derive(Deserialize)]
    struct Test<C> {
        #[allow(dead_code)]
        inner: C,
    }

    #[test]
    fn do_test_thing() {
        assert!(
            serde_json::from_str::<Test<NoInput>>(r#"{ }"#).is_err(),
            "should permit missing field"
        );

        assert!(
            serde_json::from_str::<Test<NoInput>>(r#"{ "inner": null }"#).is_err(),
            "should permit null values"
        );

        assert!(
            serde_json::from_str::<Test<NoInput>>(r#"{ "inner": 8 }"#).is_err(),
            "should not permit non-null values"
        );
    }
}
