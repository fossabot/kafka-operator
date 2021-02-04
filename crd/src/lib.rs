use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use stackable_operator::CRD;

#[derive(Clone, CustomResource, Debug, Deserialize, JsonSchema, Serialize)]
#[kube(
    group = "kafka.stackable.de",
    version = "v1",
    kind = "KafkaCluster",
    shortname = "kafka",
    namespaced
)]
#[kube(status = "KafkaClusterStatus")]
#[serde(rename_all = "camelCase")]
pub struct KafkaClusterSpec {
    pub version: KafkaVersion,
    pub brokers: Vec<KafkaBroker>,
    pub zoo_keeper_reference: String,
}

impl CRD for KafkaCluster {
    const RESOURCE_NAME: &'static str = "kafkaclusters.kafka.stackable.de";
    const CRD_DEFINITION: &'static str = include_str!("../kafkaclusters.crd.yaml");
}

#[allow(non_camel_case_types)]
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
pub enum KafkaVersion {
    #[serde(rename = "2.6.0")]
    v2_6_0,
}

#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
pub struct KafkaBroker {
    pub node_name: String,
}

#[derive(Clone, Debug, Default, Deserialize, JsonSchema, Serialize)]
pub struct KafkaClusterStatus {}
