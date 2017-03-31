(function() {var implementors = {};
implementors["bytes"] = [];
implementors["futures"] = ["impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/error/trait.Error.html' title='std::error::Error'>Error</a> for <a class='struct' href='futures/sync/oneshot/struct.Canceled.html' title='futures::sync::oneshot::Canceled'>Canceled</a>","impl&lt;T:&nbsp;<a class='trait' href='https://doc.rust-lang.org/nightly/core/any/trait.Any.html' title='core::any::Any'>Any</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/error/trait.Error.html' title='std::error::Error'>Error</a> for <a class='struct' href='futures/sync/mpsc/struct.SendError.html' title='futures::sync::mpsc::SendError'>SendError</a>&lt;T&gt;","impl&lt;T:&nbsp;<a class='trait' href='https://doc.rust-lang.org/nightly/core/any/trait.Any.html' title='core::any::Any'>Any</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/error/trait.Error.html' title='std::error::Error'>Error</a> for <a class='struct' href='futures/unsync/mpsc/struct.SendError.html' title='futures::unsync::mpsc::SendError'>SendError</a>&lt;T&gt;",];
implementors["libc"] = [];
implementors["log"] = ["impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/error/trait.Error.html' title='std::error::Error'>Error</a> for <a class='struct' href='log/struct.SetLoggerError.html' title='log::SetLoggerError'>SetLoggerError</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/std/error/trait.Error.html' title='std::error::Error'>Error</a> for <a class='struct' href='log/struct.ShutdownLoggerError.html' title='log::ShutdownLoggerError'>ShutdownLoggerError</a>",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
