# Web Starter Rust

This repo an MVP for a triggger application that connects to https://github.com/ToferC/security_converter, currently live at https://security-converter.obrienlabs.dev/.

## The intent 

This application:
- [] Connects and authenticates a User via Azure SDK
- [] Validates the User and their Authority with the API, adding user, role and AUTH token header to a session
- [] Accepts a text communication or piece of data and uses a local Small Language Model to generate a ConversionRequest for the API
- [] Retreives the ConversionRequestResponse from the API along with the DataObject identifier and displays this to the User.

## The app will include

- [x] Actix-Web w/ async
- [x] Tera for templates
- [x] Authentication and sign-in
- [x] Static files
- [x] Fluent integration for i18n
- [ ] MCP connections to a Small Language Model
- [ ] Azure SDK links to authenticate
- [ ] 

## Setup
* Clone the repository
* Create `.env` file with the following environmental variables:
    * COOKIE_SECRET_KEY=MINIMUM32CHARACTERS
    * ENVIRONMENT=test
* Change APP_NAME const in lib.rs to your app
* `diesel migration run`
* `cargo run`
