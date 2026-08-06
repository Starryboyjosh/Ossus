//! Closed trusted taxonomies used by canonical manifests.

use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::ContractError;

macro_rules! contract_enum {
    (
        $(#[$meta:meta])*
        $name:ident, $error:ident {
            $($variant:ident => $wire:literal),+ $(,)?
        }
    ) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
        $(#[$meta])*
        pub enum $name {
            $(
                #[serde(rename = $wire)]
                $variant
            ),+
        }

        impl $name {
            pub const fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $wire),+
                }
            }
        }

        impl FromStr for $name {
            type Err = ContractError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                match value {
                    $($wire => Ok(Self::$variant)),+,
                    _ => Err(ContractError::$error {
                        value: value.to_owned(),
                    }),
                }
            }
        }

        impl TryFrom<&str> for $name {
            type Error = ContractError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                value.parse()
            }
        }
    };
}

contract_enum! {
    ResourceType, UnknownResourceType {
        Skill => "skill",
        PromptPack => "prompt-pack",
        McpServer => "mcp-server",
    }
}

contract_enum! {
    SourceMode, UnknownSourceMode {
        RemoteIndex => "remote-index",
        Vendored => "vendored",
        LocalPrivate => "local-private",
    }
}

contract_enum! {
    DistributionMode, UnknownDistributionMode {
        SourceOnly => "source-only",
        ApprovedInstallOnly => "approved-install-only",
        VendoredRedistributable => "vendored-redistributable",
    }
}

contract_enum! {
    Portability, UnknownPortability {
        PortableStandard => "portable-standard",
        PortableWithAdapter => "portable-with-adapter",
        HostExtension => "host-extension",
        HostExclusive => "host-exclusive",
        Unknown => "unknown",
    }
}

contract_enum! {
    Scope, UnknownScope {
        Project => "project",
        UserGlobal => "user-global",
        Plugin => "plugin",
        CustomPath => "custom-path",
        RemoteOnly => "remote-only",
    }
}

contract_enum! {
    RuntimeRequirement, UnknownRuntimeRequirement {
        InstructionOnly => "instruction-only",
        FilesystemOnly => "filesystem-only",
        ShellRequired => "shell-required",
        NetworkRequired => "network-required",
        McpRequired => "mcp-required",
        ExternalCliRequired => "external-cli-required",
        HostApiRequired => "host-api-required",
    }
}

contract_enum! {
    #[derive(Ord, PartialOrd)]
    RiskTier, UnknownRiskTier {
        R0 => "R0",
        R1 => "R1",
        R2 => "R2",
        R3 => "R3",
        R4 => "R4",
        R5 => "R5",
    }
}

contract_enum! {
    ReviewStatus, UnknownReviewStatus {
        Approved => "approved",
        Deprecated => "deprecated",
        Revoked => "revoked",
    }
}

contract_enum! {
    #[derive(Ord, PartialOrd)]
    ReviewTier, UnknownReviewTier {
        LightHuman => "light-human",
        FullHuman => "full-human",
        SecurityHuman => "security-human",
    }
}

contract_enum! {
    Surface, UnknownSurface {
        AgentSkillsStandard => "agent-skills-standard",
        ClaudeCodeCli => "claude-code-cli",
        ClaudeAgentSdk => "claude-agent-sdk",
        ClaudeApiHost => "claude-api-host",
        CodexCli => "codex-cli",
        CodexIde => "codex-ide",
        GenericTerminalAgent => "generic-terminal-agent",
        GenericMcpClient => "generic-mcp-client",
        StandaloneCli => "standalone-cli",
    }
}

contract_enum! {
    CapabilityStatus, UnknownCapabilityStatus {
        Active => "active",
        Deprecated => "deprecated",
    }
}

/// Returns whether provenance and redistribution terms form a coherent pair.
///
/// `remote-index` grants no redistribution right; `vendored` means a copy already
/// exists, so `source-only` is a contradiction; and `local-private` is private
/// by definition.
pub const fn distribution_is_permitted(source: SourceMode, distribution: DistributionMode) -> bool {
    match (source, distribution) {
        (SourceMode::RemoteIndex, DistributionMode::SourceOnly)
        | (SourceMode::RemoteIndex, DistributionMode::ApprovedInstallOnly)
        | (SourceMode::Vendored, DistributionMode::ApprovedInstallOnly)
        | (SourceMode::Vendored, DistributionMode::VendoredRedistributable)
        | (SourceMode::LocalPrivate, DistributionMode::SourceOnly)
        | (SourceMode::LocalPrivate, DistributionMode::ApprovedInstallOnly) => true,
        (SourceMode::RemoteIndex, DistributionMode::VendoredRedistributable)
        | (SourceMode::Vendored, DistributionMode::SourceOnly)
        | (SourceMode::LocalPrivate, DistributionMode::VendoredRedistributable) => false,
    }
}

#[cfg(test)]
mod tests {
    use std::fmt;

    use serde::de::value::{Error as ValueError, StrDeserializer};
    use serde::{Deserialize, Serialize};

    use super::{
        CapabilityStatus, DistributionMode, Portability, ResourceType, ReviewStatus, ReviewTier,
        RiskTier, RuntimeRequirement, Scope, SourceMode, Surface, distribution_is_permitted,
    };

    /// Asserts that the serde wire form is the same governed string as `as_str`.
    ///
    /// The typed parser and the deserializer must not be two sources of truth:
    /// if they drift, a manifest can carry a value that deserializes cleanly and
    /// is then rejected — or worse, accepted — by the parser that guards it.
    fn assert_serde_agrees_with_wire<T>(wire: &str)
    where
        T: Serialize + for<'de> Deserialize<'de> + fmt::Debug,
    {
        let deserializer = StrDeserializer::<ValueError>::new(wire);
        let value = match T::deserialize(deserializer) {
            Ok(value) => value,
            Err(error) => panic!("serde rejected governed value `{wire}`: {error}"),
        };
        match toml::Value::try_from(&value) {
            Ok(toml::Value::String(rendered)) => assert_eq!(rendered, wire),
            other => panic!("serde rendered `{wire}` as {other:?}, not the governed string"),
        }
    }

    fn assert_serde_rejects<T>(unknown: &str) -> bool
    where
        T: for<'de> Deserialize<'de>,
    {
        T::deserialize(StrDeserializer::<ValueError>::new(unknown)).is_err()
    }

    macro_rules! assert_enum_contract {
        ($type:ty, $code:literal, [$($wire:literal),+ $(,)?]) => {{
            $(
                let parsed = <$type>::try_from($wire);
                assert!(matches!(parsed, Ok(value) if value.as_str() == $wire));
                assert_serde_agrees_with_wire::<$type>($wire);
            )+
            let unknown = <$type>::try_from("not-valid");
            assert!(matches!(unknown, Err(error) if error.reason_code() == $code));
            assert!(
                assert_serde_rejects::<$type>("not-valid"),
                "serde accepted a value the typed parser rejects"
            );
        }};
    }

    #[test]
    fn every_enum_value_round_trips_and_unknowns_have_stable_codes() {
        assert_enum_contract!(
            ResourceType,
            "resource-type.unknown",
            ["skill", "prompt-pack", "mcp-server"]
        );
        assert_enum_contract!(
            SourceMode,
            "source.mode.unknown",
            ["remote-index", "vendored", "local-private"]
        );
        assert_enum_contract!(
            DistributionMode,
            "distribution.mode.unknown",
            [
                "source-only",
                "approved-install-only",
                "vendored-redistributable"
            ]
        );
        assert_enum_contract!(
            Portability,
            "portability.unknown",
            [
                "portable-standard",
                "portable-with-adapter",
                "host-extension",
                "host-exclusive",
                "unknown"
            ]
        );
        assert_enum_contract!(
            Scope,
            "scope.unknown",
            [
                "project",
                "user-global",
                "plugin",
                "custom-path",
                "remote-only"
            ]
        );
        assert_enum_contract!(
            RuntimeRequirement,
            "runtime.requirement.unknown",
            [
                "instruction-only",
                "filesystem-only",
                "shell-required",
                "network-required",
                "mcp-required",
                "external-cli-required",
                "host-api-required"
            ]
        );
        assert_enum_contract!(
            RiskTier,
            "risk.tier.unknown",
            ["R0", "R1", "R2", "R3", "R4", "R5"]
        );
        assert_enum_contract!(
            ReviewStatus,
            "review.status.unknown",
            ["approved", "deprecated", "revoked"]
        );
        assert_enum_contract!(
            ReviewTier,
            "review.tier.unknown",
            ["light-human", "full-human", "security-human"]
        );
        assert_enum_contract!(
            Surface,
            "compatibility.surfaces.unknown",
            [
                "agent-skills-standard",
                "claude-code-cli",
                "claude-agent-sdk",
                "claude-api-host",
                "codex-cli",
                "codex-ide",
                "generic-terminal-agent",
                "generic-mcp-client",
                "standalone-cli"
            ]
        );
        assert_enum_contract!(
            CapabilityStatus,
            "capability.status.unknown",
            ["active", "deprecated"]
        );
    }

    #[test]
    fn surface_parsing_is_case_sensitive_and_closed() {
        for invalid in ["claude-code-CLI", "not-a-surface"] {
            let result = Surface::try_from(invalid);
            assert!(matches!(
                result,
                Err(error) if error.reason_code() == "compatibility.surfaces.unknown"
            ));
        }
    }

    #[test]
    fn risk_tiers_are_ordered_by_severity() {
        assert!(RiskTier::R0 < RiskTier::R5);
    }

    #[test]
    fn every_source_distribution_pair_is_explicitly_checked() {
        assert!(distribution_is_permitted(
            SourceMode::RemoteIndex,
            DistributionMode::SourceOnly
        ));
        assert!(distribution_is_permitted(
            SourceMode::RemoteIndex,
            DistributionMode::ApprovedInstallOnly
        ));
        assert!(!distribution_is_permitted(
            SourceMode::RemoteIndex,
            DistributionMode::VendoredRedistributable
        ));
        assert!(!distribution_is_permitted(
            SourceMode::Vendored,
            DistributionMode::SourceOnly
        ));
        assert!(distribution_is_permitted(
            SourceMode::Vendored,
            DistributionMode::ApprovedInstallOnly
        ));
        assert!(distribution_is_permitted(
            SourceMode::Vendored,
            DistributionMode::VendoredRedistributable
        ));
        assert!(distribution_is_permitted(
            SourceMode::LocalPrivate,
            DistributionMode::SourceOnly
        ));
        assert!(distribution_is_permitted(
            SourceMode::LocalPrivate,
            DistributionMode::ApprovedInstallOnly
        ));
        assert!(!distribution_is_permitted(
            SourceMode::LocalPrivate,
            DistributionMode::VendoredRedistributable
        ));
    }
}
