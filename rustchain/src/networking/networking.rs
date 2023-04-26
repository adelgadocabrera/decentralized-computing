// TO DO: implement! How do we get ip? We want to make this work
// not only in localhost or private network but for any exposed node.
// Hole punching?
pub fn get_self_ip() -> String {
    return String::from("[::1]");
}

pub fn get_self_port() -> u32 {
    return 5002;
}
