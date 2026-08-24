use crate::{
    AppError,
    entities::{admin::feature, auth::user},
    enums::features::Feature,
};

#[derive(Clone)]
pub struct FeatureManager {
    pub flags: Vec<feature::Model>,
}

impl FeatureManager {
    pub async fn enabled(
        &self,
        feature: Feature,
        _user: Option<&user::Model>,
    ) -> Result<(), AppError> {
        let find = self.flags.iter().find(|f| f.name == feature.to_string());

        if let Some(find) = find {
            if find.value.parse::<bool>().unwrap_or(false) == false {
                return Err(AppError::ForbiddenWithMessage(
                    "Feature disabled".to_string(),
                ));
            }
        }

        Ok(())
    }
}
