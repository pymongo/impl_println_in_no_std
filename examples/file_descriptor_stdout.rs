extern crate unsafe_redis_client_no_std_no_dependencies;
use unsafe_redis_client_no_std_no_dependencies::FileDescriptor;

fn main() {
    let stdout = FileDescriptor::new_stdout();
    stdout.print_hello_world();
}