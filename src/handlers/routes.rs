use actix_web::web;

use crate::handlers::{
    // base
    index,
    raw_index,

    //about,
    toggle_language,
    toggle_language_index,
    toggle_language_two,
    toggle_language_three,

    // admin
    // errors
    internal_server_error,
    not_found,

    // auth
    login_form_input,
    login_handler,
    logout,

    // user
    user_by_id,

    // authorities
    authority_by_id,

    // conversion_request
    conversion_request_form,
    submit_document,
    validate_conversion,
    confirm_conversion,

    // analytics
    authority_analytics,

};

pub fn configure_services(config: &mut web::ServiceConfig) {
    config.service(index);
    config.service(raw_index);

    // login
    config.service(login_handler);
    config.service(login_form_input);
    config.service(logout);

    // user
    config.service(user_by_id);

    // authority
    config.service(authority_by_id);

    // conversion request
    config.service(conversion_request_form);
    config.service(submit_document);
    config.service(validate_conversion);
    config.service(confirm_conversion);

    // analytics
    config.service(authority_analytics);

    //config.service(about);
    config.service(toggle_language);
    config.service(toggle_language_index);
    config.service(toggle_language_two);
    config.service(toggle_language_three);

    // errors
    config.service(internal_server_error);
    config.service(not_found);

}
