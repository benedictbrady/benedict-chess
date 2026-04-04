use crate::moves::Move;
use std::collections::HashMap;
use std::sync::OnceLock;

/// Minimal solution book for 1.e3 — only reachable positions.
/// Cleaned by reachable-extractor from 1710 entries.
const BOOK_DATA: &[(u64, &str)] = &[
    (0x3eb9976bbb8190ba, "b1c3"),   // mate in 15 half-moves
    (0x87834ea2f65f083c, "c3e4"),   // mate in 13 half-moves
    (0xc8b76cd20b758f82, "d1f3"),   // Qf3! safe from Nf6, reclaims Ne4,   // Qg4! Nf6 f3 reclaims, Qe6 Nd6# (mate in 4),   // mate in 11 half-moves
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
    (0x0f86f066042cdd5d, "e4d6"),
    (0x1bc025f13fe30bf7, "e4d6"),
    (0x5c2dca8d64c09422, "e4d6"),
    (0xa9c5d32663027ca5, "e4d6"),
    (0x05bd8b0c3579c4ff, "d2d3"),
    (0x66c0419b7cb542a1, "e4d6"),
    (0x7286940c477a940b, "e4d6"),
    (0x49f3b47f612422da, "e4d6"),
    (0x356b7b701c590bde, "e4d6"),
    (0xc08362db1b9be359, "e4d6"),
    (0x57f2901fc73aa6bd, "e4d6"),
    (0xe4f2fcf624dc78f2, "e4d6"),
    (0x49d62267cf31aaa1, "e4d6"),
    (0x66834f055694d492, "e4d6"),
    (0xdce7dfd8b20375db, "e4d6"),
    (0xc74e27ed9812add4, "e4d6"),
    (0xdd6a9d11822390eb, "e4d6"),
    (0x56c502a03f96ea75, "e4d6"),
    (0x90634422d94af688, "e4d6"),
    (0xe0499e575d7e1f84, "f3g4"),
    (0x83be8af803eab6e6, "e4d6"),
    (0x97f85f6f3825604c, "e4d6"),
    (0xac8d7f1c1e7bd69d, "e4d6"),
    (0xd015b0136306ff99, "e4d6"),
    (0x25fda9b864c4171e, "e4d6"),
    (0xb28c5b7cb86552fa, "e4d6"),
    (0x018c37955b838cb5, "e4d6"),
    (0xaeeec893745ed7f7, "e4d6"),
    (0x2230ec8ee74d5993, "e4d6"),
    (0xf6b10539ae9d2971, "e4d6"),
    (0x681a5cc05506fc7f, "e4d6"),
    (0x5b6349b270f10ae6, "e4d6"),
    (0x38145672fd7c64ac, "e4d6"),
    (0xb3bbc9c340c91e32, "e4d6"),
    (0x751d8f41a61502cf, "e4d6"),
    (0xa071fd1c5b15bc9f, "e4d6"),
    (0xe7e92508202fdd54, "e4d6"),
    (0xb0b18a644847312c, "e4d6"),
    (0xe0466f3e027c29ca, "e4d6"),
    (0x1326eec87f4b3b20, "e4d6"),
    (0x5d1196a322a407fe, "e4d6"),
    (0x793302a2051ac697, "g4d7"),
    (0x5bb0a106a6e36564, "e4d6"),
    (0x0130415ced59b665, "e4d6"),
    (0x3a9237d57fd95887, "e4c5"),
    (0x3310e73775d32127, "e4d6"),
    (0x450f367f244a48d8, "e4d6"),
    (0x0297ee6b5f702913, "e4d6"),
    (0xc2b89d2f8e1a9300, "e4d6"),
    (0x0538a45d7d23dd8d, "e4d6"),
    (0xf65825ab0014cf67, "e4d6"),
    (0x9c4dc9c17a4532d0, "e4f6"),
    (0x29b9b64d60bec748, "e4d6"),
    (0x733956172b041449, "e4d6"),
    (0xdfecfcb60086acc0, "e4c5"),
    (0x2b9aace4c2e2894c, "e4f6"),
    (0x3eb421e2bfa33941, "e4d6"),
    (0x8db44d0b5c45e70e, "e4d6"),
    (0x2090939ab7a8355d, "e4d6"),
    (0x0fc5fef82e0d4b6e, "e4d6"),
    (0xb5a16e25ca9aea27, "e4d6"),
    (0xae089610e08b3228, "e4d6"),
    (0xb42c2cecfaba0f17, "e4d6"),
    (0x3f83b35d470f7589, "e4d6"),
    (0x890f2faa25e78078, "d2d3"),
    (0xea72e53d6c2b0626, "e4d6"),
    (0xfe3430aa57e4d08c, "e4d6"),
    (0xb9d9dfd60cc74f59, "e4d6"),
    (0x4c31c67d0b05a7de, "e4d6"),
    (0xce6eb9bfaae55237, "f3g3"),
    (0x3ede381aafa4a19b, "d6d7"),
    (0x2a98ed8d946b7731, "d6d7"),
    (0x6d7502f1cf48e8e4, "d6d7"),
    (0x989d1b5ac88a0063, "d6d7"),
    (0x0fece99e142b4587, "d6d7"),
    (0xbcec8577f7cd9bc8, "d6d7"),
    (0x84f9a659611296e1, "d6d7"),
    (0x9f505e6c4b034eee, "d6d7"),
    (0xe682d1e0ccc82021, "d6d7"),
    (0xecb177b80097fe36, "d6d7"),
    (0x17897aab038c384b, "d6d7"),
    (0xd57aee22f948eb02, "e4f6"),
    (0xe603fb50dcbf1d9b, "d6d7"),
    (0x8574e490513273d1, "d6d7"),
    (0x0edb7b21ec87094f, "d6d7"),
    (0xc87d3da30a5b15b2, "a2a4"),
    (0xb955173729d19800, "b2b4"),
    (0x4f628d79dd67bd00, "g3h3"),
    (0x92baa27485ce21e3, "e4f6"),
    (0x0bc077cb5b73e253, "e4f6"),
    (0x3fe9c44f4bd53fce, "e4f6"),
    (0x923b88c495b91c59, "e4f6"),
    (0x931a680fc15cf98d, "h3e6"),
    (0xd931a38759d46609, "h3e6"),
    (0x9f78aa65cc263522, "h3e6"),
    (0xb11b5a4aa815a23d, "h3e6"),
    (0xad21e641a5877e34, "f7f8r"),
    (0x8a3921e5c729405f, "h3e6"),
    (0x8af729916635a02d, "h3e6"),
    (0x8db134a8f438d19c, "h3e6"),
    (0x9abe760e413c1c0b, "h3e6"),
    (0x944957d5c7ec1141, "h3e6"),
    (0x44fe6df5d622c283, "h3e6"),
    (0x8b25ab745819ec09, "h3e6"),
    (0x7570985a62164637, "g7g8n"),
    (0x77a319533314d191, "e7e6"),
    (0x963f12445eaf64d7, "h3d7"),
    (0x84f3e6417ab27d43, "h3d7"),
    (0x293da391c1c14422, "e7e6"),
    (0xe1870bf1603b0f3a, "e7e6"),
    (0xf84bcf6349d18fd4, "e7e6"),
    (0xd6283f4c2de218cb, "e7e6"),
    (0x4b198d8c6ac1f3c1, "e7e6"),
    (0xe47c23f336ad4bcd, "e7e6"),
    (0x4121001122eddbf4, "e7e6"),
    (0xb24181e75fdac91e, "e7e6"),
    (0xb5079cdecdd7b8af, "e7e6"),
    (0xa208de7878d37538, "e7e6"),
    (0x31631d1fff0bcd5f, "e7e6"),
    (0x0a323202eb90db30, "e4g5"),
    (0x9aba5dde55960e92, "g5e6"),
    (0xebe472e035142a1f, "g5e6"),
    (0x542c65710a7f7e7c, "g5e6"),
    (0x46e091742e6267e8, "g5e6"),
    (0x4f7d7461a7913bb7, "g5e6"),
    (0x2ca32d9960a2c9a5, "g5e6"),
    (0x4f5dbca5bd347977, "g5e6"),
    (0x23947cc434eb1591, "g5e6"),
    (0x1cbab3bf0c56661d, "g5e6"),
    (0x143b487979320260, "g5e6"),
    (0xf42c8a507eb82114, "f7f8r"),
    (0x266f54c6627d5166, "f7f8r"),
    (0x634df86a3f2393d8, "g5e6"),
    (0x902d799c42148132, "g5e6"),
    (0x976b64a5d019f083, "g5e6"),
    (0x80642603651d3d14, "g5e6"),
    (0x2a1496e5f959ddcc, "g8e7"),
    (0xdf0b1e3560d1a569, "e7c6"),
    (0xae55310b005381e4, "e7c6"),
    (0x2dab55a6ad5939dd, "e7c6"),
    (0x2dae3edcddb6f980, "e7c6"),
    (0x49bfe47c24638912, "c5c6"),
    (0xf663f63e1756e06a, "c6c7"),
    (0xf74395ad99b0015f, "c6c7"),
    (0xfc262e28d2c1069f, "c6c7"),
    (0x2dbc2bd0632f71b2, "c6c7"),
    (0x8e2e52108c8e43a9, "c6c7"),
    (0x15ef508b52a7dea9, "c6c7"),
    (0x21ccd1fbe64791a6, "c6c7"),
    (0xe1c205ceb5de2ad6, "c6c7"),
    (0x1966da47214befab, "c6c7"),
    (0xd29e977779c978bb, "d7b6"),
    (0x434813e851a3e6b6, "a8b8"),
    (0x4268707bdf450783, "a8b8"),
    (0x490dcbfe94340043, "a8b8"),
    (0x3853e4c0f4b624ce, "a8b8"),
    (0x9897ce0625da776e, "g5f7"),
    (0x3b05b7c6ca7b4575, "a8b8"),
    (0xe940e123cf62bf6f, "c6c7"),
    (0x1653ea4607325b4f, "c6c7"),
    (0xcaf101fac86db322, "c6c7"),
    (0x28ac4af18c462f05, "a8b8"),
    (0x40ed71f3d89f7ae5, "a8b8"),
    (0x308be4264d681ff6, "a8b8"),
    (0xecaf7dc9fbf9679a, "d3d4"),
    (0x13c28874f0b1ac26, "g5f3"),
    (0x789d0369723e8536, "c1g5"),
    (0x02a697d259cda67b, "c1g5"),
    (0xa262bd1488a1f5db, "c1g5"),
    (0xe621cde1f578352a, "c1h6"),
    (0xad4c5cbc637f977e, "f8e7"),
    (0x2bd1fcd8729fdb16, "f8e7"),
    (0xd777c807488cb433, "f8e7"),
    (0xcc8cbde33ffe7896, "h6g5"),
    (0xf075a0e9dc5b6065, "f8e7"),
    (0x55d311f7743ffeb6, "f8e7"),
    (0xeb7dfbe12dbadcab, "h6g7"),
    (0xed1c87dfdc9960b7, "f8e7"),
    (0x6b8127bbcd792cdf, "f8e7"),
    (0x97271364f76a43fa, "f8e7"),
    (0x0d6ccf3716612541, "c6c7"),
    (0xb9271de204ee3c7b, "c6c7"),
    (0x9216437c9e80d361, "c6c7"),
    (0xe5af6c5c585c3768, "f8e7"),
    (0xad19b19456ec50ef, "f8e7"),
    (0xeca562af755e71aa, "f8e7"),
    (0xe5ed5895a5bd9712, "f8e7"),
    (0xb6bea0773015760a, "f8e7"),
    (0xabb0145d76b07228, "a8b8"),
    (0xcb869b713655ec10, "f8e7"),
    (0xfb463cc838adea4d, "f8e7"),
    (0x4c88e2fed0276aa1, "f8e7"),
    (0x6e6762157383891b, "f8e7"),
    (0x93aa10cceb33b8ad, "f8e7"),
    (0xb2b6348949a94a2d, "f8e7"),
    (0x10b95df97fd19221, "f8e7"),
    (0xc9342ae5b6b25bc5, "f8e7"),
    (0xaae6291ab3c88dd1, "f8e7"),
    (0x69b6999332b99c59, "f8e7"),
    (0x34a9813bcd01e71a, "f8e7"),
    (0xfc97c74e7997ff0f, "f8e7"),
    (0xa454300abb03d0af, "f3g5"),
    (0xcbf4377ffcce3034, "g5e6"),
    (0x336ef2a4aa53ac94, "g5e6"),
    (0xb1cfa3c4d73d1379, "g5e6"),
    (0xaa34d620a04fdfdc, "g5e6"),
    (0x5e31aa48fb3e85f9, "c6c7"),
    (0x9fcfad4224b96cf8, "c6c7"),
    (0x896dadfc90eadc76, "c6c7"),
    (0x436d46feebe68495, "c6c7"),
    (0xc347dcfc780b67eb, "g5e6"),
    (0x8bf1013476bb006c, "g5e6"),
    (0xca4dd20f55092129, "g5e6"),
    (0xc305e83585eac791, "g5e6"),
    (0x905610d710422689, "g5e6"),
    (0xa2d8efda9afd1c04, "h3h5"),
    (0x8d519a89d48cad4b, "g5e6"),
    (0x75cb5f52821131eb, "g5e6"),
    (0xf76a0e32ff7f8e06, "g5e6"),
    (0xec917bd6880d42a3, "g5e6"),
    (0x189407bed37c1886, "c6c7"),
    (0xd96a00b40cfbf187, "c6c7"),
    (0xcfc8000ab8a84109, "c6c7"),
    (0x05c8eb08c3a419ea, "c6c7"),
    (0x85e2710a5049fa94, "g5e6"),
    (0xcd54acc25ef99d13, "g5e6"),
    (0x8ce87ff97d4bbc56, "g5e6"),
    (0x85a045c3ada85aee, "g5e6"),
    (0xd6f3bd213800bbf6, "g5e6"),
    (0xe95b932e7e8a4eea, "g5e6"),
    (0x01d3ac513b4cb75b, "g5f7"),
    (0x47e9f7e17dd356f9, "g5e6"),
    (0x28b4a98342d94818, "h5e8"),
    (0x8be37ce965aa050a, "g5e6"),
    (0x7b1e3abce7f2b974, "h5g6"),
    (0x43898dcfd64ce3af, "g5f7"),
    (0xd054559b5ab28505, "g5e6"),
    (0xeb31269e0150f4af, "g5e6"),
    (0x3dbb974acfb0240b, "h3h5"),
    (0xb5bcfd3b93c7ea45, "h8g8"),
    (0xf034715c704dfaf5, "f8e7"),
    (0x76a9d13861adb69d, "f8e7"),
    (0x8a0fe5e75bbed9b8, "f8e7"),
    (0x91f490032ccc151d, "g7f6"),
    (0x65f1ec6b77bd4f38, "c6c7"),
    (0xa40feb61a83aa639, "c6c7"),
    (0xcac8884764a31eac, "c6c7"),
    (0xfa8b6020ff5cb918, "c6c7"),
    (0xf8879adff488ad2a, "f8e7"),
    (0xb0314717fa38caad, "f8e7"),
    (0xf18d942cd98aebe8, "f8e7"),
    (0xf8c5ae1609690d50, "f8e7"),
    (0xab9656f49cc1ec48, "f8e7"),
    (0x374950c81e2a9152, "a8b8"),
    (0xd6ae6df29a817652, "f8e7"),
    (0xe66eca4b9479700f, "f8e7"),
    (0x2415c1a2b1fb00d8, "f8e7"),
    (0x06fa4149125fe362, "f8e7"),
    (0x8e82e64f47e722ef, "f8e7"),
    (0x688c7bd2f4e1033a, "f8e7"),
    (0xda2b17d528752054, "f8e7"),
    (0x1b624d505ef7de7a, "f8e7"),
    (0xc2ef3a4c9794179e, "f8e7"),
    (0xa13d39b392eec18a, "f8e7"),
    (0x3f729192ec27ab41, "f8e7"),
    (0xa1bc62d141859679, "f8e7"),
    (0xec8f38a37307a337, "f8e7"),
    (0xbbd10f680de6e8e8, "f8e7"),
    (0x5beb476a936315f7, "h7h8n"),
    (0x00fc835f328c6236, "h8f7"),
    (0x8661233b236c2e5e, "h8f7"),
    (0x7ac717e4197f417b, "h8f7"),
    (0x613c62006e0d8dde, "h8f7"),
    (0x95391e68357cd7fb, "c6c7"),
    (0x54c71962eafb3efa, "c6c7"),
    (0x8a7f2e966330dc5c, "c6c7"),
    (0x8865f2de25a4d697, "c6c7"),
    (0x084f68dcb64935e9, "h8f7"),
    (0x40f9b514b8f9526e, "h8f7"),
    (0x0145662f9b4b732b, "h8f7"),
    (0x080d5c154ba89593, "h8f7"),
    (0x5b5ea4f7de00748b, "h8f7"),
    (0xa1ca6cb069271c2e, "h8f7"),
    (0xa440ff0c407919dd, "h8f7"),
    (0x948058b54e811f80, "h8f7"),
    (0xbd892672720cf5b9, "f3g5"),
    (0xd6d6ad6ff083dca9, "g5e6"),
    (0x07a5ac55e05bb239, "g5e6"),
    (0x2e4c68b4a61e4009, "g5e6"),
    (0xaced39d4db70ffe4, "g5e6"),
    (0xb7164c30ac023341, "g5e6"),
    (0x43133058f7736964, "c6c7"),
    (0x82ed375228f48065, "c6c7"),
    (0x27454f841c026f17, "c6c7"),
    (0x5e4fdceee7ab6808, "c6c7"),
    (0xde6546ec74468b76, "g5e6"),
    (0x96d39b247af6ecf1, "g5e6"),
    (0xd76f481f5944cdb4, "g5e6"),
    (0xde27722589a72b0c, "g5e6"),
    (0x8d748ac71c0fca14, "g5e6"),
    (0x4b1f43781e545be8, "g5e6"),
    (0xe1e4bfd5cac438ba, "g1f3"),
    (0xc52151af58dd5c10, "g5e6"),
    (0xbf1ac514732e7f5d, "g5e6"),
    (0xa4e1b0f0045cb3f8, "g5e6"),
    (0x50e4cc985f2de9dd, "c6c7"),
    (0x911acb9280aa00dc, "c6c7"),
    (0x87b8cb2c34f9b052, "c6c7"),
    (0x4db8202e4ff5e8b1, "c6c7"),
    (0xcd92ba2cdc180bcf, "g5e6"),
    (0x852467e4d2a86c48, "g5e6"),
    (0xc498b4dff11a4d0d, "g5e6"),
    (0x4988f67fa7153df2, "g5e6"),
    (0x3a09745f5f02ea2b, "g5e6"),
    (0xcdd08ee521f9abb5, "g5e6"),
    (0x9e837607b4514aad, "g5e6"),
    (0xac0d890a3eee7020, "h4g3"),
    (0x6decb34cecd41022, "g5e6"),
    (0x17d727f7c727336f, "g5e6"),
    (0x0c2c5213b055ffca, "g5e6"),
    (0xf8292e7beb24a5ef, "c6c7"),
    (0x39d7297134a34cee, "c6c7"),
    (0x2f7529cf80f0fc60, "c6c7"),
    (0xe575c2cdfbfca483, "c6c7"),
    (0x655f58cf681147fd, "g5e6"),
    (0x2de9850766a1207a, "g5e6"),
    (0x6c55563c4513013f, "g5e6"),
    (0x92c496bceb0ba619, "g5e6"),
    (0x651d6c0695f0e787, "g5e6"),
    (0x364e94e40058069f, "g5e6"),
    (0xccda5ca3b77f6e3a, "g5e6"),
    (0x6b7ba83764009b8f, "g5e6"),
    (0xcea5bbebacf4aaf0, "g8h8"),
    (0x45fe6318977e8eca, "f8e7"),
    (0xc363c37c869ec2a2, "f8e7"),
    (0x3fc5f7a3bc8dad87, "f8e7"),
    (0x243e8247cbff6122, "f8e7"),
    (0xd03bfe2f908e3b07, "c6c7"),
    (0x11c5f9254f09d206, "c6c7"),
    (0xcd67129980563a6b, "c6c7"),
    (0x4d4d889b13bbd915, "f8e7"),
    (0x05fb55531d0bbe92, "f8e7"),
    (0x444786683eb99fd7, "f8e7"),
    (0x4d0fbc52ee5a796f, "f8e7"),
    (0x1e5c44b07bf29877, "f8e7"),
    (0x0352f09a3d579c55, "a8b8"),
    (0xb85bb93e34367c5c, "g5e6"),
    (0x61d6ce22fd55b5b8, "g5e6"),
    (0x9fb1f21954f9cd60, "g5e6"),
    (0x0204cdddf82f63ac, "g5e6"),
    (0xaf86f0fc9e27c340, "f8e7"),
    (0x9c4b65fc86e60967, "g5e6"),
    (0x1eb53911817b9138, "f8e7"),
    (0xfb2561efa5d539a5, "g5e6"),
    (0xd1a8dfdbd6f372e5, "g5e6"),
    (0x7d1d27384bc2c40c, "g3h3"),
    (0xa8d39abb49aeebdd, "e6c4"),
    (0x99c6faad7f77fa78, "h3e6"),
    (0x8e99a176148d5a44, "h3e6"),
    (0x7e9bbb96321659f8, "c4e6"),
    (0x8a9ec7fe696703dd, "h3e6"),
    (0x4b60c0f4b6e0eadc, "h3e6"),
    (0x5dc2c04a02b35a52, "f7d8"),
    (0xfb185a022e1868c5, "h8h7"),
    (0xf82a899de38c88e5, "h8h7"),
    (0xef75d246887628d9, "h8h7"),
    (0x1f77c8a6aeed2b65, "h8h7"),
    (0xad310efaf7c55c23, "h8h7"),
    (0xbd372a98f9ef8145, "e6f4"),
    (0xe941eb6775ca350e, "f4d5"),
    (0xea7338f8b85ed52e, "f4d5"),
    (0xfd2c6323d3a47512, "f4g6"),
    (0x0d2e79c3f53f76ae, "f4d5"),
    (0xce5819d0558e6cc2, "f4d5"),
    (0xc712c1d372226607, "f4d5"),
    (0x6d577dec0079885b, "f4d5"),
    (0x86bca4e7e19237c6, "f4d5"),
    (0x8fe628d28173a1fa, "f4d5"),
    (0x875420d9a2bb43d2, "f4d5"),
    (0xdfe213faf4f97a2f, "e2e3"),
    (0xffe71874de182d50, "f4d5"),
    (0xe8b843afb5e28d6c, "f4g6"),
    (0x18ba594f93798ed0, "f4d5"),
    (0xfb77aff361cf4f85, "f4d5"),
    (0x7b4fa5a26ecc974b, "f4d5"),
    (0x751d473d67850bf1, "f4d5"),
    (0xde19454600af1e89, "f4d5"),
    (0xc624ac30b37be2d3, "f4d5"),
    (0xdbcc395c33c894bc, "f4d5"),
    (0xd286e15f14649e79, "f4d5"),
    (0x78c35d60663f7025, "f4d5"),
    (0x9328846b87d4cfb8, "f4d5"),
    (0x2e438df95a63ba20, "f4d5"),
    (0x9a72085ee7355984, "f4d5"),
    (0x92c00055c4fdbbac, "f4d5"),
    (0x8361d15b14331b2a, "f4d5"),
    (0xbce140ba43c35599, "f4d5"),
    (0x47daccbbaf5bb047, "g2g3"),
    (0x7bdaa00b40e0c5e2, "f4d5"),
    (0x6c85fbd02b1a65de, "f4g6"),
    (0x9c87e1300d816662, "f4d5"),
    (0x7f4a178cff37a737, "f4d5"),
    (0xf120ff42f97de343, "f4d5"),
    (0x5a24fd399e57f63b, "f4d5"),
    (0x4219144f2d830a61, "f4d5"),
    (0x5ff18123ad307c0e, "f4d5"),
    (0x56bb59208a9c76cb, "f4d5"),
    (0xfcfee51ff8c79897, "f4d5"),
    (0xf03f15a96a869150, "f4d5"),
    (0xaa7e3586c49b5292, "f4d5"),
    (0x1e4fb02179cdb136, "f4d5"),
    (0x16fdb82a5a05531e, "f4d5"),
    (0x075c69248acbf398, "f4d5"),
    (0xa5d9d9aa4f5d77a0, "f4d5"),
    (0x38dcf8c5dd3bbd2b, "f4d5"),
    (0x9464a34b57c79fb4, "f4d5"),
    (0xc3e774c431a358f5, "h3h4"),
    (0xcd531e0f9e97283d, "f4d5"),
    (0xda0c45d4f56d8801, "f4g6"),
    (0xc9c3a98821404ae8, "f4d5"),
    (0x4d042fb1eb015bad, "f4d5"),
    (0x47a94146270a0e9c, "f4d5"),
    (0xecad433d40201be4, "f4d5"),
    (0xf490aa4bf3f4e7be, "f4d5"),
    (0xe9783f27734791d1, "f4d5"),
    (0xe032e72454eb9b14, "f4d5"),
    (0x4a775b1b26b07548, "f4d5"),
    (0xa5630e780219035e, "f4d5"),
    (0x1cf78b821aecbf4d, "f4d5"),
    (0xa8c60e25a7ba5ce9, "f4d5"),
    (0xa074062e8472bec1, "f4d5"),
    (0xb1d5d72054bc1e47, "f4d5"),
    (0x17afebc6546853f4, "f4d5"),
    (0x8e5546c1034c50f4, "f4d5"),
    (0x40f5290c868cfba3, "f4d5"),
    (0x741944a68a92dc05, "f4d5"),
    (0xa5511de112b2acbd, "f4d5"),
    (0x37c8fc7abd126f1f, "f4d5"),
    (0x4174ce648ea083ec, "c6c7"),
    (0xbd73c18433661808, "f4h5"),
    (0x629907da0eaec0cc, "h5g7"),
    (0x75c65c01655460f0, "h5g7"),
    (0xb651df32fc9129ad, "c4e6"),
    (0x6609b05db179a219, "h5g7"),
    (0xe2ce36647b38b35c, "h5g7"),
    (0xe8635893b733e66d, "h5g7"),
    (0xe0b84795c4b31b99, "c4e6"),
    (0x5b5ab39e63cd0f4f, "h5g7"),
    (0x46b226f2e37e7920, "h5g7"),
    (0x4ff8fef1c4d273e5, "h5g7"),
    (0xe5bd42ceb6899db9, "h5g7"),
    (0x0aa917ad9220ebaf, "f6f7"),
    (0xb33d92578ad557bc, "f6f7"),
    (0x070c17f03783b418, "h5g7"),
    (0x0fbe1ffb144b5630, "h5g7"),
    (0xa2a8343cb5679e3b, "h5g7"),
    (0x1e1fcef5c485f6b6, "h5g7"),
    (0xb865f213c451bb05, "h5g7"),
    (0x219f5f149375b805, "h5g7"),
    (0xef3f30d916b51352, "h5g7"),
    (0xa44424bad8328af9, "h5g7"),
    (0xdbd35d731aab34f4, "h5f4"),
    (0x46f6ff7a25601011, "e6c7"),
    (0x51a9a4a14e9ab02d, "e6c7"),
    (0xa1abbe416801b391, "e6c7"),
    (0x2012ba05cab355dc, "c4d5"),
    (0x0224ed71233f7b18, "d6d7"),
    (0x192bbfc47f9f17ee, "d6d7"),
    (0xd4898adfac73632c, "d6d7"),
    (0xd555b19ea92769bd, "d6d7"),
    (0x06f61381fffb6ab7, "d6d7"),
    (0x9351e6de281b2292, "d6d7"),
    (0xf9304f11abc01e09, "d6d7"),
    (0xd911c7394eb58124, "d6d7"),
    (0x248a0b36f801cfa7, "d6d7"),
    (0xe69156d81945cf97, "d6d7"),
    (0x311fadab28e69bd1, "d6d7"),
    (0x85eff46a257039ea, "d6d7"),
    (0x775dc0fff19b817e, "d6d7"),
    (0xcaa0c4040912e4c5, "d6d7"),
    (0x8dd2cc6277e18133, "e6c7"),
    (0xda81c978fee2f328, "d6d7"),
    (0xd63239071efc1d27, "d6d7"),
    (0x22a5128d23636077, "d6d7"),
    (0x062caa4ef029e590, "c5a6"),
    (0x278d94d193fce185, "a6c5"),
    (0x11e8e92d9a025a3b, "a6b8"),
    (0x3c82c664cf5c8d73, "a6c5"),
    (0x2b43f2fc02ef757d, "e5e6"),
    (0x5b88b6e1f7ab5d63, "e5e6"),
    (0x6b8d36d9bcc5e29f, "e5e6"),
    (0x8cd21fff6bbd2855, "e5e6"),
    (0x3820c23817b8b3f5, "e5e6"),
    (0xb3ef97281eb30590, "a6c5"),
    (0x64b53e3682a570a6, "a6c5"),
    (0x235f6a214f38f02a, "a6c5"),
    (0x607e0d21019ac7a8, "a6c5"),
    (0x0a1fa4ee8241fb33, "a6c5"),
    (0xfcb8be99fe761bb9, "a6c5"),
    (0x0123729648c2553a, "a6c5"),
    (0xc3382f78a986550a, "a6c5"),
    (0x5679b05c9a26fd6d, "a6c5"),
    (0xa0468dca95b3a377, "a6c5"),
    (0x52f4b95f41581be3, "a6c5"),
    (0x5bd53c401b09e936, "a6c5"),
    (0xff28b0d84e2169b5, "a6c5"),
    (0x150438ea44b84903, "a6c5"),
    (0xdfa7d156241eb58e, "a6c5"),
    (0xf39b40a7ae3f87ba, "a6c5"),
    (0x070c6b2d93a0faea, "a6c5"),
    (0x0e414b8d471db4a6, "b5b6"),
    (0xb8977c5e3529fd95, "f4g6"),
    (0xb865a384ba4695f7, "d5e4"),
    (0x5ed367661a9fa854, "c6c7"),
    (0x59b2193e1c450af5, "c6c7"),
    (0x42496cda6b37c650, "c6c7"),
    (0x55885842a6843e5e, "c6c7"),
    (0x25431c5f53c01640, "c6c7"),
    (0xd5a4476617515fde, "f4e6"),
    (0x417e318f9edbfa24, "c6c7"),
    (0xcd243d96bad84eb3, "c6c7"),
    (0x9d761b9ee4ac4d6b, "c6c7"),
    (0xcfa66cc911bbf1de, "a8b8"),
    (0x62b0470eb09739d5, "a8b8"),
    (0x85e64d2e77155268, "c6c7"),
    (0x787d8121c1a11ceb, "c6c7"),
    (0xbdf385c60ded1e29, "c6c7"),
    (0x2f2743eb1345b4bc, "c6c7"),
    (0x1e6ffc753e271e36, "c6c7"),
    (0x2baa4ae8c83b5232, "c6c7"),
    (0x228bcff7926aa0e7, "c6c7"),
    (0x8676436fc7422064, "c6c7"),
    (0xa6f922e1ad7dfc5f, "c6c7"),
    (0x8ac5b310275cce6b, "c6c7"),
    (0x7e52989a1ac3b33b, "c6c7"),
    (0x4ef6010ff01ffa67, "b5b6"),
    (0x17bdc26caa67a381, "a6c5"),
    (0x2ea6a4831b69ccf3, "f4d5"),
    (0xd25374b929c03f75, "a6c7"),
    (0x4462ac6407800e13, "a6c7"),
    (0x3f37cca141278ae2, "f4g6"),
    (0x8ea4feae1060ec39, "c6c7"),
    (0xb3b2a3e8ac67180e, "f4g6"),
    (0x6b970651ef1ca338, "e6c7"),
    (0xc1d2ba6e9d474d64, "e6c7"),
    (0xc77d48e5e0e1aa0f, "c4b4"),
    (0xc397839a549f1664, "d6d7"),
    (0x24cac2a119feb5e4, "d6d7"),
    (0xc707341deb4874b1, "d6d7"),
    (0x43c0b224210965f4, "d6d7"),
    (0x496ddcd3ed0230c5, "d6d7"),
    (0xfa5437de39fcd9e7, "d6d7"),
    (0xaba793edc8113d07, "d6d7"),
    (0x12331617d0e48114, "d6d7"),
    (0x4dfbfcb43c5bada9, "d6d7"),
    (0xaeb09bbb4e7a8098, "d6d7"),
    (0x03a6b07cef564893, "d6d7"),
    (0xf96475bed60be34b, "d6d7"),
    (0x00d35052931d505b, "f4g6"),
    (0xbeb034f8b048b16e, "h8f8"),
    (0xe96a1114fb219b7d, "h8f8"),
    (0x5d7dc24442fe703b, "h8f8"),
    (0xd9ba447d88bf617e, "h8f8"),
    (0xd3172a8a44b4344f, "h8f8"),
    (0x602ec187904add6d, "h8f8"),
    (0x7d8291e688310796, "h4g4"),
    (0x78d276dfad5fb3aa, "d7f6"),
    (0x60c8fbbbc5b26d22, "g4e6"),
    (0xd95c7e41dd47d131, "g4e6"),
    (0xd91159928de02e49, "g4e6"),
    (0x65dff3ed43d9d0bd, "a8b8"),
    (0x0f597968026890c8, "a8b8"),
    (0x580dd6c34c1c84bb, "h8f8"),
    (0x54ae7a8efb196aa5, "h8f8"),
    (0x2f9fd20a2577730b, "g4e6"),
    (0xd2041e0593c33d88, "g4e6"),
    (0xd78fe2a9921ab5c6, "g4e6"),
    (0x122900e7f825c3b4, "d7c5"),
    (0xc7d4144985b300ac, "d6d7"),
    (0x48b94505545c884f, "d6d7"),
    (0xf12dc0ff4ca9345c, "d6d7"),
    (0xaee52a5ca01618e1, "d6d7"),
    (0x4dae4d53d23735d0, "d6d7"),
    (0x2728c7d6938675a5, "d6d7"),
    (0x9aa4159c5eafee3a, "d6d7"),
    (0x26dec52423edca3d, "h8f8"),
    (0x7cdfc4306af78fc8, "d6d7"),
    (0x1d37fd9c3740bc9b, "d6d7"),
    (0xfb069809856ec8c8, "d6d7"),
    (0x9872c5c984c2905e, "d6d7"),
    (0x4f1475bc61870dca, "d6d7"),
    (0x5becce6e993a5bed, "d6d7"),
    (0x3c1f2b13145e9949, "d6d7"),
    (0xfec503e2fcc38beb, "d6d7"),
    (0x6461dcb4c12c9328, "d6d7"),
    (0xcea87a592dbdde2e, "d6d7"),
    (0xc6f50e4bd15e3f06, "d6d7"),
    (0x63435c2d204a8994, "c5a6"),
    (0xd40eb86d937b5f68, "a6c7"),
    (0xbeb0713c525e42f7, "a6c7"),
    (0x421645e3684d2dd2, "a6c7"),
    (0x59ed30071f3fe177, "a6c7"),
    (0xd680614bced06994, "a6c7"),
    (0x6f14e4b1d625d587, "a6c7"),
    (0x30dc0e123a9af93a, "a6c7"),
    (0xd397691d48bbd40b, "a6c7"),
    (0xb911e398090a947e, "a6c7"),
    (0x7df4a2f495dc04c8, "a6c7"),
    (0xe2e6e07ef07b6e13, "a6c7"),
    (0x830ed9d2adcc5d40, "a6c7"),
    (0x653fbc471fe22913, "a6c7"),
    (0x7074a2a56d223163, "a6c7"),
    (0x002875af8c472751, "a6c7"),
    (0x2980dff8b1aed147, "a6c7"),
    (0xd12d51f2fb0bec11, "a6c7"),
    (0x717a4182ff1e2195, "a6c7"),
    (0xa7cf17c3a4a50c3f, "a6c7"),
    (0x51804851b5af451b, "a6c7"),
    (0x6cec1d5988becb3b, "a6c7"),
    (0x8434c87ce9634bec, "a6c7"),
    (0x0ce7593bb1dfa20d, "a6c7"),
    (0xf78bc8584ea11db0, "a6c5"),
    (0xce90aeb7ffaf72c2, "g6h4"),
    (0xb14a493849000abc, "h4f5"),
    (0xdbf4806988251723, "h4f5"),
    (0x2752b4b6b2367806, "h4f5"),
    (0x3ca9c152c544b4a3, "h4g6"),
    (0xb3c4901e14ab3c40, "h4f5"),
    (0x0a5015e40c5e8053, "h4f5"),
    (0x5598ff47e0e1acee, "h4f5"),
    (0xb6d3984892c081df, "h4f5"),
    (0x1a479718526cdc8c, "e1g1"),
    (0x6688b4ddfec291a3, "a6c7"),
    (0x99c18333df5ccad8, "c7d5"),
    (0x57f65be42a7de75c, "c7d5"),
    (0x91729558552c59c9, "c7d5"),
    (0x906b70726b87f168, "c7d5"),
    (0xe6173bd38b81d2f0, "c7d5"),
    (0x1ab10f0cb192bdd5, "b4a3"),
    (0x014a7ae8c6e07170, "c7d5"),
    (0x589900ed69b01046, "c7d5"),
    (0xe2598edb6bda9807, "c7d5"),
    (0x42ba28f73e54a313, "c7d5"),
    (0xd652a8054d443ff8, "c7d5"),
    (0xd71b173b445fe014, "c7d5"),
    (0x6e8f92c15caa5c07, "c7d5"),
    (0x687b44fde345693d, "c7d5"),
    (0x8b3023f29164440c, "c7d5"),
    (0x3db4d2c865e291c0, "c7d5"),
    (0x71efd4d5e7adb8e3, "c7d5"),
    (0xc623a29de65526af, "c7d5"),
    (0x7127951768714140, "c7d5"),
    (0x898a1b1d22d47c16, "c7d5"),
    (0x661d80cc12b51535, "c7d5"),
    (0x449a50d6fef153a3, "c7d5"),
    (0x1517600f3e6378ba, "c7d5"),
    (0x97b26524092381c9, "c7d5"),
    (0x8b0d7612055f05a6, "c7d5"),
    (0xb2f1343a091eecd9, "c7d5"),
    (0x104d363ec6f549ae, "c7d5"),
    (0xc2e00c54479d95b6, "c7d5"),
    (0x28cc84664d04b500, "c7d5"),
    (0xe26f6dda2da2498d, "c7d5"),
    (0xce53fc2ba7837bb9, "c7d5"),
    (0x3ac4d7a19a1c06e9, "c7d5"),
    (0xfc3f8b1dbc91339b, "a8b8"),
    (0xb63f04b506b71f4f, "c8b6"),
    (0x7808dc62f39632cb, "c8b6"),
    (0xbe8c12de8cc78c5e, "c8b6"),
    (0xbf95f7f4b26c24ff, "c8b6"),
    (0xc9e9bc55526a0767, "c8b6"),
    (0xbd6b73d57e2d7dea, "c8b6"),
    (0x354f888a68796842, "c8b6"),
    (0x2eb4fd6e1f0ba4e7, "c8b6"),
    (0x7767876bb05bc5d1, "c8b6"),
    (0xcda7095db2314d90, "c8b6"),
    (0x347893eeb4ef6f03, "c8b6"),
    (0x390087b130301531, "b7a6"),
    (0x939a377b47e08802, "c8b6"),
    (0x5dadefacb2c1a586, "c8b6"),
    (0x9b292110cd901b13, "c8b6"),
    (0x9a30c43af33bb3b2, "c8b6"),
    (0xec4c8f9b133d902a, "c8b6"),
    (0x98ce401b3f7aeaa7, "c8b6"),
    (0x10eabb44292eff0f, "c8b6"),
    (0x0b11cea05e5c33aa, "c8b6"),
    (0x52c2b4a5f10c529c, "c8b6"),
    (0xe8023a93f366dadd, "c8b6"),
    (0xb81a9e8aa91de95f, "c8b6"),
    (0xf49d54994c64fd9c, "c8b6"),
    (0x67dd4801df30313d, "c8b6"),
    (0xe2485e7e15f568e3, "c8b6"),
    (0x3571ff4b0f406df6, "c8b6"),
    (0x482a1fbbca8bf7b5, "c8b6"),
    (0x1832bba290f0c437, "c8b6"),
    (0x8176d251e710801e, "c8b6"),
    (0x847c9fec8fb3bb49, "c8b6"),
    (0x532636f213a5ce7f, "c8b6"),
    (0x37ef6680fd5ed31a, "c8b6"),
    (0x22885c022c41e3be, "c8b6"),
    (0x95442a4a2db97df2, "c8b6"),
    (0x7b7c215ff0cd039a, "c8b6"),
    (0x83d1af55ba683ecc, "c8b6"),
    (0x6c4634848a0957ef, "c8b6"),
    (0x4ec1e49e664d1179, "c8b6"),
    (0x1f4cd447a6df3a60, "c8b6"),
    (0x9de9d16c919fc313, "c8b6"),
    (0x8156c25a9de3477c, "c8b6"),
    (0xb8aa807291a2ae03, "c8b6"),
    (0x1a1682765e490b74, "c8b6"),
    (0xc8bbb81cdf21d76c, "c8b6"),
    (0x26003ff3342fb191, "a6b7"),
    (0x5a1c1d66dabd5305, "c8b6"),
    (0x942bc5b12f9c7e81, "c8b6"),
    (0x52af0b0d50cdc014, "c8b6"),
    (0x53b6ee276e6668b5, "c8b6"),
    (0x25caa5868e604b2d, "c8b6"),
    (0xd96c9159b4732408, "c8b6"),
    (0xc297e4bdc301e8ad, "c8b6"),
    (0x9b449eb86c51899b, "c8b6"),
    (0x2184108e6e3b01da, "c8b6"),
    (0x719cb49734403258, "c8b6"),
    (0x3d1b7e84d139269b, "c8b6"),
    (0x7d4791433d10d5b4, "c8b6"),
    (0xae5b621c426dea3a, "c8b6"),
    (0x2bce746388a8b3e4, "c8b6"),
    (0xfcf7d556921db6f1, "c8b6"),
    (0x81ac35a657d62cb2, "c8b6"),
    (0xd1b491bf0dad1f30, "c8b6"),
    (0x48f0f84c7a4d5b19, "c8b6"),
    (0x4dfab5f112ee604e, "c8b6"),
    (0x5c7f2012985fb17e, "c8b6"),
    (0xfe694c9d6003081d, "c8b6"),
    (0xeb0e761fb11c38b9, "c8b6"),
    (0x5cc20057b0e4a6f5, "c8b6"),
    (0xb2fa0b426d90d89d, "c8b6"),
    (0x4a5785482735e5cb, "c8b6"),
    (0xa5c01e9917548ce8, "c8b6"),
    (0x514ba32c3255054c, "c8b6"),
    (0x8747ce83fb10ca7e, "c8b6"),
    (0xd6cafe5a3b82e167, "c8b6"),
    (0x546ffb710cc21814, "c8b6"),
    (0x48d0e84700be9c7b, "c8b6"),
    (0x712caa6f0cff7504, "c8b6"),
    (0xd390a86bc314d073, "c8b6"),
    (0x672b4c5690093258, "b7a6"),
    (0xcdb1fc9ce7d9af6b, "c8b6"),
    (0x0386244b12f882ef, "c8b6"),
    (0xc502eaf76da93c7a, "c8b6"),
    (0xc41b0fdd530294db, "c8b6"),
    (0xb267447cb304b743, "c8b6"),
    (0x4ec170a38917d866, "c8b6"),
    (0x553a0547fe6514c3, "c8b6"),
    (0x0ce97f42513575f5, "c8b6"),
    (0xb629f174535ffdb4, "c8b6"),
    (0xe631556d0924ce36, "c8b6"),
    (0xaab69f7eec5ddaf5, "c8b6"),
    (0x39f683e67f091654, "c8b6"),
    (0x6b5a34acaf794a9f, "c8b6"),
    (0x1601d45c6ab2d0dc, "c8b6"),
    (0x4619704530c9e35e, "c8b6"),
    (0xdf5d19b64729a777, "c8b6"),
    (0xda57540b2f8a9c20, "c8b6"),
    (0xdb8b6f4a2ade96b1, "c8b6"),
    (0xcbd2c1e8a53b4d10, "c8b6"),
    (0x69c4ad675d67f473, "c8b6"),
    (0x7ca397e58c78c4d7, "c8b6"),
    (0xcb6fe1ad8d805a9b, "c8b6"),
    (0x2557eab850f424f3, "c8b6"),
    (0xddfa64b21a5119a5, "c8b6"),
    (0x326dff632a307086, "c8b6"),
    (0xc6e642d60f31f922, "c8b6"),
    (0x10ea2f79c6743610, "c8b6"),
    (0x41671fa006e61d09, "c8b6"),
    (0xc3c21a8b31a6e47a, "c8b6"),
    (0xdf7d09bd3dda6015, "c8b6"),
    (0xe6814b95319b896a, "c8b6"),
    (0x443d4991fe702c1d, "c8b6"),
    (0xf086adacad6dce36, "b8a8"),
    (0xba862204174be2e2, "c8b6"),
    (0x74b1fad3e26acf66, "c8b6"),
    (0xb235346f9d3b71f3, "c8b6"),
    (0xb32cd145a390d952, "c8b6"),
    (0xc5509ae44396faca, "c8b6"),
    (0x39f6ae3b798595ef, "c8b6"),
    (0x220ddbdf0ef7594a, "c8b6"),
    (0x7bdea1daa1a7387c, "c8b6"),
    (0xc11e2feca3cdb03d, "c8b6"),
    (0x91068bf5f9b683bf, "c8b6"),
    (0xdd8141e61ccf977c, "c8b6"),
    (0xcb544b01455e0203, "c8b6"),
    (0x1c6dea345feb0716, "c8b6"),
    (0x61360ac49a209d55, "c8b6"),
    (0x312eaeddc05baed7, "c8b6"),
    (0xa86ac72eb7bbeafe, "c8b6"),
    (0xad608a93df18d1a9, "c8b6"),
    (0xbce51f7055a90099, "c8b6"),
    (0x1ef373ffadf5b9fa, "c8b6"),
    (0x0b94497d7cea895e, "c8b6"),
    (0xbc583f357d121712, "c8b6"),
    (0x52603420a066697a, "c8b6"),
    (0xaacdba2aeac3542c, "c8b6"),
    (0x455a21fbdaa23d0f, "c8b6"),
    (0xb1d19c4effa3b4ab, "c8b6"),
    (0x67ddf1e136e67b99, "c8b6"),
    (0x3650c138f6745080, "c8b6"),
    (0xb4f5c413c134a9f3, "c8b6"),
    (0xa84ad725cd482d9c, "c8b6"),
    (0x91b6950dc109c4e3, "c8b6"),
    (0x330a97090ee26194, "c8b6"),
    (0x87b173345dff83bf, "a6b7"),
    (0x2d2bc3fe2a2f1e8c, "c8b6"),
    (0xe31c1b29df0e3308, "c8b6"),
    (0x2598d595a05f8d9d, "c8b6"),
    (0x248130bf9ef4253c, "c8b6"),
    (0x52fd7b1e7ef206a4, "c8b6"),
    (0xae5b4fc144e16981, "c8b6"),
    (0xb5a03a253393a524, "c8b6"),
    (0xec7340209cc3c412, "c8b6"),
    (0x56b3ce169ea94c53, "c8b6"),
    (0x06ab6a0fc4d27fd1, "c8b6"),
    (0x4a2ca01c21ab6b12, "c8b6"),
    (0x3eeee0b5831d1845, "c8b6"),
    (0x8bc00bce628ffb78, "c8b6"),
    (0xf69beb3ea744613b, "c8b6"),
    (0xa6834f27fd3f52b9, "c8b6"),
    (0x3fc726d48adf1690, "c8b6"),
    (0x3acd6b69e27c2dc7, "c8b6"),
    (0x8359ee93fa8991d4, "c8b6"),
    (0x2b48fe8a68cdfcf7, "c8b6"),
    (0x895e920590914594, "c8b6"),
    (0x9c39a887418e7530, "c8b6"),
    (0x2bf5decf4076eb7c, "c8b6"),
    (0xc5cdd5da9d029514, "c8b6"),
    (0x3d605bd0d7a7a842, "c8b6"),
    (0xd2f7c001e7c6c161, "c8b6"),
    (0x267c7db4c2c748c5, "c8b6"),
    (0xf070101b0b8287f7, "c8b6"),
    (0xa1fd20c2cb10acee, "c8b6"),
    (0x235825e9fc50559d, "c8b6"),
    (0x3fe736dff02cd1f2, "c8b6"),
    (0x061b74f7fc6d388d, "c8b6"),
    (0xa4a776f333869dfa, "c8b6"),
    (0x101c92ce609b7fd1, "b4c4"),
    (0xc1b2544c0f43308d, "e6g4"),
    (0x0f858c9bfa621d09, "e6g4"),
    (0xc7ac7c3856c261f0, "e6g4"),
    (0xc6b599126869c951, "e6g4"),
    (0xbe64ecac5b9e28a5, "e6g4"),
    (0xa93bb77730648899, "e6g4"),
    (0x5939ad9716ff8b25, "a8b8"),
    (0x1339223facd9a7f1, "c8b6"),
    (0xdd0efae859f88a75, "c8b6"),
    (0x15270a4bf558f68c, "c8b6"),
    (0x143eef61cbf35e2d, "c8b6"),
    (0x6cef9adff804bfd9, "c8b6"),
    (0xe17297df89f1929a, "c8b6"),
    (0x7bb0c10493fe1fe5, "c8b6"),
    (0xd261a1e11a357d6f, "c8b6"),
    (0x38b98bce4224c6ac, "c8b6"),
    (0xca8be02f9325dc2a, "c8b6"),
    (0x8ad70fe87f0c2f05, "c8b6"),
    (0xe77e5d4534091ece, "c8b6"),
    (0x62eb4b3afecc4710, "c8b6"),
    (0x0b674bfdd0014c40, "c8b6"),
    (0x9891aee67bc9ebc4, "c8b6"),
    (0x01d5c7150c29afed, "c8b6"),
    (0x5de3b63737da8d3d, "c8b6"),
    (0x155a1f4bee3b458a, "c8b6"),
    (0xb74c73c41667fce9, "c8b6"),
    (0xa22b4946c778cc4d, "c8b6"),
    (0xfbdf341b1bf42c69, "c8b6"),
    (0x0372ba115151113f, "c8b6"),
    (0xad9e01f6a42843c2, "c8b6"),
    (0xece521c06130781c, "c8b6"),
    (0x186e9c754431f1b8, "c8b6"),
    (0xce62f1da8d743e8a, "c8b6"),
    (0x9fefc1034de61593, "c8b6"),
    (0x1d4ac4287aa6ece0, "c8b6"),
    (0x01f5d71e76da688f, "c8b6"),
    (0x380995367a9b81f0, "c8b6"),
    (0x9ab59732b5702487, "c8b6"),
    (0x2e0e730fe66dc6ac, "b7a6"),
    (0x8494c3c591bd5b9f, "c8b6"),
    (0x4aa31b12649c761b, "c8b6"),
    (0x828aebb1c83c0ae2, "c8b6"),
    (0x83930e9bf697a243, "c8b6"),
    (0xfb427b25c56043b7, "c8b6"),
    (0x76df7625b4956ef4, "c8b6"),
    (0xec1d20feae9ae38b, "c8b6"),
    (0x45cc401b27518101, "c8b6"),
    (0xaf146a347f403ac2, "c8b6"),
    (0x5d2601d5ae412044, "c8b6"),
    (0x70d3bcbf096de2a0, "c8b6"),
    (0x9ccaaa07ed65b02e, "c8b6"),
    (0x0f3c4f1c46ad17aa, "c8b6"),
    (0x967826ef314d5383, "c8b6"),
    (0xca4e57cd0abe7153, "c8b6"),
    (0xcb926c8c0fea7bc2, "c8b6"),
    (0x82f7feb1d35fb9e4, "c8b6"),
    (0x20e1923e2b030087, "c8b6"),
    (0x3586a8bcfa1c3023, "c8b6"),
    (0x6c72d5e12690d007, "c8b6"),
    (0x94df5beb6c35ed51, "c8b6"),
    (0x3a33e00c994cbfac, "c8b6"),
    (0x7b48c03a5c548472, "c8b6"),
    (0x8fc37d8f79550dd6, "c8b6"),
    (0x59cf1020b010c2e4, "c8b6"),
    (0x084220f97082e9fd, "c8b6"),
    (0x8ae725d247c2108e, "c8b6"),
    (0x965836e44bbe94e1, "c8b6"),
    (0xafa474cc47ff7d9e, "c8b6"),
    (0x0d1876c88814d8e9, "c8b6"),
    (0xb9a392f5db093ac2, "e6d5"),
    (0xa9025a8fd3bc4c35, "e5e6"),
    (0x67358258269d61b1, "e5e6"),
    (0xaf1c72fb8a3d1d48, "e5e6"),
    (0xae0597d1b496b5e9, "e5e6"),
    (0x5b49ef6ff694795e, "e5e6"),
    (0x685ad951655096ab, "e5e6"),
    (0xad49cd0192a18daa, "d5e6"),
    (0xdacaefc2c73da61f, "b8b7"),
    (0x5d4525f54b6cf50a, "e5e6"),
    (0x2b7eba085d825e4c, "e5e6"),
    (0x0d61e829ab4ca0c2, "d5e6"),
    (0xbbeebfa5734c4429, "e5e6"),
    (0xf1c0c52c0256932f, "e5e6"),
    (0xbbf3fa4e9d84c2e1, "e5e6"),
    (0x0d770b746902172d, "e5e6"),
    (0x181031f6b81d2789, "e5e6"),
    (0x6e2f72d4cb71676f, "d5e6"),
    (0xb949c2a12e34fafb, "e5e6"),
    (0x386e473974ad08c4, "d5e6"),
    (0x56de59701e5593d8, "e5e6"),
    (0x8d9edaba94b4babe, "d5e6"),
    (0x7459896af211d54e, "e5e6"),
    (0x25d4b9b33283fe57, "e5e6"),
    (0xa771bc9805c30724, "e5e6"),
    (0xbbceafae09bf834b, "e5e6"),
    (0x8232ed8605fe6a34, "e5e6"),
    (0x208eef82ca15cf43, "e5e6"),
    (0x90a20462789f6b23, "e5e6"),
    (0xd564c56bf258b3d3, "e5e6"),
    (0x4809be19cf212447, "e5e6"),
    (0x180f5dda41e433ed, "e5e6"),
    (0x7b146a6c619e36d0, "f3g5"),
    (0x209399ac023a9e39, "d6d7"),
    (0x5f3ceab8318ef47b, "a6b7"),
    (0xf5a65a72465e6948, "c8b6"),
    (0x3b9182a5b37f44cc, "c8b6"),
    (0xf3b872061fdf3835, "c8b6"),
    (0xf2a1972c21749094, "c8b6"),
    (0x8a70e29212837160, "c8b6"),
    (0x07edef9263765c23, "c8b6"),
    (0x9d2fb9497979d15c, "c8b6"),
    (0x34fed9acf0b2b3d6, "c8b6"),
    (0xde26f383a8a30815, "c8b6"),
    (0x2c14986279a21293, "c8b6"),
    (0x6c4877a5958be1bc, "c8b6"),
    (0x01e12508de8ed077, "c8b6"),
    (0x84743377144b89a9, "c8b6"),
    (0xedf833b03a8682f9, "c8b6"),
    (0x7e0ed6ab914e257d, "c8b6"),
    (0xe74abf58e6ae6154, "c8b6"),
    (0xbb7cce7add5d4384, "c8b6"),
    (0x02e84b80c5a8ff97, "c8b6"),
    (0xf3c5670604bc8b33, "c8b6"),
    (0x51d30b89fce03250, "c8b6"),
    (0x44b4310b2dff02f4, "c8b6"),
    (0x1d404c56f173e2d0, "c8b6"),
    (0xe5edc25cbbd6df86, "c8b6"),
    (0x4b0179bb4eaf8d7b, "c8b6"),
    (0xfef1e438aeb63f01, "c8b6"),
    (0x28fd899767f3f033, "c8b6"),
    (0x7970b94ea761db2a, "c8b6"),
    (0xfbd5bc6590212259, "c8b6"),
    (0xe76aaf539c5da636, "c8b6"),
    (0xde96ed7b901c4f49, "c8b6"),
    (0x7c2aef7f5ff7ea3e, "c8b6"),
    (0xe4ad9ab386cb3a21, "e6d5"),
    (0xf40c52c98e7e4cd6, "e5e6"),
    (0x3a3b8a1e7b5f6152, "e5e6"),
    (0xf2127abdd7ff1dab, "e5e6"),
    (0xf30b9f97e954b50a, "e5e6"),
    (0x0647e729ab5679bd, "e5e6"),
    (0x3554d11738929648, "e5e6"),
    (0xf047c547cf638d49, "d5e6"),
    (0x6de27f1e5dabc422, "e5e6"),
    (0x87c4e7849affa6fc, "b7a8"),
    (0xf13cb0830e850205, "b8b7"),
    (0x3f0b6854fba42f81, "b8b7"),
    (0xf72298f757045378, "b8b7"),
    (0xf63b7ddd69affbd9, "b8b7"),
    (0xd904a93d5b6038d5, "b8b7"),
    (0x037705632bad376e, "b8b7"),
    (0x3064335db869d89b, "b8b7"),
    (0xf577270d4f98c39a, "b8b7"),
    (0x3e8a4ffe9e3aef50, "e5e6"),
    (0x98376b9080db6c21, "e5e6"),
    (0x674769252f288379, "e5e6"),
    (0xd727244cfb6443e7, "e5e6"),
    (0x6d154d04cfdd872b, "e5e6"),
    (0x555f02257675eef2, "b8b7"),
    (0xe3d055a9ae750a19, "b8b7"),
    (0x38486b2f7bd41192, "b8b7"),
    (0xe3cd104240bd8cd1, "b8b7"),
    (0x5549e178b43b591d, "b8b7"),
    (0x402edbfa652469b9, "b8b7"),
    (0x361198d81648295f, "b8b7"),
    (0xe17728adf30db4cb, "b8b7"),
    (0x6050ad35a99446f4, "b8b7"),
    (0x0ee0b37cc36cdde8, "b8b7"),
    (0xd5a030b6498df48e, "b8b7"),
    (0x2c6763662f289b7e, "b8b7"),
    (0x7dea53bfefbab067, "b8b7"),
    (0x4c452efc585f16e8, "b8b7"),
    (0xe3f045a2d486cd7b, "b8b7"),
    (0xda0c078ad8c72404, "b8b7"),
    (0x78b0058e172c8173, "b8b7"),
    (0xc89cee6ea5a62513, "b8b7"),
    (0x8d5a2f672f61fde3, "b8b7"),
    (0x1037541512186a77, "b8b7"),
    (0x4031b7d69cdd7ddd, "b8b7"),
    (0x4ac27002948e8d3f, "b5b6"),
    (0x78ad73a0df03d009, "d6d7"),
    (0x004b2db316aef5e9, "e5e6"),
    (0x7670b24e00405eaf, "e5e6"),
    (0x506fe06ff68ea021, "d5e6"),
    (0xe6e0b7e32e8e44ca, "e5e6"),
    (0xaccecd6a5f9493cc, "e5e6"),
    (0xe6fdf208c046c202, "e5e6"),
    (0x5079033234c017ce, "e5e6"),
    (0x451e39b0e5df276a, "e5e6"),
    (0x33217a9296b3678c, "d5e6"),
    (0xe447cae773f6fa18, "e5e6"),
    (0x65604f7f296f0827, "d5e6"),
    (0x0bd051364397933b, "e5e6"),
    (0xd090d2fcc976ba5d, "d5e6"),
    (0x2957812cafd3d5ad, "e5e6"),
    (0x78dab1f56f41feb4, "e5e6"),
    (0xfa7fb4de580107c7, "e5e6"),
    (0xe6c0a7e8547d83a8, "e5e6"),
    (0xdf3ce5c0583c6ad7, "e5e6"),
    (0x7d80e7c497d7cfa0, "e5e6"),
    (0xcdac0c24255d6bc0, "e5e6"),
    (0x886acd2daf9ab330, "e5e6"),
    (0x1507b65f92e324a4, "e5e6"),
    (0x4501559c1c26330e, "e5e6"),
    (0x261a622a3c5c3633, "f3g5"),
    (0x7d9d91ea5ff89eda, "d6d7"),
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
