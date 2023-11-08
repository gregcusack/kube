use k8s_openapi::api::core::v1::Pod;
use kube::{
    api::{Api, ListParams},
    Client,
};
use tracing::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let client = Client::try_default().await?;

    // Equivalent to `kubectl get pods --all-namespace \
    // -o jsonpath='{.items[*].spec.containers[*].image}'`
    let field_selector = std::env::var("FIELD_SELECTOR").unwrap_or_default();
    let jsonpath = format!(
        "{}{}",
        "$",
        std::env::var("JSONPATH").unwrap_or_else(|_| ".items[*].spec.containers[*].image".into())
    );

    let pods: Api<Pod> = Api::<Pod>::all(client);
    let list_params = ListParams::default().fields(&field_selector);
    let list = pods.list(&list_params).await?;

    // Use the given JSONPATH to filter the ObjectList
    let list_json = serde_json::to_value(&list)?;
    let res = jsonpath_lib::select(&list_json, &jsonpath).unwrap();
    info!("greg 1 res: \t\t {:?}", res);

    let json_str = list_json.to_string();
    let jsonpath = jsonpath_rust::JsonPathFinder::from_str(json_str.as_str(), jsonpath.as_str()).unwrap();
    let val: Vec<serde_json::Value> = jsonpath.find_slice()
            .into_iter()
            .filter(|v| v.has_value())
            .map(|v| v.to_data())
            .collect();
    info!("greg 1 val: \t\t {:?}", val);

    let val2: Vec<&serde_json::Value> = val.iter().map(|s| s).collect();

    assert_eq!(res, val2);


    Ok(())
}
