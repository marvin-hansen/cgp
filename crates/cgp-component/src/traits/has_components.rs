use cgp_async::Async;

pub trait HasComponents: Async {
    type Components: Async;
}
