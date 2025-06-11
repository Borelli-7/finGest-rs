// In SQLx 0.7, we need to connect to the database at compile time
// or create a mock database for testing.
fn main() {
    // Print this to show that the build script ran
    println!("cargo:rerun-if-env-changed=DATABASE_URL");
    println!("cargo:rerun-if-changed=src/db/migrations");
}
