/// A logging [Backend] initializes and provides the actual logging functionality
pub trait Backend {
    /// The [BackendBuilder] for [Self]
    type Builder: BackendBuilder;

    /// Given logging options parsed from the commandline, initialize logging
    fn init_from_options<Opts>(opts: &Opts)
    where
        Opts: LoggingOptions<Self>,
    {
        opts.configure(Self::builder()).init();
    }

    /// Get a default-configured builder
    fn builder() -> Self::Builder;
}

/// A [BackendBuilder] can initialize the backend
pub trait BackendBuilder {
    /// Initialize logging for this backend
    fn init(self);
}

/// A set of commandline options which can configure the backend
pub trait LoggingOptions<B>
where
    B: ?Sized + Backend,
{
    /// Configure `B`
    fn configure(&self, builder: B::Builder) -> B::Builder;
}
