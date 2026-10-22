mod project;

use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::error::Error;
use crate::json;
use crate::wit::synt::extension::resources;

pub use project::Project;

#[derive(Debug, Clone, PartialEq)]
pub struct Resource {
    pub address: String,
    pub project: String,
    pub name: String,
    pub kind: String,
    pub fields: Value,
}

impl Resource {
    pub fn list(project: &str, kind: Option<&str>) -> Result<Vec<Self>, Error> {
        resources::listing(project, kind)?
            .into_iter()
            .map(Self::try_from)
            .collect()
    }

    pub fn get(address: &str) -> Result<Self, Error> {
        resources::get(address)?.try_into()
    }

    pub fn put(
        project: &str,
        name: &str,
        kind: &str,
        fields: &(impl Serialize + ?Sized),
    ) -> Result<Self, Error> {
        let fields = json::encode(fields)?;

        resources::put(project, name, kind, &fields)?.try_into()
    }

    pub fn remove(project: &str, name: &str) -> Result<(), Error> {
        Ok(resources::remove(project, name)?)
    }

    pub fn fields<T: DeserializeOwned>(&self) -> Result<T, Error> {
        Ok(T::deserialize(&self.fields)?)
    }
}

impl TryFrom<resources::Resource> for Resource {
    type Error = Error;

    fn try_from(resource: resources::Resource) -> Result<Self, Error> {
        Ok(Self {
            fields: json::value(&resource.fields)?,
            address: resource.address,
            project: resource.project,
            name: resource.name,
            kind: resource.kind,
        })
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use super::*;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Container {
        host: String,
        container: String,
    }

    #[test]
    fn reads_the_fields_of_a_resource() {
        let resource = Resource::try_from(resources::Resource {
            address: "@orders/web-1".to_owned(),
            project: "orders".to_owned(),
            name: "web-1".to_owned(),
            kind: "docker.container".to_owned(),
            fields: r#"{"host":"api-1","container":"web-1"}"#.to_owned(),
        })
        .unwrap();

        assert_eq!(resource.kind, "docker.container");
        assert_eq!(
            resource.fields::<Container>().unwrap(),
            Container {
                host: "api-1".to_owned(),
                container: "web-1".to_owned(),
            }
        );
    }

    #[test]
    fn refuses_fields_that_are_not_json() {
        let error = Resource::try_from(resources::Resource {
            address: "@orders/web-1".to_owned(),
            project: "orders".to_owned(),
            name: "web-1".to_owned(),
            kind: "host".to_owned(),
            fields: "{".to_owned(),
        })
        .unwrap_err();

        assert_eq!(error.code(), "invalid_json");
    }
}
