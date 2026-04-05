use crate::moves::Move;
use std::collections::HashMap;
use std::sync::OnceLock;

/// Minimal solution book for 1.e3 — only reachable positions.
/// Cleaned by reachable-extractor from 2601 entries.
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
    (0x38a0d78a9a5fa6a4, "d1g4"),   // Qg4! reclaims Ne4 + flips c8 via open d7, Bd7# mate in 2,   // mate in 3 half-moves
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
    (0x4a265066327375c4, "d1h5"),   // mate in 1 half-moves
    (0xddb48b830b17d56a, "g8f6"),   // mate in 1 half-moves
    (0x82f2788fce10c92f, "e4d6"),   // mate in 1 half-moves
    (0x94dc6ef919c26a60, "e4d6"),   // mate in 1 half-moves
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
    (0x6bd85e2690b0882e, "e4d6"),
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
    (0xda82053fdad6fad3, "f1c4"),   // Bc4!! flips b5+Ng8 via f7 vacancy, ALL responses mate,   // f3 reclaims Ne4+Qg4 (Bc4 fails — leaves queen on g4),
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
    (0xa163f5925bcd9e59, "g4f4"),   // Qf4! → b4 → Nf6# mate in 2,
    (0x3f189a50d877c56f, "e4d6"),
    (0x8b3aecbc5f7778cf, "e4d6"),
    (0x785a6d4a22406a25, "e4d6"),
    (0x8e9951687d694226, "e4d6"),
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
    (0x8f7d58129931e847, "e4d6"),
    (0x16ecad40d90e00e4, "f5d5"),
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
    (0x1d50de0fef197e9c, "f1c4"),   // Bc4!! instant mate (f7 vacancy diagonal),
    (0xbffb502bd6f2a9e1, "e4d6"),
    (0x024090211c9a02bd, "e4d6"),
    (0x8e9eb43c8f898cd9, "e4d6"),
    (0x94ba0ec095b8b1e6, "e4d6"),
    (0x1f159171280dcb78, "e4d6"),
    (0x4d4c9e3087de9bcf, "g7g8q"),
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
    (0x5a004a1c79d5cd4f, "g7g8q"),
    (0x17f6fb07e297f858, "e4d6"),
    (0x4248a86f472001b4, "e4d6"),
    (0x93ffde8127bee0cc, "e4d6"),
    (0x87b90b161c713666, "e4d6"),
    (0xbccc2b653a2f80b7, "e4d6"),
    (0xc054e46a4752a9b3, "e4d6"),
    (0xa2cd0f059c3104d0, "e4d6"),
    (0x11cd63ec7fd7da9f, "e4d6"),
    (0xa1bfd2c4a389fdfc, "c8d7"),
    (0x03145ce09a622a81, "e4d6"),
    (0xbeaf9cea500a81dd, "e4d6"),
    (0x3271b8f7c3190fb9, "e4d6"),
    (0x2855020bd9283286, "e4d6"),
    (0xa3fa9dba649d4818, "e4d6"),
    (0x655cdb38824154e5, "e4d6"),
    (0x8201dd65bf77eb82, "c8d7"),
    (0xd3c9c7edaea801bd, "e4d6"),
    (0xf0073b4726287fe0, "e4d6"),
    (0x0367bab15b1f6d0a, "e4d6"),
    (0x697256db214e90bd, "e4f6"),
    (0xb379d6421efbee6a, "e4d6"),
    (0xd25b4091c7c1d389, "e4c5"),
    (0x95a3feeec4e9e9b1, "g8f6"),
    (0x99772f1b039ec132, "g8f6"),
    (0xc608c405a405a0ce, "g8f6"),
    (0x83f84243444e8460, "g8f6"),
    (0xa4912f6a7f660dad, "g8f6"),
    (0x179143839c80d3e2, "g8f6"),
    (0x2f8460ad0a5fdecb, "g8f6"),
    (0x342d9898204e06c4, "g8f6"),
    (0x4a638dcba8a6a578, "g8f6"),
    (0x74a8c95ad9043f02, "g8f6"),
    (0x11b97831e9b9312c, "g8f6"),
    (0x3b0d1005f2de86d1, "g8f6"),
    (0x68bf60c5e26d758a, "g8f6"),
    (0x3257581b242141e4, "c4f7"),
    (0x2e0922643a7f3bfb, "g8f6"),
    (0xa5a6bdd587ca4165, "g8f6"),
    (0xf99f5fa17e648ffd, "h2h3"),
    (0x8e2f8eb90ba6d710, "g4h5"),
    (0x82fb5f4cccd1ff93, "g4h5"),
    (0xdd84b4526b4a9e6f, "g4h5"),
    (0x987432148b01bac1, "g4h5"),
    (0xbf1d5f3db029330c, "g4h5"),
    (0x0c1d33d453cfed43, "g4h5"),
    (0xa37fccd27c12b601, "f2f3"),
    (0xfe6bef1436b70595, "e4d6"),
    (0xf2bf3ee1f1c02d16, "e4d6"),
    (0xd404a3350adeae1d, "e4d6"),
    (0xadc0d5ff565b4cea, "e4d6"),
    (0xe83053b9b6106844, "e4d6"),
    (0xcf593e908d38e189, "e4d6"),
    (0x7c5952796ede3fc6, "e4d6"),
    (0x45c1339ec821d7df, "e4d6"),
    (0xce6eac2f7594ad41, "e4d6"),
    (0x9c37a36eda47fdf6, "g7g8r"),
    (0x220ce2fe8f311e40, "g8f8"),   // depth 1
    (0x6143595c1be65ad6, "g8f8"),   // depth 1
    (0x4d366b4ea98cddeb, "g4g6"),   // depth 1
    (0x1e38127233ffd819, "g8f8"),   // depth 1
    (0x04b77f2a742f9d4b, "g8f8"),   // depth 1
    (0x3866ede4f07b1d1c, "g8f8"),   // depth 1
    (0x39517f5b08d751d4, "g8f8"),   // depth 1
    (0x78ae48df12c60a75, "c4f7"),   // depth 1
    (0x2efec97f1ddf3553, "g8f8"),   // depth 1
    (0xc3d007d63692cf3b, "g4g6"),   // depth 1
    (0x8a5113b2eb318f9b, "g8f8"),   // depth 1
    (0x183f4f0532c028a1, "g8f8"),   // depth 1
    (0x7cd53a6d8013958a, "g8f8"),   // depth 1
    (0x124980500e17bc17, "g4g6"),   // depth 1
    (0x6b8df69a52925ee0, "g4h5"),   // depth 1
    (0x5bc89434d3b4fcb7, "g8f8"),   // depth 1
    (0x0863aedfb358b5c8, "g8f8"),   // depth 1
    (0xb3c972554dce6782, "g8f8"),   // depth 1
    (0x3483865037f57eab, "g8f8"),   // depth 1
    (0x41e2babb6efbf568, "g4g6"),   // depth 1
    (0x65fee639a835a179, "g4g6"),   // depth 1
    (0x70d06b3fd5741174, "g4g6"),   // depth 1
    (0xb97e6da85e120404, "g8f8"),   // depth 1
    (0x2c7ff25d7596e1d9, "g8f8"),   // depth 1
    (0x01d2cbb1838d068e, "g8f8"),   // depth 1
    (0xbfd82ef43da98794, "c4f7"),   // depth 1
    (0x57b90616ee5c98b9, "g4g6"),   // depth 1
    (0x4e279da2c4523044, "d2d3"),
    (0xfaba874ee6b77b8c, "f8h6"),
    (0x555fd98c34b60b9a, "e1f1"),
    (0x21ebb03ba3f0d7af, "a2a3"),
    (0x2c56091ca2d99b31, "f2d1"),
    (0x628897bc1a3bc355, "d1f2"),
    (0xab8c7f3384d6f722, "g4h5"),
    (0x97b6cdf87e815542, "g4h5"),
    (0xd2464bbe9eca71ec, "g4h5"),
    (0x907290ff7de8e4ba, "g4h5"),
    (0xe001ab91d8a3482c, "g4h5"),
    (0x2364a5600f2b51c3, "g4h5"),
    (0x1672b8b1b15e4390, "g4h5"),
    (0x66bc1a907056159f, "g4g6"),
    (0xb0ab63a12d263c7f, "g4h5"),
    (0x7fb72b99e0fbce77, "g4h5"),
    (0xf418b4285d4eb4e9, "g4h5"),
    (0xb51084adce24c306, "g4h5"),
    (0xf8d96e8f0b2ffd72, "g4h5"),
    (0x0fe3ed1bdde7390f, "g4h5"),
    (0xd45a3f88a663ed8f, "g4h5"),
    (0xce7015346ed89a2a, "b2b3"),
    (0xa53eda4db4b30ef3, "c4f7"),
    (0x0556c37669f5d3b4, "c4f7"),
    (0x396c71bd93a271d4, "c4f7"),
    (0x7c9cf7fb73e9557a, "c4f7"),
    (0xbf1b10fe76465a16, "e3e4"),
    (0x4a38f68a8ac153e1, "f5g6"),
    (0xea50efb157878ea6, "f5g6"),
    (0xd66a5d7aadd02cc6, "f5g6"),
    (0xc81f5c9065660432, "f5g6"),
    (0xb4f3b61576b381a5, "f5g6"),
    (0xd18d3ff57e57b896, "h8h7"),
    (0x018fafacaf570919, "h7e7"),
    (0xa1e7b6977211d45e, "h7e7"),
    (0x54798cf0880ac56c, "h7e7"),
    (0x56a28f1d108eb446, "h7e7"),
    (0x6cd7d334869136e3, "h7e7"),
    (0x25d2e88fbe4675d8, "h7e7"),
    (0xcdbc03f4df30b258, "h7e7"),
    (0xfe737d8cab899795, "h7e7"),
    (0x38d53b0e4d558b68, "h7e7"),
    (0xfbe0bf0f7e1af711, "c7c8n"),
    (0xf3f45e3db58c555a, "c8d6"),
    (0x8eaaa0ab4d59cd59, "c8d6"),
    (0x539c470668ca881d, "c8d6"),
    (0xa6027d6192d1992f, "c8d6"),
    (0xa4d97e8c0a55e805, "c8d6"),
    (0x9eac22a59c4a6aa0, "c8d6"),
    (0xd7a9191ea49d299b, "c8d6"),
    (0x87a713ac0ce7b148, "c8d6"),
    (0x0c088c1db152cbd6, "c8d6"),
    (0xcaaeca9f578ed72b, "c8d6"),
    (0x3338ad6116d763e2, "c8d6"),
    (0x1a43075ad385e350, "c8d6"),
    (0x963298d301f6246d, "c8d6"),
    (0x9fc510d838fb0e8b, "c8d6"),
    (0x01ac0aabeac77c78, "c8d6"),
    (0xb7c888066b1fc89a, "b5b6"),
    (0x7af5c167463b20ad, "c7e7"),
    (0x07ab3ff1beeeb8ae, "c7e7"),
    (0xda9dd85c9b7dfdea, "c7e7"),
    (0x2f03e23b6166ecd8, "c7e7"),
    (0x2dd8e1d6f9e29df2, "c7e7"),
    (0x17adbdff6ffd1f57, "c7e7"),
    (0x08116564624800f2, "h6g5"),
    (0x9af0195da0bea0c7, "d8e7"),
    (0xe7aee7cb586b38c4, "d8e7"),
    (0xcf063a0187e36cb2, "d8e7"),
    (0x08949e6d4e78d61c, "d8e7"),
    (0x65379986c39e93cf, "d8e7"),
    (0x6e6bc52793a80f0a, "c7e7"),
    (0x887e97be277080d7, "c7e7"),
    (0xbead5e7eb1afdc06, "c7e7"),
    (0x314c897ca45b465b, "c7e7"),
    (0xe6c0f6088feadb0a, "d8e7"),
    (0x6e2fbe3e9ce4eed6, "g5f6"),
    (0x82daf88e99b7ac22, "d8e7"),
    (0xff84061861623421, "d8e7"),
    (0xd72cdbd2beea6057, "d8e7"),
    (0x10be7fbe7771daf9, "d8e7"),
    (0x7d1d7855fa979f2a, "d8e7"),
    (0x4aae34707a693ecf, "d8e7"),
    (0xa2393bd9d9935dd4, "d8e7"),
    (0x17ef795d0da25f6c, "d8e7"),
    (0x56f230eff8a590cf, "d8e7"),
    (0x6fc901772b73f2f3, "d8e7"),
    (0x72c7b55d6dd6f6d1, "c7c5"),
    (0x52a963a82a645291, "c8d6"),
    (0x2ff79d3ed2b1ca92, "c8d6"),
    (0x417ed8dc222fbe70, "c8d6"),
    (0xad6ee37349446199, "c8d6"),
    (0xc194d10f4b27e8bb, "c8d6"),
    (0x6bf3f70ac866d0e0, "c8d6"),
    (0xf915f3892e81b40d, "c8d6"),
    (0x9b7ea8f33e8e7766, "c8d6"),
    (0xa9975b38e681d8ff, "c8d6"),
    (0x926590f4893f6429, "c8d6"),
    (0xbb1e3acf4c6de49b, "c8d6"),
    (0x376fa5469e1e23a6, "c8d6"),
    (0xafd29c0fa5fd0c56, "c8d6"),
    (0xbe9e808b1e138ef7, "c8d6"),
    (0xea20a8c74541f148, "c8d6"),
    (0x194029313876e3a2, "c8d6"),
    (0xfb832da230ecc86e, "c4a6"),
    (0x72b3132b5c6627b0, "c8d6"),
    (0x9d896a8d4a75c968, "c8d6"),
    (0xf02a6d66c7938cbb, "c8d6"),
    (0xee9c1d003e9104c3, "c8d6"),
    (0xa4517d9ca056592f, "c8d6"),
    (0xc63a26e6b0599a44, "c8d6"),
    (0xf4d3d52d685635dd, "c8d6"),
    (0xcf211ee107e8890b, "c8d6"),
    (0xe65ab4dac2ba09b9, "c8d6"),
    (0x6a2b2b5310c9ce84, "c8d6"),
    (0xf296121a2b2ae174, "c8d6"),
    (0x23e52f3524340ffe, "c8d6"),
    (0x0ef69f495a0305b2, "c5c3"),
    (0xf3e2bf544ca77564, "c8d6"),
    (0x1cd8c6f25ab49bbc, "c8d6"),
    (0x717bc119d752de6f, "c8d6"),
    (0x89d8e3e69a88d9ca, "c8d6"),
    (0x6fcdb17f2e505617, "c8d6"),
    (0xb7e6d56056706f16, "c8d6"),
    (0x2500d1e3b0970bfb, "c8d6"),
    (0x476b8a99a098c890, "c8d6"),
    (0x7582795278976709, "c8d6"),
    (0xe32b437d7461019a, "c8d6"),
    (0xd9c1d1dd9386bc7e, "c8d6"),
    (0x3d982f4c621a9b57, "c3c4"),
    (0xc6b2087aa140b3ad, "c8d6"),
    (0x7c8b2e76a233c076, "c8d6"),
    (0x6c83abfde53d4cd0, "c8d6"),
    (0xa48649b6af6d2136, "c8d6"),
    (0x0f9e59fbd6d70ddd, "c8d6"),
    (0xaa1ebf84641ffda9, "c8d6"),
    (0x38f8bb0782f89944, "c8d6"),
    (0x5a93e07d92f75a2f, "c8d6"),
    (0x687a13b64af8f5b6, "c8d6"),
    (0x632e18ce22cc2323, "c8d6"),
    (0x7af37241e014c9d2, "c8d6"),
    (0xf682edc832670eef, "c8d6"),
    (0x5e9914350e0e4b5c, "c8d6"),
    (0x064e4064b260ccc7, "h4h5"),
    (0xa1988399b3cf3055, "g6f7"),
    (0x660a27f57a548afb, "g6f7"),
    (0x0ba9201ef7b2cf28, "g6f7"),
    (0xc3acc255bde2a2ce, "g6f7"),
    (0xa07b2f7675d381db, "d8d7"),
    (0xcd34346776907e51, "g6f7"),
    (0xeab64feaeb14dc37, "d8d7"),
    (0x188e42c041cf6d8d, "g6f7"),
    (0x0f5098555877764e, "g6f7"),
    (0x5836db5ed2752c14, "g6f7"),
    (0x972ff6bf33db79f1, "g6f7"),
    (0x50ec4e84ae8a3659, "g6f7"),
    (0x1858463a8b90c001, "g6f7"),
    (0xff9ab494fa3ad331, "g6f7"),
    (0x7eec782f365b865f, "g6f7"),
    (0xcb30008df3e59d25, "g6f7"),
    (0x5c2a16eab945571d, "g6f7"),
    (0x5760d24bd1f2f83c, "g6f7"),
    (0x215957a90a3398bf, "g6f7"),
    (0x4b70daa0e6d01539, "g6f7"),
    (0x1cbfc1b688ebe93a, "g6f7"),
    (0x4ad57099fa6533da, "h5h6"),
    (0xe9dd5a3c62b281b8, "g7g8"),
    (0x2e4ffe50ab293b16, "g7g8"),
    (0x43ecf9bb26cf7ec5, "g7g8"),
    (0x8be91bf06c9f1323, "g7g8"),
    (0x5d5a89dddfcdf6bd, "g7g8"),
    (0x8571edc2a7edcfbc, "g7g8"),
    (0x1797e941410aab51, "d8d7"),
    (0xde544d3abea4c68e, "d8e7"),
    (0x471541f0890ac7a3, "g7e7"),
    (0x9edb0339df643efe, "g7e7"),
    (0x107302fb03089df9, "g7g8"),
    (0xdf6a2f1ae2a6c81c, "g7g8"),
    (0x18a997217ff787b4, "g7g8"),
    (0x501d9f9f5aed71ec, "g7g8"),
    (0x4c414a88e13e1136, "g7g8"),
    (0x559c200723e6fbc7, "g7g8"),
    (0xd9edbf8ef1953cfa, "g7g8"),
    (0x71f64673cdfc7949, "g7g8"),
    (0xb7df6d312b4762dc, "g7g8"),
    (0x36a9a18ae72637b2, "g7g8"),
    (0x8375d92822982cc8, "g7g8"),
    (0x146fcf4f6838e6f0, "g7g8"),
    (0x1f250bee008f49d1, "g7g8"),
    (0x691c8e0cdb4e2952, "g7g8"),
    (0x0335030537ada4d4, "g7g8"),
    (0x54fa1813599658d7, "g7g8"),
    (0xeb48d80d631581ed, "g7g8"),
    (0x38f25a4610eae9a3, "g7g8"),
    (0xcb9ca49ae4f7c200, "g7g8"),
    (0x87cdd538776f342c, "g7g8"),
    (0xb6b31044510cee09, "g7g8"),
    (0x973e9da4ab533317, "g7g8"),
    (0x904203794ac419a5, "d8e8"),
    (0x695c86cd17734d25, "d8e8"),
    (0xbd32946179873701, "g6f7"),
    (0xc7a06495a1da577c, "g6f7"),
    (0x16697ea6c90bf68b, "d8c7"),
    (0x394d305e2b01819d, "c8d6"),
    (0x937c93d96f7c7ee0, "c8d6"),
    (0x8b2a6b8072726c60, "c8d6"),
    (0x0d36e233f13bb1ff, "c8d6"),
    (0xd4cdfc508e1f8de6, "c8d6"),
    (0x805bf107d901dc45, "c8d6"),
    (0x9ca65ed1f83d171b, "c8d6"),
    (0xea17b25340b49d21, "c8d6"),
    (0x841afa20f2c93782, "c7d8"),
    (0x9cd120eaa88d1113, "c8d6"),
    (0x8e98be275f94e339, "c8d6"),
    (0xa1662c11844f796c, "c8d6"),
    (0xf83167009420622b, "c8d6"),
    (0xd6449e8cc4d04227, "c7d8"),
    (0x6764bc5019c84499, "c8d6"),
    (0x25e727a6e14986c9, "c8d6"),
    (0x5f75d7523914e6b4, "c8d6"),
    (0xd16e5b40ac48875f, "c8d6"),
    (0x520b85ad065b6616, "c8d6"),
    (0xcc36f995931a94c4, "c8d6"),
    (0x123620d8f08d2c17, "c8d6"),
    (0xb09988480c1515ab, "d7e5"),
    (0x04179c705a14e648, "f6g5"),
    (0x4b5c83c067f60b76, "e7c6"),
    (0xe16d2047238bf40b, "e7c6"),
    (0xf93bd81e3e85e68b, "e7c6"),
    (0x9594c10e0dca2664, "e7c6"),
    (0xff3a1cb84e5dd26c, "e7c6"),
    (0x2968c20c69db99ed, "e7c6"),
    (0xcf69e3db0d4219e4, "e7c6"),
    (0x8db44cc0b1820e67, "e7c6"),
    (0x27f0343ea2a94572, "e7c6"),
    (0xb51630bd444e219f, "e7c6"),
    (0xcf595967cca4de42, "e7c6"),
    (0x7debf6e6e7e242d2, "e7c6"),
    (0xc783ccbdb05bc40f, "e7c6"),
    (0xf29c46635fa9fb22, "e7c6"),
    (0xeec09374e47a9bf8, "e7c6"),
    (0xfc890db9136369d2, "e7c6"),
    (0xd3779f8fc8b8f387, "e7c6"),
    (0x81981cdf08036779, "e7c6"),
    (0xa1b4daf932e92e1a, "e7c6"),
    (0xa0c222cf69b08e87, "e7c6"),
    (0xe81e00d172577d50, "h5h6"),
    (0x4b162a74ea80cf32, "g7g8"),
    (0xe12789f3aefd304f, "g7g8"),
    (0xf97171aab3f322cf, "g7g8"),
    (0x95de68ba80bce220, "g7g8"),
    (0x81a48200e25f00ed, "g7g8"),
    (0x29226bb8e4ad5da9, "g7g8"),
    (0xcf234a6f8034dda0, "g7g8"),
    (0x0a1b8ebfdb4615c5, "g7g8"),
    (0x27ba9d8a2fdf8136, "g7g8"),
    (0xb55c9909c938e5db, "c4c3"),
    (0xa49b99cf62dd106c, "c7d6"),
    (0xfbe2dcb6423cd88e, "c7d6"),
    (0x16fcc2113baefd91, "c7d6"),
    (0x8f1b3dff6c7d0ae1, "c7d6"),
    (0x9b61d7450e9ee82c, "c7d6"),
    (0x665a6837da5760c5, "g5h4"),
    (0xf43811bed57db175, "g5h4"),
    (0x7f33754237224dd4, "g5h4"),
    (0xa87d27f667fb3bb2, "c7d7"),
    (0x67640a1786556e57, "c7d6"),
    (0xf9202f6c109b5184, "g7g8"),
    (0xda381e65f0587ab1, "g7g8"),
    (0x3237d97ec9f20b21, "c7d6"),
    (0x5358cc7206e8ba9f, "c7d6"),
    (0xe813ba923e1ed7a7, "c7d6"),
    (0xf44f6f8585cdb77d, "c7d6"),
    (0xed92050a47155d8c, "c7d6"),
    (0x61e39a8395669ab1, "c7d6"),
    (0xc9f8637ea90fdf02, "c7d6"),
    (0xad699bbd0930059e, "c7d6"),
    (0x90af286fb960c445, "g5f6"),
    (0x2aacd121e01e1ee4, "e7c6"),
    (0x75d59458c0ffd606, "e7c6"),
    (0x98cb8affb96df319, "e7c6"),
    (0xeb9fe5b25eb81919, "e7c6"),
    (0x4cb0ccdd00e833c5, "e7c6"),
    (0x264a6f18e538353a, "e7c6"),
    (0x2ef134f41eed8a6d, "e7c6"),
    (0xe75a955e4e28bc28, "e7c6"),
    (0xbc0091904b3105a9, "e7c6"),
    (0xdd6f849c842bb417, "e7c6"),
    (0x6624f27cbcddd92f, "e7c6"),
    (0x7a78276b070eb9f5, "e7c6"),
    (0x63a54de4c5d65304, "e7c6"),
    (0xefd4d26d17a59439, "e7c6"),
    (0x47cf2b902bccd18a, "e7c6"),
    (0x235ed3538bf30b16, "e7c6"),
    (0xa76c2e3d7801f5de, "e7c6"),
    (0x350c6ee6d19d0c17, "e7c6"),
    (0xcef60006c50de69a, "e7c6"),
    (0x3347e9e74e6a1116, "e7c6"),
    (0xd3aea2d3fa5ff501, "e7c6"),
    (0x0dae7b9e99c84dd2, "e7c6"),
    (0xf11ed7fdb36982f6, "e7c6"),
    (0x72a28d063ce440b3, "b3b4"),
    (0x944a8a701baaacca, "c5d6"),
    (0xab1a4f8135f115f2, "c5d5"),
    (0x6c88ebedfc6aaf5c, "c5b6"),
    (0x012bec06718cea8f, "c5d6"),
    (0x197d145f6c82f80f, "c5d6"),
    (0x9f619decefcb2590, "c5d6"),
    (0x384eb483b19b0f4c, "c5d6"),
    (0x9c935887e9e752c4, "f6h4"),
    (0x9db852e9f42f9ed3, "f6h4"),
    (0xdc1c16848827aab4, "g7d7"),
    (0x52b41746544b09b3, "c5c7"),
    (0x17d11ba9abfb003e, "g7g8"),
    (0x93a4ed00ff5b80a1, "g7g8"),
    (0x62fb830182b79b7b, "c5d6"),
    (0xa991fcc23558889e, "c5d6"),
    (0x12da8a220daee5a6, "c5d6"),
    (0x0e865f35b67d857c, "c5d6"),
    (0x175b35ba74a56f8d, "c5d6"),
    (0x9b2aaa33a6d6a8b0, "c5d6"),
    (0x333153ce9abfed03, "c5d6"),
    (0x57a0ab0d3a80379f, "c5d6"),
    (0x20de75158b2f751a, "c5d6"),
    (0x146a6b6d3d72520d, "f6h4"),
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
