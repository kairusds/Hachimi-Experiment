mod Socket;
mod Dns;

pub fn init() {
    get_assembly_image_or_return!(image, "System.dll");

    Socket::init(image);
    Dns::init(image);
}