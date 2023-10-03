use cgp_async::Async;

pub trait DelegateComponent<Name>: Async {
    type Delegate;
}
