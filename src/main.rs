pub mod performance;
pub mod user;

fn main() {
    user::fetch_user();
    //performance::benchmarker();
}