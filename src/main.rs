use kube::ResourceExt;
use tracing_subscriber::EnvFilter;
use kube::Client;
use kube::api::{Api, DeleteParams};
use kube::runtime::{watcher, WatchStreamExt};
use k8s_openapi::api::core::v1::Node;
use futures::{StreamExt, TryStreamExt};
#[macro_use] extern crate tracing;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!(
        "libvirt_nodemanager {} {}",
        env!("CARGO_PKG_VERSION"),
        env!("GIT_HASH")
    );
    let client = Client::try_default().await.unwrap();
    let nodes: Api<Node> = Api::all(client);
    let _ = watcher(nodes, watcher::Config::default()).applied_objects()
        .try_for_each(|p: Node| async move {
            for taint in p.clone().spec.unwrap().taints.iter() {
                for f in taint.iter() {
                    if f.key == "DeletionCandidateOfClusterAutoscaler" || f.key == "ToBeDeletedByClusterAutoscaler"{
                        info!{"Node {} is candidate for deletion.", p.name_any()};
                        let conditions = p.status.clone().unwrap().conditions.unwrap();
                        for condition in conditions.iter() {
                            if condition.type_ == "Ready" {
                                if condition.status == "Unknown" {
                                    // double check that it is our node to edit
                                    if let Some(provider) = p.clone().spec.unwrap().provider_id {
                                        if provider.contains("libvirt") {
                                            info!("Found a node of ours that needs deletion.");
                                            let client = Client::try_default().await.unwrap();
                                            let nodes: Api<Node> = Api::all(client);
                                            if let Err(e) = nodes.delete(&p.name_any(),&DeleteParams::default()).await {
                                                error!("Couldn't delete node {}: {e}", p.name_any());
                                            }
                                        }
                                    } else {
                                        info!("providerId wasn't specified, so skipping delete of {}",p.name_any());
                                    }

                                }
                            }
                        }
                    }
                }
            }
            Ok(())
        }).await;
}
