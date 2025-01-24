use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings<D> {
    updated_at: String,
    template: String,
    settings: D
}
