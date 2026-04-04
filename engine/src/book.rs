use crate::moves::Move;
use std::collections::HashMap;
use std::sync::OnceLock;

/// Minimal solution book for 1.e3 — only reachable positions.
/// Cleaned by reachable-extractor from 2401 entries.
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
    (0x5f6629b70c07a8b7, "f2f3"),   // Nf6: Qg4 reclaims e4 knight + flips g7/d7,   // mate in 3 half-moves
    (0x0dac1df5b61ed387, "f1d3"),   // mate in 3 half-moves
    (0xec66455eefe176f8, "f2f3"),   // Nf6: Qg4 reclaims e4 knight + flips g7/d7,   // mate in 3 half-moves
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
    (0x48d289ff1796bae3, "f2f3"),   // Nf6: Qg4 reclaims e4 knight + flips g7/d7,   // 3...Nf6 -> Qf3 (Nf6 flipped, then mate)
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
    (0xc3ad686f8481b0e9, "d2d3"),   // mate in 3 half-moves
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
    (0x4f1fccc9941ff46e, "f2f3"),   // Nf6: Qg4 reclaims e4 knight + flips g7/d7,   // mate in 3 half-moves
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
    (0x6b6e52e4ab586fc5, "f2f3"),   // Nf6: Qg4 reclaims e4 knight + flips g7/d7,   // mate in 3 half-moves
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
    (0xd86e3e0d48beb18a, "f2f3"),   // Nf6: Qg4 reclaims e4 knight + flips g7/d7,   // mate in 3 half-moves
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
    (0xda82053fdad6fad3, "f2f3"),
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
    (0xe5ca6a837c83b124, "g7g8n"),   // g8=N! flips Nf6, ALL responses mate,
    (0xc7a03f951965a836, "e4d6"),
    (0x2a2392d36d4412bf, "g4h5"),
    (0xa459511dc88c433e, "e4d6"),
    (0xce37da11955f30fd, "g4h5"),
    (0x504cb5d316e56bcb, "e4d6"),
    (0xe46ec33f91e5d66b, "e4d6"),
    (0x170e42c9ecd2c481, "e4d6"),
    (0xe1cd7eebb3fbec82, "e4d6"),
    (0x9b955978e52a3c72, "e4g5"),
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
    (0x88918676f726678f, "e4d6"),
    (0x3b91ea9f14c0b9c0, "e4d6"),
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
    (0x817ea798fb24f77e, "g2g3"),
    (0x9879a29e4a1ce917, "a1b1"),
    (0x7d641682760a8c7e, "f6d7"),
    (0xa333971b3074964f, "d7f6"),
    (0x5b44e42a5b24fa17, "d7f6"),
    (0x5eebce3cb3da21d4, "d7f6"),
    (0x785053e848c4a2df, "d7f6"),
    (0x27dc2b252659d313, "d7f6"),
    (0xd234328e219b3b94, "d7f6"),
    (0x4e1da3d634812851, "d7f6"),
    (0x09022df81077776e, "d7f6"),
    (0x3f1694dfb60f7e3f, "d7f6"),
    (0x2fb56ba1be28f979, "g8f6"),
    (0x432e6755dc245533, "d7f6"),
    (0xb8c02c4c61355044, "d7f6"),
    (0xffa435d56e5c12d1, "b8c6"),
    (0x867b9014a28f2d97, "d8b8"),
    (0x83d4ba024a71f654, "d8b8"),
    (0xa56f27d6b16f755f, "d8b8"),
    (0xb3e0437809ab15b4, "d8b8"),
    (0xefa7c56d513ec0d1, "d8b8"),
    (0x7a4b2c5829da7cc6, "d8d7"),
    (0x470f902129a80bbf, "d8d7"),
    (0xfcbc8414b9ca71d5, "d8d7"),
    (0x9f24f43e518832be, "d8d7"),
    (0x7e09a0b48f0afaf6, "d8b8"),
    (0x79786c4afa0ddcf5, "d8b8"),
    (0x0d30b9444db19c4d, "c6e5"),
    (0x0a077f56a1bb8118, "g4g6"),
    (0x0fa8554049455adb, "g4g6"),
    (0x2913c894b25bd9d0, "g4h5"),
    (0xe5d6c3bece24b5bb, "g4g6"),
    (0x8377a9f2db04409b, "g4g6"),
    (0x3f9cac3a0a9fb93b, "d7f6"),
    (0x63db2a2f520a6c5e, "d7f6"),
    (0x650293de353c0012, "d7b6"),
    (0x4f8f02adeb2abfd4, "c8d7"),
    (0x69349f7910343cdf, "c8d7"),
    (0xa5f194536c4b50b4, "c8d7"),
    (0x98d579b35196a9ce, "c8d7"),
    (0x7fbbfbd7a8f05c34, "c8d7"),
    (0xf4f2ed1e06317b07, "c8a6"),
    (0x67d87b98ddde7a5b, "d8b8"),
    (0x4163e64c26c0f950, "d8b8"),
    (0x8da6ed665abf953b, "d8b8"),
    (0xb082008667626c41, "d8b8"),
    (0xaa076d39b2497001, "d8b8"),
    (0xfdf519a496c54105, "a8b8"),
    (0x893f0c3a44a174d6, "d8c8"),
    (0x74e7551dc70fc34d, "d8c8"),
    (0x525cc8c93c114046, "d8c8"),
    (0x6f495b6b89ef9ce9, "d8e7"),
    (0x87ccb02e42f44efc, "d8e7"),
    (0x1b64b26c4d4c1d00, "d8c8"),
    (0xaeb2f0e8997d1fb8, "d8c8"),
    (0xb655adcb0963e920, "d8c8"),
    (0x7c91e20dc759107c, "d8c8"),
    (0x79bc98fca4186a21, "b8a8"),
    (0x339b810d61ab52d3, "d8b8"),
    (0xce43d82ae205e548, "d8b8"),
    (0xe8f845fe191b6643, "d8b8"),
    (0xd5edd65cace5baec, "d8e7"),
    (0x3d683d1967fe68f9, "d8e7"),
    (0xa1c03f5b68463b05, "d8b8"),
    (0x14167ddfbc7739bd, "d8b8"),
    (0x0cf120fc2c69cf25, "d8b8"),
    (0xc6356f3ae2533679, "d8b8"),
    (0x398758b957171f74, "d8c8"),
    (0xda81522a82f0868a, "d8d7"),
    (0xca57667ecab5d3d8, "d8d7"),
    (0x54fe564c697b1833, "d8d7"),
    (0xff211e3bb1cb0389, "d8d7"),
    (0x028bd11871aa5012, "d8b8"),
    (0xcc1e3892e4a5939f, "d8b8"),
    (0x37f0738b59b496e8, "d8b8"),
    (0xc83b6088b9969cb9, "d8b8"),
    (0xdd9669aca2e6645b, "d8b8"),
    (0x781da7afe0a102d3, "d8b8"),
    (0xa0177f57c0824c20, "d8b8"),
    (0x5cf66211a0b4955b, "d8b8"),
    (0xa4d2b9e47d350604, "d8b8"),
    (0xedb5fb8b5dacf88e, "d8b8"),
    (0x112a2880b4348ec7, "d8b8"),
    (0x6e0839b65c12f37a, "d8b8"),
    (0xf097b9793a67717a, "d8b8"),
    (0xfa76523896c1b3ed, "d8b8"),
    (0x45a4653c39e29db5, "d8b8"),
    (0xcb4caa5fc71b547b, "d8b8"),
    (0x9f7a1e938d6f6baa, "d8b8"),
    (0xee5b42f23848a94e, "d8b8"),
    (0x893a4fab0274cfef, "d8c8"),
    (0x6025df1da7faa08f, "d8d7"),
    (0x70f3eb49efbff5dd, "d8d7"),
    (0xee5adb7b4c713e36, "d8d7"),
    (0xc2b92a98c5ab24d6, "d8c8"),
    (0xe652ef2685beb35e, "d8c8"),
    (0x1e7634d3583f2001, "d8c8"),
    (0x571176bc78a6de8b, "d8c8"),
    (0xab8ea5b7913ea8c2, "d8c8"),
    (0xd4acb4817918d57f, "d8c8"),
    (0x4a33344e1f6d577f, "d8c8"),
    (0x40d2df0fb3cb95e8, "d8c8"),
    (0xff00e80b1ce8bbb0, "d8c8"),
    (0x71e82768e211727e, "d8c8"),
    (0x25de93a4a8654daf, "d8c8"),
    (0x0f7a486935bf8311, "d8c8"),
    (0x2f9953740639d999, "d8e8"),
    (0x0904250308682272, "d8b8"),
    (0x1ed5482d763a281e, "a8b8"),
    (0x97c7049427f0aa56, "d8c8"),
    (0xb17c9940dcee295d, "d8c8"),
    (0x7db9926aa0914536, "d8c8"),
    (0x409d7f8a9d4cbc4c, "d8c8"),
    (0x1409cda84d554d7f, "d8c8"),
    (0x83058e944705c994, "d8d7"),
    (0x93d3bac00f409cc6, "a6b5"),
    (0x0d7a8af2ac8e572d, "a6b5"),
    (0x5b0f0da6b45f1f0c, "d8c8"),
    (0x959ae42c2150dc81, "d8c8"),
    (0x6e74af359c41d9f6, "d8c8"),
    (0x21997b1125544dcd, "d8c8"),
    (0x0572beaf6541da45, "d8c8"),
    (0xfd56655ab8c0491a, "d8c8"),
    (0x277854d28abbaa37, "d8c8"),
    (0xdbe787d96323dc7e, "d8c8"),
    (0xdad6456524db4a48, "d8c8"),
    (0x3a5a1620ed7023c3, "d8c8"),
    (0x4ea82eebee080adf, "d8c8"),
    (0x62336a0853c93920, "d8c8"),
    (0xac257a8e15391644, "d8c8"),
    (0xc6fec22d489a24b4, "d8c8"),
    (0xb7df9e4cfdbde650, "d8c8"),
    (0x195969647e0d3613, "d8b8"),
    (0x6f9da2bf698df692, "d8b8"),
    (0x2e577f0572ead1ac, "a6b7"),
    (0x76867a2f201279f4, "c8d6"),
    (0x503de7fbdb0cfaff, "c8d6"),
    (0x9cf8ecd1a7739694, "c8d6"),
    (0xa1dc01319aae6fee, "c8d6"),
    (0x85ad9f1ca5e9f445, "c8d6"),
    (0xbb596c8e4f8573ae, "c8d6"),
    (0x92b8cb99c4d7a921, "c8d6"),
    (0x6649f73e5a7aca56, "c8d6"),
    (0x0f8b499a8bf62bb1, "c8d6"),
    (0x7bddb7ce4305d098, "c8d6"),
    (0x8b5b6099e5697556, "c8d6"),
    (0x375152ce5e1845e4, "c8d6"),
    (0xd30a7929842ef312, "b7c6"),
    (0xec3bf449ab6c848f, "c8d6"),
    (0xaab9a06470827665, "c8d6"),
    (0x7b11cc30c8d39372, "c8d6"),
    (0xe433c01462a309e7, "c8d6"),
    (0x1c171be1bf229ab8, "c8d6"),
    (0x2b6436338e0790c5, "c8d6"),
    (0x044bf50d731602fd, "c8d6"),
    (0x4584e8548ce77261, "c8d6"),
    (0xdb1b689bea92f061, "c8d6"),
    (0xd1fa83da463432f6, "c8d6"),
    (0xfd61c739fbf50109, "c8d6"),
    (0x7389085a050cc8c7, "c8d6"),
    (0x9c76750ca51dfa0b, "c8d6"),
    (0x0d1b675bd2a239a8, "c8d6"),
    (0x2df87c46e1246320, "c8d6"),
    (0x6a83b679bec9d337, "a6c8"),
    (0xa62ced8fd93e01ae, "d7f6"),
    (0x8097705b222082a5, "d7f6"),
    (0x4c527b715e5feece, "d7f6"),
    (0x71769691638217b4, "d7f6"),
    (0x6bf3fb2eb6a90bf4, "d7f6"),
    (0x3722faf0aaa1ea6d, "d7f6"),
    (0xc8f0b3140c885987, "d7f6"),
    (0xd8adff737aed4de6, "d7f6"),
    (0xae6934a86d6d8d67, "d7f6"),
    (0xa4710d37df9e7779, "d7f6"),
    (0xa7176513fec3af2a, "d7f6"),
    (0xa054552d82ad785f, "d7f6"),
    (0xb5f95c0999dd80bd, "d7f6"),
    (0xabbb5b9031ffeb28, "d7f6"),
    (0xc8784af2fbb9a8c6, "d7f6"),
    (0x349957b49b8f71bd, "d7f6"),
    (0xccbd8c41460ee2e2, "d7f6"),
    (0xfbcea193772be89f, "d7f6"),
    (0xd4e162ad8a3a7aa7, "d7f6"),
    (0x952e7ff475cb0a3b, "d7f6"),
    (0x0bb1ff3b13be883b, "d7f6"),
    (0x7f43c7f010c6a127, "d7f6"),
    (0x53d88313ad0792d8, "d7f6"),
    (0xa3239ffafc20b09d, "d7f6"),
    (0x4cdce2ac5c318251, "d7f6"),
    (0x8634775703734da8, "d7f6"),
    (0xe416b2a4b77b9d72, "d8b8"),
    (0x731af198bd2b1999, "d8d7"),
    (0xfd65f5fe56a08720, "a6b5"),
    (0x56babd898e109c9a, "d8d7"),
    (0x6a4fcd87351f90dd, "d8b8"),
    (0x8b7e122930b1e1c3, "d8b8"),
    (0x0d491a5642ee9917, "d8b8"),
    (0xd7672bde70957a3a, "d8b8"),
    (0x1515f4ba8eda0152, "d8b8"),
    (0x54dae9e3712b71ce, "d8b8"),
    (0xca45692c175ef3ce, "d8b8"),
    (0xbeb751e71426dad2, "d8b8"),
    (0x922c1504a9e7e92d, "d8b8"),
    (0x62d709edf8c0cb68, "d8b8"),
    (0xf33ba731f70f122f, "d8b8"),
    (0x1c4566ec2f6e3a07, "d8b8"),
    (0x42b5ae7bb3368b04, "d8e8"),
    (0x23fc7dc2f0658951, "c8d7"),
    (0xc3ddcd9fc28053e7, "c8d7"),
    (0x342511750c198314, "c8d7"),
    (0xcc41cb91818f58fd, "c8d7"),
    (0xa3296b1c0645244c, "c8d7"),
    (0x1747574ca971f1e7, "c8d7"),
    (0x126d4eb1453f56e5, "c8d7"),
    (0x755e5fd92bdd5c50, "c8d7"),
    (0x0a7c4eefc3fb21ed, "c8d7"),
    (0x94e3ce20a58ea3ed, "c8d7"),
    (0xe8f3fb588d0cf4d6, "c8d7"),
    (0xc468bfbb30cdc729, "c8d7"),
    (0x4a8070d8ce340ee7, "c8d7"),
    (0xe935ea2b1c907adf, "c8d7"),
    (0x064b2bf6c4f152f7, "c8d7"),
    (0xebc821deeb2c4e53, "b6a4"),
    (0xe40f7191334e440a, "a4c5"),
    (0xc2b4ec45c850c701, "a4c5"),
    (0x0e71e76fb42fab6a, "a4b6"),
    (0x33550a8f89f25210, "a4c5"),
    (0x172494a2b6b5c9bb, "a4c5"),
    (0xd43b88eb7094a7ea, "a4c5"),
    (0x887c0efe2801728f, "a4c5"),
    (0xb49c4ad404fb3b25, "a4c5"),
    (0x164e6d29b53a43b2, "a4c5"),
    (0x9486170aecf9a857, "a4c5"),
    (0x19d26b27f63548a8, "a4c5"),
    (0x910fa62e6a5fea42, "a4c5"),
    (0xb498401a829e653f, "a4c5"),
    (0x4d654b168ea72e0f, "a4c5"),
    (0x8718a4cc8e4d102f, "a4c5"),
    (0xb9ed3d8d9d5bad3b, "a4c5"),
    (0xe033208ae46eaaaf, "a4c5"),
    (0xa1fc3dd31b9fda33, "a4c5"),
    (0x3f63bd1c7dea5833, "a4c5"),
    (0x4373886455680f08, "a4c5"),
    (0x6fe8cc87e8a93cf7, "a4c5"),
    (0xe10003e41650f539, "a4c5"),
    (0x3ca64a9d6b2a6a8a, "a4c5"),
    (0xadcb58ca1c95a929, "a4c5"),
    (0xc76bd0b6b5990a22, "a4c5"),
    (0x34c72914c7fb8b59, "b4b5"),
    (0x7a9d549f34c69a3b, "a8a6"),
    (0x9bcec80a5947de7c, "a4c5"),
    (0xc47663851a6de4d6, "e5f7"),
    (0x4750928d7d4d0edf, "f7d6"),
    (0x61eb0f5986538dd4, "f7d6"),
    (0xad2e0473fa2ce1bf, "f7d6"),
    (0x900ae993c7f118c5, "f7d6"),
    (0x5cfe9cfb33ad517b, "f7d6"),
    (0xb47b77bef8b6836e, "f7d6"),
    (0x77646bf73e97ed3f, "f7d6"),
    (0x86689b47decefffe, "f7d6"),
    (0x197ad9cdbb699525, "f7d6"),
    (0x7e59e4fcd1d46589, "f7d6"),
    (0x32504532245ca097, "f7d6"),
    (0xa3ace371bbe7b365, "d8d7"),
    (0x17c7a306cc9d2fea, "f7d6"),
    (0xa0a951ca44db6f8d, "f7d6"),
    (0xafe768eedd8a237d, "f7d6"),
    (0x1ab2de91d358e7ee, "f7d6"),
    (0x359d1daf2e4975d6, "f7d6"),
    (0x745200f6d1b8054a, "f7d6"),
    (0x2185eaf09d20eba4, "f7d6"),
    (0xe02c6b781b6b45dd, "f7d6"),
    (0x07ff45528c471acc, "f7d6"),
    (0xe0eddac9725839c6, "d8d7"),
    (0x217a812ab0a2a0c0, "d8d7"),
    (0x20da7e18c50d03d7, "d8d7"),
    (0x5e53c1f5b181d3e1, "a4c5"),
    (0x58bbe36158a9e3f4, "c8e6"),
    (0x7aa882ff1054f493, "g4g6"),
    (0x814c56064e8530c2, "g4g6"),
    (0x33aa888c1ded263f, "g4g6"),
    (0x8f897ae312ed64da, "g4g6"),
    (0xb4c59fb0589c5c63, "g4g6"),
    (0x47a51e4625ab4e89, "g4g6"),
    (0x832376b75972af2d, "g4g6"),
    (0x29119cd836a92ca5, "g4g6"),
    (0xabe35214f9411e55, "g4g6"),
    (0x0aeacb0656f87650, "g4g6"),
    (0xbfd667ce1ed98ab0, "d8a8"),
    (0x03f595a111d9c855, "d8a8"),
    (0xf876e48510c13b97, "d8a8"),
    (0xcbd9f104269fe206, "d8a8"),
    (0x97359f517020ce98, "c6e5"),
    (0x900259439c2ad3cd, "g4g6"),
    (0x95ad735574d4080e, "g4g6"),
    (0xb316ee818fca8b05, "g4h5"),
    (0x7fd3e5abf3b5e76e, "g4g6"),
    (0x19728fe7e695124e, "g4g6"),
    (0xa5998a2f370eebee, "d7f6"),
    (0xf9de0c3a6f9b3e8b, "d7f6"),
    (0xff07b5cb08ad52c7, "f8g7"),
    (0x289818360ed0371a, "h8f8"),
    (0x2d373220e62eecd9, "h8f8"),
    (0x0b8caff41d306fd2, "h8f8"),
    (0xc749a4de614f03b9, "h8f8"),
    (0xa1e8ce92746ff699, "h8f8"),
    (0xd79be4685652c678, "h8f8"),
    (0x3c798f528c9cef16, "h8f8"),
    (0x807a2a73ee660744, "h8f8"),
    (0xfc56fc165df65505, "h8f8"),
    (0x20831dba48c57423, "h8f8"),
    (0x4b67540cb1d53a4f, "h8f8"),
    (0x2b50f99060c609b2, "h8f8"),
    (0x728ee49719f30e26, "h8f8"),
    (0x3341f9cee6027eba, "h8f8"),
    (0x3d13946482679c0e, "h8f8"),
    (0x3b6c623374467442, "h8f8"),
    (0x5e992ce47b53dc1f, "h8f8"),
    (0xd071e38785aa15d1, "h8f8"),
    (0x4dd7372ef8e8d24c, "h8f8"),
    (0x0084ae641be92345, "h8f8"),
    (0x115bd1403ea5713d, "e5g6"),
    (0x6f01a51dc4a822af, "g4g6"),
    (0x84e3ce271e660bc1, "g4g6"),
    (0x38e06b067c9ce393, "g4g6"),
    (0x44ccbd63cf0cb1d2, "g4g6"),
    (0x98195ccfda3f90f4, "g4g6"),
    (0xf3fd1579232fde98, "g4g6"),
    (0xfbf9d4285653a166, "g4g6"),
    (0x790b1ae499bb9396, "g4g6"),
    (0xf54d765b6a12369b, "g4g6"),
    (0x82999299c757542e, "g4g6"),
    (0xf02dfbccc976f6fe, "c6e5"),
    (0xf71a3dde257cebab, "g4g6"),
    (0xf2b517c8cd823068, "g4g6"),
    (0xd40e8a1c369cb363, "g4h5"),
    (0x18cb81364ae3df08, "g4g6"),
    (0x7e6aeb7a5fc32a28, "g4g6"),
    (0xc281eeb28e58d388, "d7f6"),
    (0x9ec668a7d6cd06ed, "d7f6"),
    (0x981fd156b1fb6aa1, "f8g7"),
    (0x4f807cabb7860f7c, "h8f8"),
    (0x4a2f56bd5f78d4bf, "h8f8"),
    (0x6c94cb69a46657b4, "h8f8"),
    (0xa051c043d8193bdf, "h8f8"),
    (0xc6f0aa0fcd39ceff, "h8f8"),
    (0xb08380f5ef04fe1e, "h8f8"),
    (0x5b61ebcf35cad770, "h8f8"),
    (0xe7624eee57303f22, "h8f8"),
    (0x2c7f309108830229, "h8f8"),
    (0x4c489d0dd99031d4, "h8f8"),
    (0x1596800aa0a53640, "h8f8"),
    (0x54599d535f5446dc, "h8f8"),
    (0x5a0bf0f93b31a468, "h8f8"),
    (0x5c7406aecd104c24, "h8f8"),
    (0x39814879c205e479, "h8f8"),
    (0xb769871a3cfc2db7, "h8f8"),
    (0x679ccaf9a2bf1b23, "h8f8"),
    (0x7643b5dd87f3495b, "e5g6"),
    (0x0819c1807dfe1ac9, "g4g6"),
    (0xe3fbaabaa73033a7, "g4g6"),
    (0x5ff80f9bc5cadbf5, "g4g6"),
    (0x94e571e49a79e6fe, "g4g6"),
    (0x9ce1b0b5ef059900, "g4g6"),
    (0x1e137e7920edabf0, "g4g6"),
    (0xe581f6047e016c48, "g4g6"),
    (0xa56d739a359d802a, "d8b8"),
    (0x279fbd56fa75b2da, "d8b8"),
    (0x457ce5c587f0d844, "d8b8"),
    (0xaa9cb28c31888107, "d8b8"),
    (0x95407c5f61a71a10, "d8b8"),
    (0x2e49a8b680f17fc1, "d8b8"),
    (0x24a843f72c57bd56, "d8b8"),
    (0x0833071491968ea9, "d8b8"),
    (0x86dbc8776f6f4767, "d8b8"),
    (0x256e5284bdcb335f, "d8b8"),
    (0x8696244455ccdadf, "c6e5"),
    (0xa447187403a60b75, "d7f6"),
    (0x4fa5734ed968221b, "g8f6"),
    (0x8f8a000a08029808, "g8f6"),
    (0xf1a06017f5904097, "d7f6"),
    (0xdecae19fe8721fd5, "d7f6"),
    (0xe58604cca203276c, "d7f6"),
    (0x16e6853adf343586, "d7f6"),
    (0x785207a4cc3657aa, "d7f6"),
    (0xfaa0c96803de655a, "d7f6"),
    (0x603faf6f6bb8b5e6, "d7f6"),
    (0x14cd97a468c09cfa, "d7f6"),
    (0x43a96144d7bf22b9, "d7f6"),
    (0xb3527dad869800fc, "d7f6"),
    (0xab2e2ec4be5979d9, "d7f6"),
    (0x00474aff5e72eee9, "g8f6"),
    (0xb31f919ebc3b9d44, "d7e5"),
    (0x461136f150543aaf, "f6d5"),
    (0x67f32c7040c97091, "d5f6"),
    (0x592926158e691d83, "d5f6"),
    (0x7f92bbc175779e88, "d5f6"),
    (0x49df4bff09321406, "d5f6"),
    (0x0ec0c5d12dc44b39, "d5f6"),
    (0xd658b46d329f0d9b, "g8f6"),
    (0x77f24ef83f8316eb, "d5f6"),
    (0x44ec8f7ce1976964, "d5f6"),
    (0xbf02c4655c866c13, "d5f6"),
    (0x496fd81756d4b2b0, "c7c8q"),
    (0xf05062bed1a788ca, "b7c6"),
    (0xe831f50fe41966d3, "b7c6"),
    (0xf4c412b64c7db4ab, "b7c6"),
    (0xacedf86ad22e249a, "b7c6"),
    (0xbf5d3f6cf2e70a90, "b7c6"),
    (0xbf55a86273296da7, "b7c6"),
    (0x504f588878e2313e, "b7c6"),
    (0xfebe91a15cdd0638, "b7c6"),
    (0x31fee0dc4feae551, "d8b6"),
    (0xa41e194281055e14, "b6g6"),
    (0x08f36116cee84045, "b6g6"),
    (0xe0c21427b7321c5d, "b6g6"),
    (0xf372d32197fb3257, "b6g6"),
    (0xcf6a53655d76a35c, "b6d8"),
    (0xac782b28fa77c5d0, "b6g6"),
    (0xc372ac770411e10f, "b6c6"),
    (0x86b13ac252abeb8f, "b7c6"),
    (0x439d8d5cd9982c3e, "b7c6"),
    (0x139387ee71e2b4ed, "b6g6"),
    (0x5525b3bde32c7a04, "b6d8"),
    (0x70e4982efc6fdaa7, "b6d8"),
    (0xb3c5252a21e9c0be, "b6g6"),
    (0x917035103af85c0c, "b6g6"),
    (0x3c384ae1127f6865, "b6g6"),
    (0x9ee452d5bcf4e3a0, "b6g6"),
    (0xb29c34d8c63a2f37, "b6g6"),
    (0xc50e77180fb23f31, "c8d8"),
    (0x72e0712b0b547fcf, "b6g6"),
    (0x58da747061874ff7, "c8d8"),
    (0x26ee83c2ca1f9991, "b6g6"),
    (0x78cff24e5d036203, "b6g6"),
    (0xc3c626a7bc5507d2, "b6g6"),
    (0x7b89a56621a43fcd, "b6g6"),
    (0xbbec510090ab5f6a, "b6g6"),
    (0xb7d17497a5fc99b9, "c8d8"),
    (0xa54c2057d06fea97, "b6g6"),
    (0xb5be2effb8cd5ceb, "b6e6"),
    (0xaa1085edceb3a9a1, "b6a5"),
    (0x90a0239bd9237e22, "b7c6"),
    (0xffeac96744f5f879, "b7c6"),
    (0xdd5fd95d5fe464cb, "b7c6"),
    (0xc31ddec4f7c60f5e, "b7c6"),
    (0x61c1c6f0594d849b, "b7c6"),
    (0x4eab477844afdbd9, "b7c6"),
    (0x75e7a22b0edee360, "b7c6"),
    (0x3ecf9d666e484708, "c8c6"),
    (0xe833a14360eb93a6, "b7c6"),
    (0x6ac16f8faf03a156, "b7c6"),
    (0x87ea666bb8ba0538, "b7c6"),
    (0x02af334faa829141, "b7c6"),
    (0xfabfe2c96bc3b37d, "b7c6"),
    (0x89d06ec75a698c26, "b7c6"),
    (0x0738a1a4a49045e8, "b7c6"),
    (0x5a69b47235d68dac, "b7c6"),
    (0x027337893408394c, "b7c6"),
    (0xf991c2b2ddd1642c, "c8e6"),
    (0x707fc4a44c9c2b22, "d5f6"),
    (0x3da3695697fbe564, "d5f6"),
    (0x8516fc5edef91212, "d5f6"),
    (0x507a8e0323f9ac42, "d5f6"),
    (0xf96e9e8ce1e0827a, "g8f6"),
    (0xd7cd255389a9779a, "d5f6"),
    (0x4afc9793ce8a9c90, "d5f6"),
    (0x54be900a66a8f705, "d5f6"),
    (0xf662883ec8237cc0, "d5f6"),
    (0xd90809b6d5c12382, "d5f6"),
    (0xe244ece59fb01b3b, "d5f6"),
    (0x11246d13e28709d1, "d5f6"),
    (0x7f90ef8df1856bfd, "d5f6"),
    (0xfd6221413e6d590d, "d5f6"),
    (0xdf9da68f9f07d6e2, "d5f6"),
    (0x130f7f8d5573a0ad, "d5f6"),
    (0x446b896dea0c1eee, "d5f6"),
    (0xb4909584bb2b3cab, "d5f6"),
    (0x257c3b58b4e4e5ec, "d5f6"),
    (0xad63f09bd5d5d3f9, "g8f6"),
    (0x1433f4e9b8fcd8f9, "g8f6"),
    (0xde2c0fa2e5bbaf80, "d5f6"),
    (0xa4d3cd5b4f95ce45, "d5b6"),
    (0x86844d985ce30c6c, "d5f6"),
    (0x0c4a9bf2e47f7be4, "e4g5"),
    (0x329091972adf16f6, "e4g5"),
    (0x293e9fe1643f4952, "e4g5"),
    (0x9a3ef30887d9971d, "e4g5"),
    (0x1bc67326e82a2057, "e4g5"),
    (0x28090d5e9c93059a, "e4g5"),
    (0x5f88e23a8e01cafe, "e4g5"),
    (0xb5b9fa4f2287f6b6, "g8h6"),
    (0x3bc33981874fa737, "e4g5"),
    (0x220ffd13aea527d9, "g8h6"),
    (0xbc7492d12d1f7cef, "e4g5"),
    (0x214520116a3c97e5, "e4g5"),
    (0x8e208e6e36502fe9, "e4g5"),
    (0x8fad4036c0ab071e, "e4g5"),
    (0xa0c7c1bedd49585c, "e4g5"),
    (0x2aac8d0b6376b37c, "e4g5"),
    (0x68eba51bea0f720f, "e4g5"),
    (0x51d9747be4fca9f7, "e4g5"),
    (0x1c1fc0db7084a315, "e4g5"),
    (0xef10edda13ece45a, "g8h6"),
    (0xe762f5ee93d0af6d, "e4g5"),
    (0xef15587a0ab777f4, "e4g5"),
    (0xb595b820410da4f5, "e4g5"),
    (0x23ad049f7163f5a1, "e4g5"),
    (0xa3837891d267cb8e, "c7c8b"),
    (0xbac9bb8c3bd80cc6, "g4g5"),
    (0x33f65119efe19f0b, "h5h6"),
    (0x5231174214b11634, "e4d6"),
    (0x43be1ce7b8aae16c, "f6d7"),
    (0x6a110aae9389ccd8, "d7f6"),
    (0x29d5b71edc0aa1f5, "d7f6"),
    (0xd0a2a9ece8b50d53, "d7f6"),
    (0x60c2578ab47c5cf6, "d7f6"),
    (0x19062140e8f9be01, "d7f6"),
    (0xecee38ebef3b5686, "d7f6"),
    (0x70c7a9b3fa214543, "d7f6"),
    (0x37d8279dded71a7c, "d7f6"),
    (0x01cc9eba78af132d, "d7f6"),
    (0x116f61c47088946b, "g8f6"),
    (0x7df46d3012843821, "d7f6"),
    (0x861a2629af953d56, "d7f6"),
    (0xc17e3fb0a0fc7fc3, "b8c6"),
    (0x24670d7778c006ff, "d8b8"),
    (0x67a3b0c737436bd2, "d8b8"),
    (0x109dc9e55893ebf3, "d8b8"),
    (0x9ed4ae3503fcc774, "d8b8"),
    (0x2eb450535f3596d1, "d8b8"),
    (0x383b34fde7f1f63a, "d8b8"),
    (0x4234bcef8d7cb064, "d8b8"),
    (0xd7d855daf5980c73, "d8d7"),
    (0xea9ce9a3f5ea7b0a, "d8d7"),
    (0x512ffd9665880160, "d8d7"),
    (0x32b78dbc8dca420b, "d8d7"),
    (0xd39ad93653488a43, "d8b8"),
    (0x27146b963ba33720, "d8b8"),
    (0xa0a3c0c691f3ecf8, "c6e5"),
    (0xa81be2357bf4aa70, "g4g6"),
    (0xebdf5f853477c75d, "g4g6"),
    (0x9ce126a75ba7477c, "g4g6"),
    (0x12a8417700c86bfb, "g4g6"),
    (0xa2c8bf115c013a5e, "g4h5"),
    (0xdb0cc9db0084d8a9, "g4g6"),
    (0x2ee4d0700746302e, "g4g6"),
    (0xb447dbbfe4c55ab5, "d7f6"),
    (0xce4853ad8e481ceb, "d7f6"),
    (0xc891ea5ce97e70a7, "f8g7"),
    (0x586ccda5b46591b1, "h8f8"),
    (0x6b52fa8f8fdaeaba, "h8f8"),
    (0x6c96093794367cbd, "h8f8"),
    (0xe2df6ee7cf59503a, "h8f8"),
    (0x52bf90819390019f, "h8f8"),
    (0x2b7be64bcf15e368, "h8f8"),
    (0xde93ffe0c8d70bef, "h8f8"),
    (0xd4b3aab31e6b2487, "h8f8"),
    (0xdca800145d567bb6, "h8f8"),
    (0xb5eac39c41b7affa, "h8f8"),
    (0x9a8042145c55f0b8, "h8f8"),
    (0x10eb0ea1e26a1b98, "h8f8"),
    (0x52ac26b16b13daeb, "h8f8"),
    (0x05633da7052826e8, "h8f8"),
    (0xf1162ea1e80407a8, "h8f8"),
    (0xf227bf50ed59d696, "h8f8"),
    (0xc4678a0b0ce6bda8, "h8f8"),
    (0xd8d51af01aec94f3, "h8f8"),
    (0x7abda2ac16e6e0fc, "h8f8"),
    (0x25648e795604718e, "h8f8"),
    (0x34bbf15d734823f6, "e5g6"),
    (0x24c48523d1fa1f46, "g4g6"),
    (0x2cdf2f8492c74077, "g4g6"),
    (0x0d7082e9d34d4b2d, "g4g6"),
    (0x221a0361ceaf146f, "g4g6"),
    (0x1956e63284de2cd6, "g4g6"),
    (0xea3667c4f9e93e3c, "g4g6"),
    (0xbdf97cd297d2c23f, "g4g6"),
    (0x8482e55aeaeb5c10, "g4g6"),
    (0x06702b9625036ee0, "g4g6"),
    (0xa779b2848aba06e5, "g4e6"),
    (0x810c6dabd079e7a2, "d8a8"),
    (0xae66ec23cd9bb8e0, "d8a8"),
    (0x55e59d07cc834b22, "d8a8"),
    (0x664a8886fadd92b3, "d8a8"),
    (0x3aa6e6d3ac62be2d, "c6e5"),
    (0x321ec4204665f8a5, "g4g6"),
    (0x89ad9d96533837ad, "g4g6"),
    (0x06e400b2663615a9, "g4g6"),
    (0x88ad67623d59392e, "g4g6"),
    (0x38cd99046190688b, "g4h5"),
    (0x4109efce3d158a7c, "g4g6"),
    (0xb4e1f6653ad762fb, "g4g6"),
    (0x2e42fdaad9540860, "d7f6"),
    (0x544d75b8b3d94e3e, "d7f6"),
    (0x5294cc49d4ef2272, "e5c4"),
    (0x88dd974256f30f41, "c4d6"),
    (0x557ae14108e3dcfd, "c4d6"),
    (0x3ce12a1545a39b94, "c4d6"),
    (0x39bd93d0645b5067, "c4d6"),
    (0x4079e51a38deb290, "c4d6"),
    (0xb591fcb13f1c5a17, "c4d6"),
    (0x2f32f77edc9f308c, "c4d6"),
    (0x553d7f6cb61276d2, "c4d6"),
    (0xb51ccf3184f7ac64, "c4d6"),
    (0xe69e72ba58de1c12, "c4d6"),
    (0x3f36b8a1f6ce39a6, "c4d6"),
    (0xba80c93fc7f8a77e, "c4d6"),
    (0x301da81500cdf196, "c4d6"),
    (0x2800bd71c7ef4398, "c4d6"),
    (0x94031850a515abca, "c4d6"),
    (0xe82fce351685f98b, "c4d6"),
    (0x6614c4be68a82915, "c4d6"),
    (0x5f1e662ffaa696c1, "c4d6"),
    (0xd5e869b24032dbcf, "c4d6"),
    (0x618655e2ef060e64, "c4d6"),
    (0x2b338416eb8185c9, "c4d6"),
    (0x5ef3cbde6af8c15b, "c4d6"),
    (0x21ddf4b1399331c7, "c4d6"),
    (0xc13d29dde07c1968, "c4d6"),
    (0xe9f6022fa7b7f93f, "c4d6"),
    (0xb9bce68268ff1667, "c4d6"),
    (0x0d47a0f398633215, "c4d6"),
    (0x59ae050db39b7ec2, "c4d6"),
    (0x2e7ae1cf1ede1c77, "c4e5"),
    (0x316da2c10506c97a, "g4g6"),
    (0x2970b7a5c2247b74, "g4g6"),
    (0x95731284a0de9326, "g4g6"),
    (0xe95fc4e1134ec167, "g4g6"),
    (0x358a254d067de041, "g4g6"),
    (0x5e6e6cfbff6dae2d, "g4g6"),
    (0x566aadaa8a11d1d3, "g4g6"),
    (0xd498636645f9e323, "g4g6"),
    (0x58de0fd9b650462e, "g4g6"),
    (0x2f0aeb1b1b15249b, "g4e6"),
    (0x5dbe824e1534864b, "c6e5"),
    (0x5506a0bdff33c0c3, "g4g6"),
    (0xeeb5f90bea6e0fcb, "g4g6"),
    (0x61fc642fdf602dcf, "g4g6"),
    (0xefb503ff840f0148, "g4g6"),
    (0x5fd5fd99d8c650ed, "g4h5"),
    (0x26118b538443b21a, "g4g6"),
    (0xd3f992f883815a9d, "g4g6"),
    (0x495a993760023006, "d7f6"),
    (0x335511250a8f7658, "d7f6"),
    (0x358ca8d46db91a14, "f8g7"),
    (0xed9ce1c86dc92414, "h8f8"),
    (0xc155e7294045390c, "h8f8"),
    (0xd966255a4d9ac918, "h8f8"),
    (0x572f428a16f5e59f, "h8f8"),
    (0xe74fbcec4a3cb43a, "h8f8"),
    (0x9e8bca2616b956cd, "h8f8"),
    (0x6b63d38d117bbe4a, "h8f8"),
    (0xeeef87292eaa15cb, "h8f8"),
    (0xf6f2924de988a7c5, "h8f8"),
    (0x4af1376c8b724f97, "h8f8"),
    (0x8d10643894870496, "h8f8"),
    (0x81ec4913d4c1729c, "h8f8"),
    (0x44e602cc31a8b20d, "h8f8"),
    (0x47d7933d34f56333, "h8f8"),
    (0x7197a666d54a080d, "h8f8"),
    (0x6d25369dc3402156, "h8f8"),
    (0xcf4d8ec1cf4a5559, "h8f8"),
    (0xca0fb37b7efd6b96, "h8f8"),
    (0xdbd0cc5f5bb139ee, "e5g6"),
    (0x5675c65cbc50f11c, "g4g6"),
    (0x4e68d3387b724312, "g4g6"),
    (0xf26b76191988ab40, "g4g6"),
    (0x39760866463b964b, "g4g6"),
    (0x3172c9373347e9b5, "g4g6"),
    (0xb38007fbfcafdb45, "g4g6"),
    (0x48128f86a2431cfd, "g4e6"),
    (0x08fe0a18e9dff09f, "d8b8"),
    (0x8a0cc4d42637c26f, "d8b8"),
    (0x3c3a4795d0ef437f, "d8b8"),
    (0x167fc52ded8ed428, "d8b8"),
    (0x36d4373202fdf771, "d8b8"),
    (0xfc42f4fa51525d36, "d8b8"),
    (0xfeffc1ac9cd93f89, "d8b8"),
    (0xaeb525015391d0d1, "d8b8"),
    (0x2b055dc6898eaa6a, "c6e5"),
    (0x69626c4fd0eafd07, "d7f6"),
    (0x717f792b17c84f09, "g8f6"),
    (0xb1500a6fc6a2f51a, "g8f6"),
    (0xcf7a6a723b302d85, "d7f6"),
    (0xe010ebfa26d272c7, "d7f6"),
    (0xdb5c0ea96ca34a7e, "d7f6"),
    (0x283c8f5f11945894, "d7f6"),
    (0x46880dc102963ab8, "d7f6"),
    (0xc47ac30dcd7e0848, "d7f6"),
    (0x0c5f93c6947862d3, "d7f6"),
    (0xc62a45f57a58fbd5, "g8f6"),
    (0xee072285563646c6, "d7f6"),
    (0xc5c86bc05d361fb4, "b8c6"),
    (0x5ec28a8d2ad48d03, "d8b8"),
    (0x1d06373d6557e02e, "d8b8"),
    (0x76796b31ee3c002c, "d8b8"),
    (0x6a384e1f0a87600f, "d8b8"),
    (0xe47129cf51e84c88, "d8b8"),
    (0x5411d7a90d211d2d, "d8b8"),
    (0x44142990437c0498, "d8b8"),
    (0xd7e50e54d60f577a, "d8b8"),
    (0x351f1e99c1f252f6, "d8b8"),
    (0x7a392c9775cd0675, "d8b8"),
    (0x9dc944f91a5c7527, "d8b8"),
    (0xb2c9a60a16c87c8d, "d8b8"),
    (0x429eb307b5e57dc6, "d8b8"),
    (0xad7dd220a78c878f, "d8d7"),
    (0x90396e59a7fef0f6, "d8d7"),
    (0x2b8a7a6c379c8a9c, "d8d7"),
    (0x95da4db067a4fc3c, "d8d7"),
    (0x48120a46dfdec9f7, "d8d7"),
    (0xa6a358c242530aa5, "d8b8"),
    (0xa93f5ecc015c01bf, "d8b8"),
    (0x5db1ec6c69b7bcdc, "d8b8"),
    (0xda06473cc3e76704, "c6b8"),
    (0x6bb9cdc89f471d60, "d7f6"),
    (0x287d7078d0c4704d, "d7f6"),
    (0x43022c745baf904f, "d7f6"),
    (0x5f43095abf14f06c, "d7f6"),
    (0xd10a6e8ae47bdceb, "d7f6"),
    (0x616a90ecb8b28d4e, "d7f6"),
    (0x18aee626e4376fb9, "d7f6"),
    (0xed46ff8de3f5873e, "d7f6"),
    (0x716f6ed5f6ef94fb, "d7f6"),
    (0xe29e4911639cc719, "d7f6"),
    (0x006459dc7461c295, "d7f6"),
    (0x10c7a6a27c4645d3, "e1e2"),
    (0x6ebb77ea00521830, "b8a6"),
    (0x4c3d2afb3d1d43b9, "c5a4"),
    (0xdc1dd74d32a9304c, "a4c5"),
    (0x898688569f5b5372, "d7f6"),
    (0x71f1fb67f40b3f2a, "d7f6"),
    (0x74ad42a2d5f3f4d9, "d7f6"),
    (0x37e2f9004124b04f, "d7f6"),
    (0xd99a00b926993094, "e6f7"),
    (0x8983cbc7d98ee84b, "d7f6"),
    (0x0765bef0bea790ad, "d7f6"),
    (0x819bf37f465b1390, "d7f6"),
    (0x93eb656744feae9d, "d7f6"),
    (0x7b21aeec44759d36, "d7f6"),
    (0xa0ef5cbfd8cdb641, "d7f6"),
    (0x472dc8f93217a1d0, "d7e5"),
    (0xe768f73e14f7869d, "a4b6"),
    (0x1e8f6db44a6c7acf, "d7f6"),
    (0x20a6f4024dca07ed, "d7f6"),
    (0x63e94fa0d91d437b, "d7f6"),
    (0x8d91b619bea0c3a0, "a6c5"),
    (0xe59c6d829ca982d0, "e6f7"),
    (0xdbb5f4349b0ffff2, "e6f7"),
    (0x98fa4f960fd8bb64, "e6f7"),
    (0x27e2ed7ba09610f5, "d7f6"),
    (0xeae2d8cb5a150f68, "d7f6"),
    (0x6e178e3804612878, "d7f6"),
    (0xb73518884946d947, "d7f6"),
    (0x5714a8d57ba303f1, "d7f6"),
    (0x22689fcd485f94ba, "e6f7"),
    (0xa09a510187b7a64a, "e6f7"),
    (0x14f46d51288373e1, "e6f7"),
    (0x0234e5e38a0b7875, "d7e5"),
    (0xdd887d6741b71b7f, "d7f6"),
    (0xd97b9ffb9e9a6ca5, "d7f6"),
    (0x5b89513751725e55, "d7f6"),
    (0x7e0f51ef5a8b4c05, "d7f6"),
    (0xa1713cd1c14ef01e, "c8b7"),
    (0x1c9699ee94e03051, "d8b8"),
    (0x22bf005893464d73, "d8b8"),
    (0x9e44d7ad81d87f0e, "d8d7"),
    (0x26582ddf72f06bd3, "b7c6"),
    (0xe3749a41f9c3ac62, "b7c6"),
    (0x03ac2507af424c8f, "b7c6"),
    (0xd00d8f33dc345afb, "d8b8"),
    (0xdf91893d9f3b51e1, "d8b8"),
    (0xe58f67ce57c12b1a, "d8b8"),
    (0x5990a56d8ffe14cb, "d8b8"),
    (0xb000cf71b2576910, "d8b8"),
    (0x4592de27df24704f, "d8b8"),
    (0xb1431379742d1095, "d8b8"),
    (0xd74c76643bf9c136, "d8b8"),
    (0x07d21fc7e54bf5bd, "d8b8"),
    (0xa2022d10d112d373, "b7d5"),
    (0xb464e593a534a948, "d7f6"),
    (0x2c3dfb7522da2214, "d7f6"),
    (0x8afe4256b7a49f00, "b6d5"),
    (0x1bd0373a2012ffed, "d5f6"),
    (0x8a0c8b21016dac22, "d5f6"),
    (0xb425129706cbd100, "d5f6"),
    (0x5f80095f4ee51bc1, "d5f6"),
    (0xb0967bd909f455ff, "d5f6"),
    (0xfddb0e8afa5e5b42, "d5f6"),
    (0x584ce19587f49310, "d8d7"),
    (0x9b624015d56d7fe2, "c8d7"),
    (0x953637c83acfd0fc, "d5f6"),
    (0xa81b5567f09a897b, "c8b7"),
    (0x27b5b542204bf9ea, "d8b8"),
    (0xb66909590134aa25, "d8b8"),
    (0x884090ef0692d707, "d8b8"),
    (0x63e58b274ebc1dc6, "d8b8"),
    (0x9ecff441be56b7ec, "d5f6"),
    (0xe4c07c53d4dbf1b2, "d5f6"),
    (0x04e1cc0ee63e2b04, "d5f6"),
    (0x34bb471a140ce57a, "d8d7"),
    (0xc1be8cf2fa075d45, "b7c6"),
    (0x9ca140d01be14833, "b7c6"),
    (0xa953b5b03a96d6fb, "b7c6"),
    (0x719dfb16d5c2bc4f, "d8b8"),
    (0xf36f35da1a2a8ebf, "d8b8"),
    (0x4701098ab51e5b14, "d8b8"),
    (0xd483b0b190b3becf, "d8b8"),
    (0xa2747f0a57d346d9, "d8b8"),
    (0xe6ca8ea9e4348023, "d8b8"),
    (0x2203c48a60765887, "d8b8"),
    (0xd6469f7f3e807703, "d8b8"),
    (0xe8f8ea24939910be, "d8b8"),
    (0x580b0110c57e7e5f, "d8b8"),
    (0x0841e5bd0a369107, "d8b8"),
    (0x52366b574389fa86, "d8b8"),
    (0x08fdbda744c64907, "d8e8"),
    (0x490b9bf20ab6cd92, "d5f6"),
    (0xb02715a97009a415, "d5f6"),
    (0xcf0ab7a21a7388b8, "d5f6"),
    (0xe8e632c990eab8c8, "d5f6"),
    (0x9e11fd72578a40de, "d5f6"),
    (0xa4bcdf5b4bb36daf, "d5f6"),
    (0x2c3f72ddbd44f3ff, "e3e4"),
    (0x53f0960fce7dcdc0, "d5f6"),
    (0x19936e9893307efd, "d5f6"),
    (0x884fd283b24f2d32, "d5f6"),
    (0xb6664b35b5e95010, "d5f6"),
    (0xf529f097213e1486, "d5f6"),
    (0x718c9c91a0b025cc, "d5f6"),
    (0x0c67297654904132, "d5f6"),
    (0x5a0fb83734d61200, "f5g6"),
    (0xa987d90361c594b1, "c8d7"),
    (0x97756e6a89ed51ec, "d5f6"),
    (0x9afecc7144326228, "f5g6"),
    (0xf956ed64640bd3c5, "d5f6"),
    (0x107c2ec3413d5b65, "d5f6"),
    (0xbb026ef9942eaaa6, "d5f6"),
    (0xb2644c0bc32b2505, "d5f6"),
    (0xcd49ee00a95109a8, "d5f6"),
    (0x048252522ad6636f, "f5e6"),
    (0x9430ce8d91079a8f, "c8b7"),
    (0x1b9e2ea841d6ea1e, "d5f6"),
    (0x8a4292b360a9b9d1, "d5f6"),
    (0xb46b0b05670fc4f3, "d5f6"),
    (0xaf3696a0e0e98b67, "d5f6"),
    (0xf2ba32d2ba2c1adb, "d8e7"),
    (0xf724b0a7f3d88065, "d5f6"),
    (0x0890dcf07591f68e, "d5f6"),
    (0xfd9517189b9a4eb1, "d5f6"),
    (0xa08adb3a7a7c5bc7, "b7c6"),
    (0x9b2c5987b4a96a11, "b7c6"),
    (0x95782e5a5b0bc50f, "d5f6"),
    (0xa8554cf5915e9c88, "d8b8"),
    (0x494582606b72d861, "d5f6"),
    (0xb0690c3b11cdb1e6, "d5f6"),
    (0xb1577dbad46976c0, "d5f6"),
    (0x59997d3257b4bc0c, "d5f6"),
    (0xac0b6c643ac7a553, "d5f6"),
    (0x6bd0432a233382e0, "d5f6"),
    (0xc6895bf472cefbfa, "d8b8"),
    (0x9d403d267a1af73b, "d8b8"),
    (0x5f003668ac1ad67a, "d8b8"),
    (0x8024003e7b12ec96, "d5f6"),
    (0x37f06e5e7c3c94a3, "d5f6"),
    (0x53a25bdebe16f618, "d5f6"),
    (0x4ac5f5c78a85b178, "d8e8"),
    (0xd49d685c93c016b9, "c8b7"),
    (0xa6ece4bee6837875, "d5f6"),
    (0x373058a5c7fc2bba, "d5f6"),
    (0x0919c113c05a5698, "d5f6"),
    (0xe2bcdadb88749c59, "d5f6"),
    (0xb5e216e6d2c464e5, "d5f6"),
    (0x40e7dd0e3ccfdcda, "d5f6"),
    (0x1df8112cdd29c9ac, "b7c6"),
    (0x265e939113fcf87a, "b7c6"),
    (0x280ae44cfc5e5764, "d5f6"),
    (0x152786e3360b0ee3, "d8b8"),
    (0xf4374876cc274a0a, "d5f6"),
    (0xf0c4aaea130a3dd0, "d5f6"),
    (0x0c25b7ac733ce4ab, "d5f6"),
    (0x9a52f52aede5de4c, "d5f6"),
    (0xf518f1e4e680b2da, "d8b8"),
    (0x19372bc747f63f05, "d5f6"),
    (0x287c7a85072b6d6d, "d8b8"),
    (0x0f831e07d105fd98, "d5f6"),
    (0x790f9be0111dcf69, "d8b8"),
    (0x4cfc25ee1102b344, "d8b8"),
    (0xea3c71cbd12b4c2c, "d8b8"),
    (0x9df2e7f7dadaf897, "d5f6"),
    (0x82cc29fd010d0ef5, "d5f6"),
    (0xebbf589316fc6ba2, "d8b8"),
    (0xf7b73fd12dd02313, "d8e8"),
    (0x99b1efaf60b56605, "d5f6"),
    (0x342467c50a6f9700, "d5f6"),
    (0x34983fdf449f4f00, "d8e8"),
    (0x88d9fe1177b6ef29, "c8b7"),
    (0xcf23d76c1e66086b, "d5f7"),
    (0x9ec5c28b56379695, "d5f7"),
    (0x2aa4405f1e71ab1b, "d5f7"),
    (0xd19839a535a38c34, "d5f7"),
    (0xab7ae1c6a25e63ad, "d5f7"),
    (0x3aa65ddd83213062, "d5f7"),
    (0x048fc46b84874d40, "d5h5"),
    (0x47c07fc9105009d6, "d5f7"),
    (0xb874139e96197f3d, "d5c6"),
    (0x4d71d8767812c702, "b7c6"),
    (0x0233dc7accf558c1, "b7c6"),
    (0x259ce134b8834cbc, "b7c6"),
    (0x314ac908e902ae58, "d8b8"),
    (0xf9a14d0e88fa51d2, "d5f7"),
    (0xc3bfa3fd40002b29, "d5f7"),
    (0x7fa0615e983f14f8, "d5f7"),
    (0x8432e923c6d3d340, "d5b3"),
    (0xd743d59e355a80a0, "d7f6"),
    (0x1820ea1553792e79, "c8b7"),
    (0x33613a3c67c6a226, "d8b8"),
    (0xe8de7eb01bf442d4, "d8b8"),
    (0xd6f7e7061c523ff6, "d8b8"),
    (0x95b85ca488857b60, "d8b8"),
    (0x99f34ead132096eb, "d8d7"),
    (0x21efb4dfe0088236, "b7c6"),
    (0x173c7d1f76d7dee7, "b7c6"),
    (0xf7e4c25920563e0a, "b7c6"),
    (0x2445686d5320287e, "d8b8"),
    (0x2bd96e63102f2364, "d8b8"),
    (0x70ed82f0e8863483, "d8b8"),
    (0xdb93c2ca3d95c540, "d8b8"),
    (0x11c78090d8d5599f, "d8b8"),
    (0xadd8423300ea664e, "d8b8"),
    (0xafbbead16949294e, "d8b8"),
    (0xb3339581a8654215, "d8b8"),
    (0x843494561fa34650, "d8b8"),
    (0x02b53285e449161a, "d8b8"),
    (0xbbad23cb5cfdc78d, "d8b8"),
    (0x860cceee21458a35, "d8b8"),
    (0xaee58045d73d9e59, "d8b8"),
    (0xc67455b625ccc3a2, "d8b8"),
    (0xb55e78cd54adcc13, "d8b8"),
    (0xd5cfd760ecfc2b6b, "d8b8"),
    (0x6fbb0de57ba761a2, "d8b8"),
    (0xe71a701afb4aa042, "d8b8"),
    (0x6413fe61836d0c89, "b7d5"),
    (0x522cf1f2b59c4c40, "c8b7"),
    (0xc1517a111b2ed48b, "c6g6"),
    (0x508dc60a3a518744, "c6g6"),
    (0x6ea45fbc3df7fa66, "c6g6"),
    (0x2debe41ea920bef0, "c6g6"),
    (0x10d6de935722a61e, "c6g6"),
    (0xd32e52eb3d93fcc9, "c6g6"),
    (0xe3dbffca2c7a093e, "c6g6"),
    (0x27d8c8f25bc541a1, "c6g6"),
    (0xd25f88492f69c81b, "c6g6"),
    (0x50f3559d98cf90fb, "d8d7"),
    (0xaf6fc5a557721b77, "c6d7"),
    (0x381e6cdf585e1b45, "d8d7"),
    (0x9c16d0d77285edee, "d8b8"),
    (0x938ad6d9318ae6f4, "c6g6"),
    (0xe0d02279b70a71f1, "d8b8"),
    (0x158bfa89214fa3de, "c6g6"),
    (0xee1972f47fa36466, "c6c4"),
    (0xc181065c58cb08a7, "a6b8"),
    (0xf27b2f3b77763077, "d7f6"),
    (0xcc52b68d70d04d55, "d7f6"),
    (0xb596c0472c55afa2, "d7f6"),
    (0x407ed9ec2b974725, "d7f6"),
    (0xd70f2b28f73602c1, "d7f6"),
    (0x640f47c114d0dc8e, "d7f6"),
    (0x8f1d0d2fe40709c3, "d7f6"),
    (0xb22037a21a05112d, "d7f6"),
    (0x71d8bbda70b44bfa, "d7f6"),
    (0x3f3ec571ce835586, "d7f6"),
    (0x6165f49683ba8918, "f8g7"),
    (0xd1bd4c86b1a5e4ea, "h8f8"),
    (0xef94d530b60399c8, "h8f8"),
    (0x9650a3faea867b3f, "h8f8"),
    (0x63b8ba51ed4493b8, "h8f8"),
    (0xf4c9489531e5d65c, "h8f8"),
    (0x47c9247cd2030813, "h8f8"),
    (0xacdb6e9222d4dd5e, "h8f8"),
    (0x91e6541fdcd6c5b0, "h8f8"),
    (0x521ed867b6679f67, "h8f8"),
    (0xaddf0f2afc1e5282, "h8f8"),
    (0x16f8d0b91cae036a, "c8d7"),
    (0xeb73f8b757b5bf68, "c8d7"),
    (0xa1be982bc972e284, "c8d7"),
    (0x5482da684937df1e, "h8f8"),
    (0x23d1a68b852da0e5, "h8f8"),
    (0x5fe98233cfd0397f, "h8f8"),
    (0x28a2ff69a3b8e272, "h8f8"),
    (0xb2b6c990276f1bfd, "h8f8"),
    (0xee57899779eaf997, "h8f8"),
    (0xbcc7092d2d1cadef, "h8f8"),
    (0xad1876090850ff97, "f6h7"),
    (0x317c3fe87cad51c7, "d7f6"),
    (0x358fdd74a380261d, "d7f6"),
    (0xc96ec032c3b6ff66, "d7f6"),
    (0x7744b9d58fe40b83, "d7f6"),
    (0x0017c53643fe7478, "d7f6"),
    (0x23aa2cfeb51b3e27, "c8b7"),
    (0x64500583dccbd965, "d5f7"),
    (0x35b61064949a479b, "d5f7"),
    (0x81d792b0dcdc7a15, "d5f7"),
    (0x7aebeb4af70e5d3a, "d5f7"),
    (0x0009332960f3b2a3, "d5f7"),
    (0x91d58f32418ce16c, "d5f7"),
    (0xaffc1684462a9c4e, "d5h5"),
    (0xd638604e1aaf7eb9, "d5f7"),
    (0xe4a7fbed3f9a62ae, "d8c8"),
    (0xb4a18b21c1ccd3da, "d5e6"),
    (0xc0d665c000ddf905, "d8e7"),
    (0xecb3ad26d2fdd8d8, "d5f7"),
    (0xd18e97ab2cffc036, "d5f7"),
    (0x12761bd3464e9ae1, "d5f7"),
    (0x989352408fc6cc02, "d5f7"),
    (0xeed8adb6f126b06e, "d5c6"),
    (0xe6020a99babf160c, "b7c6"),
    (0xa9400e950e5889cf, "b7c6"),
    (0x9a391be72baf7f56, "d8c8"),
    (0x52d29fe14a5780dc, "d5f7"),
    (0x56217d7d957af706, "d5f7"),
    (0xaac0603bf54c2e7d, "d5f7"),
    (0x5152e846aba0e9c5, "d5b3"),
    (0xba4335329125e576, "d7f6"),
    (0x7fd2cebba9eacf3c, "c8b7"),
    (0xecaf4558075857f7, "c6g6"),
    (0x7d73f94326270438, "c6g6"),
    (0x435a60f52181791a, "c6g6"),
    (0x5807fd50a667368e, "c6e6"),
    (0xeb0791b94581e8c1, "d8e7"),
    (0x0015db57b5563d8c, "c6g6"),
    (0x3d28e1da4b542562, "c6g6"),
    (0xfed06da221e57fb5, "c6g6"),
    (0xb03613099fd261c9, "c6g6"),
    (0x74352431e86d2956, "c6g6"),
    (0x027edbc7968d553a, "c6g6"),
    (0x7d0d6ad484b91387, "d8d7"),
    (0x8291faec4b04980b, "c6d7"),
    (0xb1e8ef9e6ef36e92, "d8c8"),
    (0xbe74e9902dfc6588, "c6g6"),
    (0xcd2e1d30ab7cf28d, "d8c8"),
    (0x4666164a92e7cb29, "c6g6"),
    (0xbdf49e37cc0b0c91, "c6c4"),
    (0x23338ebcf76f2d56, "c8a6"),
    (0xcb5e070398eeb538, "d8c8"),
    (0x9ab812e4d0bf2bc6, "d8c8"),
    (0x2ed9903098f91648, "d8c8"),
    (0xd5e5e9cab32b3167, "d8c8"),
    (0x00f21404020ff013, "d8c8"),
    (0x793662ce5e8a12e4, "d8c8"),
    (0x3cc6e488bec1364a, "d8c8"),
    (0x43bdafa696d8b485, "d8c8"),
    (0x7e80952b68daac6b, "d8c8"),
    (0xbd781953026bf6bc, "d8c8"),
    (0xf39e67f8bc5ce8c0, "d8c8"),
    (0x379d50c0cbe3a05f, "a6b7"),
    (0xa413d53ecafc5d1c, "c8d6"),
    (0xf5f5c0d982adc3e2, "c8d6"),
    (0x4194420dcaebfe6c, "c8d6"),
    (0xbaa83bf7e139d943, "c8d6"),
    (0x6fbfc639501d1837, "c8d6"),
    (0x167bb0f30c98fac0, "c8d6"),
    (0x538b36b5ecd3de6e, "c8d6"),
    (0x2cf07d9bc4ca5ca1, "c8d6"),
    (0x2e9b7d0be7113417, "c8d6"),
    (0x6b58ebbeb1ab3e97, "c8d6"),
    (0x9145d14015da8ebb, "b7c6"),
    (0xae745c203a98f926, "c8d6"),
    (0x9d0d49521f6f0fbf, "c8d6"),
    (0x92914f5c5c6004a5, "c8d6"),
    (0x9662adc0834d737f, "c8d6"),
    (0x6a83b086e37baa04, "c8d6"),
    (0x77d6087a436e4946, "c8d6"),
    (0xb1edc39a60813c01, "c8d6"),
    (0x05dd5dcf75ddf358, "c8d6"),
    (0x2089111e139f27b1, "c8d6"),
    (0x9a62bd1d166b531f, "c8d6"),
    (0xe33b612e5107c85b, "c8d6"),
    (0xee314a0ba9d6e78b, "c8d6"),
    (0xb8a48ab3153f2e4f, "c8d6"),
    (0xf8ffd5eb67a0b071, "c8d6"),
    (0x9c820bceca553e10, "c8d6"),
    (0x499b7728ae4deff0, "c8d6"),
    (0x2983c73b88017d41, "c8b6"),
    (0xb0bf5e3a9154f089, "d7f6"),
    (0xe1594bddd9056e77, "d7f6"),
    (0x5538c909914353f9, "d7f6"),
    (0xae04b0f3ba9174d6, "d7f6"),
    (0x7b134d3d0bb5b5a2, "d7f6"),
    (0x02d73bf757305755, "d7f6"),
    (0x4727bdb1b77b73fb, "d7f6"),
    (0x604ed0988c53fa36, "d7f6"),
    (0xd34ebc716fb52479, "d7f6"),
    (0x385cf69f9f62f134, "d7f6"),
    (0xd6240f26f8df71ef, "d8c8"),
    (0x863dc45807c8a930, "d7f6"),
    (0x82ce26c4d8e5deea, "d7f6"),
    (0x7e2f3b82b8d30791, "d7f6"),
    (0x701ddd8813083f5d, "d7f6"),
    (0x39a3ad04c8c54d87, "d7f6"),
    (0xef95b73a49002dbd, "d8c8"),
    (0xbd1a516f0dc9d1a8, "d7f6"),
    (0x7bd8ad53a19d0737, "d8c8"),
    (0x5c478d2078660e30, "d8c8"),
    (0xb469ad478bcfa98b, "d8c8"),
    (0x6b4d9b115cc79367, "d7f6"),
    (0xdc99f5715be9eb52, "d7f6"),
    (0x85bdb3ffe63fc029, "d7e5"),
    (0xd9afba4c38079289, "c8d6"),
    (0x068b8c1aef0fa865, "c8d6"),
    (0xb15fe27ae821d050, "c8d6"),
    (0xcb7f3748e361e4a9, "c8d6"),
    (0x911138fbbd976dbc, "c8d6"),
    (0x14c30dd7abfc83e1, "d8d7"),
    (0x04153983e3b9d6b3, "d8d7"),
    (0xc1398e1d688a1102, "d8d7"),
    (0xf2409b6f4d7de79b, "d8c8"),
    (0xfddc9d610e72ec81, "d8c8"),
    (0xf92f7ffdd15f9b5b, "d8c8"),
    (0x05ce62bbb1694220, "d8c8"),
    (0x189bda47117ca162, "d8c8"),
    (0xdea011a73293d425, "d8c8"),
    (0x6a908ff227cf1b7c, "d8c8"),
    (0x727ac8f9c2c13e61, "d8c8"),
    (0xf52f6f204479bb3b, "d8c8"),
    (0xb1c8b8c98059d18b, "d8c8"),
    (0x817c9836fbc40faf, "d8c8"),
    (0xd7e9588e472dc66b, "d8c8"),
    (0x1b68aecf7f13f620, "d8c8"),
    (0x46ce1506da139565, "d8c8"),
    (0xb6e268716a157aad, "d8c8"),
    (0x69c65e27bd1d4041, "d8c8"),
    (0xde123047ba333874, "d8c8"),
    (0xa432e575b1730c8d, "d8c8"),
    (0xfe5ceac6ef858598, "d8e8"),
    (0x3dc55a98e77d5c5d, "c8b7"),
    (0xcd3bce3daff030f0, "d8c8"),
    (0xf312578ba8564dd2, "d8c8"),
    (0x8ad62141f4d3af25, "d8c8"),
    (0x7f3e38eaf31147a2, "d8c8"),
    (0xe84fca2e2fb00246, "d8e7"),
    (0x5b4fa6c7cc56dc09, "d8e7"),
    (0xb05dec293c810944, "d8c8"),
    (0x8d60d6a4c28311aa, "d8c8"),
    (0x4e985adca8324b7d, "d8c8"),
    (0x007e247716055501, "d8c8"),
    (0xc47d134f61ba1d9e, "d7e5"),
    (0x6921fbf3bf5937f7, "b7c6"),
    (0x57086245b8ff4ad5, "b7c6"),
    (0x2ecc148fe47aa822, "b7c6"),
    (0xdb240d24e3b840a5, "d8d7"),
    (0xff559309dcffdb0e, "b7c6"),
    (0x1447d9e72c280e43, "b7c6"),
    (0xe59193e0effe5def, "b7c6"),
    (0x9628baeb9fa7dbf0, "d8d7"),
    (0xd1dfe905642252d4, "b7c6"),
    (0xaa26eb20b4825647, "b7c6"),
    (0xf11207b34c2b41a0, "b7c6"),
    (0x5a6c47899938b063, "b7c6"),
    (0xaed509bc6baf219d, "b7c6"),
    (0x523414fa0b99f8e6, "b7c6"),
    (0xade5de8d688453ad, "b7c6"),
    (0xf151820cd1c15156, "b7c6"),
    (0x0514d9f98f377ed2, "b7c6"),
    (0xf5a23742fa11f7bb, "d8d7"),
    (0x91dfe96757e479da, "d8d7"),
    (0x44c6958133fca83a, "d8d7"),
    (0xe91411054e99b393, "b7c6"),
    (0x536800656a112e08, "b7c6"),
    (0xbbcef15e95372386, "b7c6"),
    (0x3e43ab114f72f6cc, "b7c6"),
    (0xb2bb8a60a7891485, "b7c6"),
    (0xd13f2c6facee1f86, "b7c6"),
    (0xe25d7d057abc4fdd, "b7c6"),
    (0x9bffa8a8881e9221, "b7d5"),
    (0x0e3cdeeea42b5140, "d8c8"),
    (0x5508327d5c8246a7, "d8c8"),
    (0xfe7672478991b764, "d8c8"),
    (0x0acf3c727b06269a, "d8c8"),
    (0xf62e21341b30ffe1, "d8c8"),
    (0x09ffeb43782d54aa, "d7f6"),
    (0x47769d6c24cea746, "d7f6"),
    (0x7c3fa48bf9541038, "d8c8"),
    (0xd7ff320e1bbaec79, "d7f6"),
    (0xbf596d3f9f4d7cf8, "d8c8"),
    (0x6f5942c15e882260, "d8c8"),
    (0xde0626e242846f66, "d8c8"),
    (0x1fd4c490859e2481, "d7f6"),
    (0x9a599edf5fdbf1cb, "d7f6"),
    (0x16a1bfaeb7201382, "d7f6"),
    (0x03d4c79838638d2d, "d7f6"),
    (0x3fe59d6698b79526, "d7e5"),
    (0xe2e16cce307566b1, "d7f6"),
    (0x553502ae375b1e84, "d7f6"),
    (0x32fc484f9d5a38de, "d7e5"),
    (0xd64857929dd96bf9, "d7b8"),
    (0x5564cb014c9f2b17, "c8d7"),
    (0x6b4d52b74b395635, "c8d7"),
    (0x7010cf12ccdf19a1, "c8d7"),
    (0xc310a3fb2f39c7ee, "c8d7"),
    (0x2802e915dfee12a3, "c8d7"),
    (0x153fd39821ec0a4d, "c8d7"),
    (0xd6c75fe04b5d509a, "c8d7"),
    (0x2041c0823c6611b5, "c8d7"),
    (0x2a9a7ebf82ea302c, "c8b7"),
    (0xda64ea1aca675c81, "d8c8"),
    (0xe44d73accdc121a3, "d8c8"),
    (0xff10ee094a276e37, "d8e7"),
    (0x4c1082e0a9c1b078, "d8e7"),
    (0xa702c80e59166535, "d8c8"),
    (0x58b6a459df5f13de, "d8d7"),
    (0xe0aa5e2b2c770703, "b7c6"),
    (0x1e206b086991f164, "b7c6"),
    (0xf50409f58a0ff262, "a8b8"),
    (0x7c21434602b21019, "d8c8"),
    (0x4208daf005146d3b, "d8c8"),
    (0x5955475582f222af, "d8e7"),
    (0xea552bbc6114fce0, "d8e7"),
    (0x0147615291c329ad, "d8c8"),
    (0x97122deec69b318b, "d8c8"),
    (0x20e6526d8fc1177c, "d8c8"),
    (0x8049b1f5027e5766, "d8c8"),
    (0x4a8dfe33cc44ae3a, "d8c8"),
    (0xfef30d05178a5f46, "d8d7"),
    (0x46eff777e4a24b9b, "d8d7"),
    (0x3d9377353fa9fa8a, "d8d7"),
    (0xbf265395096971a9, "d8c8"),
    (0xe412bf06f1c0664e, "d8c8"),
    (0x4f6cff3c24d3978d, "d8c8"),
    (0xbbd5b109d6440673, "d8c8"),
    (0x4734ac4fb672df08, "d8c8"),
    (0xb8e56638d56f7443, "d8c8"),
    (0x4e0cf1de4080d8fc, "d8c8"),
    (0x92a0e480e80ee314, "d8c8"),
    (0x66e5bf75b6f8cc90, "d8c8"),
    (0x39a6e30386de8e1f, "d8c8"),
    (0x309966e953de9c4a, "d8c8"),
    (0xaece49eb28dc0468, "d8c8"),
    (0x2b4313a4f299d122, "d8c8"),
    (0xa7bb32d51a62336b, "d8c8"),
    (0xb2ce4ae39521adc4, "d8c8"),
    (0x8eff101d35f5b5cf, "d8e8"),
    (0x1963fac9c1bc3d31, "d8c8"),
    (0x4257165a39152ad6, "d8c8"),
    (0xe9295660ec06db15, "d8c8"),
    (0x1d9018551e914aeb, "d8c8"),
    (0xe17105137ea79390, "d8c8"),
    (0x1ea0cf641dba38db, "d8c8"),
    (0xe849588288559464, "d8c8"),
    (0x34e54ddc20dbaf8c, "d8c8"),
    (0xc0a016297e2d8008, "d8c8"),
    (0x672575804e4453a2, "d8c8"),
    (0xb23c09662a5c8242, "d8c8"),
    (0x623c2698eb99dcda, "d8c8"),
    (0x6b03a3723e99ce8f, "d8c8"),
    (0xf5548c70459b56ad, "d8c8"),
    (0x70d9d63f9fde83e7, "d8c8"),
    (0x01fe9b89d2b77ff3, "d8c8"),
    (0x148be3bf5df4e15c, "d8c8"),
    (0x28bab941fd20f957, "b7d5"),
    (0x9663dbd247444aa7, "c8d7"),
    (0xcd573741bfed5d40, "c8d7"),
    (0x6629777b6afeac83, "c8d7"),
    (0x9290394e98693d7d, "c8d7"),
    (0x6e712408f85fe406, "c8d7"),
    (0x91a0ee7f9b424f4d, "c8d7"),
    (0x674979990eade3f2, "c8d7"),
    (0x436d4ffa3a6f053e, "d8d7"),
    (0x4fa03732f8d5f79e, "c8d7"),
    (0xfd5d4f608a7a1a7d, "c8d7"),
    (0x1bc01e07f077658c, "c8d7"),
    (0xe154cd9381bf7a60, "d8d7"),
    (0x878bc1ac66f13f66, "c8d7"),
    (0x02069be3bcb4ea2c, "c8d7"),
    (0x8efeba92544f0865, "c8d7"),
    (0x9b8bc2a4db0c96ca, "c8d7"),
    (0x9ef08f03dc3ad14a, "a6c5"),
    (0xa7ba985a7bd88ec1, "c8e6"),
    (0x777fb24ee50fba9e, "d7f6"),
    (0xc0abdc2ee221c2ab, "d7f6"),
    (0x0cad22bab2d3c648, "d7f6"),
    (0xa01bd94a0f9e99ed, "d7e5"),
    (0xe2344efb350f4d6e, "a4c5"),
    (0x77cc087a3700db2b, "d7f6"),
    (0x4fd862d1f3ad4208, "d7f6"),
    (0x09cb60b64682cd6d, "d7f6"),
    (0xe7b3990f213f4db6, "e6f7"),
    (0xb7aa5271de289569, "d7f6"),
    (0x394c2746b901ed8f, "d7f6"),
    (0xbfb26ac941fd6eb2, "d7f6"),
    (0xadc2fcd14358d3bf, "d7f6"),
    (0x3529d20818e5600a, "d7f6"),
    (0x4508375a43d3e014, "d7f6"),
    (0x9ec6c509df6bcb63, "d7f6"),
    (0x7904514f35b1dcf2, "d7e5"),
    (0xa17bf559a1d809f8, "b1a1"),
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
