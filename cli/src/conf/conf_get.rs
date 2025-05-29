use crate::{command_title, prelude::*};

/// Print the current Nexus CLI configuration.
pub(crate) async fn get_nexus_conf(conf_path: PathBuf) -> AnyResult<CliConf, NexusCliError> {
    let conf = CliConf::load_from_path(&conf_path)
        .await
        .unwrap_or_default();

    command_title!("Current Nexus CLI Configuration");

    Ok(conf)
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        nexus_sdk::{crypto::x3dh::PreKeyBundle, test_utils::sui_mocks},
    };

    #[tokio::test]
    #[serial_test::serial(master_key_env)]
    async fn test_get_nexus_conf() {
        std::env::set_var("NEXUS_CLI_STORE_PASSPHRASE", "test_passphrase");

        let secret_home = tempfile::tempdir().unwrap();
        std::env::set_var("XDG_CONFIG_HOME", secret_home.path());
        std::env::set_var("XDG_DATA_HOME", secret_home.path());

        let tempdir = tempfile::tempdir().unwrap().keep();
        let path = tempdir.join("conf.toml");

        assert!(!tokio::fs::try_exists(&path).await.unwrap());

        let nexus_objects = NexusObjects {
            workflow_pkg_id: sui::ObjectID::random(),
            primitives_pkg_id: sui::ObjectID::random(),
            interface_pkg_id: sui::ObjectID::random(),
            network_id: sui::ObjectID::random(),
            tool_registry: sui_mocks::mock_sui_object_ref(),
            default_sap: sui_mocks::mock_sui_object_ref(),
            gas_service: sui_mocks::mock_sui_object_ref(),
        };

        let sui_conf = SuiConf {
            net: SuiNet::Mainnet,
            wallet_path: tempdir.join("wallet"),
            auth_user: Some("user".to_string()),
            auth_password: Some("pass".to_string()),
        };

        let tools = HashMap::new();

        // Create sessions for testing
        let mut sessions = HashMap::new();

        // Create sender and receiver identities
        let sender_id = IdentityKey::generate();
        let receiver_id = IdentityKey::generate();

        let spk_secret = {
            use rand::{rngs::OsRng, RngCore};
            let mut rng = OsRng;
            let mut bytes = [0u8; 32];
            rng.fill_bytes(&mut bytes);
            nexus_sdk::crypto::x3dh::IdentityKey::generate()
                .secret()
                .clone()
        };
        let spk_id = 1;
        let bundle = PreKeyBundle::new(&receiver_id, spk_id, &spk_secret, None, None);

        // Initiate a session (sender side)
        let (_, sender_session) = Session::initiate(&sender_id, &bundle, b"test session message")
            .expect("Failed to initiate session");

        // Store the sender session
        sessions.insert(*sender_session.id(), sender_session);

        let crypto_conf = CryptoConf {
            identity_key: Some(IdentityKey::generate()),
            sessions,
        };

        let conf = CliConf {
            sui: sui_conf,
            nexus: Some(nexus_objects),
            tools,
            crypto: Secret::new(crypto_conf),
        };

        // Write the configuration to the file.
        let toml_str = toml::to_string(&conf).expect("Failed to serialize NexusObjects to TOML");

        tokio::fs::write(&path, toml_str)
            .await
            .expect("Failed to write conf.toml");

        // Ensure the command returns the correct string.
        let result = get_nexus_conf(path).await.expect("Failed to print config");

        assert_eq!(result, conf);

        // Verify sessions field is properly handled during deserialization
        assert_eq!(
            result.crypto.sessions.len(),
            1,
            "Should have 1 session (shared between sender and receiver)"
        );

        // Verify we can recover the sessions from the configuration
        for (session_id, session) in result.crypto.sessions.iter() {
            // Verify session IDs are properly stored and retrieved
            assert_eq!(
                session.id(),
                session_id,
                "Session ID should match the map key"
            );
        }

        // Clean-up env vars
        std::env::remove_var("NEXUS_CLI_STORE_PASSPHRASE");
        std::env::remove_var("XDG_CONFIG_HOME");
        std::env::remove_var("XDG_DATA_HOME");
    }
}
