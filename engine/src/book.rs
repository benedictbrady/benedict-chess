use crate::moves::Move;
use std::collections::HashMap;
use std::sync::OnceLock;

/// Minimal solution book for 1.e3 — only reachable positions.
/// Cleaned by reachable-extractor from 1686 entries.
const BOOK_DATA: &[(u64, &str)] = &[
    (0x3eb9976bbb8190ba, "b1c3"),   // mate in 15 half-moves
    (0x87834ea2f65f083c, "c3e4"),   // mate in 13 half-moves
    (0xc8b76cd20b758f82, "d2d3"),   // Qg4! Nf6 f3 reclaims, Qe6 Nd6# (mate in 4),   // mate in 11 half-moves
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
    (0xabcaa64542b909dc, "e4d6"),
    (0xbf8c73d27976df76, "e4d6"),
    (0xf8619cae225540a3, "e4d6"),
    (0x0d8985052597a824, "e4d6"),
    (0xa1f1dd2f73ec107e, "f2f3"),
    (0x6b9222c1804bf581, "e4d6"),
    (0x7fd4f756bb84232b, "e4d6"),
    (0x44a1d7259dda95fa, "e4d6"),
    (0x3839182ae0a7bcfe, "e4d6"),
    (0xcdd10181e7655479, "e4d6"),
    (0x5aa0f3453bc4119d, "e4d6"),
    (0xe9a09facd822cfd2, "e4d6"),
    (0xcea5f2acbd7ebeda, "f3f4"),
    (0xb5459c0ae40115d2, "d1h5"),
    (0xa103499ddfcec378, "d1h5"),
    (0x9a7669eef99075a9, "d1h5"),
    (0xe6eea6e184ed5cad, "d1h5"),
    (0x1306bf4a832fb42a, "d1h5"),
    (0x2c9dede483777f99, "d1h5"),
    (0x37772167bc682f81, "d1h5"),
    (0x32b95f9c8b7c7385, "d1h5"),
    (0xb5069294ce2083e1, "d1f3"),
    (0xf0a66dc8f80b05a2, "e4d6"),
    (0xe4e0b85fc3c4d308, "e4d6"),
    (0xdf95982ce59a65d9, "e4d6"),
    (0x5ccd7d83a695a08e, "e4d6"),
    (0xa30d572398e74cdd, "e4d6"),
    (0x56e54e889f25a45a, "e4d6"),
    (0x697e1c269f7d6fe9, "e4d6"),
    (0x7294d0a5a0623ff1, "e4d6"),
    (0x4a81f38b36bd32d8, "e4d6"),
    (0x51280bbe1cacead7, "e4d6"),
    (0x4b0cb142069dd7e8, "e4d6"),
    (0xc0a32ef3bb28ad76, "e4d6"),
    (0x060568715df4b18b, "e4d6"),
    (0x762fb204d9c05887, "d3d4"),
    (0x863568c366f4085f, "f1d3"),
    (0x73df2df3cd3a5639, "e4d6"),
    (0x3f2e355eec9aa785, "e4d6"),
    (0x73e46796524d142a, "e4d6"),
    (0x851e99b832a36f78, "e4d6"),
    (0xb9972c9355d7371e, "e4d6"),
    (0x11779878ea6d38fa, "e4d6"),
    (0x2962bb567cb235d3, "e4d6"),
    (0x32cb436356a3eddc, "e4d6"),
    (0x17ad7791778cae4e, "e4d6"),
    (0x990e68fc00c6855c, "e4d6"),
    (0x78e1f32de4e84830, "e4d6"),
    (0x4b98e65fc11fbea9, "e4d6"),
    (0x28eff99f4c92d0e3, "e4d6"),
    (0x104a1e467182f581, "e4d6"),
    (0x65e620ac17fbb680, "e4d6"),
    (0x0092cd1c0d72c4f9, "e4d6"),
    (0x470a15087648a532, "e4d6"),
    (0x373df9a140abd308, "e4d6"),
    (0xb7daaca8a6402c03, "e4d6"),
    (0xb44e4cf8afaa5397, "e4d6"),
    (0x7be9ba6d1e6a6915, "e4d6"),
    (0xf0bdc0d3b3929d85, "e4d6"),
    (0x03dd4125cea58f6f, "e4d6"),
    (0x9a445b0c50062584, "e4d6"),
    (0xebce78dc2dc7912c, "e4c5"),
    (0xa5a8ce38bdd3d828, "b7c6"),
    (0xb772c2b4bf139e82, "b7c8"),
    (0x8b31ed1b6917e32c, "c8d7"),
    (0x9bf688add0bf1efd, "c8d7"),
    (0x1d4f916069f28e66, "c8d7"),
    (0xd1f83eeb94381ed6, "c8d7"),
    (0x2550a6cf9c6e02fc, "c8d7"),
    (0x3ef95efab67fdaf3, "c8d7"),
    (0x494f6de255287109, "c8d7"),
    (0x1b9f6a0897509961, "c8d7"),
    (0x3dd6d50f3ce33c24, "c8d7"),
    (0x9e0b93558769f0f3, "f5f6"),
    (0x0d0a685489021991, "f6f7"),
    (0x1dcd0de230aae440, "f6f7"),
    (0xa36b23807c7bf841, "f6f7"),
    (0xb8c2dbb5566a204e, "f6f7"),
    (0x2368383f531df95c, "f6f7"),
    (0x9b41a15d26ac14b0, "c1d2"),
    (0x09261497738d2e9a, "g1h3"),
    (0xe71b06c8f27d619a, "e1g1"),
    (0x20742621dde1b413, "a1b1"),
    (0xf35ac34f5d069cf9, "h3f2"),
    (0xc680a89f4cab0d61, "f2g4"),
    (0x9361210910e73088, "f6f7"),
    (0x723fb566810692e8, "f6f7"),
    (0xfce2b4c093f56f8a, "f6f7"),
    (0x7fdac35b3b3fd8b1, "f6f7"),
    (0xe11068cb70b335fd, "f6f7"),
    (0x6f74e095e4fb639b, "f6f7"),
    (0xc2b377acc236475d, "f6f7"),
    (0x65fe79a7210ee58e, "f6f7"),
    (0x9042359c3365749b, "f6f7"),
    (0xfacb2c90cbdc196e, "f6f7"),
    (0x8cea191f0f3e96c1, "c8b7"),
    (0x11157548f3f3b228, "f3h3"),
    (0xf229b4c6946f4976, "h3h4"),
    (0x18880922a9e8b508, "f6f7"),
    (0xe88871b0b667c80f, "f6f7"),
    (0x4ab146d2f5f4c71d, "f6f7"),
    (0x028e0ea719b0a94d, "f6f7"),
    (0x7d0a2af8051410ba, "f6f7"),
    (0x4fe948456d2d23b5, "f6f7"),
    (0xe6df72035a5c37b9, "f6f7"),
    (0xce5f57bea449176e, "f6f7"),
    (0x3be31b85b622867b, "f6f7"),
    (0xa444f8da3da2ed59, "f6f7"),
    (0xed1751cdea7a8040, "f6f7"),
    (0x4c458bbf16ce337b, "f6f7"),
    (0xc0da3d96ca526ed8, "f6f7"),
    (0x5035c5f955407167, "f6f7"),
    (0xa8d2d95b2ec66e8c, "f6f7"),
    (0x5db5b62748f7eff8, "f6f7"),
    (0x2ced3153e8af7f91, "f6f7"),
    (0x23a0940af8c6c2ab, "h7h8r"),
    (0xe0b1419e32a6a23d, "f8g8"),
    (0x0b55c369d861697b, "f8g8"),
    (0x10b1390c2d29df3a, "f8g8"),
    (0xb2880e6e6ebad028, "f8g8"),
    (0xfab7461b82febe78, "f8g8"),
    (0x853362449e5a078f, "b7c6"),
    (0xb7d000f9f6633480, "b7c6"),
    (0x36661f023f07005b, "a8b8"),
    (0xc3da53392d6c914e, "a8b8"),
    (0x5c7db066a6ecfa6c, "f8g8"),
    (0x152e197171349775, "f8g8"),
    (0xb47cc3038d80244e, "f8g8"),
    (0x38e3752a511c79ed, "f8g8"),
    (0xa80c8d45ce0e6652, "f8g8"),
    (0x50eb91e7b58879b9, "f8g8"),
    (0x4afa6ed65086f6e9, "h4g5"),
    (0x20f82b110bcc6051, "f6f7"),
    (0x72c164e157d01244, "f6f7"),
    (0x3afe2c94bb947c14, "f6f7"),
    (0xeba665be97af4af8, "f6f7"),
    (0x77996a76cf09f6ec, "f6f7"),
    (0xf62f758d066dc237, "f6f7"),
    (0x039339b614065322, "f6f7"),
    (0xad2230262da09007, "f6f7"),
    (0xd56773fe485e5519, "f6f7"),
    (0x7435a98cb4eae622, "f6f7"),
    (0xf8aa1fa56876bb81, "f6f7"),
    (0x6845e7caf764a43e, "f6f7"),
    (0x90a2fb688ce2bbd5, "f6f7"),
    (0x481cba9ea98acf47, "h8f8"),
    (0x1272140e8cd21cb4, "f6f7"),
    (0xb3cceeebe97beff9, "f6f7"),
    (0x10f55e8de48a3cb3, "f6f7"),
    (0xce396de6d740454e, "f6f7"),
    (0x4bad5e35e23cfaf7, "g6g7"),
    (0xb0974dc74bc60be2, "f8g8"),
    (0x3b4f27e863d3e034, "f8g8"),
    (0x0527d3b6f3d12181, "f8g8"),
    (0x5fd872bce098419d, "f8g8"),
    (0xe2ae023717da79f7, "f8g8"),
    (0x4cca25110affa7bc, "f8g8"),
    (0xfd5e65ca028a74fb, "f8g8"),
    (0x2381a8737b0f7778, "f8g8"),
    (0xa9154ce882d00764, "f8g8"),
    (0xf4c556c4a94ef682, "f8g8"),
    (0xbc738b0ca7fe9105, "b7c6"),
    (0x6640135b4667a984, "a8b8"),
    (0xedef8ceafbd2d31a, "a8b8"),
    (0x3d4d56f06daafbb4, "f8g8"),
    (0x1b5c09ae5a84df8f, "f8g8"),
    (0xe45acf5af4e08d91, "f8g8"),
    (0x3340fedf0081dc68, "f8g8"),
    (0xf82a811cb76ecf8d, "f8g8"),
    (0x00cd9dbecce8d066, "f8g8"),
    (0xf99c17f55d86be69, "f8g8"),
    (0x8194004f656d7865, "f8g8"),
    (0x5e821f8118018ce7, "f8g8"),
    (0xa4cbc74b151ac67b, "f8g8"),
    (0xd6b605e1478c5704, "f8g8"),
    (0x6915b89762f2f556, "f8g8"),
    (0xed9884a5d0acd75a, "f8g8"),
    (0xce2043d7215547d9, "f8g8"),
    (0xaf0f64f9338358a5, "h8h7"),
    (0xb24764265c096638, "g7g8b"),
    (0x399f0e09741c8dee, "g7g8b"),
    (0x07f7fa57e41e4c5b, "g7g8b"),
    (0x5d085b5df7572c47, "g7g8b"),
    (0xe07e2bd60015142d, "g7g8b"),
    (0x4e1a0cf01d30ca66, "g7g8b"),
    (0xff8e4c2b15451921, "g7g8b"),
    (0x215181926cc01aa2, "g7g8b"),
    (0xabc56509951f6abe, "g7g8b"),
    (0xf6157f25be819b58, "g7g8b"),
    (0xbea3a2edb031fcdf, "g7g8b"),
    (0x64903aba51a8c45e, "g7g8b"),
    (0xef3fa50bec1dbec0, "g7g8b"),
    (0x3f9d7f117a65966e, "g7g8b"),
    (0x198c204f4d4bb255, "g7g8b"),
    (0xe68ae6bbe32fe04b, "g7g8b"),
    (0x3190d73e174eb1b2, "g7g8b"),
    (0xfafaa8fda0a1a257, "g7g8b"),
    (0x021db45fdb27bdbc, "g7g8b"),
    (0xf10da662d5f1c073, "g7g8b"),
    (0x1271433e5e7b8de5, "g7g8b"),
    (0xd8d2aa823edd7168, "g7g8b"),
    (0xd7d772bb4a2fa839, "h2h4"),
    (0x6b8c3a9cc44d53bd, "g5h5"),
    (0xe05450b3ec58b86b, "g5h5"),
    (0xde3ca4ed7c5a79de, "g5h5"),
    (0x84c305e76f1319c2, "g5h5"),
    (0x06b2b6141d5bb5a2, "h7g7"),
    (0x39b5756c985121a8, "g5h5"),
    (0x97d1524a8574ffe3, "g5h5"),
    (0x264512918d012ca4, "g5h5"),
    (0xf89adf28f4842f27, "g5h5"),
    (0x720e3bb30d5b5f3b, "g5h5"),
    (0x2fde219f26c5aedd, "g5h5"),
    (0x6768fc572875c95a, "g5h5"),
    (0x68b19afb01fe51a6, "g5h5"),
    (0xbd5b6400c9ecf1db, "a8f8"),
    (0x36f4fbb174598b45, "a8f8"),
    (0xc36108f5239617b1, "g5h5"),
    (0xc0477ef5d50f87d0, "g5h5"),
    (0x3f41b8017b6bd5ce, "g5h5"),
    (0xe85b89848f0a8437, "g5h5"),
    (0x2331f64738e597d2, "g5h5"),
    (0xdbd6eae543638839, "g5h5"),
    (0xdc51d352702a88a6, "g5h5"),
    (0x3f2d360efba0c530, "g5h5"),
    (0xf58edfb29b0639bd, "g5h5"),
    (0xd9b24e4311270b89, "g5h5"),
    (0x7726a541d7e1c380, "g5h5"),
    (0xd449465ca4c8d57c, "b7c6"),
    (0x499e2d6f31dd5ffc, "d7e5"),
    (0xc246474019c8b42a, "d7e5"),
    (0x15d17efd7965cbcc, "d7b6"),
    (0x4c30dc102749593d, "c6e8"),
    (0xc7e8b63f0f5cb2eb, "c6e8"),
    (0x210e5098fe5fbf22, "c6e8"),
    (0x1e0993e07b552b28, "c6e8"),
    (0x28fc3b68e4ab9a57, "c6e8"),
    (0x01f9f41d6e052624, "c6e8"),
    (0x676e871f0a219325, "c6e8"),
    (0xedfa6384f3fee339, "c6e8"),
    (0x0f2473399d750a26, "c6e8"),
    (0xe89e91e83024af90, "c6e8"),
    (0x9ae7828c2ae8fb5b, "c6e8"),
    (0xe90357d066569019, "a8f8"),
    (0xe4ddee79c0921d31, "c6e8"),
    (0xe7fb9879360b8d50, "c6e8"),
    (0x806cd1231ab4b07a, "c6e8"),
    (0x9462e8a444f382ed, "c6e8"),
    (0x2c38a27917426551, "a8a7"),
    (0x4422b2d2bdc6343b, "c6e8"),
    (0xee4e8f1adcc13b6a, "c6e8"),
    (0xd358876c5f8d7a25, "a8a7"),
    (0x4314572e3059c3ea, "c6e8"),
    (0xd232393e7802333d, "a8a7"),
    (0xfe0ea8cff2230109, "c6e8"),
    (0x5a9f608641796bf3, "c6e8"),
    (0xdeb7be2f6e65ba8e, "a8a7"),
    (0x7a021a474f5758e3, "c6e8"),
    (0x0628243acee3b903, "b6c4"),
    (0x27930726203d6080, "e5e6"),
    (0xac4b6d0908288b56, "e5e6"),
    (0x0ab216f91af125d7, "e5e6"),
    (0x4aad8baef92b869f, "e5e6"),
    (0x75aa48d67c211295, "e5e6"),
    (0x435fe05ee3dfa3ea, "e5e6"),
    (0x6a5a2f2b69711f99, "e5e6"),
    (0x0ccd5c290d55aa98, "e5e6"),
    (0xbf8a9edde7f0dbf2, "e5e6"),
    (0x6487a80f9a01339b, "e5e6"),
    (0x833d4ade3750962d, "e5e6"),
    (0xf14459ba2d9cc2e6, "e5e6"),
    (0x8c58434f317fb4ed, "e5e6"),
    (0xd21c2c7a0eba88b1, "e5e6"),
    (0x5fde0411c6af8c30, "e5e6"),
    (0xab4f9fa16e223c68, "e5e6"),
    (0x85ed542cdbb502d7, "e5e6"),
    (0x904eeee8945abb9b, "e5e6"),
    (0x73320bb41fd0f60d, "e5e6"),
    (0x965adc77d096aa42, "c4d6"),
    (0x95ad73f9f55738b4, "e5e6"),
    (0x313cbbb0460d524e, "e5e6"),
    (0x494b43a7243251c2, "c4d6"),
    (0xc21d23321211aa37, "c4d6"),
    (0x161be4da0b39d5ba, "e5e6"),
    (0x3170f46d8229d425, "c4d6"),
    (0xd1e96c7938197d1d, "a8a7"),
    (0x966efc19c7aee6ab, "c4d6"),
    (0xabe555edb8b5dd75, "e5e6"),
    (0x0259e947e48f6eeb, "e5e6"),
    (0x4cfe15c655cf7581, "c4d6"),
    (0x10a46514a6d67360, "c4d6"),
    (0x3bbab829837296e5, "e5e6"),
    (0x059772e6cf019545, "e5e6"),
    (0x3a29db20d5f03820, "c4d6"),
    (0x6f8d245d85730537, "f1f3"),
    (0xecbcc92346c8a7e3, "f4f5"),
    (0xc19dd8fc7c04e2b4, "c4d6"),
    (0xa81403117fd51234, "e4c5"),
    (0x09973493384ae528, "e4c5"),
    (0xfa2d4ce123c96021, "e4c5"),
    (0xb554b0be9739a00a, "d2e1"),
    (0xbdd9bf9a930da9d7, "d6b5"),
    (0xf01a7b386b7cfce3, "b5c7"),
    (0xb6ba9105e4f557c2, "b5c7"),
    (0xa5ef2233f6cf43c4, "b5c7"),
    (0x480a5854f0b184da, "b5c7"),
    (0xfd045e4e97226247, "c3c4"),
    (0x21ec3e66703c20a3, "b5c7"),
    (0x5ef7d64572bd8799, "b5c7"),
    (0xb7ea6b4e85440921, "b5c7"),
    (0x6ff984be6f488b26, "b5c7"),
    (0x3f1e6c0a39c84e4a, "b5c7"),
    (0xf3c3060431bfc9bb, "b5c7"),
    (0x2a9cdd8a56d88048, "b5c7"),
    (0x1b6ef0bac9b997de, "b5c7"),
    (0xdef3c44a1c9306e3, "b5c7"),
    (0x6672ea4fd55ae1d5, "b5c7"),
    (0x9bc6ba1b9807ddb8, "b5c7"),
    (0xaac2dd634588da2f, "b5c7"),
    (0x1ae0b10da2fa650a, "b5c7"),
    (0x7c65fd14a6e36f22, "b5c7"),
    (0xa248c983e4966f11, "b5c7"),
    (0x9cc6c98cb8c7e9e2, "b5c7"),
    (0x6fc7fd2c3f9057ef, "b5c7"),
    (0x64e2127965467abd, "b5c7"),
    (0x7a6447e8707feea3, "b5c7"),
    (0x9918a2b4fbf5a335, "b5c7"),
    (0x53bb4b089b535fb8, "b5c7"),
    (0x7f87daf911726d8c, "b5c7"),
    (0xdb1612b0a2280776, "b5c7"),
    (0x1185579a742b1b30, "b5c7"),
    (0x3f8e3c308cb384f2, "b5c7"),
    (0x5fd21156dd207cc9, "b5c7"),
    (0x8b712cca791dfbed, "b5c7"),
    (0x88496cff8abfb727, "b5c7"),
    (0xb2b145724c88f1c0, "b5c7"),
    (0xf2c58485dc35d7b1, "b5c7"),
    (0xd7e9a2c69ee6ad52, "f2h3"),
    (0x8fbeccf6aa52b933, "g5f6"),
    (0xf0a524d5a8d31e09, "g5f6"),
    (0x3814e8f22bee977b, "g5f6"),
    (0x6703d39fdfb94178, "g5f6"),
    (0x03ace5eea2d71bcb, "g5f6"),
    (0x84cb7dba8fab83d9, "g5f6"),
    (0x3eeae69bc9988795, "d5d6"),
    (0x3355f4edc6554284, "h3f2"),
    (0xdccb046971395e7e, "e4c5"),
    (0xe1c1a9f89947a940, "e4c5"),
    (0x314c6abb6dfb13a8, "e4c5"),
    (0x4c9f4ff4bd210102, "e4c5"),
    (0x5d3b15ef8df27a50, "e4c5"),
    (0x07fdb5a80388412c, "c1e2"),
    (0x9491121020b25bfc, "d4d5"),
    (0xfe7390b38d969703, "d4d5"),
    (0x0a473a2902564958, "d4d5"),
    (0x532d76bfa9f03f41, "d4d5"),
    (0xa15ed61769a752af, "d4d5"),
    (0xd4ef8d70a9831b88, "d4d5"),
    (0xa0bf61bea9c356bd, "d4d5"),
    (0x48eb3279db21c40f, "d4d5"),
    (0xcfa1c67ca11add26, "d4d5"),
    (0x82a81f4ffa73bb45, "d4d5"),
    (0xe6e06598ed2f8f59, "d4d5"),
    (0xa00c18905929535e, "d4d5"),
    (0x77ce81daac9b0da7, "e2g3"),
    (0xb72d9957a78f4a24, "e4c5"),
    (0xe0c5b665e2d571e5, "e4c5"),
    (0x8a2734c64ff1bd1a, "e4c5"),
    (0x376c2b2c477d331d, "e4c5"),
    (0x2779d2ca6b971558, "e4c5"),
    (0xbbf56209637df73f, "e4c5"),
    (0x835085d05e6dd25d, "e4c5"),
    (0xf6fcbb3a3814915c, "e4c5"),
    (0x92b4c1ed2f48a540, "e4c5"),
    (0x14b18c59bb6096c7, "d4d5"),
    (0x8ea10e110e5b40c2, "e4c5"),
    (0x3a06fc44e1647b63, "e4c5"),
    (0x68a59c535e788fca, "e4c5"),
    (0x8f0df67ee512c8a6, "e4c5"),
    (0xba99b9f1965528c3, "e4c5"),
    (0xdcfe6fa70c270fc3, "e4c5"),
    (0xce6ee03ad0db1ef3, "e4c5"),
    (0x3c5d5b3f12038903, "e4c5"),
    (0xcf5c6f9f9554370e, "e4c5"),
    (0xb89431589f12d067, "e4c5"),
    (0x70765d8909481a46, "e4c5"),
    (0x70b855fda854fa34, "e4c5"),
    (0xc47980cacf821a5c, "e4c5"),
    (0xdaffd55bdabb8e42, "e4c5"),
    (0xca2db9858d1a314c, "e4c5"),
    (0xf320d9bb31973f59, "e4c5"),
    (0x13a1f340253f1c6e, "e4c5"),
    (0x7b8d800308ec6797, "e4c5"),
    (0x601f57909535fe3e, "e4c5"),
    (0x33f438efe96d0d2b, "e4c5"),
    (0xe75705734d508a0f, "e4c5"),
    (0xdb7c77cefc50255e, "e4c5"),
    (0xe1845e433a6763b9, "e4c5"),
    (0xd280558de29bc3a3, "e4c5"),
    (0x0d798b00c1f8eef6, "e4c5"),
    (0xfd7262853cfd860a, "e4c5"),
    (0xaff371a98f4010e1, "e4c5"),
    (0xc4f9ebe0d1011258, "e4c5"),
    (0x80a6b75add800ddb, "e4c5"),
    (0x3cbf9dddda4f7e7b, "e4c5"),
    (0x3eb0ee17e8a8eedc, "e4c5"),
    (0x60e5ed0f2956789a, "e4c5"),
    (0x4e1e12b64330a1eb, "e4c5"),
    (0x2dfb11a0f1783b31, "e4c5"),
    (0x979e87cf7f39cf45, "e4c5"),
    (0x125c748a793dd631, "f2g4"),
    (0x8bd808fd727f6fd5, "e4c5"),
    (0x6445fc4d5a337f00, "f5f6"),
    (0x0af721916472902c, "d6d7"),
    (0x19959cdb30caaad9, "d7d8n"),
    (0x4e7db3e975909118, "d7d8n"),
    (0x249f314ad8b45de7, "d7d8n"),
    (0x99d42ea0d038d3e0, "d7d8n"),
    (0x89c1d746fcd2f5a5, "d7d8n"),
    (0x154d6785f43817c2, "d7d8n"),
    (0x2de8805cc92832a0, "d7d8n"),
    (0x3c0cc461b80d45bd, "d7d8n"),
    (0xc9abc64b581c8517, "c3b5"),
    (0x943d16437d831899, "b5d6"),
    (0xc3d5397138d92358, "b5d6"),
    (0x96c0b8bb2ceb285e, "b5d6"),
    (0x7720567b24628f4f, "b5d6"),
    (0x07536d15812923d9, "h6g7"),
    (0xd193bd53027c7c4b, "e5e6"),
    (0x867b92614726478a, "e5e6"),
    (0xd36e13ab53144c8c, "e5e6"),
    (0x3d467caa8691eac4, "e5e6"),
    (0xec9910c2ea028b75, "e5e6"),
    (0x6e48eda97cc293f3, "e5e6"),
    (0x41c7f6cece642337, "e5e6"),
    (0xfacbe316897f1e85, "e5e6"),
    (0xc07b608dce29d3cb, "e5e6"),
    (0xb255e1cfce574acb, "e5e6"),
    (0x5a01b208bcb5d879, "e5e6"),
    (0xdd4b460dc68ec150, "e5e6"),
    (0xbf89a141320707f1, "g7h8"),
    (0x3bd87edc6417449a, "e5e6"),
    (0x6c3051ee214d7f5b, "e5e6"),
    (0x3925d024357f745d, "e5e6"),
    (0xd70dbf25e0fad215, "e5e6"),
    (0x06d2d34d8c69b3a4, "e5e6"),
    (0x84032e261aa9ab22, "e5e6"),
    (0xab8c3541a80f1be6, "e5e6"),
    (0x10802099ef142654, "e5e6"),
    (0x2a30a302a842eb1a, "e5e6"),
    (0x581e2240a83c721a, "e5e6"),
    (0xb04a7187dadee0a8, "e5e6"),
    (0x8f5adcf77a7df3e0, "e5e6"),
    (0xd95eb8ec58a717f2, "e5e6"),
    (0x670e8f30089f6152, "e5e6"),
    (0x7a095cb1fb8c9fe2, "e5e6"),
    (0xe8ef58321d6bfb0f, "e5e6"),
    (0x6c0dc0298b28abb1, "e5e6"),
    (0x63fa988b2ed3a399, "e5e6"),
    (0x20275a662ce6aabe, "e5e6"),
    (0x141d51aa77ef04c3, "e5e6"),
    (0xc61077542a28795c, "e5e6"),
    (0x0254e99acdc34e7c, "e5e6"),
    (0xb6f31bcf22fc75dd, "e5e6"),
    (0xe4507bd89de08174, "e5e6"),
    (0x1afb1b0d8f542915, "e5e6"),
    (0x33e6c2c08732b08c, "e5e6"),
    (0xb0a8bcb4d19b87bd, "e5e6"),
    (0x0bfb3452c2c64858, "e5e6"),
    (0x0f2d338016fbe660, "e5e6"),
    (0xfc4db2766bccf48a, "e5e6"),
    (0xab82a96005f70889, "e5e6"),
    (0xcc9acdfdbca130ef, "e5e6"),
    (0x2fe628a1372b7d79, "e5e6"),
    (0xca8eff62f86d2136, "h8g7"),
    (0x4edf20ffae7d625d, "e5e6"),
    (0x19370fcdeb27599c, "e5e6"),
    (0x4c228e07ff15529a, "e5e6"),
    (0xa20ae1062a90f4d2, "e5e6"),
    (0x73d58d6e46039563, "e5e6"),
    (0xf1047005d0c38de5, "e5e6"),
    (0xde8b6b6262653d21, "e5e6"),
    (0x65877eba257e0093, "e5e6"),
    (0x5f37fd216228cddd, "e5e6"),
    (0x2d197c63625654dd, "e5e6"),
    (0xc54d2fa410b4c66f, "e5e6"),
    (0xfa5d82d4b017d527, "e5e6"),
    (0xac59e6cf92cd3135, "e5e6"),
    (0x1209d113c2f54795, "e5e6"),
    (0x0f0e029231e6b925, "e5e6"),
    (0x9de80611d701ddc8, "e5e6"),
    (0x190a9e0a41428d76, "e5e6"),
    (0x16fdc6a8e4b9855e, "e5e6"),
    (0x55200445e68c8c79, "e5e6"),
    (0x611a0f89bd852204, "e5e6"),
    (0xb3172977e0425f9b, "e5e6"),
    (0x7753b7b907a968bb, "e5e6"),
    (0xc3f445ece896531a, "e5e6"),
    (0x915725fb578aa7b3, "e5e6"),
    (0x22b237bd263cacfc, "e5e6"),
    (0x0bafee702e5a3565, "e5e6"),
    (0x5f6ccb001d2868eb, "e5e6"),
    (0x9f5501d806486c04, "b5c7"),
    (0x7b9bf94b99d4216e, "e6f6"),
    (0x2c73d679dc8e1aaf, "e6f6"),
    (0x796657b3c8bc11a9, "e6f6"),
    (0x974e38b21d39b7e1, "e6f6"),
    (0x469154da71aad650, "e6f6"),
    (0xfbda4b3079265857, "e6f6"),
    (0xebcfb2d655cc7e12, "e6f6"),
    (0x50c3a70e12d743a0, "e6f6"),
    (0x55e9c614cbcd186f, "e6f6"),
    (0x537959a86e4aa4fe, "g7h6"),
    (0x56b9b22841a6ca17, "g5f6"),
    (0x01519d1a04fcf1d6, "g5f6"),
    (0x54441cd010cefad0, "g5f6"),
    (0x60e387aa3cd1c071, "g5f6"),
    (0xba6c73d1c54b5c98, "g5f6"),
    (0x6bb31fb9a9d83d29, "g5f6"),
    (0xd6f80053a154b32e, "g5f6"),
    (0xe5e14bbc2298e70a, "g5f6"),
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
