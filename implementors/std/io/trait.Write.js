(function() {var implementors = {};
implementors["bytes"] = ["impl&lt;B:&nbsp;<a class='trait' href='bytes/buf/trait.BufMut.html' title='bytes::buf::BufMut'>BufMut</a> + <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html' title='core::marker::Sized'>Sized</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='bytes/buf/struct.Writer.html' title='bytes::buf::Writer'>Writer</a>&lt;B&gt;",];
implementors["libc"] = [];
implementors["tokio_io"] = ["impl&lt;T:&nbsp;<a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='tokio_io/codec/length_delimited/struct.FramedRead.html' title='tokio_io::codec::length_delimited::FramedRead'>FramedRead</a>&lt;T&gt;","impl&lt;T:&nbsp;<a class='trait' href='tokio_io/trait.AsyncWrite.html' title='tokio_io::AsyncWrite'>AsyncWrite</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='tokio_io/io/struct.WriteHalf.html' title='tokio_io::io::WriteHalf'>WriteHalf</a>&lt;T&gt;",];
implementors["tokio_netstring"] = ["impl&lt;T:&nbsp;<a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Write.html' title='std::io::Write'>Write</a> for <a class='struct' href='tokio_netstring/struct.FramedRead.html' title='tokio_netstring::FramedRead'>FramedRead</a>&lt;T&gt;",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
