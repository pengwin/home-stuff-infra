use auth_service::ApiDoc;
use service_core::open_api::save_spec::save_spec;

fn main() {
    save_spec::<ApiDoc>("auth-service.openapi.spec.json").expect("Unable to write spec file");
    //println!("cargo:rerun-if-changed=build.rs");
}
