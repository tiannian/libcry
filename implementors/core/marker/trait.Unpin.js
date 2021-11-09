(function() {var implementors = {};
implementors["cry"] = [{"text":"impl&lt;P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for <a class=\"struct\" href=\"cry/dh/struct.SharedKey.html\" title=\"struct cry::dh::SharedKey\">SharedKey</a>&lt;P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;P as <a class=\"trait\" href=\"cry/primitive/bytes/trait.Bytes.html\" title=\"trait cry::primitive::bytes::Bytes\">Bytes</a>&gt;::<a class=\"type\" href=\"cry/primitive/bytes/trait.Bytes.html#associatedtype.OutputSize\" title=\"type cry::primitive::bytes::Bytes::OutputSize\">OutputSize</a> as ArrayLength&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.56.1/std/primitive.u8.html\">u8</a>&gt;&gt;::ArrayType: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,&nbsp;</span>","synthetic":true,"types":["cry::dh::SharedKey"]},{"text":"impl&lt;P, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for <a class=\"struct\" href=\"cry/key/struct.Keypair.html\" title=\"struct cry::key::Keypair\">Keypair</a>&lt;P, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;S as <a class=\"trait\" href=\"cry/primitive/bytes/trait.Bytes.html\" title=\"trait cry::primitive::bytes::Bytes\">Bytes</a>&gt;::<a class=\"type\" href=\"cry/primitive/bytes/trait.Bytes.html#associatedtype.OutputSize\" title=\"type cry::primitive::bytes::Bytes::OutputSize\">OutputSize</a> as ArrayLength&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.56.1/std/primitive.u8.html\">u8</a>&gt;&gt;::ArrayType: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,&nbsp;</span>","synthetic":true,"types":["cry::key::keypair::Keypair"]},{"text":"impl&lt;P, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for <a class=\"struct\" href=\"cry/key/struct.PublicKey.html\" title=\"struct cry::key::PublicKey\">PublicKey</a>&lt;P, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;&lt;S as <a class=\"trait\" href=\"cry/primitive/bytes/trait.Bytes.html\" title=\"trait cry::primitive::bytes::Bytes\">Bytes</a>&gt;::<a class=\"type\" href=\"cry/primitive/bytes/trait.Bytes.html#associatedtype.OutputSize\" title=\"type cry::primitive::bytes::Bytes::OutputSize\">OutputSize</a> as ArrayLength&lt;<a class=\"primitive\" href=\"https://doc.rust-lang.org/1.56.1/std/primitive.u8.html\">u8</a>&gt;&gt;::ArrayType: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,&nbsp;</span>","synthetic":true,"types":["cry::key::public_key::PublicKey"]},{"text":"impl&lt;P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for <a class=\"struct\" href=\"cry/key/struct.BarePublicKey.html\" title=\"struct cry::key::BarePublicKey\">BarePublicKey</a>&lt;P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,&nbsp;</span>","synthetic":true,"types":["cry::key::bare_public::BarePublicKey"]},{"text":"impl&lt;P&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for <a class=\"struct\" href=\"cry/primitive/point/struct.Point.html\" title=\"struct cry::primitive::point::Point\">Point</a>&lt;P&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,&nbsp;</span>","synthetic":true,"types":["cry::primitive::point::Point"]},{"text":"impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for <a class=\"struct\" href=\"cry/primitive/scalar/struct.Scalar.html\" title=\"struct cry::primitive::scalar::Scalar\">Scalar</a>&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,&nbsp;</span>","synthetic":true,"types":["cry::primitive::scalar::Scalar"]},{"text":"impl&lt;P, S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for <a class=\"struct\" href=\"cry/schnorr/struct.Signature.html\" title=\"struct cry::schnorr::Signature\">Signature</a>&lt;P, S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;P: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a>,&nbsp;</span>","synthetic":true,"types":["cry::schnorr::Signature"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for <a class=\"struct\" href=\"cry/ristretto255/struct.Point.html\" title=\"struct cry::ristretto255::Point\">Point</a>","synthetic":true,"types":["cry::ristretto255::point::Point"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/1.56.1/core/marker/trait.Unpin.html\" title=\"trait core::marker::Unpin\">Unpin</a> for <a class=\"struct\" href=\"cry/ristretto255/struct.Scalar.html\" title=\"struct cry::ristretto255::Scalar\">Scalar</a>","synthetic":true,"types":["cry::ristretto255::scalar::Scalar"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()