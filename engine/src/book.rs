use crate::moves::Move;
use std::collections::HashMap;
use std::sync::OnceLock;

/// Minimal solution book for 1.e3 — only reachable positions.
/// Cleaned by reachable-extractor from 4890 entries.
const BOOK_DATA: &[(u64, &str)] = &[
    (0x3eb9976bbb8190ba, "b1c3"),   // mate in 15 half-moves
    (0x87834ea2f65f083c, "c3e4"),   // mate in 13 half-moves
    (0xc8b76cd20b758f82, "d1g4"),   // Qg4! Nf6 f3 reclaims, Qe6 Nd6# (mate in 4),   // mate in 11 half-moves
    (0xfb86e6fdac38074c, "c3e4"),   // mate in 11 half-moves
    (0x5d86cb23e1373087, "c3e4"),   // mate in 11 half-moves
    (0x3e3d207bca803998, "c3e4"),   // mate in 11 half-moves
    (0xdff778d0937f9ce7, "c3e4"),   // mate in 11 half-moves
    (0x40519592c3908c73, "c3e4"),   // mate in 11 half-moves
    (0x0e6eff56abfaefcb, "c3e4"),   // mate in 11 half-moves
    (0xaba1d1c2a65332db, "f1b5"),   // mate in 9 half-moves
    (0x762a14aeb9c2c9d9, "f1d3"),   // mate in 9 half-moves
    (0xe66f1937358074fe, "c3e4"),   // mate in 9 half-moves
    (0x6cf71439709942a8, "c3e4"),   // mate in 7 half-moves
    (0x1ac47050bf6952c4, "f2f3"),   // mate in 7 half-moves
    (0xb412732c8ac1e223, "d1g4"),   // Nf6: Qg4 reclaims e4 knight + flips g7/d7,   // mate in 7 half-moves
    (0xe7e25bfe05a091ce, "c3e4"),   // mate in 5 half-moves
    (0x72d3a64178924eb4, "c3e4"),   // mate in 5 half-moves
    (0xfc4ba3cb2fb149c1, "c3e4"),   // mate in 5 half-moves
    (0x8f5ebfe679b30b26, "e4c3"),   // mate in 5 half-moves
    (0x3c5ed30f9a55d569, "b2b3"),   // mate in 5 half-moves
    (0x5dc5c5bdcb16a6b4, "c3e4"),   // mate in 5 half-moves
    (0x18a1bd2b45b5ec94, "f1d3"),   // mate in 5 half-moves
    (0xcd5da18db7b72b72, "c3e4"),   // mate in 5 half-moves
    (0x6dc0868688350e60, "c3e4"),   // mate in 5 half-moves
    (0x5f6629b70c07a8b7, "d1g4"),   // Qg4! reclaims Ne4, mate in 2-3,   // Nf6: Qg4 reclaims e4 knight + flips g7/d7,   // mate in 3 half-moves
    (0x0dac1df5b61ed387, "f1d3"),   // mate in 3 half-moves
    (0xec66455eefe176f8, "d1g4"),   // Qg4! reclaims Ne4, mate in 2-3,   // Nf6: Qg4 reclaims e4 knight + flips g7/d7,   // mate in 3 half-moves
    (0x55cb1959ab705223, "d1g4"),   // mate in 3 half-moves
    (0xcfda9e45532fa3de, "f2f3"),   // mate in 3 half-moves
    (0xd4736670793e7bd1, "f2f3"),   // mate in 3 half-moves
    (0x5e51bb08f4abe47f, "d2d3"),   // mate in 3 half-moves
    (0x851806f7b80d1ab4, "c3e4"),   // mate in 3 half-moves
    (0x8a8400f9fb0211ae, "c3e4"),   // mate in 3 half-moves
    (0x38a0d78a9a5fa6a4, "d2d3"),   // mate in 3 half-moves
    (0x6e54f833b7884cab, "f2f3"),   // mate in 3 half-moves
    (0xd2e31faee435a0f8, "d2d3"),   // mate in 3 half-moves
    (0x987ddd65481e874a, "f2f3"),   // mate in 3 half-moves
    (0xfecc9c03cb29c16d, "f2f3"),   // mate in 3 half-moves
    (0xf7cd69d165c06daf, "f1c4"),   // mate in 3 half-moves
    (0x9f5780b21e5b25d2, "e4d6"),   // mate in 1 half-moves
    (0xef353c72b51a2a5c, "e4d6"),   // mate in 1 half-moves
    (0x9f897be6bac7c95f, "e4d6"),   // mate in 1 half-moves
    (0xc2113bdde529ecb9, "e4d6"),   // mate in 1 half-moves
    (0xe9506536a663f4be, "e4d6"),   // mate in 1 half-moves
    (0x696558d6440d387c, "e4d6"),   // mate in 1 half-moves
    (0x943244e6070c0180, "e4d6"),   // mate in 1 half-moves
    (0xd3d48d111c99ec30, "f1b5"),   // mate in 1 half-moves
    (0x54c711a67a62d43c, "e4d6"),   // mate in 1 half-moves
    (0x79d9993f0dd8f2a5, "f1b5"),   // mate in 1 half-moves
    (0x4953418972e4fefb, "e4d6"),   // mate in 1 half-moves
    (0xe89e2961bc6287bd, "e4d6"),   // mate in 1 half-moves
    (0xa3df1c5067769286, "e4d6"),   // mate in 1 half-moves
    (0x63c86838e37ac2e8, "e4d6"),   // mate in 1 half-moves
    (0x9f865c8dacfc54f8, "e4d6"),   // mate in 1 half-moves
    (0x9af8cb1a4f1132c9, "e4d6"),   // mate in 1 half-moves
    (0x80716d29148a3d18, "e4d6"),   // mate in 1 half-moves
    (0xf54977d09ec53b49, "e4d6"),   // mate in 1 half-moves
    (0xaf7760da1b8b6cd9, "e4d6"),   // mate in 1 half-moves
    (0x6d4f1d0bb42aaaea, "e4f6"),   // mate in 1 half-moves
    (0x58c1c2d669a03f56, "f1b5"),   // mate in 1 half-moves
    (0xd89b6a5aff412f69, "e4d6"),   // mate in 1 half-moves
    (0x34af63ed388e2492, "e4d6"),   // mate in 1 half-moves
    (0xd9b8c3e8cf3834b6, "e4d6"),   // mate in 1 half-moves
    (0x7861900dc96b1ae7, "e4d6"),   // mate in 1 half-moves
    (0x9cc6bfc431bce03c, "e4d6"),   // mate in 1 half-moves
    (0x0cb81fb2428a610c, "e4d6"),   // mate in 1 half-moves
    (0x6c37d850c6bbefeb, "e4d6"),   // mate in 1 half-moves
    (0xa53b11d1d7d82a06, "f1b5"),   // mate in 1 half-moves
    (0xc47ba654254adf55, "e4d6"),   // mate in 1 half-moves
    (0xbcb0c6746d1deeae, "g8f6"),   // mate in 1 half-moves
    (0xb069276adda4b61b, "e4d6"),   // mate in 1 half-moves
    (0x43a944f43767c54b, "e4d6"),   // mate in 1 half-moves
    (0x4f6ee99350730c33, "e4d6"),   // mate in 1 half-moves
    (0xef1062c915ca5f03, "e4d6"),   // mate in 1 half-moves
    (0x41ea8ce847a5daf0, "e4f6"),   // mate in 1 half-moves
    (0x66452275c16016fb, "e4d6"),   // mate in 1 half-moves
    (0xd9cb25b90e45733d, "e4d6"),   // mate in 1 half-moves
    (0xae0c7cb7757ab0f6, "f1b5"),   // mate in 1 half-moves
    (0xd342c6958abb0f06, "e4d6"),   // mate in 1 half-moves
    (0xb69dde8060e71ec1, "e4d6"),   // mate in 1 half-moves
    (0xa0f11f7a8c17dbea, "e4d6"),   // mate in 1 half-moves
    (0x8b25c23997d58ea2, "e4d6"),   // mate in 1 half-moves
    (0xbb208102d4f62676, "e4d6"),   // mate in 1 half-moves
    (0x7defb4a9e855f201, "d1h5"),   // mate in 1 half-moves
    (0x2befd1ca3a3fc1ff, "c4b3"),   // mate in 1 half-moves
    (0x72cca0e36e1ce073, "e4d6"),   // mate in 1 half-moves
    (0xd7d62e090fbb198c, "e4d6"),   // mate in 1 half-moves
    (0x82201a3815a96679, "e4d6"),   // mate in 1 half-moves
    (0x10b33f6ffe407083, "f1b5"),   // mate in 1 half-moves
    (0x6aef9a92ce2a2bdd, "e4d6"),   // mate in 1 half-moves
    (0x2605bab01c469307, "e4d6"),   // mate in 1 half-moves
    (0xe6c02c4f97bd7b3f, "e4d6"),   // mate in 1 half-moves
    (0x4703b47a803d63dd, "d3b5"),   // mate in 1 half-moves
    (0x99dfc1fbc179076c, "e4d6"),   // mate in 1 half-moves
    (0x5377fb7568d5c3c4, "e4d6"),   // mate in 1 half-moves
    (0x9315ca964fad5073, "e4d6"),   // mate in 1 half-moves
    (0xbc35a0fd06091c79, "e4d6"),   // mate in 1 half-moves
    (0xae312eafa2929617, "e4d6"),   // mate in 1 half-moves
    (0xe441968f632988a2, "e4d6"),   // mate in 1 half-moves
    (0xc8c302b5f039e63d, "e4d6"),   // mate in 1 half-moves
    (0x53f80173e4ce32a2, "e4d6"),   // mate in 1 half-moves
    (0x4977924b516d7854, "e4d6"),   // mate in 1 half-moves
    (0x7fee6722407d755e, "e4d6"),   // mate in 1 half-moves
    (0x348bb02f1b07a23d, "e4d6"),   // mate in 1 half-moves
    (0x4e9e04bff16db076, "e4d6"),   // mate in 1 half-moves
    (0xa3fa42ebc7a6e7d9, "e4d6"),   // mate in 1 half-moves
    (0x27e26e112ff25ae4, "g8f6"),   // mate in 1 half-moves
    (0x0f99a180e5740e40, "e4d6"),   // mate in 1 half-moves
    (0x9c2224f3d2f0a47d, "e4d6"),   // mate in 1 half-moves
    (0x29cba5a3e15f6090, "e4d6"),   // mate in 1 half-moves
    (0x2e15daa5c9ba36fa, "e4d6"),   // mate in 1 half-moves
    (0x01b51860db552bca, "e4d6"),   // mate in 1 half-moves
    (0x60688d7bc6dc1b6e, "e4d6"),   // mate in 1 half-moves
    (0x1f9ddb57bab97b1e, "e4d6"),   // mate in 1 half-moves
    (0x94d88fa0cc241efe, "e4d6"),   // mate in 1 half-moves
    (0x974ff9fea57f87aa, "e4d6"),   // mate in 1 half-moves
    (0xfdf3aaf9eb02a183, "e4f6"),   // mate in 1 half-moves
    (0x28412096c210eaa8, "e4d6"),   // mate in 1 half-moves
    (0x3e44130e7441b019, "e4d6"),   // mate in 1 half-moves
    (0x0e025eab1d438efe, "e4d6"),   // mate in 1 half-moves
    (0xe6dadeda2d7b9a2a, "e4d6"),   // mate in 1 half-moves
    (0x1bb900118b0acc92, "e4d6"),   // mate in 1 half-moves
    (0x92ec40ad5fa0856a, "e4d6"),   // mate in 1 half-moves
    (0x39c6dc6927f956f8, "g8f6"),   // mate in 1 half-moves
    (0x632cf30f003686a9, "e4d6"),   // mate in 1 half-moves
    (0xec72a0a74b959892, "e4d6"),   // mate in 1 half-moves
    (0x20df555981a705b3, "e4d6"),   // mate in 1 half-moves
    (0xf7ce073866e4515a, "e4d6"),   // mate in 1 half-moves
    (0xb9a357f24ca98393, "e4d6"),   // mate in 1 half-moves
    (0xbb761d14f6af58f1, "e4d6"),   // mate in 1 half-moves
    (0x4f8a72a4b33f4872, "e4d6"),   // mate in 1 half-moves
    (0x26dd797790c569ac, "e4d6"),   // mate in 1 half-moves
    (0x98443ed0394e87d2, "e4d6"),   // mate in 1 half-moves
    (0x08b70373b65724d7, "g8f6"),   // mate in 1 half-moves
    (0x7facd53b4ae2546a, "e4d6"),   // mate in 1 half-moves
    (0x2d66e179f0fb2f5a, "e4d6"),   // mate in 1 half-moves
    (0x4d5f384209399310, "e4d6"),   // mate in 1 half-moves
    (0xd6f6700f7e31ce52, "e4d6"),   // mate in 1 half-moves
    (0xd145461830bcf997, "e4d6"),   // mate in 1 half-moves
    (0x05788de80befb70b, "e4d6"),   // mate in 1 half-moves
    (0xa4d39ceaa763ed15, "e4d6"),   // mate in 1 half-moves
    (0x2bee79248a0ed537, "e4d6"),   // mate in 1 half-moves
    (0x2e9296c799836c8d, "e4d6"),   // mate in 1 half-moves
    (0xe34785aec998a7d2, "e4d6"),   // mate in 1 half-moves
    (0x4d38cf10b1687a7f, "e4d6"),   // mate in 1 half-moves
    (0x8e100c24eeac1f0d, "e4d6"),   // mate in 1 half-moves
    (0x6dac277b3e8c6f55, "e4d6"),   // mate in 1 half-moves
    (0x9521059bdc29cbe7, "e4d6"),   // mate in 1 half-moves
    (0x7d3d8c2bf24bd2ab, "e4d6"),   // mate in 1 half-moves
    (0x293b33c230080e55, "e4d6"),   // mate in 1 half-moves
    (0xb4c39c7a0965aae2, "e4d6"),   // mate in 1 half-moves
    (0xf4b99afc3fdb870c, "e4d6"),   // mate in 1 half-moves
    (0x52fab9bc58f526f4, "e4d6"),   // mate in 1 half-moves
    (0x1687f0626dc4eb17, "e4f6"),   // mate in 1 half-moves
    (0x802b5b5f82e1fd45, "f1b5"),   // mate in 1 half-moves
    (0x1af87b621208b784, "e4d6"),   // mate in 1 half-moves
    (0xfc8307480fc80ec1, "e4d6"),   // mate in 1 half-moves
    (0xf692139a4ffea835, "e4d6"),   // mate in 1 half-moves
    (0x08a2a8b1df5d4309, "e4d6"),   // mate in 1 half-moves
    (0x5aead9a98d098309, "e4d6"),   // mate in 1 half-moves
    (0xe85c0d4f86342c34, "e4d6"),   // mate in 1 half-moves
    (0x62452af1d35a27d8, "e4d6"),   // mate in 1 half-moves
    (0xa5ba4514740f4c64, "e4d6"),   // mate in 1 half-moves
    (0x3fdfec258c7630a7, "e4d6"),   // mate in 1 half-moves
    (0x31025c7a5963b9d8, "e4d6"),   // mate in 1 half-moves
    (0xeb7bec50d646db87, "e4d6"),   // mate in 1 half-moves
    (0xe8aec1ae573e5605, "e4d6"),   // mate in 1 half-moves
    (0x6aa60d2cb9d2be6c, "e4d6"),   // mate in 1 half-moves
    (0x2ef141922af672bb, "e4d6"),   // mate in 1 half-moves
    (0xf0bfcf08d5826b17, "e4d6"),   // mate in 1 half-moves
    (0x62fffa871bd68e20, "e4d6"),   // mate in 1 half-moves
    (0x2e121254aae0c5f9, "d1h5"),   // mate in 1 half-moves
    (0xd9eff67b2dccf592, "e4d6"),   // mate in 1 half-moves
    (0xcb61fce42a8dc4a8, "e4d6"),   // mate in 1 half-moves
    (0x3106fa62391d762e, "e4d6"),   // mate in 1 half-moves
    (0xd88ef8d2f739c7dd, "e4d6"),   // mate in 1 half-moves
    (0x14cf79c35880c3cd, "f1b5"),   // mate in 1 half-moves
    (0x7e718cc2796607dc, "e4d6"),   // mate in 1 half-moves
    (0xf79c3d3624c3e02b, "e4d6"),   // mate in 1 half-moves
    (0x97ac7c8d963ff45d, "d1h5"),   // mate in 1 half-moves
    (0xa6a60956a4f30cca, "d1h5"),   // mate in 1 half-moves
    (0xc9b5c7ed59c16d75, "e4d6"),   // mate in 1 half-moves
    (0x229b41cca01f196c, "g8f6"),   // mate in 1 half-moves
    (0x7501e5d5ed95aefe, "e4d6"),   // mate in 1 half-moves
    (0x9aab32d69b62edcf, "d1h5"),   // mate in 1 half-moves
    (0xb1220e685235d6c3, "e4d6"),   // mate in 1 half-moves
    (0x49104f1758c568c8, "e4d6"),   // mate in 1 half-moves
    (0x8a44cc904d20bced, "e4d6"),   // mate in 1 half-moves
    (0x53919e8cf120c900, "e4d6"),   // mate in 1 half-moves
    (0x68e81a1f742ddd4c, "e4d6"),   // mate in 1 half-moves
    (0x0696ea5e2a14a0f6, "e4d6"),   // mate in 1 half-moves
    (0x56125b40c0640f5e, "e4d6"),   // mate in 1 half-moves
    (0x5f3e011607404bfd, "e4d6"),   // mate in 1 half-moves
    (0x1f8f1be2c7fac89f, "e4d6"),   // mate in 1 half-moves
    (0xccc17b7d508d459f, "e4f6"),   // mate in 1 half-moves
    (0x842083d390d61150, "e4d6"),   // mate in 1 half-moves
    (0xc4b289b6549c74e7, "e4d6"),   // mate in 1 half-moves
    (0x5753203359667448, "e4d6"),   // mate in 1 half-moves
    (0x2aaf0257130cae21, "e4d6"),   // mate in 1 half-moves
    (0xa0d415aabe9dad12, "e4d6"),   // mate in 1 half-moves
    (0x08472f19e97d451e, "e4d6"),   // mate in 1 half-moves
    (0x52b9b72272d4b0c7, "e4d6"),   // mate in 1 half-moves
    (0xd0de6dddb76f704d, "e4d6"),   // mate in 1 half-moves
    (0x0f821ac191dd5f83, "f1b5"),   // mate in 1 half-moves
    (0x484000f4a9c1349c, "e4d6"),   // mate in 1 half-moves
    (0xad3679c4a62d8e79, "e4d6"),   // mate in 1 half-moves
    (0x44ad0f3b3b11fdc4, "e4d6"),   // mate in 1 half-moves
    (0x2aaba44f737261d7, "e4d6"),   // mate in 1 half-moves
    (0x5b5f3706fc9afb9b, "g8f6"),   // mate in 1 half-moves
    (0x65d113e0e264b68f, "e4d6"),   // mate in 1 half-moves
    (0x4833e6a568bc7317, "e4d6"),   // mate in 1 half-moves
    (0xdd64a2e1d393fa21, "e4d6"),   // mate in 1 half-moves
    (0x03a97d6410855b1a, "e4d6"),   // mate in 1 half-moves
    (0x0530b547c3c3dfed, "g8f6"),   // mate in 1 half-moves
    (0xb0a9118df3638555, "e4d6"),   // mate in 1 half-moves
    (0x35c444dd915bcb66, "e4d6"),   // mate in 1 half-moves
    (0x98c51024bd4a6c90, "e4d6"),   // mate in 1 half-moves
    (0x444888930d31fbd3, "e4d6"),   // mate in 1 half-moves
    (0x81274d5311167e17, "e4d6"),   // mate in 1 half-moves
    (0xced696ba823d25c1, "e4d6"),   // mate in 1 half-moves
    (0x767516f8b78e445d, "d1h5"),   // mate in 1 half-moves
    (0x05fa6f358aa9d012, "e4d6"),   // mate in 1 half-moves
    (0xc3ef9f4514744e02, "e4d6"),   // mate in 1 half-moves
    (0xecb6a96655cf950a, "e4d6"),   // mate in 1 half-moves
    (0x31e6c74dba2ffd99, "e4d6"),   // mate in 1 half-moves
    (0x5720c662981b33c3, "e4d6"),   // mate in 1 half-moves
    (0x80e9fc7eea574679, "e4d6"),   // mate in 1 half-moves
    (0xb876e4654d674a89, "e4d6"),   // mate in 1 half-moves
    (0xf75e9a174c755b17, "e4f6"),   // mate in 1 half-moves
    (0xccee0bcba39bab11, "e4d6"),   // mate in 1 half-moves
    (0x182d3184a80e6ce6, "e4d6"),   // mate in 1 half-moves
    (0x7d0ce76f68434543, "e4d6"),   // mate in 1 half-moves
    (0x521e228bbbb962b5, "e4d6"),   // mate in 1 half-moves
    (0x298dcd6292f8cb5f, "e4d6"),   // mate in 1 half-moves
    (0x7836a59e2b9fdbc3, "e4d6"),   // mate in 1 half-moves
    (0x7895f59d649c8d7c, "e4d6"),   // mate in 1 half-moves
    (0x21a53d2966461230, "f1b5"),   // mate in 1 half-moves
    (0xec6491e44fd1b5a0, "d1h5"),   // mate in 1 half-moves
    (0x7ceac49737341554, "e4d6"),   // mate in 1 half-moves
    (0x3b00d310dfa0496d, "e4d6"),   // mate in 1 half-moves
    (0x784543cfeae29c48, "e4d6"),   // mate in 1 half-moves
    (0x76e6e53e9e3b72e5, "e4f6"),   // mate in 1 half-moves
    (0x32d8a27e812b2935, "e4d6"),   // mate in 1 half-moves
    (0x811eb953a6809700, "f1b5"),   // mate in 1 half-moves
    (0x09044d8ab7f2a18e, "e4d6"),   // mate in 1 half-moves
    (0x7cd8261610f9bec9, "d1h5"),   // mate in 1 half-moves
    (0x6924939277a62e6e, "d3b5"),   // mate in 1 half-moves
    (0x5301bab647b28656, "g8f6"),   // mate in 1 half-moves
    (0x4dbba375ea75d751, "e4d6"),   // mate in 1 half-moves
    (0x3ae85e2baf1b8e7e, "e4f6"),   // mate in 1 half-moves
    (0x4904741a90103fdf, "e4d6"),   // mate in 1 half-moves
    (0x4deaf792ab765ad5, "d1h5"),   // mate in 1 half-moves
    (0x9505d659ffa04d48, "e4d6"),   // mate in 1 half-moves
    (0x60d4e1f8ff7f327f, "f1b5"),   // mate in 1 half-moves
    (0xba1713bd2c5a6abe, "e4d6"),   // mate in 1 half-moves
    (0x040303d1baafb81c, "e4d6"),   // mate in 1 half-moves
    (0x45f00c2b5d7be2f6, "g8f6"),   // mate in 1 half-moves
    (0x0420dc185b019a67, "g8f6"),   // mate in 1 half-moves
    (0x4ef3c61008e47fcc, "e4f6"),   // mate in 1 half-moves
    (0xeee9e8f205c78a1a, "e4d6"),   // mate in 1 half-moves
    (0x1e93f5fbd1843300, "e4d6"),   // mate in 1 half-moves
    (0xa203c8d0d7ab32c0, "e4d6"),   // mate in 1 half-moves
    (0x38f14ed6df551a30, "g8f6"),   // mate in 1 half-moves
    (0x5c591f4d2c2cc859, "e4f6"),   // mate in 1 half-moves
    (0xfa77fea2b28ba61b, "e4d6"),   // mate in 1 half-moves
    (0x3e94188709db3d30, "f1b5"),   // mate in 1 half-moves
    (0xa47409436e0e3f67, "e4d6"),   // mate in 1 half-moves
    (0xa2702e8116d6754b, "e4d6"),   // mate in 1 half-moves
    (0xb0473169fd068875, "e4d6"),   // mate in 1 half-moves
    (0x0eae333f1dd84288, "e4d6"),   // mate in 1 half-moves
    (0x3944a079aec662a2, "e4d6"),   // mate in 1 half-moves
    (0xfa0418f373f6e190, "e4d6"),   // mate in 1 half-moves
    (0xd80695f8e189691c, "e4f6"),   // mate in 1 half-moves
    (0xb678e101e8096944, "e4d6"),   // mate in 1 half-moves
    (0x6641846da11ed90d, "e4d6"),   // mate in 1 half-moves
    (0xc54c34ebdde6939d, "e4d6"),   // mate in 1 half-moves
    (0xc4561281b7d030a6, "e4d6"),   // mate in 1 half-moves
    (0xb8b721e90efb7b97, "e4d6"),   // mate in 1 half-moves
    (0xd8d864c4d560b95a, "e4d6"),   // mate in 1 half-moves
    (0x343c66fa6f353f88, "d3b5"),   // mate in 1 half-moves
    (0x6130960bcbe6f679, "e4d6"),   // mate in 1 half-moves
    (0x710ff51993c36663, "e4d6"),   // mate in 1 half-moves
    (0xf45d01cbdc97c34d, "e4d6"),   // mate in 1 half-moves
    (0x0f9dc501ebd3c28e, "e4d6"),   // mate in 1 half-moves
    (0xf923b65d9b341222, "e4d6"),   // mate in 1 half-moves
    (0x50ac9f3fb9ee50b8, "e4d6"),   // mate in 1 half-moves
    (0xdca2e4860887e99c, "c4f7"),   // mate in 1 half-moves
    (0xcfd84afff31f6086, "d1h5"),   // mate in 1 half-moves
    (0x7decda40eb71cef4, "e4d6"),   // mate in 1 half-moves
    (0xec685232f1537987, "e4d6"),   // mate in 1 half-moves
    (0xf49cc4479f0bf253, "e4d6"),   // mate in 1 half-moves
    (0x0274249d230108ac, "e4d6"),   // mate in 1 half-moves
    (0x383740d11c1fe5e8, "e4d6"),   // mate in 1 half-moves
    (0x1a1b411298ae723b, "d3b5"),   // mate in 1 half-moves
    (0x777bcabdc6ac011a, "e4d6"),   // mate in 1 half-moves
    (0x9ca6f013305b2f88, "e4d6"),   // mate in 1 half-moves
    (0xccacb9d2a9048a25, "e4d6"),   // mate in 1 half-moves
    (0x76027e097d7736a4, "f1b5"),   // mate in 1 half-moves
    (0xe9b4fe01452fb0ff, "e4d6"),   // mate in 1 half-moves
    (0xe8dd27ff9643118e, "e4d6"),   // mate in 1 half-moves
    (0x3f9ce2bba657a694, "e4d6"),   // mate in 1 half-moves
    (0x8826334af314593a, "e4d6"),   // mate in 1 half-moves
    (0x3a0cc51c4c57ca3f, "f1b5"),   // mate in 1 half-moves
    (0x35bc2290e3abeef5, "e4d6"),   // mate in 1 half-moves
    (0xa433a1c5245166a2, "e4d6"),   // mate in 1 half-moves
    (0x8e88fdaef63813e8, "e4d6"),   // mate in 1 half-moves
    (0x23934dbf56e60ced, "e4d6"),   // mate in 1 half-moves
    (0x70b24cb59b0c6d0c, "e4d6"),   // mate in 1 half-moves
    (0x4bbe6dd4ed7a5973, "e4d6"),   // mate in 1 half-moves
    (0x059db2698301c08e, "e4d6"),   // mate in 1 half-moves
    (0xec1b3f585e7b6330, "e4d6"),   // mate in 1 half-moves
    (0x11117e6cede4ce2a, "e4d6"),   // mate in 1 half-moves
    (0xbdae5fd6fe3e9cc7, "e4d6"),   // mate in 1 half-moves
    (0x84363e3158c174de, "e4d6"),   // mate in 1 half-moves
    (0xb853badeedb73fd6, "e4d6"),   // mate in 1 half-moves
    (0x2ce8516f5ab8e6f5, "g8f6"),   // mate in 1 half-moves
    (0x7658487feb1cf6f9, "e4d6"),   // mate in 1 half-moves
    (0x2f22481a31167a32, "e4d6"),   // mate in 1 half-moves
    (0x2bc57ccd5eacb2df, "e4d6"),   // mate in 1 half-moves
    (0x96b192169f53a465, "e4d6"),   // mate in 1 half-moves
    (0x13d42134d52cf327, "e4d6"),   // mate in 1 half-moves
    (0x8151332f6500eac6, "e4d6"),   // mate in 1 half-moves
    (0x8931706a559c6d4c, "e4d6"),   // mate in 1 half-moves
    (0x8236a2de9e1e864c, "e4d6"),   // mate in 1 half-moves
    (0xd0c804d1009c1ca7, "e4d6"),   // mate in 1 half-moves
    (0xbc1be4d682a2d49d, "e4d6"),   // mate in 1 half-moves
    (0x1bce40582a0944ef, "e4d6"),   // mate in 1 half-moves
    (0x08f40ded9c76b2e4, "g8f6"),   // mate in 1 half-moves
    (0x05b961aba0884621, "e4d6"),   // mate in 1 half-moves
    (0x1b80be396099f285, "e4f6"),   // mate in 1 half-moves
    (0xc7cb440325c7f98e, "e4d6"),   // mate in 1 half-moves
    (0xa65ae306aa41dd84, "g8f6"),   // mate in 1 half-moves
    (0xf6f995879e481d92, "e4d6"),   // mate in 1 half-moves
    (0x43683ae343b1e759, "f1b5"),   // mate in 1 half-moves
    (0x0f35cc14e5efc236, "e4d6"),   // mate in 1 half-moves
    (0x20a92b25f5b19162, "e4d6"),   // mate in 1 half-moves
    (0xe839bcc8750f55cf, "e4d6"),   // mate in 1 half-moves
    (0xd301c80ba09a9935, "e4d6"),   // mate in 1 half-moves
    (0x85bb7cf868e191ee, "e4d6"),   // mate in 1 half-moves
    (0x17092f77397e6872, "e4f6"),   // mate in 1 half-moves
    (0x6aac940ce40bbdee, "e4d6"),   // mate in 1 half-moves
    (0x8f05bf67c618f6d8, "e4d6"),   // mate in 1 half-moves
    (0x6b0ce82b9de02dc8, "g8f6"),   // mate in 1 half-moves
    (0x15605dc294f09823, "g8f6"),   // mate in 1 half-moves
    (0x6cf3f728f8c9a62f, "e4d6"),   // mate in 1 half-moves
    (0x6704590658626ded, "e4d6"),   // mate in 1 half-moves
    (0xd979bdffbbee16a4, "d3b5"),   // mate in 1 half-moves
    (0x07544df63f16e1a9, "e4d6"),   // mate in 1 half-moves
    (0x792e64ff433773bc, "e4d6"),   // mate in 1 half-moves
    (0xe9eab5406eef5d46, "e4d6"),   // mate in 1 half-moves
    (0x7de87c588b0f0102, "e4d6"),   // mate in 1 half-moves
    (0x029eefdbe82917d2, "e4d6"),   // mate in 1 half-moves
    (0x960e8e6e56b52d30, "e4d6"),   // mate in 1 half-moves
    (0x1711e787689bb903, "e4d6"),   // mate in 1 half-moves
    (0x1d8dcf1c188e5706, "e4d6"),   // mate in 1 half-moves
    (0x621b61b0f89aca61, "e4d6"),   // mate in 1 half-moves
    (0xaeb72eadfb58131c, "g8f6"),   // mate in 1 half-moves
    (0x95e50c5ac273c67f, "e4d6"),   // mate in 1 half-moves
    (0x8ac3e1c1c8a2deb5, "e4d6"),   // mate in 1 half-moves
    (0xf2f99d038c722cb1, "e4d6"),   // mate in 1 half-moves
    (0x243ae8baff6d6e35, "e4d6"),   // mate in 1 half-moves
    (0xf9d1a801a3f50696, "g8f6"),   // mate in 1 half-moves
    (0x142be2f4bbcc878c, "f1b5"),   // mate in 1 half-moves
    (0x0315f03e497f4444, "e4d6"),   // mate in 1 half-moves
    (0xdfffeab49dc1e8a9, "e4d6"),   // mate in 1 half-moves
    (0x8858a99486f0cc3d, "e4d6"),   // mate in 1 half-moves
    (0x2fc6d32dd25a3e73, "e4d6"),   // mate in 1 half-moves
    (0x369d9ca2445581b9, "e4d6"),   // mate in 1 half-moves
    (0x7bc36e5c13df3872, "e4d6"),   // mate in 1 half-moves
    (0xbe13bd215e1e946b, "e4d6"),   // mate in 1 half-moves
    (0x1e10999e8a999e2e, "e4d6"),   // mate in 1 half-moves
    (0x78064d51c0c30a7b, "e4d6"),   // mate in 1 half-moves
    (0xdd7293a2d7d7d713, "d1h5"),   // mate in 1 half-moves
    (0xaf04868bdaf62b52, "e4d6"),   // mate in 1 half-moves
    (0x986c69dde0d73f22, "g8f6"),   // mate in 1 half-moves
    (0xd02c9fe6e3d058e6, "e4d6"),   // mate in 1 half-moves
    (0x8ac6b080c41f88b7, "g8f6"),   // mate in 1 half-moves
    (0x582559e18aec7b17, "f1b5"),   // mate in 1 half-moves
    (0x727e38a5dbb785ea, "f1b5"),   // mate in 1 half-moves
    (0xa05a5f4be1671fbe, "e4d6"),   // mate in 1 half-moves
    (0x2a8f5db13ced0766, "e4f6"),   // mate in 1 half-moves
    (0x04aa20d67bad8c46, "e4d6"),   // mate in 1 half-moves
    (0x3a8d91d9ef80ac5d, "e4d6"),   // mate in 1 half-moves
    (0xc6cd5ad4f982222e, "e4d6"),   // mate in 1 half-moves
    (0x977e440f8c21d4f7, "e4d6"),   // mate in 1 half-moves
    (0xb0159cd7aa999a0b, "e4d6"),   // mate in 1 half-moves
    (0x8b1c3639204367b5, "f1b5"),   // mate in 1 half-moves
    (0x5b61c64c2c5fd09f, "e4d6"),   // mate in 1 half-moves
    (0x8b216421f7ab4154, "e4d6"),   // mate in 1 half-moves
    (0x88bc32a365bc887c, "e4d6"),   // mate in 1 half-moves
    (0x347fd5bfb6f035a6, "e4d6"),   // mate in 1 half-moves
    (0x0384c9b1821fb4e9, "e4d6"),   // mate in 1 half-moves
    (0xa55ede2397430825, "e4d6"),   // mate in 1 half-moves
    (0x5c64077da7c139b8, "e4d6"),   // mate in 1 half-moves
    (0xde06608f8dcc3db0, "e4d6"),   // mate in 1 half-moves
    (0xa7188c28ceb34f83, "e4d6"),   // mate in 1 half-moves
    (0xc7cfe21b45b93678, "e4d6"),   // mate in 1 half-moves
    (0x8e6c6699157457a9, "e4d6"),   // mate in 1 half-moves
    (0x75154b66a617338b, "e4d6"),   // mate in 1 half-moves
    (0xd28c77b871fa3768, "e4d6"),   // mate in 1 half-moves
    (0xfc54a569393fe706, "e4d6"),   // mate in 1 half-moves
    (0x3e071d905e60262a, "e4d6"),   // mate in 1 half-moves
    (0x036052866153f0a8, "e4d6"),   // mate in 1 half-moves
    (0x13d479435d7b735d, "e4d6"),   // mate in 1 half-moves
    (0x51634926aa9c202a, "e4d6"),   // mate in 1 half-moves
    (0xdf75d6ff91b8b03c, "e4d6"),   // mate in 1 half-moves
    (0x878bdcc6f8e17c72, "e4d6"),   // mate in 1 half-moves
    (0xcd2e47dc76ca6cf9, "d1h5"),   // mate in 1 half-moves
    (0x70190c84f67ca958, "e4d6"),   // mate in 1 half-moves
    (0x09aaa251a8935e33, "f1b5"),   // mate in 1 half-moves
    (0xfa320c262059d1d3, "e4d6"),   // mate in 1 half-moves
    (0xcd86d236b2c2d967, "e4d6"),   // mate in 1 half-moves
    (0x329bace0ab0abf06, "e4d6"),   // mate in 1 half-moves
    (0xff4cc4acc51f9954, "g8f6"),   // mate in 1 half-moves
    (0x11bf29636db7f3d1, "e4d6"),   // mate in 1 half-moves
    (0xbbc3d2c454e09134, "d1h5"),   // mate in 1 half-moves
    (0x34ec6d7312afb2a1, "e4d6"),   // mate in 1 half-moves
    (0xe2e65c95a716082c, "f1b5"),   // mate in 1 half-moves
    (0xfcc4384613aeed4b, "e4f6"),   // mate in 1 half-moves
    (0xd9acf8e507ed63a1, "e4d6"),   // mate in 1 half-moves
    (0xc28646bbc718ef73, "e4d6"),   // mate in 1 half-moves
    (0x1ef402a969d5da6f, "e4d6"),   // mate in 1 half-moves
    (0x90278a424a8ca3c2, "e4d6"),   // mate in 1 half-moves
    (0x09177f54cfbcb4f1, "e4d6"),   // mate in 1 half-moves
    (0x8b66cca7bdf41891, "e4d6"),   // mate in 1 half-moves
    (0x1e539700a0b8081d, "e4d6"),   // mate in 1 half-moves
    (0x2327e6aba96a7d8d, "e4d6"),   // mate in 1 half-moves
    (0x45f9f96e7daec3dd, "e4d6"),   // mate in 1 half-moves
    (0xe9ff3a59819b6457, "g8f6"),   // mate in 1 half-moves
    (0x2208b294aed18c53, "e4d6"),   // mate in 1 half-moves
    (0x27646e137638dfef, "e4d6"),   // mate in 1 half-moves
    (0xdd285645197ed059, "g8f6"),   // mate in 1 half-moves
    (0xb35ed16762e060ae, "g8f6"),   // mate in 1 half-moves
    (0x0151835738196f8b, "e4d6"),   // mate in 1 half-moves
    (0x35a799d19702bf36, "f1b5"),   // mate in 1 half-moves
    (0x80ab5878d329a455, "e4d6"),   // mate in 1 half-moves
    (0xaf399ebb511bdab3, "e4f6"),   // mate in 1 half-moves
    (0xcc7fd63c25aac183, "e4d6"),   // mate in 1 half-moves
    (0xe65bf46a3d0ca790, "e4d6"),   // mate in 1 half-moves
    (0xe5da69a49b1e081c, "e4d6"),   // mate in 1 half-moves
    (0x108a814715d34e94, "e4d6"),   // mate in 1 half-moves
    (0x9cb31d7c199054d0, "e4d6"),   // mate in 1 half-moves
    (0x9ef8daf31a983196, "e4d6"),   // mate in 1 half-moves
    (0x795602b231c7562f, "e4d6"),   // mate in 1 half-moves
    (0xf8a93368b87b981a, "e4d6"),   // mate in 1 half-moves
    (0x816443cd3b37e824, "e4d6"),   // mate in 1 half-moves
    (0xc262dd8c2454ab32, "e4d6"),   // mate in 1 half-moves
    (0xf23b2b38360ea0da, "e4d6"),   // mate in 1 half-moves
    (0x7e86bedf51240728, "e4d6"),   // mate in 1 half-moves
    (0xb9b180d1ff388987, "b1c3"),   // 1.e3 a6 -> 2.Nc3
    (0x7b43b4716b0850fc, "c3e4"),   // after 2...h6 -> 3.Ne4 (17/18 instant mates, 1 deep branch)
    (0x9df06cd856b1aeb9, "e4d6"),   // 3...c5 -> Nd6#
    (0xce1858ad1c7c71f5, "e4d6"),   // 3...g6 -> Nd6#
    (0xfe267345078e6a81, "e4d6"),   // 3...h5 -> Nd6#
    (0xfe5e1508757e4f12, "e4d6"),   // 3...Nc6 -> Nd6#
    (0x141ddd2c0b14494e, "e4d6"),   // 3...b5 -> Nd6#
    (0xd3cf061c3edbcd01, "e4d6"),   // 3...b6 -> Nd6#
    (0xc4602d590708fe64, "f1b5"),   // 3...d5 -> Bb5#
    (0xea470ab1f093b3d7, "e4f6"),   // 3...d6 -> Nf6#
    (0x62879cca6926abb3, "e4d6"),   // 3...a5 -> Nd6#
    (0x6b6c865e0b68ac9d, "d1h5"),   // 3...f5 -> Qh5#
    (0x5eb0d4528b812d8b, "e4d6"),   // 3...f6 -> Nd6#
    (0x4c69eb5e6e34dd95, "e4d6"),   // 3...e6 -> Nd6#
    (0x48d289ff1796bae3, "d1g4"),   // Qg4! reclaims Ne4, mate in 2-3,   // Nf6: Qg4 reclaims e4 knight + flips g7/d7,   // 3...Nf6 -> Qf3 (Nf6 flipped, then mate)
    (0x72026ef4f05db447, "e4d6"),   // mate in 1 half-moves
    (0xa07d1c40e1315ffa, "g8f6"),   // mate in 1 half-moves
    (0xe715a63e4b4f10d7, "e4f6"),   // mate in 1 half-moves
    (0x9323f54eff759eb1, "f1b5"),   // mate in 1 half-moves
    (0x3ee84ed622310c9d, "e4d6"),   // mate in 1 half-moves
    (0xf532e06d0db7d503, "g8f6"),   // mate in 1 half-moves
    (0xaa538d3cd9c61a28, "e4d6"),   // mate in 1 half-moves
    (0xeda483b6a0b2a4d3, "e4d6"),   // mate in 1 half-moves
    (0xa7944c7def3f8abd, "e4d6"),   // mate in 1 half-moves
    (0xff6987b78dd203da, "e4d6"),   // mate in 1 half-moves
    (0x4fcb150fb480ff00, "e4d6"),   // mate in 1 half-moves
    (0xa500eeba40f98756, "g8f6"),   // mate in 1 half-moves
    (0x1f75cadf0ca79e77, "f2f3"),   // mate in 3 half-moves
    (0x9520c29f682dd231, "c4f7"),   // mate in 1 half-moves
    (0x995b80bae4011120, "e4d6"),   // mate in 1 half-moves
    (0x194f71a3b0c8ea4e, "e4d6"),   // mate in 1 half-moves
    (0x27dc0210fa24b42f, "e4d6"),   // mate in 1 half-moves
    (0xcaa1eeb6c5eed5db, "f1c4"),   // mate in 3 half-moves
    (0x4ebdd0571d787448, "e4d6"),   // mate in 1 half-moves
    (0x1bcea87e7505f901, "e4d6"),   // mate in 1 half-moves
    (0xe7dcf642bbc62b42, "f1b5"),   // mate in 1 half-moves
    (0x1df0dca00ba82bf4, "e4d6"),   // mate in 1 half-moves
    (0x70c57e6b21797492, "f1c4"),   // mate in 3 half-moves
    (0xf98ea965159d1e15, "e4d6"),   // mate in 1 half-moves
    (0x90a2c057ed6d0db9, "e4d6"),   // mate in 1 half-moves
    (0x34490614a17b2e56, "e4d6"),   // mate in 1 half-moves
    (0x84a16ade543c4227, "e4d6"),   // mate in 1 half-moves
    (0x82709a524f56ae36, "e4d6"),   // mate in 1 half-moves
    (0x24d70bea23cf8bbb, "e4d6"),   // mate in 1 half-moves
    (0x2e6dbce8bb4a1369, "e4d6"),   // mate in 1 half-moves
    (0x1b2a33499649bd40, "e4d6"),   // mate in 1 half-moves
    (0xa9f956286e4f6b86, "e4d6"),   // mate in 1 half-moves
    (0xbde04991eba29743, "e4f6"),   // mate in 1 half-moves
    (0x663e2ad1b0b40f9d, "d1h5"),   // mate in 1 half-moves
    (0xcb17f8125b6880d4, "g8f6"),   // mate in 1 half-moves
    (0x60ea4c44411988f3, "c3e4"),   // mate in 5 half-moves
    (0x7afbbd43afbbb8be, "e4f6"),   // mate in 1 half-moves
    (0xe424e4b5448f9f94, "e4d6"),   // mate in 1 half-moves
    (0x0856a85c3d0a121b, "e4c3"),   // mate in 5 half-moves
    (0xd17df70824228388, "g8f6"),   // mate in 1 half-moves
    (0x2ca9c678e2ea2be6, "f1b5"),   // mate in 7 half-moves
    (0x413b47d1d5e87e95, "e4d6"),   // mate in 1 half-moves
    (0x73b18d467b629e31, "e4d6"),   // mate in 1 half-moves
    (0xd809670ebe025b1b, "c4b3"),   // mate in 1 half-moves
    (0x597d916408086306, "e4d6"),   // mate in 1 half-moves
    (0xebff038334205b95, "c3e4"),   // mate in 7 half-moves
    (0xada715ed57b5b71c, "e4d6"),   // mate in 1 half-moves
    (0x124ecf7d9138c246, "e4d6"),   // mate in 1 half-moves
    (0x0dcbf67b8c1bc788, "e4d6"),   // mate in 1 half-moves
    (0x45192c67a190f584, "e4d6"),   // mate in 1 half-moves
    (0x65ad45b1c98e8722, "f1b5"),   // mate in 1 half-moves
    (0xfba1ddb359ac9a5a, "e4d6"),   // mate in 1 half-moves
    (0xb5d0b5c4c5923008, "e4d6"),   // mate in 1 half-moves
    (0x73551671982eda70, "e4d6"),   // mate in 1 half-moves
    (0x3170f6bbacb07079, "e4d6"),   // mate in 1 half-moves
    (0x45f79bd57ebba232, "g8f6"),   // mate in 1 half-moves
    (0xa82a5fa075af630f, "e4d6"),   // mate in 1 half-moves
    (0xd402432abd52838c, "e4d6"),   // mate in 1 half-moves
    (0xe5f7ed3d5f6f971d, "e4d6"),   // mate in 1 half-moves
    (0x2d42371d31d6f5be, "e4d6"),   // mate in 1 half-moves
    (0x94dc368e9195ea1a, "e4d6"),   // mate in 1 half-moves
    (0xca572ff84d808a2d, "e4d6"),   // mate in 1 half-moves
    (0x58ff6f6ad7c685da, "c3e4"),   // mate in 9 half-moves
    (0x79caffc8a07b37fd, "e4d6"),   // mate in 1 half-moves
    (0x7d7fe918f632bf26, "e4d6"),   // mate in 1 half-moves
    (0xb3e47ac95616ab9c, "e4d6"),   // mate in 1 half-moves
    (0x0980ea14b2810ad5, "e4d6"),   // mate in 1 half-moves
    (0xc9fbd1aa4c5d66f1, "e4f6"),   // mate in 1 half-moves
    (0xff7dbcba450754cd, "e4d6"),   // mate in 1 half-moves
    (0xe68070f90650414b, "e4d6"),   // mate in 1 half-moves
    (0xfed18e854961eb98, "f1b5"),   // mate in 1 half-moves
    (0x656a77763a12dd58, "g8f6"),   // mate in 1 half-moves
    (0x86bd0fda9fec32f7, "e4d6"),   // mate in 1 half-moves
    (0xc3ad686f8481b0e9, "d1g4"),   // Qg4! flips c8 bishop, Bd7#,   // mate in 3 half-moves
    (0xb60eedd87da46f13, "e4d6"),   // mate in 1 half-moves
    (0x848cde0bc6a6add4, "e4d6"),   // mate in 1 half-moves
    (0x8468453c25eae995, "e4d6"),   // mate in 1 half-moves
    (0x1a35708d9f3b1cf1, "e4d6"),   // mate in 1 half-moves
    (0xe686eb0610cd02c7, "g8f6"),   // mate in 1 half-moves
    (0x5d8c953cf0dea3b1, "e4d6"),   // mate in 1 half-moves
    (0x573e19d42cf6401c, "e4d6"),   // mate in 1 half-moves
    (0x761118fed0d4f3fc, "c3e4"),   // mate in 5 half-moves
    (0xc6e29b52031cc3cd, "e4f6"),   // mate in 1 half-moves
    (0xec1f99403e05acc6, "e4d6"),   // mate in 1 half-moves
    (0xedae1a96fd6ba751, "e4d6"),   // mate in 1 half-moves
    (0xbd04d2a608eed302, "f1b5"),   // mate in 1 half-moves
    (0xe0f84d82187b663c, "g8f6"),   // mate in 1 half-moves
    (0x8fa0d40075cc6a2f, "e4d6"),   // mate in 1 half-moves
    (0x4be61c71e722b22c, "e4d6"),   // mate in 1 half-moves
    (0x78a326367317968c, "e4d6"),   // mate in 1 half-moves
    (0xfb51b5b7326abe33, "g8f6"),   // mate in 1 half-moves
    (0xde9daa9385076e01, "e4d6"),   // mate in 1 half-moves
    (0x4ba4ae68edbd9318, "e4d6"),   // mate in 1 half-moves
    (0xa0f5b962ec69141c, "e4d6"),   // mate in 1 half-moves
    (0xf1eef284da826bd8, "e4f6"),   // mate in 1 half-moves
    (0x17aad7eda9d41484, "e4d6"),   // mate in 1 half-moves
    (0x4f1fccc9941ff46e, "d1g4"),   // Qg4! reclaims Ne4, mate in 2-3,   // Nf6: Qg4 reclaims e4 knight + flips g7/d7,   // mate in 3 half-moves
    (0xc309fabc8d8144c6, "e4d6"),   // mate in 1 half-moves
    (0xc866fe2914ca150e, "e4d6"),   // mate in 1 half-moves
    (0x0b0b881a9a6550fb, "e4d6"),   // mate in 1 half-moves
    (0x14483ff8bd95f3dc, "e4d6"),   // mate in 1 half-moves
    (0x43ba9e0c10256dda, "e4d6"),   // mate in 1 half-moves
    (0xc3449699a4b5b31a, "g8f6"),   // mate in 1 half-moves
    (0x54dc9aab5820f50d, "f1b5"),   // mate in 1 half-moves
    (0x458e510183a1f64e, "e4d6"),   // mate in 1 half-moves
    (0x626df7d35a95fa52, "c4b3"),   // mate in 1 half-moves
    (0x8fe3da9e5fedfc1c, "e4d6"),   // mate in 1 half-moves
    (0x45802570ac4a19e3, "f2f3"),   // mate in 3 half-moves
    (0x57c0136b4425059a, "e4d6"),   // mate in 1 half-moves
    (0xbfe699a51cce54c8, "e4d6"),   // mate in 1 half-moves
    (0x7c8ef147e8811e71, "c3e4"),   // mate in 5 half-moves
    (0x4da2fa013baba1ca, "g8f6"),   // mate in 1 half-moves
    (0x0964712351cd4e94, "e4d6"),   // mate in 1 half-moves
    (0xf8a4c2810e5b4d57, "e4d6"),   // mate in 1 half-moves
    (0x85d8a1f46eb8e5be, "f1b5"),   // mate in 1 half-moves
    (0x5724885ca76941db, "e4d6"),   // mate in 1 half-moves
    (0x4b77c1866113d8be, "e4d6"),   // mate in 1 half-moves
    (0xfbd031ac5440a7f4, "d1h5"),   // mate in 1 half-moves
    (0xce0c63a0d4a926e2, "e4d6"),   // mate in 1 half-moves
    (0xda8edc99a58e29ba, "c3e4"),   // mate in 5 half-moves
    (0x5ea4ef5f43547a9c, "e4d6"),   // mate in 1 half-moves
    (0x1a839caf2718bdfb, "g8f6"),   // mate in 1 half-moves
    (0x8238a2fd877ac6d0, "g8f6"),   // mate in 1 half-moves
    (0x188e4b37e8454dc5, "e4d6"),   // mate in 1 half-moves
    (0x172f9df80e35baff, "e4d6"),   // mate in 1 half-moves
    (0x37a10637b7da9c68, "e4d6"),   // mate in 1 half-moves
    (0xcab3b4cfaeccce6c, "e4d6"),   // mate in 1 half-moves
    (0x5f937de0bbf83654, "e4d6"),   // mate in 1 half-moves
    (0xc9d51d9b9ff53f78, "e4d6"),   // mate in 1 half-moves
    (0x4f5670b0e920744a, "g8f6"),   // mate in 1 half-moves
    (0x81ddf37a448c2ff8, "e4d6"),   // mate in 1 half-moves
    (0xce7f85f115d46169, "e4d6"),   // mate in 1 half-moves
    (0x94aa705c79d4ebf4, "g8f6"),   // mate in 1 half-moves
    (0x2f4452428cba7378, "c4f7"),   // mate in 1 half-moves
    (0x41c54d6ebd3b3b13, "e4d6"),   // mate in 1 half-moves
    (0x3fbf36534a4262aa, "e4d6"),   // mate in 1 half-moves
    (0xdfc9d56c2d19266b, "f1b5"),   // mate in 1 half-moves
    (0x8966e8ecef43f6f6, "c3e4"),   // mate in 9 half-moves
    (0xa8cec49796e3274e, "e4d6"),   // mate in 1 half-moves
    (0x93c76e791c39daf0, "f1b5"),   // mate in 1 half-moves
    (0x3f5bad64a90e26eb, "e4d6"),   // mate in 1 half-moves
    (0x853f3db94d9987a2, "e4d6"),   // mate in 1 half-moves
    (0x6b6e52e4ab586fc5, "d1g4"),   // Qg4! reclaims Ne4, mate in 2-3,   // Nf6: Qg4 reclaims e4 knight + flips g7/d7,   // mate in 3 half-moves
    (0x6ca1c36888e1e210, "d1h5"),   // mate in 1 half-moves
    (0xf93a95e617fe88d2, "e4d6"),   // mate in 1 half-moves
    (0x29a0f9de38c35de4, "e4d6"),   // mate in 1 half-moves
    (0x3c3d04309837d7ff, "e4d6"),   // mate in 1 half-moves
    (0xf10a69b339ce2f99, "f1b5"),   // mate in 1 half-moves
    (0xc93281d6bcd45d64, "f1b5"),   // mate in 1 half-moves
    (0x456aca3660edb20f, "e4d6"),   // mate in 1 half-moves
    (0x7920064440221253, "g8f6"),   // mate in 1 half-moves
    (0xd404d8d5abcfc000, "g8f6"),   // mate in 1 half-moves
    (0x55a5518162a45294, "e4d6"),   // mate in 1 half-moves
    (0xc2f1eed43917dae0, "e4d6"),   // mate in 1 half-moves
    (0x1ed7d64185c01e51, "e4d6"),   // mate in 1 half-moves
    (0x4b8a62593e15ca91, "e4f6"),   // mate in 1 half-moves
    (0x1a198c9d05a6feb3, "g8f6"),   // mate in 1 half-moves
    (0x5d5198697f6589f8, "g8f6"),   // mate in 1 half-moves
    (0xa42ff111edd364b0, "e4d6"),   // mate in 1 half-moves
    (0x13d0981a889d07c3, "e4d6"),   // mate in 1 half-moves
    (0xb377c205f2492c9b, "e4d6"),   // mate in 1 half-moves
    (0xd028d1d8dca22afe, "e4d6"),   // mate in 1 half-moves
    (0xf113515b858479f7, "e4d6"),   // mate in 1 half-moves
    (0x3520dfea72178f27, "e4d6"),   // mate in 1 half-moves
    (0xdde2ce13c9b09a34, "e4d6"),   // mate in 1 half-moves
    (0x1047ee44e1c69e97, "e4d6"),   // mate in 1 half-moves
    (0x55bc55c22e7c4760, "g8f6"),   // mate in 1 half-moves
    (0x5f0e8242a5307021, "e4f6"),   // mate in 1 half-moves
    (0x1f647e67a46e261f, "g8f6"),   // mate in 1 half-moves
    (0x9fa9aa91010cf5a9, "f1d3"),   // mate in 5 half-moves
    (0x92778ab1576bd6db, "e4d6"),   // mate in 1 half-moves
    (0xccb67a6ea9c3404e, "e4d6"),   // mate in 1 half-moves
    (0x6f31ab7231b64cf2, "e4d6"),   // mate in 1 half-moves
    (0x865994ed7ca076b6, "e4d6"),   // mate in 1 half-moves
    (0x0c722a323dd1d95b, "g8f6"),   // mate in 1 half-moves
    (0x1f7bbeae234c71da, "e4d6"),   // mate in 1 half-moves
    (0x71f1823ddaf104af, "e4d6"),   // mate in 1 half-moves
    (0x435e053bf369299b, "e4d6"),   // mate in 1 half-moves
    (0xf073dd0782151827, "e4d6"),   // mate in 1 half-moves
    (0xaccd6b771a15abe2, "e4d6"),   // mate in 1 half-moves
    (0x5ce27bdbf45aa38e, "g8f6"),   // mate in 1 half-moves
    (0x4c69eb5e6e34dd95, "e4d6"),   // mate in 1 half-moves
    (0x4a265066327375c4, "d1h5"),   // mate in 1 half-moves
    (0xddb48b830b17d56a, "g8f6"),   // mate in 1 half-moves
    (0x82f2788fce10c92f, "e4d6"),   // mate in 1 half-moves
    (0x94dc6ef919c26a60, "e4d6"),   // mate in 1 half-moves
    (0xce1858ad1c7c71f5, "e4d6"),   // mate in 1 half-moves
    (0xf3549eac0f10c2a3, "e4d6"),   // mate in 1 half-moves
    (0x654ad9fceaafe53e, "e4d6"),   // mate in 1 half-moves
    (0x0760427bec79724b, "f1b5"),   // mate in 1 half-moves
    (0x02b36b422c5888d3, "e4d6"),   // mate in 1 half-moves
    (0x590e7735c975248d, "e4d6"),   // mate in 1 half-moves
    (0xff9de22720259441, "e4d6"),   // mate in 1 half-moves
    (0x053eb564daa79f71, "e4d6"),   // mate in 1 half-moves
    (0xa0e70c72c38f1cb2, "g8f6"),   // mate in 1 half-moves
    (0xfccb79e65766214f, "e4d6"),   // mate in 1 half-moves
    (0xe14d35cf85d90fc6, "e4d6"),   // mate in 1 half-moves
    (0xae0ce2e0def2d1a6, "e4d6"),   // mate in 1 half-moves
    (0x8ea2b5ebec2a470e, "f1b5"),   // mate in 1 half-moves
    (0x4a8ec58cf67bc05a, "e4d6"),   // mate in 1 half-moves
    (0xc75982288729954e, "c3e4"),   // mate in 11 half-moves
    (0xbed10b1ae4621800, "e4d6"),   // mate in 1 half-moves
    (0xc22043cb65312120, "g8f6"),   // mate in 1 half-moves
    (0xa848dcfea576ebdd, "e4d6"),   // mate in 1 half-moves
    (0x77b7d8b2913b722a, "e4d6"),   // mate in 1 half-moves
    (0xca206aada3c4cc1c, "g8f6"),   // mate in 1 half-moves
    (0x75333c8272b7b9e7, "e4d6"),   // mate in 1 half-moves
    (0x53e278dd305d8e8b, "e4d6"),   // mate in 1 half-moves
    (0xdf2d4e5bce55622a, "f1b5"),   // mate in 1 half-moves
    (0xdf0ee7abde857c11, "g8f6"),   // mate in 1 half-moves
    (0x717368cfbfff219d, "g8f6"),   // mate in 1 half-moves
    (0x0d4cdb2a0999a5d0, "e4d6"),   // mate in 1 half-moves
    (0x86e2a487bdefdb31, "e4d6"),   // mate in 1 half-moves
    (0xa8b981c278a7617f, "g8f6"),   // mate in 1 half-moves
    (0xc34af422a7a0d2f5, "e4d6"),   // mate in 1 half-moves
    (0xd83616ac43f952c0, "e4d6"),   // mate in 1 half-moves
    (0x683d2bc8f1a33361, "e4d6"),   // mate in 1 half-moves
    (0x3aa6486cba8785fa, "e4d6"),   // mate in 1 half-moves
    (0x3ccbc57e10598809, "d1h5"),   // mate in 1 half-moves
    (0x12dd111f9c30c036, "g8f6"),   // mate in 1 half-moves
    (0x0fb4251921059141, "e4d6"),   // mate in 1 half-moves
    (0xfae4cdfaafc8d7c9, "e4d6"),   // mate in 1 half-moves
    (0x23db8b50e3daf428, "e4d6"),   // mate in 1 half-moves
    (0x4373b1ee61f3c668, "e4d6"),   // mate in 1 half-moves
    (0x21ae1eece04a15f7, "d1h5"),   // mate in 1 half-moves
    (0x0dd167f30784c64f, "e4d6"),   // mate in 1 half-moves
    (0xf30cb987cea2ec12, "e4d6"),   // mate in 1 half-moves
    (0xe91fe117d910c3a5, "f2f3"),   // mate in 3 half-moves
    (0x07797a9350332425, "e4d6"),   // mate in 1 half-moves
    (0x89a6248559615bb5, "e4d6"),   // mate in 1 half-moves
    (0xb616baefd9467812, "g8f6"),   // mate in 1 half-moves
    (0x51fe67b53a88d76f, "e4d6"),   // mate in 1 half-moves
    (0x62879cca6926abb3, "e4d6"),   // mate in 1 half-moves
    (0xa91dcd1f8d032fc7, "e4d6"),   // mate in 1 half-moves
    (0xb8d7fb9fc8cf299a, "e4d6"),   // mate in 1 half-moves
    (0x7044fa7047536d55, "g8f6"),   // mate in 1 half-moves
    (0x008b5918b2e61101, "c3e4"),   // mate in 17 half-moves
    (0x05280d8251107f44, "e4d6"),   // mate in 1 half-moves
    (0xff0e5aeb847a1346, "e4d6"),   // mate in 1 half-moves
    (0xd5b1a098366da9fa, "e4d6"),   // mate in 1 half-moves
    (0x70d724124a5b6f25, "e4d6"),   // mate in 1 half-moves
    (0x6ee2a2fa2a56447b, "e4d6"),   // mate in 1 half-moves
    (0xa14ea39472de1c09, "e4d6"),   // mate in 1 half-moves
    (0x6b178efa7abcb5fb, "e4d6"),   // mate in 1 half-moves
    (0xdc48e0753f01b563, "e4d6"),   // mate in 1 half-moves
    (0x7d0c0f49374ff8ad, "e4d6"),   // mate in 1 half-moves
    (0x805c5a4c7baff894, "e4d6"),   // mate in 1 half-moves
    (0x56df50af4d57a42b, "e4d6"),   // mate in 1 half-moves
    (0xbb56c4b5deeccc54, "b2b3"),   // mate in 5 half-moves
    (0x2a7d60e0c8b50177, "e4d6"),   // mate in 1 half-moves
    (0x2ecee0819d434abd, "g8f6"),   // mate in 1 half-moves
    (0x13795dc269948e8e, "e4d6"),   // mate in 1 half-moves
    (0x0477771437197764, "e4d6"),   // mate in 1 half-moves
    (0x9019f03d2c22a03e, "e4d6"),   // mate in 1 half-moves
    (0x783f45804950b3e2, "g8f6"),   // mate in 1 half-moves
    (0xf7c66adcdf3c0083, "g8f6"),   // mate in 1 half-moves
    (0xf5dbb1fb3c2b5789, "c3e4"),   // mate in 5 half-moves
    (0xf21d5cdce2ae2ab6, "e4d6"),   // mate in 1 half-moves
    (0x7844d31681a68069, "g8f6"),   // mate in 1 half-moves
    (0xbe4cb7c3ea7f7b9f, "e4d6"),   // mate in 1 half-moves
    (0x27fdaed8a8d00d21, "e4d6"),   // mate in 1 half-moves
    (0x48d05d45b7a679bb, "d1h5"),   // mate in 1 half-moves
    (0x0a371ec527c89c72, "g8f6"),   // mate in 1 half-moves
    (0xabff861c9923a80d, "f1b5"),   // mate in 1 half-moves
    (0xe4c07f82a7c3dbd5, "e4d6"),   // mate in 1 half-moves
    (0xd86e3e0d48beb18a, "d1g4"),   // Qg4! reclaims Ne4, mate in 2-3,   // Nf6: Qg4 reclaims e4 knight + flips g7/d7,   // mate in 3 half-moves
    (0xed8a4f87731afd5a, "e4f6"),   // mate in 1 half-moves
    (0x681875735173463e, "e4d6"),   // mate in 1 half-moves
    (0x294765931be23ff8, "f1b5"),   // mate in 1 half-moves
    (0x537b71ca3d8762ec, "f2f3"),   // mate in 3 half-moves
    (0xe513760abc23d35c, "e4d6"),   // mate in 1 half-moves
    (0xe73508b49bf228b1, "g8f6"),   // mate in 1 half-moves
    (0x0f50be2ec249d500, "e4d6"),   // mate in 1 half-moves
    (0xefd8c51fcaebe629, "g8f6"),   // mate in 1 half-moves
    (0xf8e6709804c46c63, "e4d6"),   // mate in 1 half-moves
    (0xc882651ef786514f, "e4d6"),   // mate in 1 half-moves
    (0x1fcd079ef9f375ad, "e4d6"),   // mate in 1 half-moves
    (0x5ec332034afc6a00, "e4d6"),   // 3.Ne4 Ra7 -> Nd6#
    (0x3f7ef3df09de53b4, "e4f6"),   // 3.Ne4 Rh7 -> Nf6#
    (0xeac8913ccc8c175d, "c3e4"),   // 2.Nc3 Nc6 -> Qf3 (flips Nc6, all Black moves mate)
    (0x0210114dfcb40389, "c3b5"),   // 2.Nc3 Nf6 -> Ne4
    (0x0d8c1743bfbb0893, "c3e4"),   // 2.Nc3 Nh6 -> Qf3
    (0x8596f861ac900eef, "e4d6"),
    (0x42442351995f8aa0, "e4f6"),
    (0x55eb0814a08cb9c5, "d2d3"),   // d5 -> Qg4 (avoids deep oscillation),
    (0x7bcc2ffc5717f476, "e4f6"),
    (0xfae7a313aceceb3c, "d1h5"),
    (0xcf3bf11f2c056a2a, "e4d6"),
    (0x70c61082225d4867, "e4d6"),
    (0xaf1909793ac4c72a, "e4d6"),
    (0x6fd53045d2fa08b3, "e4d6"),
    (0x57d67a67f3d66970, "e4d6"),
    (0x09181b9eaa150630, "e4d6"),
    (0xae85dad8d641d262, "e4d6"),
    (0xd959acb2b012fd42, "d2d3"),   // Nf6: Qg4 reclaims e4 knight + flips g7/d7,
    (0x8891b63aa1cd177d, "e4d6"),
    (0x98870c588343d1a2, "e4d6"),
    (0x5bdd1d1dd39320fa, "e4d6"),
    (0x4f9bc88ae85cf650, "e4f6"),
    (0x74eee8f9ce024081, "e4d6"),
    (0x087627f6b37f6985, "e4d6"),
    (0xfd9e3e5db4bd8102, "e4d6"),
    (0x6aefcc99681cc4e6, "e4d6"),
    (0xd9efa0708bfa1aa9, "e4d6"),
    (0xfeeacd70eea66ba1, "d1h5"),
    (0xcb369f7c6e4feab7, "e4d6"),
    (0x74cb7ee16017c8fa, "e4d6"),
    (0x5b9e1383f9b2b6c9, "e4d6"),
    (0xe1fa835e1d251780, "e4d6"),
    (0xfa537b6b3734cf8f, "e4d6"),
    (0xe077c1972d05f2b0, "e4d6"),
    (0x6bd85e2690b0882e, "e4d6"),
    (0xad7e18a4766c94d3, "e4f6"),
    (0xdd54c2d1f2587ddf, "d1g4"),
    (0x760bcbda4fb93bb7, "g4d7"),
    (0x624d1e4d7476ed1d, "g4d7"),
    (0x59383e3e52285bcc, "g4d7"),
    (0x25a0f1312f5572c8, "g4d7"),
    (0xd048e89a28979a4f, "g4d7"),
    (0x47391a5ef436dfab, "g4d7"),
    (0xf43976b717d001e4, "e4d6"),
    (0x5b5b89b1380d5aa6, "c8d7"),
    (0xd785adacab1ed4c2, "g4d7"),
    (0x0304441be2cea420, "g4d7"),
    (0x9daf1de21955712e, "e4f6"),
    (0xaed608903ca287b7, "g4d7"),
    (0xcda11750b12fe9fd, "g4d7"),
    (0x460e88e10c9a9363, "g4d7"),
    (0x80a8ce63ea468f9e, "e4d6"),
    (0x15f32e1c4e2fa49b, "g4d7"),
    (0xe693afea3318b671, "g4d7"),
    (0xa8a4d7816ef78aaf, "g4d7"),
    (0x8c86438049494bc6, "g4d7"),
    (0x568dc31976fc3511, "e4d6"),
    (0x37af55caafc608f2, "g4e6"),
    (0x8c9cd859e38797e0, "e4d6"),
    (0x2f98eb074250c259, "e4f6"),
    (0xb0a462ae9c236c5b, "e4f6"),
    (0xbca7e5d355043f17, "d1f3"),
    (0x74fc782aa0d05272, "e4f6"),
    (0x60baadbd9b1f84d8, "e4f6"),
    (0x5bcf8dcebd413209, "f5d7"),
    (0x275742c1c03c1b0d, "e4f6"),
    (0xd2bf5b6ac7fef38a, "e4f6"),
    (0x45cea9ae1b5fb66e, "e4f6"),
    (0x7b35a7e23b468307, "e4d6"),
    (0xe417fa4b1d0c983f, "e4d6"),
    (0x5bea1bd61354ba72, "e4f6"),
    (0xf9441411490e2f67, "e4d6"),
    (0xcedbe6696e666508, "e4f6"),
    (0xd5721e5c4477bd07, "e4f6"),
    (0xcf56a4a05e468038, "e4f6"),
    (0x44f93b11e3f3faa6, "e4f6"),
    (0x825f7d93052fe65b, "e4f6"),
    (0xf275a7e6811b0f57, "f5d7"),
    (0x2e46dfcb533b0e4e, "e4d6"),
    (0x17049deca146cd5e, "e4f6"),
    (0xe4641c1adc71dfb4, "e4f6"),
    (0x8e71f070a6202203, "e4f6"),
    (0xb67eed597f243cbd, "e4f6"),
    (0xecfe0d03349eefbc, "e4d6"),
    (0xcdd0c507dce3bc13, "e4f6"),
    (0xf7be6b0e6201ae6a, "e4f6"),
    (0xa7c26447c84dc066, "e4f6"),
    (0xffa5c1a9213cf15b, "e4d6"),
    (0x62da86ee9969bc3d, "e4f6"),
    (0x3825f8dbd205bfd6, "e4d6"),
    (0xcb45792daf32ad3c, "e4d6"),
    (0xa1509547d563508b, "e4f6"),
    (0x14a4eacbcf98a513, "e4f6"),
    (0xe2f1a030afa0ce9b, "e4f6"),
    (0x94671684df558a42, "e4d6"),
    (0x8021c313e49a5ce8, "e4d6"),
    (0xc7cc2c6fbfb9c33d, "e4d6"),
    (0x322435c4b87b2bba, "e4d6"),
    (0xa555c70064da6e5e, "e4d6"),
    (0x1655abe9873cb011, "e4d6"),
    (0xb93754efa8e1eb53, "e4d6"),
    (0x35e970f23bf26537, "e4d6"),
    (0x460859267066d5ef, "e4d6"),
    (0xe1689945722215d5, "e4d6"),
    (0x4cbad5ceac4e3642, "e4d6"),
    (0x2fcdca0e21c35808, "e4d6"),
    (0xa46255bf9c762296, "e4d6"),
    (0x380460976ec704d2, "e4d6"),
    (0x6e4bd6b67f2abac6, "d2d3"),
    (0x0d361c2136e63c98, "e4d6"),
    (0x1970c9b60d29ea32, "e4d6"),
    (0x5e9d26ca560a75e7, "e4d6"),
    (0xab753f6151c89d60, "e4d6"),
    (0x3c04cda58d69d884, "g4e6"),
    (0x8f04a14c6e8f06cb, "e4d6"),
    (0x20665e4a41525d89, "e4d6"),
    (0xacb87a57d241d3ed, "e4d6"),
    (0xdf59538399d56335, "e4d6"),
    (0x783993e09b91a30f, "e4d6"),
    (0xd5ebdf6b45fd8098, "e4d6"),
    (0xb69cc0abc870eed2, "e4d6"),
    (0x3d335f1a75c5944c, "e4d6"),
    (0xa083a80bcbd16760, "g4e6"),
    (0x724af0fad483408e, "e4d6"),
    (0x660c256def4c9624, "e4d6"),
    (0x21e1ca11b46f09f1, "e4d6"),
    (0x49d1686111a3c89d, "e4d6"),
    (0xc86d6eb91a3577f4, "e4d6"),
    (0xd3c4968c3024affb, "e4d6"),
    (0xc9e02c702a1592c4, "e4d6"),
    (0x3c5c604b387e03d1, "e4d6"),
    (0x488e29a38645c625, "e4d6"),
    (0xe2d294caa822cd48, "e4d6"),
    (0xace5eca1f5cdf196, "e4d6"),
    (0x1411a8e8f70be54b, "e4d6"),
    (0x2ef96bc56e1936e1, "e4d6"),
    (0x4497e0c933ca4522, "g4f5"),
    (0x8fed80200359dd23, "e4d6"),
    (0xbedf51a4b8d6393f, "f5e6"),
    (0xbfc1127986af7837, "e4d6"),
    (0x34475caafdcf0f69, "e4d6"),
    (0x6ddb46b9c669b821, "f5e6"),
    (0x353d971a7091aefe, "e4d6"),
    (0x6696adf1107de781, "e4d6"),
    (0xbebe906c5238eac4, "e4d6"),
    (0x8e974b908e077cb4, "e4d6"),
    (0x7b2b07ab9c6ceda1, "e4d6"),
    (0x0ff94e4322572855, "e4d6"),
    (0xa5a5f32a0c302338, "e4d6"),
    (0xeb928b4151df1fe6, "e4d6"),
    (0x5366cf0853190b3b, "e4d6"),
    (0xb5ee3356724c57b4, "f5d5"),
    (0xe0b6ac3164d8207b, "e4d6"),
    (0xb31d96da04346904, "e4d6"),
    (0x319a9294c688b93a, "e4d6"),
    (0xd1847db5df57c467, "d5e6"),
    (0xa5f393541e46eeb8, "e4d6"),
    (0x5b1c70bb9a4ef231, "e4d6"),
    (0xd0b3ef0a27fb88af, "e4d6"),
    (0x3269e01191006cce, "e4d6"),
    (0xc86676ba05d81b3f, "e4d6"),
    (0x3e19b06a45969163, "e4d6"),
    (0x3ea54a985af1333c, "e4d6"),
    (0xec1565e602cf420f, "e4d6"),
    (0x1f75e4107ff850e5, "e4d6"),
    (0x51429c7b22176c3b, "e4d6"),
    (0xe9b6d83220d178e6, "e4d6"),
    (0xdaec8f0bb0701e14, "e4d6"),
    (0x6ecef9e73770a3b4, "e4d6"),
    (0x9dae78114a47b15e, "e4d6"),
    (0xd399007a17a88d80, "e4d6"),
    (0x6b6d4433156e995d, "e4d6"),
    (0xf79ff342dec3156e, "e4d6"),
    (0x04ff72b4a3f40784, "e4d6"),
    (0x4ac80adffe1b3b5a, "e4d6"),
    (0xa6e24d3bfe297236, "e4f6"),
    (0x530a5490f9eb9ab1, "e4f6"),
    (0xff720cbaaf9022eb, "f1b5"),
    (0xd1552b52580b6f58, "e4f6"),
    (0x507ea7bda3f07012, "d1h5"),
    (0x65a2f5b12319f104, "e4d6"),
    (0xda5f142c2d41d349, "e4f6"),
    (0xf50a794eb4e4ad7a, "e4f6"),
    (0x4ee3ab5a6053e903, "e4f6"),
    (0x73c0a81cbf0e666c, "d1g4"),   // Qg4! mate in 2-3 for all responses,
    (0xe8c2097a5ee1e7e3, "e4d6"),
    (0xfc84dced652e3149, "e4d6"),
    (0xbb6933913e0dae9c, "e4d6"),
    (0x4e812a3a39cf461b, "e4d6"),
    (0xd9f0d8fee56e03ff, "e4d6"),
    (0x6af0b4170688ddb0, "e4d6"),
    (0xda82053fdad6fad3, "f2f3"),   // f3 reclaims Ne4+Qg4 (Bc4 fails — leaves queen on g4),
    (0x78298b1be33d2dae, "e4d6"),
    (0xc5924b11295586f2, "e4d6"),
    (0x494c6f0cba460896, "e4d6"),
    (0x5368d5f0a07735a9, "e4d6"),
    (0xd8c74a411dc24f37, "e4d6"),
    (0x8a9e4500b2111f80, "f2f3"),
    (0xd78a66c6f8b4ac14, "e4d6"),
    (0xc3ccb351c37b7abe, "e4d6"),
    (0x84215c2d9858e56b, "e4d6"),
    (0x71c945869f9a0dec, "e4d6"),
    (0xddb11dacc9e1b5b6, "g4d7"),
    (0xf3963a443e7af805, "g4d7"),
    (0xe6b8b742433b4808, "e4d6"),
    (0x55b8dbaba0dd9647, "e4d6"),
    (0xfada24ad8f00cd05, "e4d6"),
    (0x760400b01c134361, "e4d6"),
    (0x05e529645787f3b9, "e4d6"),
    (0xa285e90755c33383, "e4d6"),
    (0x0f57a58c8baf1014, "e4d6"),
    (0x6c20ba4c06227e5e, "e4d6"),
    (0xe78f25fdbb9704c0, "e4d6"),
    (0x7a3fd2ec0583f7ec, "g4e6"),
    (0xdb54c5836ee8232f, "e4d6"),
    (0xcf1210145527f585, "e4d6"),
    (0x88ffff680e046a50, "e4d6"),
    (0xe0cf5d18abc8ab3c, "e4d6"),
    (0x61735bc0a05e1455, "e4d6"),
    (0x7adaa3f58a4fcc5a, "e4d6"),
    (0x60fe1909907ef165, "e4d6"),
    (0x9542553282156070, "e4d6"),
    (0xe1901cda3c2ea584, "e4d6"),
    (0x4bcca1b31249aee9, "e4d6"),
    (0x05fbd9d84fa69237, "e4d6"),
    (0xbd0f9d914d6086ea, "e4d6"),
    (0xf4451122a04ba66d, "e4d6"),
    (0xed89d5b089a12683, "g4f5"),
    (0x26f3b559b932be82, "e4d6"),
    (0x17c164dd02bd5a9e, "e4d6"),
    (0x16df27003cc41b96, "e4d6"),
    (0x9d5969d347a46cc8, "e4d6"),
    (0xc4c573c07c02db80, "f5e6"),
    (0x9c23a263cafacd5f, "e4d6"),
    (0xcf889888aa168420, "e4d6"),
    (0x17a0a515e8538965, "e4d6"),
    (0x27897ee9346c1f15, "e4d6"),
    (0xd23532d226078e00, "e4d6"),
    (0xa6e77b3a983c4bf4, "e4d6"),
    (0x0cbbc653b65b4099, "e4d6"),
    (0x428cbe38ebb47c47, "e4d6"),
    (0xfa78fa71e972689a, "e4d6"),
    (0x1cf0062fc8273415, "f5d5"),
    (0x49a89948deb343da, "e4d6"),
    (0x1a03a3a3be5f0aa5, "e4d6"),
    (0x9884a7ed7ce3da9b, "e4d6"),
    (0x789a48cc653ca7c6, "e4d6"),
    (0x0ceda62da42d8d19, "e4d6"),
    (0xf20245c220259190, "e4d6"),
    (0x79adda739d90eb0e, "e4d6"),
    (0x9b77d5682b6b0f6f, "e4d6"),
    (0x617843c3bfb3789e, "e4d6"),
    (0x97078513fffdf2c2, "e4d6"),
    (0x97bb7fe1e09a509d, "e4d6"),
    (0x450b509fb8a421ae, "e4d6"),
    (0xb66bd169c5933344, "e4d6"),
    (0xf85ca902987c0f9a, "e4d6"),
    (0x40a8ed4b9aba1b47, "e4d6"),
    (0x73f2ba720a1b7db5, "e4d6"),
    (0xb4728300f9223338, "e4d6"),
    (0x471202f6841521d2, "e4d6"),
    (0x09257a9dd9fa1d0c, "e4d6"),
    (0xc273714aaf05fafc, "e4d6"),
    (0xa8f41016d7f70692, "e4d6"),
    (0x4577bd50a3d6bc1b, "g4e6"),
    (0x26994c4748ca486c, "e4d6"),
    (0x32df99d073059ec6, "e4d6"),
    (0x753276ac28260113, "e4d6"),
    (0x1d02d4dc8deac07f, "e4d6"),
    (0x9cbed204867c7f16, "e4d6"),
    (0x87172a31ac6da719, "e4d6"),
    (0x9d3390cdb65c9a26, "e4d6"),
    (0x688fdcf6a4370b33, "e4d6"),
    (0x4e6a67cebf36cf8f, "e6e5"),
    (0x2788e33d0cfe3568, "e4d6"),
    (0x33ce36aa3731e3c2, "e4d6"),
    (0xe278b75fc51f5e00, "d5c6"),
    (0x9daf7d7ec2480212, "e4d6"),
    (0x8606854be859da1d, "e4d6"),
    (0x2442de7e3b64b871, "e4d6"),
    (0x88727b6cef2c494e, "e4d6"),
    (0xf557af1df92679dd, "e4d6"),
    (0x0f7066c4b953e7fd, "e4d6"),
    (0xf927ff662db08470, "e4d6"),
    (0x8a11157892433cd3, "e4d6"),
    (0x7349eaeb41ac2bea, "e4d6"),
    (0x1c5d951e1a0ccec7, "e4d6"),
    (0xb6012877346bc5aa, "e4d6"),
    (0x40c214556b42eda9, "e4d6"),
    (0xcb0d7e9e061eed9a, "e4d6"),
    (0xa163f5925bcd9e59, "g4g6"),
    (0xa721c6c4f8884b65, "e4d6"),
    (0xb3671353c3479dcf, "e4d6"),
    (0xad1abdaec9dd52c7, "c6d7"),
    (0x833d9a463e461f74, "e4f6"),
    (0x961317404307af79, "e4d6"),
    (0x25137ba9a0e17136, "e4d6"),
    (0x1c8b1a4e061e992f, "e4d6"),
    (0xd6c7e1059ad83419, "c6b7"),
    (0x19b06455ed3ce5fe, "e4d6"),
    (0x0df6b1c2d6f33354, "e4d6"),
    (0x4a1b5ebe8dd0ac81, "e4d6"),
    (0xbff347158a124406, "e4d6"),
    (0x138b1f3fdc69fc5c, "b7c6"),
    (0x3dac38d72bf2b1ef, "b7c6"),
    (0x2882b5d156b301e2, "e4d6"),
    (0x9b82d938b555dfad, "e4d6"),
    (0xa397fa16238ad284, "e4d6"),
    (0xb83e0223099b0a8b, "e4d6"),
    (0xc1ec8daf8e506444, "e4d6"),
    (0xcbdf2bf7420fba53, "e4d6"),
    (0x6cbfeb94404b7a69, "e4d6"),
    (0xc16da71f9e2759fe, "e4d6"),
    (0xa21ab8df13aa37b4, "e4d6"),
    (0x29b5276eae1f4d2a, "e4d6"),
    (0xd423f33d8f7d3bd4, "e4d6"),
    (0xb401cd62a72c0594, "e4d6"),
    (0xf2f16068f1ee28e3, "e4d6"),
    (0xec6c91b035bd2c4a, "e4d6"),
    (0x37b9a2f48429c6a3, "e4d6"),
    (0xb2d8d148af391d8d, "e4d6"),
    (0x3f189a50d877c56f, "e4d6"),
    (0x8b3aecbc5f7778cf, "e4d6"),
    (0x785a6d4a22406a25, "e4d6"),
    (0x8e9951687d694226, "e4d6"),
    (0x879626f990734947, "e4d6"),
    (0x93d0f36eabbc9fed, "e4d6"),
    (0x8b42f70c570461c4, "e4d6"),
    (0xd43d1c12f09f0038, "e4d6"),
    (0x21d505b9f75de8bf, "e4d6"),
    (0xb6a4f77d2bfcad5b, "e4d6"),
    (0x05a49b94c81a7314, "e4d6"),
    (0xaac66492e7c72856, "e4d6"),
    (0x2618408f74d4a632, "e4d6"),
    (0x3c3cfa736ee59b0d, "e4d6"),
    (0xb79365c2d350e193, "e4d6"),
    (0xe5ca6a837c83b124, "g7g8n"),
    (0xc7a03f951965a836, "e4d6"),
    (0x2a2392d36d4412bf, "g4h5"),
    (0xa459511dc88c433e, "e4d6"),
    (0xce37da11955f30fd, "g4h5"),
    (0x504cb5d316e56bcb, "e4d6"),
    (0xe46ec33f91e5d66b, "e4d6"),
    (0x170e42c9ecd2c481, "e4d6"),
    (0xe1cd7eebb3fbec82, "e4d6"),
    (0x9b955978e52a3c72, "e4g5"),
    (0x53b5cdb4ea9a0e0d, "e4d6"),
    (0xbfb3cbc7c497df0a, "e4d6"),
    (0x001ef75f8a764772, "e4d6"),
    (0xf5f6eef48db4aff5, "e4d6"),
    (0x62871c305115ea11, "e4d6"),
    (0xd18770d9b2f3345e, "e4d6"),
    (0x7ee58fdf9d2e6f1c, "e4d6"),
    (0xf23babc20e3de178, "e4d6"),
    (0x81da821645a951a0, "e4d6"),
    (0x26ba427547ed919a, "e4d6"),
    (0x8b680efe9981b20d, "e4d6"),
    (0xe81f113e140cdc47, "e4d6"),
    (0x63b08e8fa9b9a6d9, "e4d6"),
    (0x68188d767af0d46a, "e4d6"),
    (0xa9990d864ae53e89, "d2d3"),
    (0xcae4c7110329b8d7, "e4d6"),
    (0x26e2c1622d2469d0, "e4d6"),
    (0x994ffdfa63c5f1a8, "e4d6"),
    (0x6ca7e4516407192f, "e4d6"),
    (0xfbd61695b8a65ccb, "e4d6"),
    (0x48d67a7c5b408284, "e4d6"),
    (0xe7b4857a749dd9c6, "e4d6"),
    (0x6b6aa167e78e57a2, "e4d6"),
    (0x188b88b3ac1ae77a, "e4d6"),
    (0xbfeb48d0ae5e2740, "e4d6"),
    (0x1239045b703204d7, "e4d6"),
    (0x714e1b9bfdbf6a9d, "e4d6"),
    (0xfae1842a400a1003, "e4d6"),
    (0x6751733bfe1ee32f, "g4f5"),
    (0xf249787aff336dc1, "e4d6"),
    (0x1e4f7e09d13ebcc6, "e4d6"),
    (0xa1e242919fdf24be, "e4d6"),
    (0xc9d2e0e13a13e5d2, "e4d6"),
    (0xc37ba9fe44bc89dd, "e4d6"),
    (0x5fbd51f8d8cbe13e, "e4d6"),
    (0x49e3a4f001a5bf8b, "e4d6"),
    (0xc24c3b41bc10c515, "e4d6"),
    (0x91b19dbcfea5f2ed, "e4d6"),
    (0x62d11c4a8392e007, "e4d6"),
    (0x2ce66421de7ddcd9, "e4d6"),
    (0x94122068dcbbc804, "e4d6"),
    (0x1ad4ceab463a29ce, "e4d6"),
    (0x83453bf90605c16d, "g4f4"),
    (0x0c4b9baf0e709e9b, "e4d6"),
    (0xe04d9ddc207d4f9c, "e4d6"),
    (0x0670e0c53f258739, "f4g4"),
    (0x803908bc0a125405, "e4d6"),
    (0xaf0afd581783347e, "e4d6"),
    (0x6c3f0ecf241f8502, "e4d6"),
    (0xb10bd938b19db019, "e4d6"),
    (0x020bb5d1527b6e56, "e4d6"),
    (0xad694ad77da63514, "e4d6"),
    (0x21b76ecaeeb5bb70, "e4d6"),
    (0x83f335ff3d88d91c, "e4d6"),
    (0xc7bda4e35ba333b5, "e4d6"),
    (0x12a50ffa72540ac2, "e4d6"),
    (0xeebec1f49617c30b, "e4d6"),
    (0xe46ea74c29d73fb7, "e4d6"),
    (0xee663c23288024ba, "e4d6"),
    (0xe3c1ed7a0b84cb29, "e4d6"),
    (0xa8c18d45bfbf8690, "e4d6"),
    (0x5e9614e72b5ce51d, "e4d6"),
    (0x73f3c8c931a0a8be, "g4d7"),
    (0xe057c28b6617d8d4, "e4d6"),
    (0xbad722d12dad0bd5, "e4d6"),
    (0x81755458bf2de537, "e4c5"),
    (0x2857c72dc8beca8a, "c7c8q"),
    (0xd96e1f103b8979d8, "c6b5"),
    (0x356819631584a8df, "c6d7"),
    (0xdc9de5bbd7d06123, "c6a4"),
    (0x6d1320dc21e473b3, "c6a4"),
    (0x5b5ca27d63e0438b, "c6a4"),
    (0x63498153f53f4ea2, "c6a4"),
    (0x78e07966df2e96ad, "c6a4"),
    (0xba96fad63a1fe6f4, "c6a4"),
    (0x49f67b204728f41e, "c6a4"),
    (0x07c1034b1ac7c8c0, "c6a4"),
    (0xc194455aa8e63fbb, "e4c5"),
    (0xb836a463141d94f0, "e4d6"),
    (0x8e7926c25619a4c8, "e4d6"),
    (0xb66c05ecc0c6a9e1, "e4d6"),
    (0xadc5fdd9ead771ee, "e4d6"),
    (0x0f81a6ec39ea1382, "e4d6"),
    (0x9ed79ce97636c05c, "e4d6"),
    (0x62cc52e792750995, "e4d6"),
    (0xee7d5d36cb40fb99, "f4e5"),
    (0x743650b7009d5788, "e4d6"),
    (0x983056c42e90868f, "e4d6"),
    (0xb1c604d5c97c3ce0, "d5c6"),
    (0xce11cef4ce2b60f2, "e4d6"),
    (0xd5b836c1e43ab8fd, "e4d6"),
    (0x77fc6df43707da91, "e4d6"),
    (0xdbccc8e6e34f2bae, "e4d6"),
    (0x17ceb571010bc8a4, "e4d6"),
    (0x5cced54eb530851d, "e4d6"),
    (0xaa994cec21d3e690, "e4d6"),
    (0xd9afa6f29e205e33, "e4d6"),
    (0x364828d97f65140c, "f4f5"),
    (0xb34b57dbf22fa859, "e4d6"),
    (0x6e7f802c67ad9d42, "e4d6"),
    (0x6f61c3f159d4dc4a, "e4d6"),
    (0x5c876cebebb8f447, "e4d6"),
    (0x3fc114bc63375c38, "e4d6"),
    (0xf2f8e582211522a6, "f5e6"),
    (0xff6dea429515e5fe, "e4d6"),
    (0xe238419fb244a60a, "e4d6"),
    (0x996fd7c86a4d26e7, "e4d6"),
    (0x84d654d7eb1437c0, "e4d6"),
    (0x2208cb5a83c1d9ff, "e4d6"),
    (0x229c6eda480b9271, "e4d6"),
    (0x8f7d58129931e847, "e4d6"),
    (0x16ecad40d90e00e4, "f5d5"),
    (0x4ced77c344ac1b80, "e4d6"),
    (0x91d9a034d12e2e9b, "e4d6"),
    (0xe5ae4ed5103f0444, "e4d6"),
    (0xa3214cf35d3b479e, "e4d6"),
    (0x939a92599701d819, "e4d6"),
    (0xd89af266233a95a0, "e4d6"),
    (0x7e446deb4bef7b9f, "e4d6"),
    (0x7ed0c86b80253011, "e4d6"),
    (0x8897c2825ab45bd2, "e4d6"),
    (0x6c3cb24121826bc0, "e4d6"),
    (0x9f5c33b75cb5792a, "e4d6"),
    (0x81e24df3fd6cc846, "e4d6"),
    (0x8176e87336a683c8, "e4d6"),
    (0x6fb37e690fe601b7, "e4d6"),
    (0x24b31e56bbdd4c0e, "e4d6"),
    (0xd2e487f42f3e2f83, "e4d6"),
    (0xa1d26dea90cd9720, "e4d6"),
    (0x1d3e543b85bf9a5b, "e4d6"),
    (0xa91c22d702bf27fb, "e4d6"),
    (0x5a7ca3217f883511, "e4d6"),
    (0x144bdb4a226709cf, "e4d6"),
    (0xacbf9f0320a11d12, "e4d6"),
    (0x304d2872eb0c9121, "e4d6"),
    (0xc32da984963b83cb, "e4d6"),
    (0x8d1ad1efcbd4bf15, "e4d6"),
    (0xcee437120eafe12e, "e4d6"),
    (0x0f65b7e23eba0bcd, "d1g4"),
    (0x2f10d24a6b2e63ac, "e4d6"),
    (0xc316d4394523b2ab, "e4d6"),
    (0x7cbbe8a10bc22ad3, "e4d6"),
    (0x8953f10a0c00c254, "e4d6"),
    (0x1e2203ced0a187b0, "e4d6"),
    (0xad226f27334759ff, "e4d6"),
    (0x1d50de0fef197e9c, "f1e2"),
    (0x2e24286685099f5d, "g4h5"),
    (0x22f0f993427eb7de, "g4h5"),
    (0xc2222e15ab044e5a, "g4h5"),
    (0x7d8f128de5e5d622, "g4h5"),
    (0x88670b26e2273ea5, "g4h5"),
    (0x1f16f9e23e867b41, "g4h5"),
    (0xac16950bdd60a50e, "g4h5"),
    (0x03746a0df2bdfe4c, "e2f3"),
    (0x57b1bee29443e6c6, "e4d6"),
    (0x5b656f175334ce45, "e4d6"),
    (0x7ddef2c3a82a4d4e, "e4d6"),
    (0xbbb7b891ba4e37c1, "e4d6"),
    (0x041a8409f4afafb9, "e4d6"),
    (0xf1f29da2f36d473e, "e4d6"),
    (0x66836f662fcc02da, "e4d6"),
    (0xd583038fcc2adc95, "e4d6"),
    (0xec1b62686ad5348c, "e4d6"),
    (0x67b4fdd9d7604e12, "e4d6"),
    (0x35edf29878b31ea5, "g7g8n"),
    (0x1787a78e1d5507b7, "e4d6"),
    (0xfa040ac86974bd3e, "g4g6"),
    (0x747ec906ccbcecbf, "e4d6"),
    (0x1e10420a916f9f7c, "g4g6"),
    (0x806b2dc812d5c44a, "e4d6"),
    (0x34495b2495d579ea, "e4d6"),
    (0xc729dad2e8e26b00, "e4d6"),
    (0x5a750648cbf68bcf, "e4d6"),
    (0x713a794c688cc481, "e4d6"),
    (0x31eae6f0b7cb4303, "e4d6"),
    (0x4bb2c163e11a93f3, "e4g5"),
    (0x8faa4e1061ae7028, "g4h5"),
    (0x3c48c958a40043b0, "g4h5"),
    (0xb29b1224322b15e3, "g4h5"),
    (0xaa3eaeb9a85947c0, "g4h5"),
    (0x808ac68db33ef03d, "g4h5"),
    (0xe29315e4a2895575, "g4h5"),
    (0xf611935fcc9ff178, "e2f3"),
    (0x44c8e58de510be55, "g4h5"),
    (0x481c3478226796d6, "g4h5"),
    (0xa8cee3fecb1d6f52, "g4h5"),
    (0x1763df6685fcf72a, "g4h5"),
    (0xe28bc6cd823e1fad, "g4h5"),
    (0x10a78261869546d2, "b7c6"),
    (0x60d4b90f23deea44, "f3c6"),
    (0x75fa34095e9f5a49, "g4h5"),
    (0xc6fa58e0bd798406, "g4h5"),
    (0x6998a7e692a4df44, "f3e2"),
    (0x63096f8fa68a26eb, "g4g6"),
    (0x6fddbe7a61fd0e68, "g4g6"),
    (0x496623ae9ae38d63, "g4h5"),
    (0x8f0f69fc8887f7ec, "g4g6"),
    (0x30a25564c6666f94, "g4g6"),
    (0xc54a4ccfc1a48713, "g4g6"),
    (0x37660863c50fde6c, "e2b5"),
    (0x4715330d604472fa, "e2b5"),
    (0x523bbe0b1d05c2f7, "g4g6"),
    (0xe13bd2e2fee31cb8, "g4g6"),
    (0xcd3f2f4f541daf12, "g4g6"),
    (0xb166202d09b97946, "g4g6"),
    (0x17bffb3d95c106a9, "g4g6"),
    (0xd8a3b305581cf4a1, "g4g6"),
    (0x530c2cb4e5a98e3f, "g4g6"),
    (0x233f76e32f9cc79a, "f2f3"),
    (0x207f49a337e9952b, "g4g6"),
    (0x2cab9856f09ebda8, "g4g6"),
    (0x0a1005820b803ea3, "g4h5"),
    (0xcc794fd019e4442c, "g4g6"),
    (0x73d473485705dc54, "g4g6"),
    (0x863c6ae350c734d3, "g4g6"),
    (0x74102e4f546c6dac, "e2b5"),
    (0x04631521f127c13a, "e2b5"),
    (0x114d98278c667137, "g4g6"),
    (0xa24df4ce6f80af78, "g4g6"),
    (0xd2fdb21d639a318f, "g4g6"),
    (0xf210060198daca86, "g4g6"),
    (0x54c9dd1104a2b569, "g4g6"),
    (0xf8a28ae944f2292b, "g4g6"),
    (0x0c9b2ef399155762, "e2c4"),
    (0xfd392cd1b8c91014, "f7d6"),
    (0xf1edfd247fbe3897, "f7d6"),
    (0xd75660f084a0bb9c, "f7d6"),
    (0x908c16e670495b29, "f7d6"),
    (0xae92163ad825596b, "f7d6"),
    (0x5b7a0f91dfe7b1ec, "f7d6"),
    (0x28e577793dc172a9, "f7d6"),
    (0xd92570537e074405, "c4b5"),
    (0xcc0bfd550346f408, "f7d6"),
    (0x7f0b91bce0a02a47, "f7d6"),
    (0x0fbbd76fecbab4b0, "f7d6"),
    (0x2f56637317fa4fb9, "f7d6"),
    (0x898fb8638b823056, "f7d6"),
    (0x25e4ef9bcbd2ac14, "f7d6"),
    (0x4693f05b465fc25e, "f7d6"),
    (0xcd3c6feafbeab8c0, "f7d6"),
    (0x9ec1c917b95f8f38, "f7d6"),
    (0xd5e9f65ad9c92b50, "f7d6"),
    (0xf0fd947be77c7d1d, "f7d6"),
    (0xdbb2eb7f44063253, "f7d6"),
    (0x2396308a9987a10c, "f7d6"),
    (0x508885e6f2d9f07e, "f7d6"),
    (0xf73e8ce707fd2231, "g4g6"),
    (0x9bd59529c97f4761, "g4g6"),
    (0x107a0a9874ca3dff, "g4g6"),
    (0x4387ac65367f0a07, "g4g6"),
    (0x08af932856e9ae6f, "g4g6"),
    (0x06f48e0dcb26b76c, "g4g6"),
    (0xfed055f816a72433, "g4g6"),
    (0x8dcee0947df97541, "g4g6"),
    (0x3c7c362242b0e01e, "e2c4"),
    (0x00f18a49a71cb9c7, "g4g6"),
    (0x4bd9b504c78a1daf, "g4g6"),
    (0x6ecdd725f93f4be2, "g4g6"),
    (0x4582a8215a4504ac, "g4g6"),
    (0xceb8c6b8ec9ac681, "g4g6"),
    (0x7f0a100ed3d353de, "e2c4"),
    (0xe54683fb01b75120, "g4h5"),
    (0xeafea54d178737ac, "g4h5"),
    (0x96a7aa2f4a23e1f8, "g4h5"),
    (0x31c76a4c486721c2, "g4g6"),
    (0xff6239071b866c1f, "g4h5"),
    (0x74cda6b6a6331681, "g4h5"),
    (0x04fefce16c065f24, "f3e2"),
    (0x0e6f34885828a68b, "g4h5"),
    (0x02bbe57d9f5f8e08, "g4h5"),
    (0xe26932fb7625778c, "g4h5"),
    (0x5dc40e6338c4eff4, "g4h5"),
    (0xa82c17c83f060773, "g4h5"),
    (0x5a0053643bad5e0c, "e2b5"),
    (0x2a73680a9ee6f29a, "e2b5"),
    (0x3f5de50ce3a74297, "g4h5"),
    (0x8c5d89e500419cd8, "g4h5"),
    (0xa0597448aabf2f72, "g4h5"),
    (0xdc007b2af71bf926, "g4h5"),
    (0x7b60bb49f55f391c, "g4g6"),
    (0xd6b2f7c22b331a8b, "g4h5"),
    (0x228b53d8f6d464c2, "g7g8n"),
    (0xd92ef1cc683c1191, "g4h5"),
    (0xb5c5e802a6be74c1, "g4h5"),
    (0x3e6a77b31b0b0e5f, "g4h5"),
    (0x6d97d14e59be39a7, "g4h5"),
    (0x26bfee0339289dcf, "g4h5"),
    (0xd0c028d379661793, "g4h5"),
    (0xa3de9dbf123846e1, "g4h5"),
    (0x126c4b092d71d3be, "e2c4"),
    (0x2730004be4862179, "g4h5"),
    (0x6c183f0684108511, "g4h5"),
    (0xe9794cbaaf005e3f, "g4h5"),
    (0x58cb9a0c9049cb60, "f3d5"),
    (0x958ef4ec7b9f4d17, "g4h5"),
    (0x1e216b5dc62a3789, "g4h5"),
    (0x122c789a3b29861b, "g7g8n"),
    (0x6e12310a0c1f7e2c, "e2f3"),
    (0x3ad7e5e56ae166a6, "e4d6"),
    (0x36033410ad964e25, "e4d6"),
    (0xd6d1e39644ecb7a1, "e4d6"),
    (0x697cdf0e0a0d2fd9, "e4d6"),
    (0x9c94c6a50dcfc75e, "e4d6"),
    (0x0be53461d16e82ba, "e4d6"),
    (0xb8e5588832885cf5, "e4d6"),
    (0x163382b5c41da4ef, "e4d6"),
    (0xed9620a15af5d1bc, "e4d6"),
    (0x817d396f9477b4ec, "e4d6"),
    (0x0ad2a6de29c2ce72, "e4d6"),
    (0x976251cf97d63d5e, "g4h5"),
    (0x19189201321e6cdf, "e4d6"),
    (0x7376190d6fcd1f1c, "g4h5"),
    (0xed0d76cfec77442a, "e4d6"),
    (0x592f00236b77f98a, "e4d6"),
    (0xaa4f81d51640eb60, "e4d6"),
    (0xe478f9be4bafd7be, "e4d6"),
    (0x5c8cbdf74969c363, "e4d6"),
    (0x26d49a641fb81393, "e4g5"),
    (0xddc580ca2aee2580, "g4h5"),
    (0x705f52747739c929, "g4h5"),
    (0x39d1c808d2f507c2, "g4h5"),
    (0xf9febb4c039fbdd1, "g4h5"),
    (0x4ddccda0849f0071, "g4h5"),
    (0xbebc4c56f9a8129b, "g4h5"),
    (0x487f7074a6813a98, "g4h5"),
    (0x322757e7f050ea68, "e2c4"),
    (0xbffb502bd6f2a9e1, "e4d6"),
    (0x024090211c9a02bd, "e4d6"),
    (0x8e9eb43c8f898cd9, "e4d6"),
    (0x94ba0ec095b8b1e6, "e4d6"),
    (0x1f159171280dcb78, "e4d6"),
    (0x4d4c9e3087de9bcf, "g7g8n"),
    (0x6f26cb26e23882dd, "e4d6"),
    (0x82a5666096193854, "g4f5"),
    (0x240ba72f5a08b1bf, "e4d6"),
    (0xc80da15c740560b8, "e4d6"),
    (0x77a09dc43ae4f8c0, "e4d6"),
    (0x1f903fb49f2839ac, "e4d6"),
    (0x153976abe18755a3, "e4d6"),
    (0x89ff8ead7df03d40, "e4d6"),
    (0x9fa17ba5a49e63f5, "e4d6"),
    (0x140ee414192b196b, "e4d6"),
    (0xbb9774abee6f1405, "f5e5"),
    (0xe05a380d3931b127, "e4d6"),
    (0x0c5c3e7e173c6020, "e4d6"),
    (0x25aa6c6ff0d0da4f, "f1b5"),
    (0x5a7da64ef787865d, "e4d6"),
    (0x41d45e7bdd965e52, "e4d6"),
    (0xe390054e0eab3c3e, "e4d6"),
    (0x4fa0a05cdae3cd01, "e4d6"),
    (0x83a2ddcb38a72e0b, "e4d6"),
    (0xc8a2bdf48c9c63b2, "e4d6"),
    (0x3ef52456187f003f, "e4d6"),
    (0x4dc3ce48a78cb89c, "e4d6"),
    (0xc3c52eb78ef14d7e, "e4d6"),
    (0x47f342e95b9e2e93, "e4d6"),
    (0xb493c31f26a93c79, "e4d6"),
    (0x4250ff3d7980147a, "e4d6"),
    (0x9b11937f12293d22, "e4d6"),
    (0x66b12ea26e021a16, "g4e6"),
    (0xc39099429f8acca4, "e4d6"),
    (0x2f969f31b1871da3, "e4d6"),
    (0x79b70701513cfbde, "e4d6"),
    (0x621eff347b2d23d1, "e4d6"),
    (0x783a45c8611c1eee, "e4d6"),
    (0x36bb0910ca7862ac, "f1d3"),
    (0x93d3db2bf901a473, "e4d6"),
    (0x7fd5dd58d70c7574, "e4d6"),
    (0x99e8a041c854bdd1, "c6a4"),
    (0xb7cf87a93fcff062, "c6a4"),
    (0x29f4456837b79309, "e4d6"),
    (0x325dbd5d1da64b06, "e4d6"),
    (0x4b8f32d19a6d25c9, "e4d6"),
    (0x3faf4703f9ec1055, "e4d6"),
    (0xe6dc54ea54763be4, "e4d6"),
    (0x351dcbeb25c4f3f8, "e4d6"),
    (0x287907a107977639, "e4d6"),
    (0xf02b3eedf8973b5f, "e4d6"),
    (0x034bbf1b85a029b5, "e4d6"),
    (0x4d7cc770d84f156b, "e4d6"),
    (0x862acca7aeb0f29b, "e4d6"),
    (0x96403fee96ecaf22, "e4d6"),
    (0xa0687c849e1c5388, "e4d6"),
    (0x5308fd72e32b4162, "e4d6"),
    (0xa5cbc150bc026961, "e4d6"),
    (0xf8ca4160edb84120, "e4d6"),
    (0x4ce8378c6ab8fc80, "e4d6"),
    (0xbf88b67a178fee6a, "e4d6"),
    (0x494b8a5848a6c669, "e4d6"),
    (0xcb613b2ffafdad00, "e4d6"),
    (0x1127b030c7b40388, "e4d6"),
    (0xc15a4045cba8b4a2, "f1b5"),
    (0x9f6a419630610ce0, "e4f6"),
    (0xfa53eaab4172491c, "e4d6"),
    (0x49538642a2949753, "e4d6"),
    (0x6e56eb42c7c8e65b, "d1h5"),
    (0x5b8ab94e4721674d, "e4d6"),
    (0xe47758d349794500, "e4d6"),
    (0xcb2235b1d0dc3b33, "e4d6"),
    (0x7146a56c344b9a7a, "e4d6"),
    (0x6aef5d591e5a4275, "e4d6"),
    (0x00dcc19e08398abb, "e4d6"),
    (0xfb647814b9de05d4, "e4d6"),
    (0x4de8e4e3db36f025, "d1f3"),
    (0xb96fb2591953a5ff, "e4d6"),
    (0x63293946241a0b77, "e4d6"),
    (0xd4ecc44d1d877ac9, "e4d6"),
    (0xb354c9332806bc5d, "f1b5"),
    (0xed64c8e0d3cf041f, "f1b5"),
    (0x885d63dda2dc41e3, "e4d6"),
    (0x3b5d0f34413a9fac, "e4d6"),
    (0x9679d1a5aad74dff, "e4d6"),
    (0xb92cbcc7337233cc, "e4d6"),
    (0x03482c1ad7e59285, "e4d6"),
    (0x18e1d42ffdf44a8a, "e4d6"),
    (0x72d248e8eb978244, "e4d6"),
    (0x896af1625a700d2b, "e4d6"),
    (0x6bb0fe79ec8be94a, "e4d6"),
    (0x29f7d66965f22839, "e4d6"),
    (0xb06ecc40fb5182d2, "e4d6"),
    (0xaea6e41ef06caaa2, "e4d6"),
    (0xaee265877e2728e3, "e4d6"),
    (0x1c20fe6bcae91a1a, "e4d6"),
    (0xa899dee9fb6b322c, "e4d6"),
    (0x5bf95f1f865c20c6, "e4d6"),
    (0x6d8fc48ee78c71c4, "e4d6"),
    (0xdcececf19d89201c, "e4d6"),
    (0x44a57fd5c038a9d4, "d1g4"),
    (0xeffa76de7dd9efbc, "e4d6"),
    (0xc0c9833a60488fc7, "e4d6"),
    (0x31c6b23661d95273, "e4d6"),
    (0x35bcfdc140904134, "e4d6"),
    (0xf26e26f1755fc57b, "e4d6"),
    (0xdec8a75ac6560ba0, "e4d6"),
    (0x6dc8cbb325b0d5ef, "e4d6"),
    (0xddba7a9bf9eef28c, "c8d7"),
    (0x7f11f4bfc00525f1, "e4d6"),
    (0xc2aa34b50a6d8ead, "e4d6"),
    (0x4e7410a8997e00c9, "e4d6"),
    (0x5450aa54834f3df6, "e4d6"),
    (0x19597367d8265b95, "e4d6"),
    (0xfe04753ae510e4f2, "c8d7"),
    (0xafcc6fb2f4cf0ecd, "e4d6"),
    (0x8c0293187c4f7090, "e4d6"),
    (0x7f6212ee0178627a, "e4d6"),
    (0x8c3fe8e711cb5cdf, "e4d6"),
    (0x1577fe847b299fcd, "e4f6"),
    (0x70e8a4068ab34a75, "e4d6"),
    (0xcf7c7e1d449ce11a, "e4d6"),
    (0xae5ee8ce9da6dcf9, "e4c5"),
    (0x6a82583d37a3e467, "e4f6"),
    (0xeba9d4d2cc58fb2d, "d1h5"),
    (0xde7586de4cb17a3b, "e4d6"),
    (0x6188674342e95876, "e4d6"),
    (0x4edd0a21db4c2645, "e4d6"),
    (0xf534d8350ffb623c, "e4d6"),
    (0xc817db73d0a6ed53, "d1g4"),
    (0xe862bedb85328532, "e4d6"),
    (0x365e7a33993238fd, "e4d6"),
    (0xd9506f5f3ebd612e, "e4d6"),
    (0x6a5003b6dd5bbf61, "e4d6"),
    (0xc532fcb0f286e423, "e4d6"),
    (0x49ecd8ad61956a47, "e4d6"),
    (0x3a0df1792a01da9f, "e4d6"),
    (0x9d6d311a28451aa5, "e4d6"),
    (0x30bf7d91f6293932, "e4d6"),
    (0x53c862517ba45778, "e4d6"),
    (0x8b9a5b1d84a41a1e, "e4d6"),
    (0x78fadaebf99308f4, "e4d6"),
    (0x36cda280a47c342a, "e4d6"),
    (0x8ba720e2e9203651, "d7d8r"),
    (0xfc5a00a09a2d0ef2, "e4d6"),
    (0x2d5b9a869d7f0315, "e4d6"),
    (0x2904f55a9a4bc89b, "e4d6"),
    (0x7f134c1f5993c0d6, "e4d6"),
    (0x2cb876f4397f89a9, "e4d6"),
    (0x52430e742e27a1c7, "e4d6"),
    (0xde9d2a69bd342fa3, "e4d6"),
    (0xad7c03bdf6a09f7b, "e4d6"),
    (0x0a1cc3def4e45f41, "e4d6"),
    (0xa7ce8f552a887cd6, "e4d6"),
    (0xc4b99095a705129c, "e4d6"),
    (0x4f160f241ab06802, "e4d6"),
    (0x3e39aa40c91559d4, "g7g8r"),
    (0x10845b659c565c00, "g7g8r"),
    (0xd912dd463422401a, "g7g8r"),
    (0x7efa2f9c95d65c58, "g7g8r"),
    (0x6b7a6852c69dec23, "g7g8r"),
    (0x1ceba9d958055ffa, "e4d6"),
    (0xef8b282f25324d10, "e4d6"),
    (0xa1bc504478dd71ce, "e4d6"),
    (0x1d9df74ff05f2054, "e4d6"),
    (0x5a004a1c79d5cd4f, "g7g8n"),
    (0x17f6fb07e297f858, "e4d6"),
    (0x4248a86f472001b4, "e4d6"),
    (0xb2d8851681cb517e, "h2h3"),
    (0x25f25ce88b375d03, "g4h5"),
    (0xd12e8199cfc6df39, "g4h5"),
    (0xc9bc85fb337e2110, "g4h5"),
    (0x96c36ee594e540ec, "g4h5"),
    (0x632b774e9327a86b, "g4h5"),
    (0x910733e2978cf114, "g4h5"),
    (0xe174088c32c75d82, "g4h5"),
    (0xe838166583bd6882, "d2d3"),
    (0x6bdfd414b54fba4c, "e4d6"),
    (0x9f030965f1be3876, "e4d6"),
    (0x87910d070d06c65f, "e4d6"),
    (0xa12a90d3f6184554, "e4d6"),
    (0xd8eee619aa9da7a3, "e4d6"),
    (0x2d06ffb2ad5f4f24, "e4d6"),
    (0x817ea798fb24f77e, "g8h6"),
    (0xce91a635a934af69, "g4g6"),
    (0x3a4d7b44edc52d53, "g4g6"),
    (0x01385b37cb9b9b82, "g4g6"),
    (0x0464e2f2ea635071, "g4h5"),
    (0x7da09438b6e6b286, "g4g6"),
    (0x88488d93b1245a01, "g4g6"),
    (0x30d87512df422d77, "g4g6"),
    (0xdd1154eb367822f0, "c1d2"),
    (0x5337c40f87e278f9, "h6f7"),
    (0x654321980676f1d7, "f7d6"),
    (0x919ffce9428773ed, "f7d6"),
    (0xaaeadc9a64d9c53c, "f7d6"),
    (0xafb6655f45210ecf, "f7d6"),
    (0xd672139519a4ec38, "f7d6"),
    (0x239a0a3e1e6604bf, "f7d6"),
    (0xfb89006c05d1b10f, "f7d6"),
    (0xecf9defdd1f64a59, "f7d6"),
    (0xa3444eded7cf25dc, "f7d6"),
    (0x9585c8a4f8d0382e, "f7d6"),
    (0xce9911f5377a6d85, "f7d6"),
    (0x3e73f5f487de770d, "f7d6"),
    (0xc0eccc8cad313685, "f7d6"),
    (0xc25d852128f9c2f7, "f7d6"),
    (0xa6165e9a21b7af3e, "f7d6"),
    (0xe18e868e5a8dcef5, "f7d6"),
    (0xed1c4ec0156e65e5, "f7d6"),
    (0xbc90470accc49fec, "f7d6"),
    (0x3d9dcfc0f2438c2a, "f7d6"),
    (0x00e2b86822de1bf7, "f7d6"),
    (0x8a89f4dd9ce1f0d7, "f7d6"),
    (0xc8cedccd159831a4, "f7d6"),
    (0x6e30c39e515d57f5, "f7d6"),
    (0x7c901745d44d30b7, "f7d6"),
    (0x679d06321c2b8933, "f7d6"),
    (0xc994308505c17d53, "f7d6"),
    (0x579c5a5d0c9e3cf2, "f7d6"),
    (0x15da86623676657f, "f7d6"),
    (0x7b6422d2f8d408f5, "f7d6"),
    (0xd8f172958f300425, "f7d6"),
    (0x07c6401e4c9825e4, "f7d6"),
    (0xe0d4df85b28706ee, "d8d7"),
    (0xedfe3f6ceef48eeb, "d8d7"),
    (0x9631037145f8928b, "d8d7"),
    (0xf5a568b3b65a63e8, "f7e5"),
    (0xbbaa3c5027a89ae0, "a1b1"),
    (0x5beeb744f0b7fdca, "h6f7"),
    (0xe77d0887c3efc772, "f7d6"),
    (0xdc0828f4e5b171a3, "f7d6"),
    (0xd9549131c449ba50, "f7d6"),
    (0xa090e7fb98cc58a7, "f7d6"),
    (0x5578fe509f0eb020, "f7d6"),
    (0x8d6bf40284b90590, "f7d6"),
    (0x9a1b2a93509efec6, "f7d6"),
    (0xd5a6bab056a79143, "f7d6"),
    (0xe3673cca79b88cb1, "f7d6"),
    (0xb87be59bb612d91a, "f7d6"),
    (0x4891019a06b6c392, "f7d6"),
    (0xb60e38e22c59821a, "f7d6"),
    (0xb4bf714fa9917668, "f7d6"),
    (0xd0f4aaf4a0df1ba1, "f7d6"),
    (0x976c72e0dbe57a6a, "f7d6"),
    (0x9bfebaae9406d17a, "f7d6"),
    (0xca72b3644dac2b73, "f7d6"),
    (0x4b7f3bae732b38b5, "f7d6"),
    (0x21f6f3e7005d7226, "f7d6"),
    (0x0e9c726f1dbf2d64, "f7d6"),
    (0x84f73edaa380c644, "f7d6"),
    (0xc6b016ca2af90737, "f7d6"),
    (0x18d237f0d035e36a, "f7d6"),
    (0x0a72e32b55258428, "f7d6"),
    (0xc416b1c5aa36ff24, "f7d6"),
    (0x117ff25c9d433dac, "f7d6"),
    (0xbf76c4eb84a9c9cc, "f7d6"),
    (0x217eae338df6886d, "f7d6"),
    (0x6338720cb71ed1e0, "f7d6"),
    (0x0d86d6bc79bcbc6a, "f7d6"),
    (0xae1386fb0e58b0ba, "f7d6"),
    (0x7124b470cdf0917b, "f7d6"),
    (0x96362beb33efb271, "d8d7"),
    (0x9b1ccb026f9c3a74, "d8d7"),
    (0xe0d3f71fc4902614, "d8d7"),
    (0xfbdba2b4893b557b, "f7e5"),
    (0x47af926a140c9de9, "h6f7"),
    (0x383e7eeb6a5a7dd8, "f7d6"),
    (0xc0490dda010a1180, "f7d6"),
    (0xc515b41f20f2da73, "f7d6"),
    (0xbcd1c2d57c773884, "f7d6"),
    (0x4939db7e7bb5d003, "f7d6"),
    (0xc9e79f9eb21cf160, "f7d6"),
    (0xa43ac0b552a9b939, "f7d6"),
    (0x54d024b4e20da3b1, "f7d6"),
    (0xaa4f1dccc8e2e239, "f7d6"),
    (0xa8fe54614d2a164b, "f7d6"),
    (0xccb58fda44647b82, "f7d6"),
    (0x8b2d57ce3f5e1a49, "f7d6"),
    (0x87bf9f8070bdb159, "f7d6"),
    (0xd633964aa9174b50, "f7d6"),
    (0x573e1e8097905896, "f7d6"),
    (0x2b31f4dd95d03b4e, "f7d6"),
    (0x3db7d6c9e4e61205, "f7d6"),
    (0x12dd5741f9044d47, "f7d6"),
    (0x98b61bf4473ba667, "f7d6"),
    (0xdaf133e4ce426714, "f7d6"),
    (0x049312de348e8349, "f7d6"),
    (0xa337e1c56012a9ef, "f7d6"),
    (0x3d3f8b1d694de84e, "f7d6"),
    (0x7f79572253a5b1c3, "f7d6"),
    (0x11c7f3929d07dc49, "f7d6"),
    (0xb252a3d5eae3d099, "f7d6"),
    (0x6d65915e294bf158, "f7d6"),
    (0x8a770ec5d754d252, "d8d7"),
    (0x875dee2c8b275a57, "d8d7"),
    (0xfc92d231202b4637, "d8d7"),
    (0xe79a879a6d803558, "f7e5"),
    (0x7cdab21932522b38, "h6f7"),
    (0xf4b3c9482159fc8c, "f7d6"),
    (0xfe60946c06ac6ca2, "f7d6"),
    (0x87a4e2a65a298e55, "f7d6"),
    (0x724cfb0d5deb66d2, "f7d6"),
    (0x47aef84ee973dde7, "f7d6"),
    (0x9f4fe0c674f70fe8, "f7d6"),
    (0x6fa504c7c4531560, "f7d6"),
    (0x913a3dbfeebc54e8, "f7d6"),
    (0x938b74126b74a09a, "f7d6"),
    (0xb56d7ab54fa0cbd1, "f7d6"),
    (0x030d118388e73bd1, "d8d7"),
    (0xf7c0afa9623acd53, "f7d6"),
    (0xfd4146270423007a, "f7d6"),
    (0xbccabff356e30788, "f7d6"),
    (0x587af19af27867d7, "f7d6"),
    (0x6c4b3ef3b1ceee47, "f7d6"),
    (0x1044d4aeb38e8d9f, "f7d6"),
    (0x06c2f6bac2b8a4d4, "f7d6"),
    (0x29a87732df5afb96, "f7d6"),
    (0xa3c33b87616510b6, "f7d6"),
    (0xe1841397e81cd1c5, "f7d6"),
    (0x3fe632ad12d03598, "f7d6"),
    (0xf13030f208ca9d44, "f7d6"),
    (0x9f8e9442c668f0ce, "f7d6"),
    (0x892783a6ccbd6648, "f7d6"),
    (0xe32cf68e7224dddf, "f7d6"),
    (0xb1022eb6f10a6483, "d8d7"),
    (0xbc28ce5fad79ec86, "d8d7"),
    (0xc7e7f2420675f0e6, "d8d7"),
    (0xdcefa7e94bde8389, "f7e5"),
    (0x79860bdc13aae0cb, "h6f7"),
    (0x82f85b637bd145a6, "f7d6"),
    (0x771042c87c13ad21, "f7d6"),
    (0xf7ce0628b5ba8c42, "f7d6"),
    (0x9a135903550fc41b, "f7d6"),
    (0x6af9bd02e5abde93, "f7d6"),
    (0x9466847acf449f1b, "f7d6"),
    (0x96d7cdd74a8c6b69, "f7d6"),
    (0xf29c166c43c206a0, "f7d6"),
    (0xb504ce7838f8676b, "f7d6"),
    (0xb9960636771bcc7b, "f7d6"),
    (0xe81a0ffcaeb13672, "f7d6"),
    (0x69178736903625b4, "f7d6"),
    (0x15186d6b9276466c, "f7d6"),
    (0x039e4f7fe3406f27, "f7d6"),
    (0x2cf4cef7fea23065, "f7d6"),
    (0xa69f8242409ddb45, "f7d6"),
    (0xe4d8aa52c9e41a36, "f7d6"),
    (0x3aba8b683328fe6b, "f7d6"),
    (0xe67e0d5d492be225, "f7d6"),
    (0x33174ec47e5e20ad, "f7d6"),
    (0x9d1e787367b4d4cd, "f7d6"),
    (0x031612ab6eeb956c, "f7d6"),
    (0x4150ce945403cce1, "f7d6"),
    (0x2fee6a249aa1a16b, "f7d6"),
    (0x534c08e82eed8c7a, "f7d6"),
    (0xc044ee1c6f8c0293, "d8d7"),
    (0xb45e9773d0f2af70, "d8d7"),
    (0xb974779a8c812775, "d8d7"),
    (0xc2bb4b87278d3b15, "d8d7"),
    (0xd9b31e2c6a26487a, "f7e5"),
    (0x00427d164f2f023c, "h6f7"),
    (0xc6ae216f27e19b03, "f7d6"),
    (0x8e0a70e2e93f6eb5, "f7d6"),
    (0xe3d72fc9098a26ec, "f7d6"),
    (0x133dcbc8b92e3c64, "f7d6"),
    (0xeda2f2b093c17dec, "f7d6"),
    (0xef13bb1d1609899e, "f7d6"),
    (0x8b5860a61f47e457, "f7d6"),
    (0xccc0b8b2647d859c, "f7d6"),
    (0xc05270fc2b9e2e8c, "f7d6"),
    (0x91de7936f234d485, "f7d6"),
    (0x10d3f1fcccb3c743, "f7d6"),
    (0x6cdc1ba1cef3a49b, "f7d6"),
    (0x7a5a39b5bfc58dd0, "f7d6"),
    (0x5530b83da227d292, "f7d6"),
    (0xdf5bf4881c1839b2, "f7d6"),
    (0x9d1cdc989561f8c1, "f7d6"),
    (0x437efda26fad1c9c, "f7d6"),
    (0x9fba7b9715ae00d2, "f7d6"),
    (0x34c0eb848d0529d1, "f7d6"),
    (0xe4da0eb93b31363a, "f7d6"),
    (0x7ad26461326e779b, "f7d6"),
    (0x3894b85e08862e16, "f7d6"),
    (0x562a1ceec624439c, "f7d6"),
    (0xf5bf4ca9b1c04f4c, "f7d6"),
    (0x2a887e2272686e8d, "f7d6"),
    (0xcd9ae1b98c774d87, "d8d7"),
    (0xc0b00150d004c582, "d8d7"),
    (0xbb7f3d4d7b08d9e2, "d8d7"),
    (0xa07768e636a3aa8d, "f7e5"),
    (0xf5aa64bd48edeabb, "h6f7"),
    (0x8343ca015c7abb66, "f7d6"),
    (0x7be26949eefd8632, "f7d6"),
    (0x163f36620e48ce6b, "f7d6"),
    (0xe6d5d263beecd4e3, "f7d6"),
    (0x1afba2b611cb6119, "f7d6"),
    (0x7eb0790d18850cd0, "f7d6"),
    (0x35ba69572c5cc60b, "f7d6"),
    (0x6436609df5f63c02, "f7d6"),
    (0xe53be857cb712fc4, "f7d6"),
    (0x9934020ac9314c1c, "f7d6"),
    (0x8fb2201eb8076557, "f7d6"),
    (0xa0d8a196a5e53a15, "f7d6"),
    (0x9b9444c5ef9402ac, "f7d6"),
    (0x68f4c53392a31046, "f7d6"),
    (0xb696e409686ff41b, "f7d6"),
    (0x6a52623c126ce855, "f7d6"),
    (0xbf3b21a525192add, "f7d6"),
    (0x113217123cf3debd, "f7d6"),
    (0x8f3a7dca35ac9f1c, "f7d6"),
    (0xcd7ca1f50f44c691, "f7d6"),
    (0xa3c20545c1e6ab1b, "f7d6"),
    (0x00575502b602a7cb, "f7d6"),
    (0xdf60678975aa860a, "f7d6"),
    (0x3872f8128bb5a500, "d8d7"),
    (0x355818fbd7c62d05, "d8d7"),
    (0x4e9724e67cca3165, "d8d7"),
    (0x559f714d3161420a, "f7e5"),
    (0x7574205d8144cbd8, "c2c3"),
    (0x22ea975fa5d57b79, "g2g3"),
    (0xd3706a06b4a78709, "h6f7"),
    (0x110537509766026e, "f7d6"),
    (0x9f4c5080cc092ee9, "f7d6"),
    (0x77824274b24153a8, "f7d6"),
    (0x280e3ab9dcdc2264, "f7d6"),
    (0xdde62312db1ecae3, "f7d6"),
    (0x53cde9cb85e37ccb, "f7d6"),
    (0xb05cf8b14d0c285f, "f7d6"),
    (0x1285f7d1148e8405, "f7d6"),
    (0xc00fdcd842a6b951, "f7d6"),
    (0x3e90e5a06849f8d9, "f7d6"),
    (0x3c21ac0ded810cab, "f7d6"),
    (0xbe185e49698ea0c8, "f7d6"),
    (0x5102e5a320b12e7e, "f7d6"),
    (0x586a77b6e4cf6162, "f7d6"),
    (0x1ff2afa29ff500a9, "f7d6"),
    (0x136067ecd016abb9, "f7d6"),
    (0x0406dd86fe2463fb, "f7d6"),
    (0xc3e1e6ec373b4276, "f7d6"),
    (0xa9682ea5444d08e5, "f7d6"),
    (0x8602af2d59af57a7, "f7d6"),
    (0x0c69e398e790bc87, "f7d6"),
    (0x4e2ecb886ee97df4, "f7d6"),
    (0x910b169b7d89d375, "f7d6"),
    (0x0c88c3da49f9a5a0, "f7d6"),
    (0xaf18f012a9065db0, "f7d6"),
    (0x090e546177ff2c97, "f7d6"),
    (0xd9f31fdd4e2bda3a, "f7d6"),
    (0x5e9ae8ed8e3f3175, "f7d6"),
    (0xe5bab659f82c67d9, "f7d6"),
    (0x00c555be78505942, "f7d6"),
    (0xe1aec3fa0fe7588f, "f7d6"),
    (0x4056efc1066049ce, "d8d7"),
    (0x4d7c0f285a13c1cb, "d8d7"),
    (0x36b33335f11fddab, "d8d7"),
    (0x73457ff6cd2b2fb8, "f7e5"),
    (0xb1d7adbd408558f5, "h6f7"),
    (0x747f45ffbb25c4cb, "f7d6"),
    (0x8e46ee61990c9883, "f7d6"),
    (0x830d75ae28b24eca, "f7d6"),
    (0x152585cf46638c54, "f7d6"),
    (0x4aa9fd0228fefd98, "f7d6"),
    (0xbf41e4a92f3c151f, "f7d6"),
    (0x316a2e7071c1a337, "f7d6"),
    (0xd2fb3f0ab92ef7a3, "f7d6"),
    (0xa2a81b63b68466ad, "f7d6"),
    (0x5c37221b9c6b2725, "f7d6"),
    (0x5e866bb619a3d357, "f7d6"),
    (0xdcbf99f29dac7f34, "f7d6"),
    (0x3acdb00d10edbe9e, "f7d6"),
    (0x7d5568196bd7df55, "f7d6"),
    (0x71c7a05724347445, "f7d6"),
    (0x66a11a3d0a06bc07, "f7d6"),
    (0xa1462157c3199d8a, "f7d6"),
    (0xa5ffdc6582deec15, "f7d6"),
    (0xcbcfe91eb06fd719, "f7d6"),
    (0xe4a56896ad8d885b, "f7d6"),
    (0x6ece242313b2637b, "f7d6"),
    (0x2c890c339acba208, "f7d6"),
    (0x154f714f93444cb4, "f7d6"),
    (0xd3701b694d6b6368, "f7d6"),
    (0x3c3d2f567a1dee89, "f7d6"),
    (0x871d71e20c0eb825, "f7d6"),
    (0x626292058c7286be, "f7d6"),
    (0x83090441fbc58773, "f7d6"),
    (0x22f1287af2429632, "d8d7"),
    (0x2fdbc893ae311e37, "d8d7"),
    (0x5414f48e053d0257, "d8d7"),
    (0x11e2b84d3909f044, "f7e5"),
    (0x3f9eca6d1bea7472, "h6f7"),
    (0xfa36222fe04ae84c, "f7d6"),
    (0x9b6ce21f1d0ca0d3, "f7d6"),
    (0xc4e09ad27391d11f, "f7d6"),
    (0x3108837974533998, "f7d6"),
    (0xbf2349a02aae8fb0, "f7d6"),
    (0x5cb258dae241db24, "f7d6"),
    (0x2ce17cb3edeb4a2a, "f7d6"),
    (0xd27e45cbc7040ba2, "f7d6"),
    (0xd0cf0c6642ccffd0, "f7d6"),
    (0x52f6fe22c6c353b3, "f7d6"),
    (0xf62902c16618949b, "f7d6"),
    (0x404969f7a15f649b, "d8d7"),
    (0xb484d7dd4b829219, "f7d6"),
    (0xbe053e532d9b5f30, "f7d6"),
    (0xff8ec7877f5b58c2, "f7d6"),
    (0xe8e87ded51699080, "f7d6"),
    (0x2f0f46879876b10d, "f7d6"),
    (0x2bb6bbb5d9b1c092, "f7d6"),
    (0x45868eceeb00fb9e, "f7d6"),
    (0x6aec0f46f6e2a4dc, "f7d6"),
    (0xe08743f348dd4ffc, "f7d6"),
    (0xa2c06be3c1a48e8f, "f7d6"),
    (0xb27448862172c20e, "f7d6"),
    (0x09541632576194a2, "f7d6"),
    (0xec2bf5d5d71daa39, "f7d6"),
    (0x0d406391a0aaabf4, "f7d6"),
    (0xacb84faaa92dbab5, "d8d7"),
    (0xa192af43f55e32b0, "d8d7"),
    (0xda5d935e5e522ed0, "d8d7"),
    (0x9fabdf9d6266dcc3, "f7e5"),
    (0xd750d89965a20933, "h6f7"),
    (0xd8e6a645d2ac2ea4, "f7d6"),
    (0x2c2e88260dd9ac5e, "f7d6"),
    (0xd9c6918d0a1b44d9, "f7d6"),
    (0x57ed5b5454e6f2f1, "f7d6"),
    (0xb47c4a2e9c09a665, "f7d6"),
    (0xc42f6e4793a3376b, "f7d6"),
    (0x3ab0573fb94c76e3, "f7d6"),
    (0x38011e923c848291, "f7d6"),
    (0xba38ecd6b88b2ef2, "f7d6"),
    (0x5c4ac52935caef58, "f7d6"),
    (0x1bd21d3d4ef08e93, "f7d6"),
    (0x1740d57301132583, "f7d6"),
    (0x00266f192f21edc1, "f7d6"),
    (0xc7c15473e63ecc4c, "f7d6"),
    (0xc378a941a7f9bdd3, "f7d6"),
    (0xad489c3a954886df, "f7d6"),
    (0x82221db288aad99d, "f7d6"),
    (0x08495107369532bd, "f7d6"),
    (0x4a0e7917bfecf3ce, "f7d6"),
    (0x08a8714598fc2b9a, "f7d6"),
    (0xab38428d7803d38a, "f7d6"),
    (0x0d2ee6fea6faa2ad, "f7d6"),
    (0xcb118cd878d58d71, "f7d6"),
    (0x5aba5a725f3abf4f, "f7d6"),
    (0xc7d20ac11b317ad8, "f7d6"),
    (0xc3c67f62ecfa458e, "f7d6"),
    (0x6e923d591984eb6b, "d8d7"),
    (0x623e5359e57d54cf, "d8d7"),
    (0x6f14b3b0b90edcca, "d8d7"),
    (0x14db8fad1202c0aa, "d8d7"),
    (0x7765cd691c2ea182, "f7e5"),
    (0x88dca0540b3f78ff, "h6f7"),
    (0x4e30fc2d63f1e1c0, "f7d6"),
    (0x086123993a7b833d, "f7d6"),
    (0xebf032e3f294d7a9, "f7d6"),
    (0x9ba3168afd3e46a7, "f7d6"),
    (0x653c2ff2d7d1072f, "f7d6"),
    (0x678d665f5219f35d, "f7d6"),
    (0xe5b4941bd6165f3e, "f7d6"),
    (0x03c6bde45b579e94, "f7d6"),
    (0x445e65f0206dff5f, "f7d6"),
    (0x48ccadbe6f8e544f, "f7d6"),
    (0x5faa17d441bc9c0d, "f7d6"),
    (0x984d2cbe88a3bd80, "f7d6"),
    (0x9cf4d18cc964cc1f, "f7d6"),
    (0xf2c4e4f7fbd5f713, "f7d6"),
    (0xddae657fe637a851, "f7d6"),
    (0x57c529ca58084371, "f7d6"),
    (0x158201dad1718202, "f7d6"),
    (0x57240988f6615a56, "f7d6"),
    (0x8aa7e9cab94049cd, "f7d6"),
    (0x52a29e33c867d361, "f7d6"),
    (0x949df4151648fcbd, "f7d6"),
    (0x053622bf31a7ce83, "f7d6"),
    (0xbe167c0b47b4982f, "f7d6"),
    (0x5b699fecc7c8a6b4, "f7d6"),
    (0xba0209a8b07fa779, "f7d6"),
    (0x1bfa2593b9f8b638, "d8d7"),
    (0x16d0c57ae58b3e3d, "d8d7"),
    (0x6d1ff9674e87225d, "d8d7"),
    (0x28e9b5a472b3d04e, "f7e5"),
    (0x7d34b9ff0cfd9078, "h6f7"),
    (0x0bdd1743186ac1a5, "f7d6"),
    (0xfd893a323db96bba, "f7d6"),
    (0x1e182b48f5563f2e, "f7d6"),
    (0x6e4b0f21fafcae20, "f7d6"),
    (0x92657ff455db1bda, "f7d6"),
    (0x105c8db0d1d4b7b9, "f7d6"),
    (0xf62ea44f5c957613, "f7d6"),
    (0xbd24b415684cbcc8, "f7d6"),
    (0xaa420e7f467e748a, "f7d6"),
    (0x6da535158f615507, "f7d6"),
    (0x691cc827cea62498, "f7d6"),
    (0x072cfd5cfc171f94, "f7d6"),
    (0x28467cd4e1f540d6, "f7d6"),
    (0x130a9987ab84786f, "f7d6"),
    (0xe06a1871d6b36a85, "f7d6"),
    (0xa2cc1023f1a3b2d1, "f7d6"),
    (0x015c23eb115c4ac1, "f7d6"),
    (0xa74a8798cfa53be6, "f7d6"),
    (0x6175edbe118a143a, "f7d6"),
    (0xf0de3b1436652604, "f7d6"),
    (0x4bfe65a0407670a8, "f7d6"),
    (0xae818647c00a4e33, "f7d6"),
    (0x4fea1003b7bd4ffe, "f7d6"),
    (0xee123c38be3a5ebf, "d8d7"),
    (0xe338dcd1e249d6ba, "d8d7"),
    (0x98f7e0cc4945cada, "d8d7"),
    (0xdd01ac0f757138c9, "f7e5"),
    (0xf31f732652002650, "h6f7"),
    (0x0d44127e73dd624d, "f7d6"),
    (0x4824cd6b36242e64, "f7d6"),
    (0xe060c5f8a4011808, "f7d6"),
    (0x1efffc808eee5980, "f7d6"),
    (0x1c4eb52d0b26adf2, "f7d6"),
    (0x9e7747698f290191, "f7d6"),
    (0x78056e960268c03b, "f7d6"),
    (0x3f9db6827952a1f0, "f7d6"),
    (0x330f7ecc36b10ae0, "f7d6"),
    (0x2469c4a61883c2a2, "f7d6"),
    (0xe38effccd19ce32f, "f7d6"),
    (0xe73702fe905b92b0, "f7d6"),
    (0x89073785a2eaa9bc, "f7d6"),
    (0xa66db60dbf08f6fe, "f7d6"),
    (0x2c06fab801371dde, "f7d6"),
    (0x6e41d2a8884edcad, "f7d6"),
    (0x2ce7dafaaf5e04f9, "f7d6"),
    (0x8f77e9324fa1fce9, "f7d6"),
    (0x29614d4191588dce, "f7d6"),
    (0xf99c06fda88c7b63, "f7d6"),
    (0x7ef5f1cd6898902c, "f7d6"),
    (0xc5d5af791e8bc680, "f7d6"),
    (0x20aa4c9e9ef7f81b, "f7d6"),
    (0xc1c1dadae940f9d6, "f7d6"),
    (0x6039f6e1e0c7e897, "d8d7"),
    (0x6d131608bcb46092, "d8d7"),
    (0x16dc2a1517b87cf2, "d8d7"),
    (0x532a66d62b8c8ee1, "f7e5"),
    (0x108e625c9aef72c4, "b1c1"),
    (0xd66f43f6ec1a01ca, "h6f7"),
    (0x377ec26d6cc6f947, "f7d6"),
    (0x76bdd91b3bf95b51, "f7d6"),
    (0x6afcfc35df423b72, "f7d6"),
    (0xe4b59be5842d17f5, "f7d6"),
    (0xe11ab1f36cd3cc36, "f7d6"),
    (0x729d6b84eafcd56b, "f7d6"),
    (0x2d1113498461a4a7, "f7d6"),
    (0xd8f90ae283a34c20, "f7d6"),
    (0x79fe6a4b8b38bf9f, "f7d6"),
];

static BOOK: OnceLock<HashMap<u64, Move>> = OnceLock::new();

fn init_book() -> HashMap<u64, Move> {
    let mut map = HashMap::with_capacity(BOOK_DATA.len());
    for &(hash, uci) in BOOK_DATA {
        if let Some(m) = Move::from_uci(uci) {
            map.insert(hash, m);
        }
    }
    map
}

pub fn probe(hash: u64) -> Option<Move> {
    let book = BOOK.get_or_init(init_book);
    book.get(&hash).copied()
}
