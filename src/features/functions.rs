use crate::AppError;
use crate::entities::admin::feature;
use crate::entities::auth::user;
use crate::enums::features::Feature;

#[derive(Clone)]
pub struct FeatureManager {
    pub flags: Vec<feature::Model>,
}

impl FeatureManager {
    pub fn is_enabled(&self, feature: Feature, _user: Option<&user::Model>, default: bool) -> bool {
        let find = self.flags.iter().find(|f| f.name == feature.to_string());

        if let Some(find) = find {
            return find.value.parse::<bool>().unwrap_or(default);
        }

        default
    }

    pub fn enabled(&self, feature: Feature, _user: Option<&user::Model>) -> Result<(), AppError> {
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
