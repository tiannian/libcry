var searchIndex = JSON.parse('{\
"cry":{"doc":"Cry is a advance cryptography library based on ECC and …","t":[0,3,12,11,0,3,12,12,12,12,3,12,12,3,12,0,0,6,8,16,10,10,8,10,0,8,16,10,10,10,10,10,10,10,10,0,8,18,16,10,10,10,10,10,10,10,11,3,12,11,11,11,0,8,18,10,10,10,10,10,10,10,11,3,12,11,11,0,8,0,3,12,3,12,0,3,12,12,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11],"n":["dh","SharedKey","key","new","keypair","Keypair","seed","code","secret","public","PublicKey","code","public","BarePublicKey","public","primitive","bytes","Output","Bytes","OutputSize","from_bytes","to_bytes","FromBytesRef","from_bytes_ref","digest","Digest","OutputSize","new","update","chain","finalize","finalize_reset","reset","output_size","digest","point","DisLogPoint","SIZE","Scalar","zero","one","basepoint","add","mul","neg","eq","sub","Point","0","zero","one","basepoint","scalar","ScalarNumber","SIZE","zero","one","invert","reduce","neg","add","mul","sub","Scalar","0","zero","one","sponge","Sponge","ristretto255","Point","0","Scalar","0","schnorr","Signature","R","s","sign","sign_multi_party","sign_multi_party_complete","verify","borrow","borrow_mut","try_from","from","into","try_into","type_id","to_owned","clone_into","borrow","borrow_mut","try_from","from","into","try_into","type_id","to_owned","clone_into","borrow","borrow_mut","try_from","from","into","try_into","type_id","borrow","borrow_mut","try_from","from","into","try_into","type_id","borrow","borrow_mut","try_from","from","into","try_into","type_id","to_owned","clone_into","borrow","borrow_mut","try_from","from","into","try_into","type_id","to_owned","clone_into","borrow","borrow_mut","try_from","from","into","try_into","type_id","to_owned","clone_into","borrow","borrow_mut","try_from","from","into","try_into","type_id","to_owned","clone_into","borrow","borrow_mut","try_from","from","into","try_into","type_id","to_owned","clone_into","to_bytes","from_bytes","to_bytes","from_bytes","from_bytes","to_bytes","to_bytes","from_bytes","from_bytes_ref","from_bytes_ref","zero","one","basepoint","add","mul","neg","eq","zero","one","invert","reduce","neg","add","mul","sub","fmt","fmt","fmt","fmt","fmt","fmt","fmt","sub","sub","sub","sub","eq","ne","eq","eq","ne","add","add","add","add","mul","mul","mul","mul","mul","mul","neg","neg","add_assign","add_assign","add_assign","sub_assign","sub_assign","sub_assign","mul_assign","mul_assign","mul_assign","clone","clone","clone","clone","clone","clone","clone","new","to_public","to_bare_public","derive","from_keypair","derive","to_bare_public","from_keypair"],"q":["cry","cry::dh","","","cry","cry::keypair","","","","","","","","","","cry","cry::primitive","cry::primitive::bytes","","","","","","","cry::primitive","cry::primitive::digest","","","","","","","","","","cry::primitive","cry::primitive::point","","","","","","","","","","","","","","","","cry::primitive","cry::primitive::scalar","","","","","","","","","","","","","","cry::primitive","cry::primitive::sponge","cry","cry::ristretto255","","","","cry","cry::schnorr","","","","","","","cry::dh","","","","","","","","","cry::keypair","","","","","","","","","","","","","","","","","","","","","","","cry::primitive::point","","","","","","","","","cry::primitive::scalar","","","","","","","","","cry::ristretto255","","","","","","","","","","","","","","","","","","cry::schnorr","","","","","","","","","cry::primitive::point","","cry::primitive::scalar","","cry::ristretto255","","","","cry::primitive::scalar","cry::ristretto255","","","","","","","","","","","","","","","","cry::dh","cry::keypair","cry::primitive::point","cry::primitive::scalar","cry::ristretto255","","cry::schnorr","cry::primitive::point","","cry::primitive::scalar","","cry::dh","","cry::primitive::point","cry::ristretto255","","cry::primitive::point","","cry::primitive::scalar","","cry::primitive::point","","cry::primitive::scalar","","","","cry::primitive::point","cry::primitive::scalar","cry::primitive::point","cry::primitive::scalar","","cry::primitive::point","cry::primitive::scalar","","cry::primitive::point","cry::primitive::scalar","","cry::dh","cry::keypair","cry::primitive::point","cry::primitive::scalar","cry::ristretto255","","cry::schnorr","cry::keypair","","","","","","",""],"d":["","","","","","","","","","","","","","","","Define primitives for cry.","Define serialize and deserialize behavior.","","Convert data format between bytes and struct.","","","","","","Define hash function behavior.","The <code>Digest</code> trait specifies an interface common for digest …","Output size for <code>Digest</code>","Create new hasher instance","Digest data, updating the internal state.","Digest input data in a chained manner.","Retrieve result and consume hasher instance.","Retrieve result and reset hasher instance.","Reset hasher instance to its initial state.","Get output size of the hasher","Convenience function to compute hash of the <code>data</code>. It will …","Define Point behavier.","Point trait.","","","","","","","","","","","Point.","","","","","Define scalar behavior.","Trait for scalar.","","","","","","","","","","Scalar types.","","","","Define sponge function behavior.","","","","","","","Schnorr signature and schnorr multi-signature.","Schorr Signature","","","Create signature from keypair and message.","","","Verify signature","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","",""],"i":[0,0,1,1,0,0,2,2,2,2,0,3,3,0,4,0,0,0,0,5,5,5,0,6,0,0,7,7,7,7,7,7,7,7,7,0,0,8,8,8,8,8,8,8,8,8,8,0,9,9,9,9,0,0,10,10,10,10,10,10,10,10,10,0,11,11,11,0,0,0,0,12,0,13,0,0,14,14,14,14,14,14,1,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,3,3,3,3,3,3,3,4,4,4,4,4,4,4,9,9,9,9,9,9,9,9,9,11,11,11,11,11,11,11,11,11,12,12,12,12,12,12,12,12,12,13,13,13,13,13,13,13,13,13,14,14,14,14,14,14,14,14,14,9,9,11,11,12,12,13,13,11,13,12,12,12,12,12,12,12,13,13,13,13,13,13,13,13,1,2,9,11,12,13,14,9,9,11,11,1,1,9,12,12,9,9,11,11,9,9,11,11,11,11,9,11,9,11,11,9,11,11,9,11,11,1,2,9,11,12,13,14,2,2,2,2,3,3,3,4],"f":[null,null,null,[[["barepublickey",3],["keypair",3]]],null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,null,[[["output",6]]],[[],["output",6]],null,[[],["option",4]],null,null,null,[[]],[[]],[[]],[[],[["u8",15],["genericarray",3]]],[[],[["u8",15],["genericarray",3]]],[[]],[[],["usize",15]],[[],[["u8",15],["genericarray",3]]],null,null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[],["bool",15]],[[]],null,null,[[]],[[]],[[]],null,null,null,[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],null,null,[[]],[[]],null,null,null,null,null,null,null,null,null,null,null,[[["keypair",3]],["signature",3]],[[["keypair",3]]],[[]],[[["barepublickey",3]],["bool",15]],[[]],[[]],[[],["result",4]],[[]],[[]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[]],[[]],[[],["result",4]],[[]],[[]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[]],[[]],[[],["result",4]],[[]],[[]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[],["result",4]],[[]],[[]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[],["result",4]],[[]],[[]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[]],[[]],[[],["result",4]],[[]],[[]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[]],[[]],[[],["result",4]],[[]],[[]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[]],[[]],[[],["result",4]],[[]],[[]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[]],[[]],[[],["result",4]],[[]],[[]],[[],["result",4]],[[],["typeid",3]],[[]],[[]],[[],["output",6]],[[["output",6]]],[[],["output",6]],[[["output",6]]],[[["output",6]]],[[],["output",6]],[[],["output",6]],[[["output",6]]],[[],["option",4]],[[],["option",4]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["bool",15]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["formatter",3]],["result",6]],[[["point",3]],["point",3]],[[["point",3]],["point",3]],[[["scalar",3]],["scalar",3]],[[["scalar",3]],["scalar",3]],[[["sharedkey",3]],["bool",15]],[[["sharedkey",3]],["bool",15]],[[],["bool",15]],[[["point",3]],["bool",15]],[[["point",3]],["bool",15]],[[["point",3]],["point",3]],[[["point",3]],["point",3]],[[["scalar",3]],["scalar",3]],[[["scalar",3]],["scalar",3]],[[["scalar",3]],["point",3]],[[["scalar",3]],["point",3]],[[["scalar",3]],["scalar",3]],[[["scalar",3]],["scalar",3]],[[["point",3]],["point",3]],[[["point",3]],["point",3]],[[]],[[],["scalar",3]],[[["point",3]]],[[["scalar",3]]],[[["scalar",3]]],[[["point",3]]],[[["scalar",3]]],[[["scalar",3]]],[[["scalar",3]]],[[["scalar",3]]],[[["scalar",3]]],[[],["sharedkey",3]],[[],["keypair",3]],[[],["point",3]],[[],["scalar",3]],[[],["point",3]],[[],["scalar",3]],[[],["signature",3]],[[["output",6]]],[[],["publickey",3]],[[],["barepublickey",3]],[[["output",6]]],[[["keypair",3]]],[[["output",6]]],[[],["barepublickey",3]],[[["keypair",3]]]],"p":[[3,"SharedKey"],[3,"Keypair"],[3,"PublicKey"],[3,"BarePublicKey"],[8,"Bytes"],[8,"FromBytesRef"],[8,"Digest"],[8,"DisLogPoint"],[3,"Point"],[8,"ScalarNumber"],[3,"Scalar"],[3,"Point"],[3,"Scalar"],[3,"Signature"]]}\
}');
initSearch(searchIndex);