use chrono::{DateTime, Utc};
use letta::types::{AgentState, AgentType, Block, LettaId, Passage, Timestamp, Tool, ToolType};
use proptest::prelude::*;
use proptest::string::string_regex;
use serde::{Deserialize, Serialize};

fn arb_uuid() -> impl Strategy<Value = uuid::Uuid> {
    any::<[u8; 16]>().prop_map(uuid::Uuid::from_bytes)
}

fn arb_prefixed_id(prefix: &'static str) -> impl Strategy<Value = LettaId> {
    arb_uuid().prop_map(move |uuid| {
        format!("{prefix}-{uuid}")
            .parse::<LettaId>()
            .expect("generated prefixed LettaId must parse")
    })
}

fn arb_letta_id() -> impl Strategy<Value = LettaId> {
    (
        prop_oneof![
            Just("agent"),
            Just("tool"),
            Just("block"),
            Just("passage"),
            Just("source"),
            Just("project")
        ],
        arb_uuid(),
    )
        .prop_map(|(prefix, uuid)| {
            format!("{prefix}-{uuid}")
                .parse::<LettaId>()
                .expect("generated LettaId must parse")
        })
}

fn arb_agent_type() -> impl Strategy<Value = AgentType> {
    prop_oneof![
        Just(AgentType::MemGPT),
        Just(AgentType::MemGPTv2),
        Just(AgentType::React),
        Just(AgentType::Workflow),
        Just(AgentType::SplitThread),
        Just(AgentType::Sleeptime),
        Just(AgentType::VoiceConvo),
        Just(AgentType::VoiceSleeptime),
    ]
}

fn arb_timestamp() -> impl Strategy<Value = Timestamp> {
    (0_i64..4_102_444_800_i64).prop_map(|secs| {
        DateTime::<Utc>::from_timestamp(secs, 0)
            .expect("seconds range always yields valid UTC datetime")
    })
}

fn arb_block() -> impl Strategy<Value = Block> {
    (
        proptest::option::of(arb_prefixed_id("block")),
        string_regex("[a-z]{1,16}").expect("valid regex"),
        string_regex("[a-zA-Z0-9 .,!?_-]{1,160}").expect("valid regex"),
        proptest::option::of(1_u32..4096_u32),
        any::<bool>(),
        any::<bool>(),
        any::<bool>(),
        proptest::option::of(string_regex("[a-zA-Z0-9 .,!?_-]{0,120}").expect("valid regex")),
        proptest::option::of(string_regex("[a-zA-Z0-9 _-]{1,40}").expect("valid regex")),
    )
        .prop_map(
            |(
                id,
                label,
                value,
                limit,
                is_template,
                preserve_on_migration,
                read_only,
                description,
                name,
            )| {
                Block {
                    id,
                    label,
                    value,
                    limit,
                    is_template,
                    preserve_on_migration,
                    read_only,
                    description,
                    metadata: None,
                    name,
                    organization_id: None,
                    created_by_id: None,
                    last_updated_by_id: None,
                    created_at: None,
                    updated_at: None,
                }
            },
        )
}

fn arb_tool() -> impl Strategy<Value = Tool> {
    (
        proptest::option::of(arb_prefixed_id("tool")),
        string_regex("[a-z][a-z0-9_]{0,20}").expect("valid regex"),
        proptest::option::of(string_regex("[a-zA-Z0-9 .,!?_-]{0,120}").expect("valid regex")),
        proptest::option::of(proptest::collection::vec(
            string_regex("[a-z][a-z0-9_-]{0,12}").expect("valid regex"),
            0..5,
        )),
        proptest::option::of(0_u32..5000_u32),
    )
        .prop_map(|(id, name, description, tags, return_char_limit)| Tool {
            id,
            tool_type: Some(ToolType::Custom),
            description,
            source_type: None,
            organization_id: None,
            name,
            tags,
            source_code: None,
            json_schema: None,
            args_json_schema: None,
            return_char_limit,
            pip_requirements: None,
            created_by_id: None,
            last_updated_by_id: None,
            metadata: None,
            created_at: None,
            updated_at: None,
        })
}

fn arb_passage() -> impl Strategy<Value = Passage> {
    (
        arb_prefixed_id("passage"),
        string_regex("[a-zA-Z0-9 .,!?_-]{1,200}").expect("valid regex"),
        proptest::option::of(arb_prefixed_id("agent")),
        proptest::option::of(proptest::collection::vec(-1_000.0_f32..1_000.0_f32, 0..8)),
        proptest::option::of(string_regex("[a-zA-Z0-9._-]{1,32}\\.txt").expect("valid regex")),
    )
        .prop_map(|(id, text, agent_id, embedding, file_name)| Passage {
            id,
            text,
            agent_id,
            embedding,
            embedding_config: None,
            source_id: None,
            file_id: None,
            file_name,
            metadata: None,
            organization_id: None,
            created_by_id: None,
            last_updated_by_id: None,
            created_at: None,
            updated_at: None,
            is_deleted: None,
        })
}

fn arb_agent_state() -> impl Strategy<Value = AgentState> {
    (
        arb_prefixed_id("agent"),
        string_regex("[a-zA-Z0-9 _-]{1,40}").expect("valid regex"),
        proptest::option::of(string_regex("[a-zA-Z0-9 .,!?_-]{0,200}").expect("valid regex")),
        proptest::collection::vec(
            string_regex("[a-z][a-z0-9_-]{0,16}").expect("valid regex"),
            0..5,
        ),
        arb_agent_type(),
    )
        .prop_map(|(id, name, description, tags, agent_type)| AgentState {
            id,
            name,
            system: None,
            agent_type,
            llm_config: None,
            embedding_config: None,
            memory: None,
            tools: Vec::new(),
            sources: Vec::new(),
            tags,
            description,
            metadata: None,
            project_id: None,
            created_by_id: None,
            last_updated_by_id: None,
            created_at: None,
            updated_at: None,
            tool_rules: None,
            message_ids: Vec::new(),
            multi_agent_group: None,
            template_id: None,
            base_template_id: None,
            identity_ids: None,
            tool_exec_environment_variables: None,
            organization_id: None,
            timezone: None,
            last_run_completion: None,
            last_run_duration_ms: None,
            enable_sleeptime: None,
            response_format: None,
            message_buffer_autoclear: None,
            model: None,
            embedding: None,
            model_settings: None,
            secrets: Vec::new(),
            deployment_id: None,
            entity_id: None,
            identities: Vec::new(),
            managed_group: None,
            last_stop_reason: None,
            max_files_open: None,
            per_file_view_window_char_limit: None,
            hidden: None,
            blocks: Vec::new(),
        })
}

fn serialize_deserialize_serialize<T>(
    value: &T,
) -> Result<(serde_json::Value, serde_json::Value), String>
where
    T: Serialize + for<'de> Deserialize<'de>,
{
    let first = serde_json::to_value(value).map_err(|e| format!("serialize failed: {e}"))?;
    let roundtripped: T =
        serde_json::from_value(first.clone()).map_err(|e| format!("deserialize failed: {e}"))?;
    let second =
        serde_json::to_value(roundtripped).map_err(|e| format!("re-serialize failed: {e}"))?;
    Ok((first, second))
}

proptest! {
    #[test]
    fn letta_id_roundtrip(id in arb_letta_id()) {
        let result = serialize_deserialize_serialize(&id);
        prop_assert!(result.is_ok(), "{}", result.err().unwrap_or_default());
        if let Ok((first, second)) = result {
            prop_assert_eq!(first, second);
        }
    }

    #[test]
    fn agent_type_roundtrip(agent_type in arb_agent_type()) {
        let result = serialize_deserialize_serialize(&agent_type);
        prop_assert!(result.is_ok(), "{}", result.err().unwrap_or_default());
        if let Ok((first, second)) = result {
            prop_assert_eq!(first, second);
        }
    }

    #[test]
    fn timestamp_roundtrip(ts in arb_timestamp()) {
        let result = serialize_deserialize_serialize(&ts);
        prop_assert!(result.is_ok(), "{}", result.err().unwrap_or_default());
        if let Ok((first, second)) = result {
            prop_assert_eq!(first, second);
        }
    }

    #[test]
    fn block_roundtrip(block in arb_block()) {
        let result = serialize_deserialize_serialize(&block);
        prop_assert!(result.is_ok(), "{}", result.err().unwrap_or_default());
        if let Ok((first, second)) = result {
            prop_assert_eq!(first, second);
        }
    }

    #[test]
    fn tool_roundtrip(tool in arb_tool()) {
        let result = serialize_deserialize_serialize(&tool);
        prop_assert!(result.is_ok(), "{}", result.err().unwrap_or_default());
        if let Ok((first, second)) = result {
            prop_assert_eq!(first, second);
        }
    }

    #[test]
    fn passage_roundtrip(passage in arb_passage()) {
        let result = serialize_deserialize_serialize(&passage);
        prop_assert!(result.is_ok(), "{}", result.err().unwrap_or_default());
        if let Ok((first, second)) = result {
            prop_assert_eq!(first, second);
        }
    }

    #[test]
    fn agent_state_roundtrip(agent in arb_agent_state()) {
        let result = serialize_deserialize_serialize(&agent);
        prop_assert!(result.is_ok(), "{}", result.err().unwrap_or_default());
        if let Ok((first, second)) = result {
            prop_assert_eq!(first, second);
        }
    }

    #[test]
    fn option_agent_state_null_roundtrip(_ in Just(())) {
        let result: Result<Option<AgentState>, _> = serde_json::from_str("null");
        prop_assert!(result.is_ok());
        if let Ok(value) = result {
            prop_assert!(value.is_none());
        }
    }

    #[test]
    fn option_agent_state_some_roundtrip(agent in arb_agent_state()) {
        let some_agent = Some(agent.clone());
        let first = serde_json::to_value(&some_agent);
        prop_assert!(first.is_ok());
        if let Ok(first_json) = first {
            let back: Result<Option<AgentState>, _> = serde_json::from_value(first_json.clone());
            prop_assert!(back.is_ok());
            if let Ok(back_value) = back {
                prop_assert!(back_value.is_some());
                if let Some(ref parsed) = back_value {
                    prop_assert_eq!(agent.id.to_string(), parsed.id.to_string());
                }

                let second = serde_json::to_value(back_value);
                prop_assert!(second.is_ok());
                if let Ok(second_json) = second {
                    prop_assert_eq!(first_json, second_json);
                }
            }
        }
    }
}
