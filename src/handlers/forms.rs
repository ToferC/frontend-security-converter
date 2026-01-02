use serde::{Deserialize, Serialize};


#[derive(Deserialize, Debug)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct RegisterForm {
    user_name: String,
    email: String,
    password: String,
}

#[derive(Deserialize, Debug)]
pub struct VerifyForm {
    code: String,
}

#[derive(Deserialize, Debug)]
pub struct PasswordForm {
    password: String,
}

#[derive(Deserialize, Debug)]
pub struct DeleteForm {
    pub verify: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct CapabilityForm {
    pub name: String,
    pub level: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct DocumentSubmissionForm {
    #[serde(rename = "targetNations")]
    pub target_nations: Vec<String>,

    #[serde(rename = "releasableToOrganizations")]
    pub releasable_to_organizations: Vec<String>,

    #[serde(rename = "disclosureCategory")]
    pub disclosure_category: String,

    #[serde(rename = "handlingRestrictions")]
    pub handling_restrictions: Vec<String>,

    #[serde(rename = "handlingAuthority")]
    pub handling_authority: String,

    pub content: String,
}
