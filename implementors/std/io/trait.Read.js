(function() {var implementors = {};
implementors["bytes"] = ["impl&lt;B:&nbsp;<a class='trait' href='bytes/buf/trait.Buf.html' title='bytes::buf::Buf'>Buf</a> + <a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html' title='core::marker::Sized'>Sized</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='bytes/buf/struct.Reader.html' title='bytes::buf::Reader'>Reader</a>&lt;B&gt;",];
implementors["libc"] = [];
implementors["tokio_io"] = ["impl&lt;T:&nbsp;<a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a>, B:&nbsp;<a class='trait' href='bytes/buf/into_buf/trait.IntoBuf.html' title='bytes::buf::into_buf::IntoBuf'>IntoBuf</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='tokio_io/codec/length_delimited/struct.FramedWrite.html' title='tokio_io::codec::length_delimited::FramedWrite'>FramedWrite</a>&lt;T, B&gt;","impl&lt;T:&nbsp;<a class='trait' href='tokio_io/trait.AsyncRead.html' title='tokio_io::AsyncRead'>AsyncRead</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='tokio_io/io/struct.ReadHalf.html' title='tokio_io::io::ReadHalf'>ReadHalf</a>&lt;T&gt;",];
implementors["tokio_netstring"] = ["impl&lt;T:&nbsp;<a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a>, B:&nbsp;<a class='trait' href='bytes/buf/into_buf/trait.IntoBuf.html' title='bytes::buf::into_buf::IntoBuf'>IntoBuf</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/std/io/trait.Read.html' title='std::io::Read'>Read</a> for <a class='struct' href='tokio_netstring/struct.FramedWrite.html' title='tokio_netstring::FramedWrite'>FramedWrite</a>&lt;T, B&gt;",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
