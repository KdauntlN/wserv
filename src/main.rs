use wserv::args;

fn main() {
    let function = args::get_function();
    function();
}
