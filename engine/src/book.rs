use crate::moves::Move;
use std::collections::HashMap;
use std::sync::OnceLock;

/// Minimal solution book for 1.e3 — only reachable positions.
/// Cleaned by reachable-extractor from 2221 entries.
const BOOK_DATA: &[(u64, &str)] = &[
    (0x3eb9976bbb8190ba, "b1c3"),   // mate in 15 half-moves
    (0x87834ea2f65f083c, "c3e4"),   // mate in 13 half-moves
    (0xc8b76cd20b758f82, "d2d3"),   // d3! reclaims Ne4 via diagonal, safe, opens bishop,   // Qf3! safe from Nf6, reclaims Ne4,   // Qg4! Nf6 f3 reclaims, Qe6 Nd6# (mate in 4),   // mate in 11 half-moves
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
    (0x7a20768eda5580bb, "g1h1"),
    (0x69c1b9bbf26de0f0, "c7e8"),
    (0xca9fcb8a27b80214, "e8f6"),
    (0xc158a48bbe49c1d7, "h7g7"),
    (0x6a41938b71dbd049, "h7g7"),
    (0xee7f597822a81aad, "h7g7"),
    (0xa4d4248b25342cee, "h7g7"),
    (0x602d0d7da9e8c08e, "h7g7"),
    (0x4c6f3aa4c9619512, "h7g7"),
    (0xdcb1928dadd391fc, "h7g7"),
    (0xb7bc638d0dceb1aa, "h7g7"),
    (0x3f88cd9fa1dff69e, "h7g7"),
    (0x484dbcc3eab260e7, "h7g7"),
    (0x2c4aea76f9108cef, "h7g7"),
    (0x309edb246a77078d, "h7g7"),
    (0x3b2459d15e0cf12f, "h7g7"),
    (0xfcb0db9fd92eadca, "h7g7"),
    (0xe91eb65693332f87, "h7g7"),
    (0xa6faa95bcd435d69, "h7g7"),
    (0x70e9e1e7d6393170, "h7g7"),
    (0xeaf963af6302e775, "h7g7"),
    (0x5e5e91fa8c3ddcd4, "h7g7"),
    (0x2ff143e49c075a1c, "h7g7"),
    (0xa54b4b28912a3308, "h7g7"),
    (0xace559a7094d3a30, "h7g7"),
    (0x4f466f58962040e0, "e4g5"),
    (0x57ac55518434aa9c, "e4g5"),
    (0xc494de7ce83a645b, "h7g7"),
    (0x6921e6beaca14941, "h7g7"),
    (0x6a8486b98d8fd325, "h7g7"),
    (0x380595953e3245ce, "h7g7"),
    (0xbb7b17940cc179a9, "h7g7"),
    (0x07623d130b0e0a09, "h7g7"),
    (0xf8ed7a42ca554906, "h7g7"),
    (0x89e5a09b5f00fd8f, "h7g7"),
    (0x2d9295176a36c20f, "h7g7"),
    (0xcfc3b7349d6738fa, "h6f8"),
    (0x606aa89121543891, "e4c5"),
    (0xe76408fed28f84bf, "e6e7"),
    (0x4c7d3ffe1d1d9521, "e6e7"),
    (0xe01fdf177441d0fc, "e6e7"),
    (0x62469fc2369bfd9e, "e6e7"),
    (0x82e888fe49f26986, "e6e7"),
    (0x651d13016a08f787, "e6e7"),
    (0x6a5396d1a5a7d07a, "e6e7"),
    (0xfa8d3ef8c115d494, "e6e7"),
    (0x52fcee45f3d70998, "e6e7"),
    (0x6c525277d57fe49d, "e6e7"),
    (0x19b461eacd19b3f6, "e6e7"),
    (0xf08b28551b23c7f4, "e6e7"),
    (0xdf56b950723af616, "e6e7"),
    (0x67ada3a447af71bd, "f5f6"),
    (0x59031f9661079cb8, "f5f6"),
    (0x8d82d723928f8c06, "f5f6"),
    (0x487032a12eb6405f, "f5f6"),
    (0xcb9a127d8285f1a5, "f5f6"),
    (0x0a76460395d6c987, "e6e7"),
    (0x16a2775106b142e5, "e6e7"),
    (0x046e01c76dbcc933, "e6e7"),
    (0xda8c77eab5e8e8a2, "e6e7"),
    (0x652770ec8700c8b4, "e6e7"),
    (0xf9a9d2cd3e45d144, "e6e7"),
    (0x80c6052ea1851801, "e6e7"),
    (0xb8c353368eccdc41, "c5e4"),
    (0x83dc0f73446307f9, "g3h5"),
    (0x28c538738bf11667, "g3h5"),
    (0x06fe984fa0777ed8, "g3h5"),
    (0xe6508f73df1eeac0, "g3h5"),
    (0x01a5148cfce474c1, "g3h5"),
    (0x6ece418e033a4ac1, "g3h5"),
    (0x721a70dc905dc1a3, "g3h5"),
    (0x79a0f229a4263701, "g3h5"),
    (0xbe34706723046be4, "g3h5"),
    (0x6de8876f900b249b, "g3h5"),
    (0xa87dc8579928215b, "g3h5"),
    (0x1cda3a0276171afa, "g3h5"),
    (0x4e795a15c90bee53, "g3h5"),
    (0x6f15f1e3112da24d, "f6h5"),
    (0xe7cfe0d06b00f526, "g3h5"),
    (0x6fb0404937298526, "g3h5"),
    (0xa5759db0421e9551, "g3h5"),
    (0x97c75aabb4d8a1b7, "g3h5"),
    (0x9b3b7780f49ed7bd, "g3h5"),
    (0xfc23131d4dc8efdb, "g3h5"),
    (0x40da3b317a5a7188, "g3h5"),
    (0xd5fc1ffda6e45ec0, "g3h5"),
    (0x48e727ead88bbf6d, "g3h5"),
    (0x5d5146459f9f060e, "g3h5"),
    (0x2c151f4b6ae5002e, "g3h5"),
    (0xde25eaf99266fef1, "g3h5"),
    (0xdbaea4c3ab8ee793, "g3h5"),
    (0x232add2060c6d323, "g3h5"),
    (0xbda0e0d6fa6aa004, "g3h5"),
    (0xf9ffbc6cf6ebbf87, "g3h5"),
    (0x45e696ebf124cc27, "g3h5"),
    (0x3bb863acf431f610, "g3h5"),
    (0xe0b5d3ba1991e0c2, "g3h5"),
    (0xeec78cf954527d19, "g3h5"),
    (0xe5cfa65d2a95e6b2, "d7c5"),
    (0x3d5df1b5321439bc, "e6e7"),
    (0x9644c6b5fd862822, "e6e7"),
    (0xb87f6689d600409d, "e6e7"),
    (0x58d171b5a969d485, "e6e7"),
    (0xbf24ea4a8a934a84, "e6e7"),
    (0x58a39c211477a726, "e6f6"),
    (0xd9489e9ca061e435, "e6f6"),
    (0x997e2ddaa95ebfdc, "e6f6"),
    (0x11a3eb36621e4ca6, "e6f6"),
    (0xd04fbf48754d7484, "e6e7"),
    (0xcc9b8e1ae62affe6, "e6e7"),
    (0xde57f88c8d277430, "e6e7"),
    (0x00b58ea1557355a1, "e6e7"),
    (0x2fa4ef9f83c61073, "e6e7"),
    (0x18a81b7ab23ede22, "e6e7"),
    (0xe7d92656687928cc, "e6e7"),
    (0x16fc3691ef5f1f1e, "e6e7"),
    (0xa25bc4c4006024bf, "e6e7"),
    (0xf0f8a4d3bf7cd016, "e6e7"),
    (0xd1940f25675a9c08, "e6e7"),
    (0xa40063bff307d6df, "e6e7"),
    (0x1bf463763469ab14, "e6e7"),
    (0x5c24b84a2e864580, "e6e7"),
    (0x2946a46dc2af9ff2, "e6e7"),
    (0x86f2d2005b4b3090, "e6e7"),
    (0xa87a903ab8e25e72, "e6e7"),
    (0x527081056c1e6e90, "e6e7"),
    (0x20591d44eb26d395, "b7a7"),
    (0x474170ca5ab252b1, "e6e7"),
    (0xe3d0b883e9e8384b, "e6e7"),
    (0x96407ae362d973dd, "b7a7"),
    (0xea69f17da8ac0b6e, "g3h5"),
    (0xd23fdf24b7eec1d3, "d6d7"),
    (0x132e8d82b62fae00, "c5d7"),
    (0xf410d9790fd9b4ca, "f6g4"),
    (0x00ffe24f251d7364, "h6g7"),
    (0xabe6d54fea8f62fa, "h6g7"),
    (0x85dd7573c1090a45, "h6g7"),
    (0xfde2ede13cbbf169, "h6g7"),
    (0x8286f9b09d9a005c, "h6g7"),
    (0xededacb262443e5c, "h6g7"),
    (0xf1399de0f123b53e, "h6g7"),
    (0xfa831f15c558439c, "h6g7"),
    (0x3d179d5b427a1f79, "h6g7"),
    (0xeecb6a53f1755006, "h6g7"),
    (0x2b5e256bf85655c6, "h6g7"),
    (0x9ff9d73e17696e67, "h6g7"),
    (0xcd5ab729a8759ace, "h6g7"),
    (0xec361cdf7053d6d0, "h6g7"),
    (0x1ad81f9beec8333f, "h6g7"),
    (0x4b273393a59e2179, "h6g7"),
    (0xc358930af9b75179, "h6g7"),
    (0x268489ad4632c902, "h6g7"),
    (0x341406309aced832, "h6g7"),
    (0xc627bd3558164fc2, "g5f6"),
    (0x6aa344e56359220a, "g5f6"),
    (0x3e0366c08597dc9d, "g5f6"),
    (0xde72ab79fee17293, "h6g7"),
    (0xaf36f2770b9b74b3, "h6g7"),
    (0x5d0607c5f3188a6c, "h6g7"),
    (0x588d49ffcaf0930e, "h6g7"),
    (0xa009301c01b8a7be, "h6g7"),
    (0x3e830dea9b14d499, "h6g7"),
    (0x7adc51509795cb1a, "h6g7"),
    (0xc6c57bd7905ab8ba, "h6g7"),
    (0xb89b8e90954f828d, "h6g7"),
    (0x63963e8678ef945f, "h6g7"),
    (0x6de461c5352c0984, "h6g7"),
    (0x1fbb8ed4237fd831, "f5f6"),
    (0x2e8c9e78a126e4d8, "g5f6"),
    (0x3bc6002c2e9e0391, "f5f6"),
    (0x10e0d76ccdfdd92f, "e4f6"),
    (0x73d3e3ba9d87b139, "h4h5"),
    (0xf0edf7cdf98fbd03, "g6g7"),
    (0x5bf4c0cd361dac9d, "g6g7"),
    (0x75cf60f11d9bc422, "g6g7"),
    (0x956177cd62f2503a, "g6g7"),
    (0x7294ec324108ce3b, "g6g7"),
    (0x1dffb930bed6f03b, "g6g7"),
    (0x012b88622db17b59, "g6g7"),
    (0x0a910a9719ca8dfb, "g6g7"),
    (0xcd0588d99ee8d11e, "g6g7"),
    (0x1ed97fd12de79e61, "g6g7"),
    (0x643fb4e48deb8904, "d7f8"),
    (0xc1b75d852c9d723d, "h7h6"),
    (0x6aae6a85e30f63a3, "h7h6"),
    (0x4495cab9c8890b1c, "h7h6"),
    (0xa43bdd85b7e09f04, "h7h6"),
    (0x43ce467a941a0105, "h7h6"),
    (0xa44930110afeeca7, "h7h6"),
    (0xf57a71a26a2ed17f, "h7h6"),
    (0xed4947067c970727, "h7h6"),
    (0x2ca513786bc43f05, "h7h6"),
    (0x3071222af8a3b467, "h7h6"),
    (0x3bcba0dfccd842c5, "h7h6"),
    (0xfc5f22914bfa1e20, "h7h6"),
    (0x700618e944ed829a, "h7h6"),
    (0x62bf3f5af14aaed4, "g6g7"),
    (0xea169aa1f1d6549f, "h7h6"),
    (0x5eb168f41ee96f3e, "h7h6"),
    (0x0c1208e3a1f59b97, "h7h6"),
    (0x2d7ea31579d3d789, "h7h6"),
    (0x6e7e4dbd021b75ea, "h7h6"),
    (0xc3cb757f468058f0, "h7h6"),
    (0x99c5f635c3709257, "h7h6"),
    (0x61418fd60838a6e7, "h7h6"),
    (0xa04e7f502e8c0605, "h7h6"),
    (0xe41123ea220d1986, "h7h6"),
    (0x5808096d25c26a26, "h7h6"),
    (0x112386d82c904991, "h7h6"),
    (0x35400869b3ca7519, "h7h6"),
    (0x79da21af77ba3270, "h7h6"),
    (0x03e5bcf3c26b291a, "h7h6"),
    (0xd8e80ce52fcb3fc8, "h7h6"),
    (0xc23b62a9e5a5c2c6, "h7h6"),
    (0x2a69202ea3e2ac73, "g6g7"),
    (0xdb4c30e924c49ba1, "g6g7"),
    (0x6febc2bccbfba000, "g6g7"),
    (0x3d48a2ab74e754a9, "g6g7"),
    (0x1c24095dacc118b7, "g6g7"),
    (0x94fe186ed6ec4fdc, "g6g7"),
    (0x1c81b8f78ac53fdc, "g6g7"),
    (0xd6969c2f9aa00765, "g6g7"),
    (0xb99c015e2c9bd4cf, "g6g7"),
    (0x98e9c5c2b41b0ebe, "f6e8"),
    (0xe9600b3b7d92287d, "g7h7"),
    (0x42793c3bb20039e3, "g7h7"),
    (0x6c429c079986515c, "g7h7"),
    (0x8cec8b3be6efc544, "g7h7"),
    (0x6b1910c4c5155b45, "g7h7"),
    (0xbb9e8f3b3d42f03d, "g7h7"),
    (0x19cb9d6935bd5751, "g7h7"),
    (0x919fc8b56e6361c1, "g7h7"),
    (0xba9c79349696df41, "g7h7"),
    (0x287a7db77071bbac, "g7h7"),
    (0x047245c63acb6545, "g7h7"),
    (0x18a67494a9acee27, "g7h7"),
    (0x131cf6619dd71885, "g7h7"),
    (0xd488742f1af54460, "g7h7"),
    (0x58d14e5715e2d8da, "g7h7"),
    (0xc2c1cc1fa0d90edf, "g7h7"),
    (0x76663e4a4fe6357e, "g7h7"),
    (0x24c55e5df0fac1d7, "g7h7"),
    (0x05a9f5ab28dc8dc9, "g7h7"),
    (0xf347f6efb6476826, "g7h7"),
    (0xcd08ddf65f0f7bbf, "e4f6"),
    (0x17fde65f571b3c90, "g8f8"),
    (0xbce4d15f98892d0e, "g8f8"),
    (0x48a894ad75214bd7, "g8f8"),
    (0x92df7163b30f45b1, "g8f8"),
    (0xefa9dd846e68f842, "g8f8"),
    (0x085c467b4d926643, "g8f8"),
    (0xfaefa8a2104271a8, "g8f8"),
    (0xe63b99f08325faca, "g8f8"),
    (0xed811b05b75e0c68, "g8f8"),
    (0x2a15994b307c508d, "g8f8"),
    (0x3c5c217b8a501a32, "g8f8"),
    (0x88fbd32e656f2193, "g8f8"),
    (0xda58b339da73d53a, "g8f8"),
    (0xa560044950857801, "g8f8"),
    (0x8157f1a552814278, "g8f8"),
    (0xa9f91cab08012aa2, "g8f8"),
    (0x16dfe543aa3d7fa7, "g8f8"),
    (0x8ea074559608d3f3, "g8f8"),
    (0xb834f667799d3b47, "g8f8"),
    (0xd7dcb80e2310ec73, "g8f8"),
    (0x4f8f4defb8f6dcfa, "g8f8"),
    (0xb70b340c73bee84a, "g8f8"),
    (0xb459b2214b1cb286, "g8f8"),
    (0x6dde5540e59384ee, "g8f8"),
    (0xd1c77fc7e25cf74e, "g8f8"),
    (0xddf42416fbc39a61, "g8f8"),
    (0x9b212fdbb46da558, "g8f8"),
    (0x95537098f9ae3883, "g8f8"),
    (0x0c56bae7f07b166e, "d7f8"),
    (0xb074240335392c80, "g7h7"),
    (0x46a91b0353142faa, "g7h7"),
    (0xeb1c23c1178f02b0, "g7h7"),
    (0xb112a08b927fc817, "g7h7"),
    (0x4996d9685937fca7, "g7h7"),
    (0x889929ee7f835c45, "g7h7"),
    (0xccc67554730243c6, "g7h7"),
    (0x70df5fd374cd3066, "g7h7"),
    (0xbeb172a97344a767, "g7h7"),
    (0x65bcc2bf9ee4b1b5, "g7h7"),
    (0x344b508c6f3fffab, "g7h7"),
    (0x835104c611de7790, "e4f6"),
    (0x60cd1e37699a9de1, "e4c5"),
    (0x314fabeef18a2230, "e6e8"),
    (0x9a569cee3e1833ae, "e6e8"),
    (0x6e1ad91cd3b05577, "e6e8"),
    (0xb46d3cd2159e5b11, "e6e8"),
    (0x54c32bee6af7cf09, "e6e8"),
    (0xb336b011490d5108, "e6e8"),
    (0xdc5de513b6d36f08, "e6e8"),
    (0xc089d44125b4e46a, "e6e8"),
    (0xd245a2d74eb96fbc, "e6e8"),
    (0x0ca7d4fa96ed4e2d, "e6e8"),
    (0x16c227d9f62d5310, "e6e8"),
    (0x22676432ae90391f, "e6e8"),
    (0x1aee6cca2cc10492, "e6e8"),
    (0xae499e9fc3fe3f33, "e6e8"),
    (0xfceafe887ce2cb9a, "e6e8"),
    (0x83d249f8f61466a1, "e6e8"),
    (0x4bc6048c3d3ad278, "b7a8"),
    (0x6fe86582b0b4b2c2, "e6e8"),
    (0x5036e211ed185e0c, "e6e8"),
    (0x532915eb8ded7d0b, "g8f8"),
    (0xf83022eb427f6c95, "g8f8"),
    (0xd60b82d769f9042a, "g8f8"),
    (0x36a595eb16909032, "g8f8"),
    (0xd1500e14356a0e33, "g8f8"),
    (0xbe3b5b16cab43033, "g8f8"),
    (0xa2ef6a4459d3bb51, "g8f8"),
    (0xb0231cd232de3087, "g8f8"),
    (0x6ec16affea8a1116, "g8f8"),
    (0x74a499dc8a4a0c2b, "g8f8"),
    (0x4001da37d2f76624, "g8f8"),
    (0xb9c15f3080e4d8b0, "f6d5"),
    (0x246641fa5170c234, "g5f6"),
    (0x8f7f76fa9ee2d3aa, "g5f6"),
    (0xa144d6c6b564bb15, "g5f6"),
    (0x63dbb26beda771fd, "g5f6"),
    (0x4893929e50d4feff, "g5f6"),
    (0x3a411f33a07cf2eb, "g5f6"),
    (0x09b98bf25452909f, "b7g7"),
    (0x779a33f5ba743508, "g5f6"),
    (0xe57c37765c9351e5, "g5f6"),
    (0xc9740f0716298f0c, "g5f6"),
    (0xd5a03e55854e046e, "g5f6"),
    (0xc76c48c3ee438fb8, "g5f6"),
    (0x198e3eee3617ae29, "g5f6"),
    (0xae1743e226760973, "g5f6"),
    (0x2e58954a7eba8568, "d5f6"),
    (0x67aed7ada2e497e2, "h7h6"),
    (0xccb7e0ad6d76867c, "h7h6"),
    (0x38fba55f80dee0a5, "h7h6"),
    (0xe28c409146f0eec3, "h7h6"),
    (0xce9feca7a7106bd8, "h7h6"),
    (0xe5d7cc521a63e4da, "h7h6"),
    (0x8abc9950e5bddada, "h7h6"),
    (0x9668a80276da51b8, "h7h6"),
    (0x84a4de941dd7da6e, "h7h6"),
    (0x5a46a8b9c583fbff, "h7h6"),
    (0xbd36b04a54de29f7, "h7h6"),
    (0x8282af38a88f6a6e, "h7h6"),
    (0xbe9038db2197e1c5, "g8f8"),
    (0x165440742d9a7751, "h7h6"),
    (0x4c0f10897fafb140, "h7h6"),
    (0xf8a8e2dc90908ae1, "h7h6"),
    (0xaa0b82cb2f8c7e48, "h7h6"),
    (0xd53335bba57ad373, "h7h6"),
    (0xfee296190b1dd344, "g5f6"),
    (0x0fc786de8c3be496, "g5f6"),
    (0xbb60748b6304df37, "g5f6"),
    (0xe9c3149cdc182b9e, "g5f6"),
    (0xb9309d93f90e2667, "g5f6"),
    (0x57d52fbecde360a5, "g5f6"),
    (0x83b5d3ffcc32aba4, "g5f6"),
    (0xbd3bd3f090632d57, "e8e7"),
    (0x87f91d0de08d1a90, "g7h7"),
    (0x2ce02a0d2f1f0b0e, "g7h7"),
    (0x02db8a31049963b1, "g7h7"),
    (0xc044ee9c5c5aa959, "g7h7"),
    (0xeb0cce69e129265b, "g7h7"),
    (0x6aeb53f0a7d457a8, "g7h7"),
    (0x763f62a234b3dcca, "g7h7"),
    (0x64f314345fbe571c, "g7h7"),
    (0xba11621987ea768d, "g7h7"),
    (0xac58da293dc63c32, "g7h7"),
    (0x18ff287cd2f90793, "g7h7"),
    (0x4a5c486b6de5f33a, "g7h7"),
    (0x1aafc16448f3fec3, "g7h7"),
    (0x93e248d51036bcde, "e7e8"),
    (0x3c563eb889d213bc, "e7e8"),
    (0x12de7c826a7b7d5e, "e7e8"),
    (0x44ff294fdeb46ce1, "e7e8"),
    (0x9abebaec9adae367, "g7h7"),
    (0xfde59c72882b719d, "g7h7"),
    (0xedc812f08cc1457d, "d5f6"),
    (0x451f08054de2be08, "e8a8"),
    (0x543708537a72620b, "g5f6"),
    (0xd469ad33889474f4, "e8e7"),
    (0xc08f493c3969889f, "f6d5"),
    (0x5d2857f6e8fd921b, "g5f6"),
    (0xf63160f6276f8385, "g5f6"),
    (0xd80ac0ca0ce9eb3a, "g5f6"),
    (0xd6281f6dcaa330d1, "g5f6"),
    (0x31dd8492e959aed0, "g5f6"),
    (0xb03a190bafa4df23, "g5f6"),
    (0xacee28593cc35441, "g5f6"),
    (0xbe225ecf57cedf97, "g5f6"),
    (0x60c028e28f9afe06, "g5f6"),
    (0x57168346c737d547, "c5e6"),
    (0xd83f2ce2498463a0, "g7f6"),
    (0x73261be28616723e, "g7f6"),
    (0x876a5e106bbe14e7, "g7e5"),
    (0x5d1dbbdead901a81, "g7f6"),
    (0x533f64796bdac16a, "g7f6"),
    (0xb4caff8648205f6b, "g7f6"),
    (0x352d621f0edd2e98, "g7f6"),
    (0x29f9534d9dbaa5fa, "g7f6"),
    (0x2243d1b8a9c15358, "g7f6"),
    (0xe5d753f62ee30fbd, "g7f6"),
    (0xf39eebc694cf4502, "g7f6"),
    (0x473919937bf07ea3, "g7f6"),
    (0x159a7984c4ec8a0a, "g7f6"),
    (0x6aa2cef44e1a2731, "g7f6"),
    (0x2db0f09a9b397360, "g7f6"),
    (0x9c0442d94297f9dc, "g7f6"),
    (0x148fc3410e818a8a, "g7f6"),
    (0xbb3bb52c976525e8, "g7f6"),
    (0x7f6b8af7f791c4e6, "g7f6"),
    (0xc392a2dbc0035ab5, "g7f6"),
    (0x7a8817e6969c47c9, "g7f6"),
    (0xa223ad9d212208ad, "g7f6"),
    (0x87ac8015b290836b, "g5f6"),
    (0x4e2ceb42fece50a8, "g5f6"),
    (0x768990d235b6b4b9, "g5f6"),
    (0xc22e6287da898f18, "g5f6"),
    (0x908d029065957bb1, "g5f6"),
    (0xc07e8b9f40837648, "g5f6"),
    (0xc475c5fc29ee7d78, "c5e6"),
    (0x3931ebf6300630f8, "g7h7"),
    (0x9228dcf6ff942166, "g7h7"),
    (0xbc137ccad41249d9, "g7h7"),
    (0xb231a36d12589232, "g7h7"),
    (0x55c4389231a20c33, "g7h7"),
    (0xd423a50b775f7dc0, "g7h7"),
    (0xc8f79459e438f6a2, "g7h7"),
    (0xc34d16acd0430000, "g7h7"),
    (0x04d994e257615ce5, "g7h7"),
    (0x12902cd2ed4d165a, "g7h7"),
    (0xa637de8702722dfb, "g7h7"),
    (0xf494be90bd6ed952, "g7h7"),
    (0xa467379f9878d4ab, "g7h7"),
    (0x7d0a85cd3b15aa84, "g7h7"),
    (0xf58104557703d9d2, "g7h7"),
    (0x5a357238eee776b0, "g7h7"),
    (0x9e654de38e1397be, "g7h7"),
    (0x229c65cfb98109ed, "g7h7"),
    (0x9b86d0f2ef1e1491, "g7h7"),
    (0x432d6a8958a05bf5, "g7h7"),
    (0x5300e40b5c4a6f15, "d5f6"),
    (0x6d21af31e87c429e, "g5f6"),
    (0x741763e54e0d5484, "g5f6"),
    (0x9198b855aff87b31, "g5f6"),
    (0x3e2cce38361cd453, "g5f6"),
    (0x10a48c02d5b5bab1, "g5f6"),
    (0x4685d9cf617aab0e, "g5f6"),
    (0xff9f6cf237e5b672, "g5f6"),
    (0x3c511e09f46fee27, "g5e7"),
    (0x4507010afe54f88a, "e8a8"),
    (0xee1e360a31c6e914, "e8a8"),
    (0x4c3366f7ee311eb3, "e8a8"),
    (0xc02596361a4081ab, "e8a8"),
    (0xce074991dc0a5a40, "e8a8"),
    (0x29f2d26efff0c441, "e8a8"),
    (0xa8154ff7b90db5b2, "e8a8"),
    (0xb4c17ea52a6a3ed0, "e8a8"),
    (0xa60d08334167b506, "e8a8"),
    (0x78ef7e1e99339497, "e8a8"),
    (0x1b3324f4d365130f, "e8a8"),
    (0x6ea6c62e231fde28, "e8a8"),
    (0xda01347bcc20e589, "e8a8"),
    (0x88a2546c733c1120, "e8a8"),
    (0xd851dd63562a1cd9, "e8a8"),
    (0x2e338023a1624f84, "e7f6"),
    (0x9cbe23b74d1f5fbe, "d5f6"),
    (0x2734d689805bf916, "g5f6"),
    (0xad27bb3f311924db, "f5f6"),
    (0xc7fb56c2f989490c, "g8f8"),
    (0x89adc208d7806c7b, "e8e7"),
    (0x9bf359e6cc6010ec, "h7h6"),
    (0x30ea6ee603f20172, "h7h6"),
    (0x1ed1ceda287469cd, "h7h6"),
    (0xfe7fd9e6571dfdd5, "h7h6"),
    (0x198a421974e763d4, "h7h6"),
    (0x76e1171b8b395dd4, "h7h6"),
    (0x6a352649185ed6b6, "h7h6"),
    (0x78f950df73535d60, "h7h6"),
    (0xa61b26f2ab077cf1, "h7h6"),
    (0xbc7ed5d1cbc761cc, "h7h6"),
    (0x88db963a937a0bc3, "h7h6"),
    (0x711b133dc169b557, "h7h6"),
    (0x0218cde73b402e4a, "h7h6"),
    (0x4e12023e497f62bd, "g8a8"),
    (0xa880ee5b6b7bd15e, "h7h6"),
    (0xb0529ec2112b364e, "h7h6"),
    (0x04f56c97fe140def, "h7h6"),
    (0x56560c804108f946, "h7h6"),
    (0x06a5858f641ef4bf, "h7h6"),
    (0x510f8d945018ae7e, "h5h6"),
    (0x7888d2cf50a65ba9, "g8f8"),
    (0xcc2f209abf996008, "g8f8"),
    (0x9e8c408d008594a1, "g8f8"),
    (0xce7fc98225939958, "g8f8"),
    (0x50d488a00286ac9d, "g8f8"),
    (0x1a58e37858f914c7, "g8f8"),
    (0x1420f88734a83895, "e8e7"),
    (0xa3b56c01e9ae391f, "g7h7"),
    (0x08ac5b01263c2881, "g7h7"),
    (0x2697fb3d0dba403e, "g7h7"),
    (0xc639ec0172d3d426, "g7h7"),
    (0x21cc77fe51294a27, "g7h7"),
    (0x4ea722fcaef77427, "g7h7"),
    (0x527313ae3d90ff45, "g7h7"),
    (0x40bf6538569d7493, "g7h7"),
    (0x9e5d13158ec95502, "g7h7"),
    (0x8438e036ee09483f, "g7h7"),
    (0xb09da3ddb6b42230, "g7h7"),
    (0xc88c94cc20e9e59c, "g7h7"),
    (0xbb8f4a16dac07e81, "g7h7"),
    (0x8814ab2534e51fbd, "g7h7"),
    (0x3cb35970dbda241c, "g7h7"),
    (0x6e10396764c6d0b5, "g7h7"),
    (0x3ee3b06841d0dd4c, "g7h7"),
    (0xb7ae39d919159f51, "e7e8"),
    (0x181a4fb480f13033, "e7e8"),
    (0x36920d8e63585ed1, "e7e8"),
    (0x60b35843d7974f6e, "e7e8"),
    (0xbef2cbe093f9c0e8, "g7h7"),
    (0x688e44987546818b, "g7h7"),
    (0xc98463fc85e266f2, "d7f8"),
    (0x93d0e2d26409616a, "c5e6"),
    (0xe1e202a5d5164852, "e6e8"),
    (0xe4f6a21509341b4d, "g6g7"),
    (0xe80a8f3e49726d47, "g6g7"),
    (0x8f12eba3f0245521, "g6g7"),
    (0x33ebc38fc7b6cb72, "g6g7"),
    (0xa6cde7431b08e43a, "g6g7"),
    (0x3bd6df5465670597, "g6g7"),
    (0x5f24e7f5d709bad4, "g6g7"),
    (0xad1412472f8a440b, "g6g7"),
    (0xa89f5c7d16625d69, "g6g7"),
    (0x501b259edd2a69d9, "g6g7"),
    (0xce91186847861afe, "g6g7"),
    (0x8ace44d24b07057d, "g6g7"),
    (0x36d76e554cc876dd, "g6g7"),
    (0x48899b1249dd4cea, "g6g7"),
    (0x93842b04a47d5a38, "g6g7"),
    (0x9df67447e9bec7e3, "g6g7"),
    (0x1f19df97e4ba5741, "d7c5"),
    (0xc78b887ffc3b884f, "e6e7"),
    (0x6c92bf7f33a999d1, "e6e7"),
    (0x42a91f43182ff16e, "e6e7"),
    (0xa207087f67466576, "e6e7"),
    (0x45f2938044bcfb77, "e6e7"),
    (0xa275e5ebda5816d5, "f5f6"),
    (0x63a8541067710e2f, "f5f6"),
    (0xeb7592fcac31fd55, "f5f6"),
    (0x2a99c682bb62c577, "e6e7"),
    (0x364df7d028054e15, "e6e7"),
    (0x248181464308c5c3, "e6e7"),
    (0xfa63f76b9b5ce452, "e6e7"),
    (0xd57296554de9a180, "e6e7"),
    (0xec2a4f5b2170aeed, "e6e7"),
    (0x588dbd0ece4f954c, "e6e7"),
    (0x0a2edd19715361e5, "e6e7"),
    (0x2b4276efa9752dfb, "e6e7"),
    (0xe1221abcfa461ae7, "e6e7"),
    (0xd390dda70c802e01, "e6e7"),
    (0x7c24abca95648163, "e6e7"),
    (0x52ace9f076cdef81, "e6e7"),
    (0xa8a6f8cfa231df63, "e6e7"),
    (0xda8f648e25096266, "b7a7"),
    (0xbd970900949de342, "e6e7"),
    (0x1906c14927c789b8, "e6e7"),
    (0x5b9b74c85d36098d, "b7a7"),
    (0xb0475e75b7813b7c, "c5d7"),
    (0x378464b625df3e8f, "b7g7"),
    (0xccc5cfda0fc4a21d, "e6e7"),
    (0x78623d8fe0fb99bc, "e6e7"),
    (0x2ac15d985fe76d15, "e6e7"),
    (0xc1cd9a3dd4f21617, "e6e7"),
    (0xf37f5d26223422f1, "e6e7"),
    (0x5ccb2b4bbbd08d93, "e6e7"),
    (0x989b1490db246c9d, "e6e7"),
    (0x24623cbcecb6f2ce, "e6e7"),
    (0xfa60e40f0bbd6e96, "b7a8"),
    (0xd6f3450a5ae6e769, "e8e7"),
    (0x7a91a5e333baa2b4, "e8e7"),
    (0xf8c8e53671608fd6, "e8e7"),
    (0x1866f20a0e091bce, "e8e7"),
    (0xd726db47e1507dcc, "e8e7"),
    (0xf0ddec25e25ca232, "e8e7"),
    (0x6003440c86eea6dc, "e8e7"),
    (0xc87294b1b42c7bd0, "e8e7"),
    (0xf6dc2883928496d5, "e8e7"),
    (0x833a1b1e8ae2c1be, "e8e7"),
    (0x6a0552a15cd8b5bc, "e8e7"),
    (0x45d8c3a435c1845e, "e8e7"),
    (0xfd23d950005403f5, "f5f6"),
    (0xc38d656226fceef0, "f5f6"),
    (0x170cadd7d574fe4e, "f5f6"),
    (0xd2fe4855694d3217, "f5f6"),
    (0x0068c6f6a89409e2, "f5f6"),
    (0x90f83cf7d22dbbcf, "e8e7"),
    (0x8c2c0da5414a30ad, "e8e7"),
    (0xcc0e901441594ac3, "e8e7"),
    (0x40020d1ef2139aea, "e8e7"),
    (0x04a55e09232121ed, "e8e7"),
    (0xe2ec477ba700ebf4, "e8e7"),
    (0xb04f276c181c1f5d, "e8e7"),
    (0x13b4557ff9b3cf1a, "c5e6"),
    (0x510e281fd0adaf6d, "a8d8"),
    (0x9cb279ac8640499d, "f6h5"),
    (0x30d09945ef1c0c40, "f6h5"),
    (0xb289d990adc62122, "f6h5"),
    (0x5227ceacd2afb53a, "f6h5"),
    (0x9d67e7e13df6d338, "f6h5"),
    (0xba9cd0833efa0cc6, "f6h5"),
    (0x2a4278aa5a480828, "f6h5"),
    (0x331401f19cc406bd, "e8e7"),
    (0x0dbabdc3ba6cebb8, "e8e7"),
    (0xc97b27b856446f4a, "f6h5"),
    (0x9163c7e17430c8d1, "e8e7"),
    (0xbebe56e41d29f933, "e8e7"),
    (0xdab900510e8b153b, "f6h5"),
    (0xc66d31039dec9e59, "f6h5"),
    (0x864facb29dffe437, "f6h5"),
    (0x016044fb1631e483, "f6h5"),
    (0x4ee462afff878f19, "f6h5"),
    (0xa8ad7bdd7ba64500, "f6h5"),
    (0xfa0e1bcac4bab1a9, "f6h5"),
    (0x59f569d9251561ee, "c5e6"),
    (0x1102dc6f4fafcaab, "f6h5"),
    (0xa6db4d20185b3c0f, "d8e7"),
    (0xe36bcf4cd18a92c6, "f6h5"),
    (0xb9b5857f61bbd934, "f6h5"),
    (0xedaa8b1f3396f08e, "e8e7"),
    (0x5bf52f546480727a, "d8e7"),
    (0xd355e63ea621ab58, "d7f8"),
    (0x4a2a507041cc2002, "f6h7"),
    (0x9982a1dbe19392fc, "e8f8"),
    (0xf3f4b9d9bd1d77c0, "e8e7"),
    (0xa7ebb7b9ef305e7a, "e8e7"),
    (0xa093ba144c680f17, "d7f8"),
    (0x00de090012136aab, "e6e7"),
    (0xacbce9e97b4f2f76, "e6e7"),
    (0x2ee5a93c39950214, "e6e7"),
    (0xce4bbe0046fc960c, "e6e7"),
    (0x010b974da9a5f00e, "e6e7"),
    (0x26f0a02faaa92ff0, "e6e7"),
    (0xb62e0806ce1b2b1e, "e6e7"),
    (0x1e5fd8bbfcd9f612, "e6e7"),
    (0x20f16489da711b17, "e6e7"),
    (0x55175714c2174c7c, "e6e7"),
    (0xbc281eab142d387e, "e6e7"),
    (0x93f58fae7d34099c, "e6e7"),
    (0x2b0e955a48a18e37, "e6f6"),
    (0x15a029686e096332, "e6f6"),
    (0xc121e1dd9d81738c, "e6f6"),
    (0x04d3045f21b8bfd5, "e6f6"),
    (0x75d26fc6186f579b, "e6f6"),
    (0xd6458afce0618420, "e6f6"),
    (0x46d570fd9ad8360d, "e6e7"),
    (0x5a0141af09bfbd6f, "e6e7"),
    (0x1a23dc1e09acc701, "e6e7"),
    (0x962f4114bae61728, "e6e7"),
    (0xf4045966030f3665, "e6e7"),
    (0xd28812036bd4ac2f, "e6e7"),
    (0x34c10b71eff56636, "e6e7"),
    (0x66626b6650e9929f, "e6e7"),
    (0xb9920563533c7dbd, "c6e8"),
    (0x57082b31efb39ee9, "c6e8"),
    (0xb2a097d1e2b3fcd7, "e6e7"),
    (0x8ac749bd79b1bbda, "c6e8"),
    (0x8b0598a6c5777519, "e6e7"),
    (0xdf1a96c6975a5ca3, "e6e7"),
    (0x7aec38ffd3e89b9a, "h7f7"),
    (0xa21cfd5f5c9ecec1, "e6e7"),
    (0x2c62d5a891799d78, "e6e7"),
    (0x932d5e89f72d7721, "d3c4"),
    (0xbd16feb5dcab1f9e, "c6d5"),
    (0x5db8e989a3c28b86, "c6d5"),
    (0x92f8c0c44c9bed84, "e7g7"),
    (0xb503f7a64f97327a, "c6d5"),
    (0x25dd5f8f2b253694, "c6d5"),
    (0x8dac8f3219e7eb98, "c6d5"),
    (0x9cc90d7f90afa65f, "c6d5"),
    (0xc6e4009d272951f6, "c6d5"),
    (0x2fdb4922f11325f4, "c6d5"),
    (0x0006d827980a1416, "c6d5"),
    (0xb8fdc2d3ad9f93bd, "c6d5"),
    (0xa998409e24d7de7a, "c6d5"),
    (0x19b5013be06fcec8, "h7g7"),
    (0x972053d6c486a25f, "c6d5"),
    (0xb3dc8b990c15401d, "c6d5"),
    (0x28ad2c49b85a7010, "e7g7"),
    (0x3c3d5c9b19e1da40, "c6d5"),
    (0xd52627747fe62b87, "c6d5"),
    (0xc9f21626ec81a0e5, "c6d5"),
    (0x89d08b97ec92da8b, "c6d5"),
    (0x05dc169d5fd80aa2, "c6d5"),
    (0x1dc18346b8958121, "c6d5"),
    (0x417b458a8eeab1a5, "c6d5"),
    (0xa7325cf80acb7bbc, "c6d5"),
    (0xf5913cefb5d78f15, "c6d5"),
    (0x18f6cf2f20496893, "c6d5"),
    (0x4ce9c14f72644129, "c6d5"),
    (0x20543e0fb4946c28, "e5e6"),
    (0x32c4c8c0d5fd81a8, "c5e6"),
    (0xe29ce38969bfae2e, "g6g7"),
    (0x006b6cd69d6a8ef6, "d7b6"),
    (0xdfac59025f082b21, "c5d7"),
    (0x5df519d71dd20643, "c5d7"),
    (0xf424bb9be5f7c207, "c5d7"),
    (0x721b27a68de2f459, "c5d7"),
    (0x55e010c48eee2ba7, "c5d7"),
    (0xc53eb8edea5c2f49, "c5d7"),
    (0x6d4f6850d89ef245, "b6d7"),
    (0x53e1d462fe361f40, "c5d7"),
    (0x2607e7ffe650482b, "c5d7"),
    (0xcf38ae40306a3c29, "b6d7"),
    (0xe0e53f4559730dcb, "c5d7"),
    (0x581e25b16ce68a60, "b6d7"),
    (0x66b099834a4e6765, "c5d7"),
    (0x8cdc5d59ae117afa, "c5d7"),
    (0x77c3b4b405ffbb82, "c5d7"),
    (0x42195eeb755bf423, "a8d8"),
    (0x0f4382f11632cbca, "g5f6"),
    (0x8d1ac22454e8e6a8, "g5f6"),
    (0x24cb6068accd22ec, "g5f6"),
    (0xa2f4fc55c4d814b2, "g5f6"),
    (0x850fcb37c7d4cb4c, "g5f6"),
    (0xe91a66a7a03f5e1b, "d8f6"),
    (0xabb3cbfb11a00cfb, "d8f6"),
    (0x830e0f91b70cffab, "g5f6"),
    (0xf6e83c0caf6aa8c0, "g5f6"),
    (0x09c40debf954c297, "d8f6"),
    (0x300ae4b61049ed20, "g5f6"),
    (0x0956fc47d8f5bd23, "g5f6"),
    (0x55ba67d4781c940d, "d8f6"),
    (0xc092cc6b2e4610b5, "g5f6"),
    (0xa1d882223c26d1b3, "g5f6"),
    (0xe52a1be5f7a5d2b1, "g5f6"),
    (0xf9fe2ab764c259d3, "g5f6"),
    (0x5304cae7e78cac51, "g5f6"),
    (0x0b3d2663c04cfeb5, "g5f6"),
    (0x2dcdbfd730d67817, "g5f6"),
    (0x7177791b06a94893, "g5f6"),
    (0x973e60698288828a, "g5f6"),
    (0xc59d007e3d947623, "g5f6"),
    (0x121fa8cc7b29dfd2, "d8e7"),
    (0x6666726ddc3ba664, "g5f6"),
    (0x240ef62c9077eedd, "g5f6"),
    (0x3cd3b1670f2f34df, "g5e7"),
    (0xcee00a62cdf7a32f, "g5f6"),
    (0x2e91c7dbb6810d21, "g5f6"),
    (0x16128416cdbe2f73, "f5f6"),
    (0x2c06e1f97763c307, "g5f6"),
    (0x7ff48312fb462cba, "f5f6"),
    (0x5f56c690063ef018, "d7f6"),
    (0xa5553a17c4268077, "c5d7"),
    (0x35c5c016be9f325a, "c5d7"),
    (0x2911f1442df8b938, "c5d7"),
    (0x69336cf52debc356, "b6d7"),
    (0xdbd2fd9089761e5e, "c5d7"),
    (0xfd22642479ec98fc, "c5d7"),
    (0xa198a2e84f93a878, "c5d7"),
    (0x47d1bb9acbb26261, "c5d7"),
    (0x1572db8d74ae96c8, "c5d7"),
    (0xe341ac386abd5e78, "c5e6"),
    (0xd185ed0382bf43ab, "c5d7"),
    (0xcedcaf525fd5e205, "c5d7"),
    (0xda6f0312036729a7, "c5d7"),
    (0xc74efe2fbc8331c1, "a8c8"),
    (0x50fcf9f595bfe5f4, "d7f7"),
    (0xe0000af7444f5b20, "d7f7"),
    (0xd2a5b920d765c896, "d7f7"),
    (0x7b741b6c2f400cd2, "d7f7"),
    (0xfd4b875147553a8c, "d7f7"),
    (0xdab0b0334459e572, "d7f7"),
    (0x4a6e181a20ebe19c, "d7f7"),
    (0x11b14125ce02ce08, "d7f7"),
    (0xdcb174953481d195, "d7f7"),
    (0xa95747082ce786fe, "d7f7"),
    (0xb3c6873526f60064, "d7f7"),
    (0x6fb59fb293c4c31e, "d7f7"),
    (0x24e00cc47a7ab62d, "d7f7"),
    (0xe9e0397480f9a9b0, "d7f7"),
    (0x038cfdae64a6b42f, "d7f7"),
    (0xf8931443cf487557, "d7f7"),
    (0x866ecd184ec1d2e0, "d7f7"),
    (0x2a059ae00e914ea2, "d7f7"),
    (0xba9560e17428fc8f, "d7f7"),
    (0xa64151b3e74f77ed, "d7f7"),
    (0xe663cc02e75c0d83, "d7f7"),
    (0x54825d6743c1d08b, "d7f7"),
    (0x7272c4d3b35b5629, "d7f7"),
    (0x2ec8021f852466ad, "d7f7"),
    (0xc8811b6d0105acb4, "d7f7"),
    (0x9a227b7abe19581d, "d7f7"),
    (0x965944656ee1acbf, "c5e6"),
    (0xd999e5cf1b183080, "d7f7"),
    (0x8d86ebaf4935193a, "d7f7"),
    (0x1ea04edf9bcb07ad, "b6d5"),
    (0x73b99afdf4eeed39, "d7a7"),
    (0x204bf81678cb0284, "d7a7"),
    (0x00e9bd9485b3de26, "d7g7"),
    (0xa129287926f116c1, "c5d7"),
    (0x7f041cee648416f2, "c5d7"),
    (0x418a1ce138d59001, "c5d7"),
    (0xfe7e1c28ffbbedca, "c5d7"),
    (0x81a649f74ccecafa, "c5d7"),
    (0xcac670d3fa729f0a, "c5d7"),
    (0xbb86aa939850524e, "c5d7"),
    (0x83e174ff03521543, "c5d7"),
    (0x56c94538d1affe55, "c5d7"),
    (0x02d64b588382d7ef, "c5d7"),
    (0x98162f0043bc07f2, "b6d5"),
    (0xf6a4f2dc7dfde8de, "a8a7"),
    (0xaf1b58e1b27ccc51, "a8a7"),
    (0x8fb91d634f0410f3, "a8c8"),
    (0xa32a66bad0c280f6, "e8d8"),
    (0x2173266f9218ad94, "e8d8"),
    (0x88a284236a3d69d0, "e8d8"),
    (0x0e9d181e02285f8e, "e8d8"),
    (0x29662f7c01248070, "e8d8"),
    (0xb9b887556596849e, "e8d8"),
    (0x11c957e857545992, "e8d8"),
    (0x2f67ebda71fcb497, "e8d8"),
    (0x5a81d847699ae3fc, "e8d8"),
    (0xb3be91f8bfa097fe, "e8d8"),
    (0x9c6300fdd6b9a61c, "e8d8"),
    (0x24981a09e32c21b7, "e8d8"),
    (0x1a36a63bc584ccb2, "e8d8"),
    (0xf05a62e121dbd12d, "e8d8"),
    (0x0b458b0c8a351055, "e8d8"),
    (0x3e9f6153fa915ff4, "e8d8"),
    (0x4943ffae3155998d, "e8d8"),
    (0x5597cefca23212ef, "e8d8"),
    (0x15b5534da2216881, "e8d8"),
    (0xa754c22806bcb589, "e8d8"),
    (0x81a45b9cf626332b, "e8d8"),
    (0x7ed566b02c61c5c5, "c8b7"),
    (0x603da73891c3c3a4, "h7g7"),
    (0xd0c1543a40337d70, "h7g7"),
    (0xe264e7edd319eec6, "h7g7"),
    (0x4bb545a12b3c2a82, "h7g7"),
    (0xcd8ad99c43291cdc, "h7g7"),
    (0xea71eefe4025c322, "h7g7"),
    (0x7aaf46d72497c7cc, "h7g7"),
    (0x21701fe8ca7ee858, "h7g7"),
    (0xec702a5830fdf7c5, "h7g7"),
    (0x999619c5289ba0ae, "h7g7"),
    (0x8307d9f8228a2634, "h7g7"),
    (0x5f74c17f97b8e54e, "h7g7"),
    (0x142152097e06907d, "h7g7"),
    (0xd92167b984858fe0, "h7g7"),
    (0x334da36360da927f, "h7g7"),
    (0xc8524a8ecb345307, "h7g7"),
    (0xfd88a0d1bb901ca6, "h7g7"),
    (0x8a543e2c7054dadf, "h7g7"),
    (0x96800f7ee33351bd, "h7g7"),
    (0xd6a292cfe3202bd3, "h7g7"),
    (0x644303aa47bdf6db, "h7g7"),
    (0x4e6c2eb0b14b740f, "h7g7"),
    (0xd670232ae6f8f807, "h7g7"),
    (0xe5126e8a645885cc, "h7h8"),
    (0x1e095cd2815840fd, "h7g7"),
    (0xf84045a005798ae4, "h7g7"),
    (0xaae325b7ba657e4d, "h7g7"),
    (0xe958bb021f6416d0, "h7g7"),
    (0xb742962938d59d99, "b7f7"),
    (0x23b3dc8a4e8704f6, "b7f7"),
    (0xd1b9013228dc01b5, "h7g7"),
    (0xc36abcedead0ffeb, "e6e7"),
    (0xbdeefad1660d5f76, "b7g7"),
    (0xdd1e9d50c05903af, "e8d8"),
    (0x3b5784224478c9b6, "e8d8"),
    (0x69f4e435fb643d1f, "e8d8"),
    (0x41b3e2641e559a15, "e8d8"),
    (0xb25a90ead01f49d2, "e8d8"),
    (0x24a91c3c97d602ec, "e8d8"),
    (0xbbc8c19733499a16, "e8d8"),
    (0x9cc62368d12ac534, "d6d7"),
    (0x90bfd17f14674cf4, "e8d8"),
    (0x12e691aa56bd6196, "e8d8"),
    (0xbb3733e6ae98a5d2, "e8d8"),
    (0x3d08afdbc68d938c, "e8d8"),
    (0x1af398b9c5814c72, "e8d8"),
    (0x8a2d3090a133489c, "e8d8"),
    (0xd1f269af4fda6708, "e8d8"),
    (0x1cf25c1fb5597895, "e8d8"),
    (0x69146f82ad3f2ffe, "e8d8"),
    (0x7385afbfa72ea964, "e8d8"),
    (0xaff6b738121c6a1e, "e8d8"),
    (0xe4a3244efba21f2d, "e8d8"),
    (0x29a311fe012100b0, "e8d8"),
    (0xc3cfd524e57e1d2f, "e8d8"),
    (0x38d03cc94e90dc57, "e8d8"),
    (0x462de592cf197be0, "e8d8"),
    (0x7ad6486bf5f0558f, "e8d8"),
    (0x660279396697deed, "e8d8"),
    (0x2620e4886684a483, "e8d8"),
    (0x94c175edc219798b, "e8d8"),
    (0xb231ec593283ff29, "e8d8"),
    (0xbeee58f734effb5f, "d7d8n"),
    (0x08dc2e4f2442ae48, "e6e5"),
    (0x69b569d8046f130e, "c6d5"),
    (0xc064cb94fc4ad74a, "c6d5"),
    (0x6557e5a03b799375, "e7g7"),
    (0x61a060cb97533eea, "c6d5"),
    (0xf17ec8e2f3e13a04, "c6d5"),
    (0x81822297aea44805, "c6d5"),
    (0x124797f0ffed5d66, "c6d5"),
    (0xd4a54f4a40ce1886, "c6d5"),
    (0xb4d36f761adc3020, "c6d5"),
    (0xf3fb9a392f7ccf79, "c8e6"),
    (0x4383c4bb1c42aecf, "c6d5"),
    (0x907aa81e25c9a366, "c6d5"),
    (0x3d7e1de09dcb0978, "c6d5"),
    (0x0185b019a7222717, "c6d5"),
    (0x1d51814b3445ac75, "c6d5"),
    (0x5d731cfa3456d61b, "c6d5"),
    (0xef928d9f90cb0b13, "c6d5"),
    (0xc5bda085663d89c7, "c6d5"),
    (0x6ec3e0bfb32e7804, "c6d5"),
    (0x95d8d2e7562ebd35, "c6d5"),
    (0x7391cb95d20f772c, "c6d5"),
    (0x023e198bc235f1e4, "c6d5"),
    (0x06a13b3921052ff5, "c6d5"),
    (0x3aa7902c4ce6c81c, "c6d5"),
    (0x6eb89e4c1ecbe1a6, "c6d5"),
    (0xa26771f4ec6d5bcd, "c6d5"),
    (0x5a2bc4175ccfefa3, "c6d5"),
    (0x42be1193483aa0d0, "c5e6"),
    (0x28cb13f4f2f045d6, "e7a7"),
    (0x3c3a57afc4e7004d, "d8f7"),
    (0x8a856e9a6698832a, "e6d6"),
    (0x50e8c8fd61ba5190, "c6d5"),
    (0x995071a8b02e70ed, "c6d5"),
    (0xdaa4813bb05c5116, "c6d5"),
    (0x4a7a2912d4ee55f8, "c6d5"),
    (0xc58fd181f2461985, "c6d5"),
    (0x6d82ec7b34cab668, "c6d5"),
    (0xa9437600d8e2329a, "c6d5"),
    (0x67f817911ab2d7e9, "c6d5"),
    (0xde86075c938fa4e3, "c6d5"),
    (0xba8151e9802d48eb, "c6d5"),
    (0xa65560bb134ac389, "c6d5"),
    (0xe677fd0a1359b9e7, "c6d5"),
    (0x54966c6fb7c464ef, "c6d5"),
    (0x7eb941754132e63b, "c6d5"),
    (0x64e0a8a9606fc461, "c6d5"),
    (0x81ea9f15724c488c, "c6d5"),
    (0x9a364a724a1cec79, "c6d5"),
    (0xb26254bddfea8c12, "c8d7"),
    (0xe9b67248d1ec1814, "d7f7"),
    (0x200ecb1d00783969, "d7f7"),
    (0x63fa3b8e000a1892, "d7f7"),
    (0xf32493a764b81c7c, "d7f7"),
    (0xcdf6c2d2b65e8398, "d7f7"),
    (0x65fbff2870d22c75, "d7f7"),
    (0x101dccb568b47b1e, "d7f7"),
    (0x6f8104c25eaa4df4, "d7f7"),
    (0xd6ff140fd7973efe, "d7f7"),
    (0xf8a78f330226fbbd, "d7f7"),
    (0x50aab2c9c4aa5450, "d7f7"),
    (0xbac6761320f549cf, "d7f7"),
    (0x41d99ffe8b1b88b7, "d7f7"),
    (0x3f2446a50a922f00, "d7f7"),
    (0x03dfeb5c307b016f, "d7f7"),
    (0x1f0bda0ea31c8a0d, "d7f7"),
    (0x5f2947bfa30ff063, "d7f7"),
    (0xedc8d6da07922d6b, "d7f7"),
    (0xc7e7fbc0f164afbf, "e8f8"),
    (0x0b413a01799a09c0, "e8f8"),
    (0x6c99bbfa24775e7c, "e8f8"),
    (0x38b425a0c21a0108, "d7f7"),
    (0x2368f0c7fa4aa5fd, "d7f7"),
    (0x60d36e725f4bcd60, "d7f7"),
    (0x34cc60120d66e4da, "d7f7"),
    (0xa03d2ab17b347db5, "d7f7"),
    (0x5832d44268f3da05, "d7a7"),
    (0x40e44ad6df6386a8, "e6e7"),
    (0x2a9148b165a963ae, "e6e7"),
    (0x3e600cea53be2635, "d8f7"),
    (0x450440442523348d, "c6d5"),
    (0xb6ed32caeb69e74a, "c6d5"),
    (0x201ebe1caca0ac74, "f6d5"),
    (0xb2a1d5fb4dc57d6c, "d5f6"),
    (0x7b196cae9c515c11, "d5f6"),
    (0x38ed9c3d9c237dea, "d5f6"),
    (0xa8333414f8917904, "d5f6"),
    (0x96e165612a77e6e0, "d5f6"),
    (0x3eec589becfb490d, "d5f6"),
    (0x4b0a6b06f49d1e66, "d5f6"),
    (0x3496a371c283288c, "d5f6"),
    (0x8de8b3bc4bbe5b86, "d5f6"),
    (0x58c84cefac526417, "d5f6"),
    (0x441c7dbd3f35ef75, "d5f6"),
    (0x043ee00c3f26951b, "d5f6"),
    (0xb6df71699bbb4813, "d5f6"),
    (0xc854156939cddf88, "d5f6"),
    (0xcecfbf14e0f1fd3a, "d5f6"),
    (0x9a5e12cf3f164e7b, "c7g7"),
    (0x2f5deecfe34d96df, "d5f6"),
    (0x365b082dd9f34b16, "c7g7"),
    (0x9cf05c736d4dcac7, "d5f6"),
    (0x50569db2e5b36cb8, "d5f6"),
    (0x378e1c49b85e3b04, "d5f6"),
    (0x63a382135e336470, "d5f6"),
    (0x787f57746663c085, "d5f6"),
    (0x3bc4c9c1c362a818, "d5f6"),
    (0x6fdbc7a1914f81a2, "d5f6"),
    (0xa337020faa06ef37, "d5e7"),
    (0xfb2a8d02e71d18cd, "d5f6"),
    (0x6e9c932212b6e613, "d5f6"),
    (0x876f9617da582860, "d5f6"),
    (0xf43f61a50d0fca4c, "c7g7"),
    (0xb1cae243c6ca12f4, "d5f6"),
    (0xfbf855fe61d783ea, "d5f6"),
    (0xd2f2a5f42a150c01, "d5f6"),
    (0x3434b323899d2956, "d5f6"),
    (0x48018f8ecf6f0c6d, "d6f6"),
    (0x1bf3ed65434ae3d0, "f5f6"),
    (0x7186ef02f98006d6, "d5f6"),
    (0x6577ab59cf97434d, "d8f7"),
    (0x186b266e0b59ec61, "c6d5"),
    (0xb61228902476c024, "e8e7"),
    (0x524a5faa117e39b3, "f7g7"),
    (0x11beaf39110c1848, "f7g7"),
    (0x8160071075be1ca6, "f7g7"),
    (0xbfb25665a7588342, "f7g7"),
    (0x387455e0ce348c6d, "f7g7"),
    (0x6259580279b27bc4, "f7g7"),
    (0x1dc590754fac4d2e, "f7g7"),
    (0xa4bb80b8c6913e24, "f7g7"),
    (0x8ae31b841320fb67, "f7g7"),
    (0x0d2518017a4cf448, "f7g7"),
    (0x83e555cba923e9db, "f7g7"),
    (0x339d0b499a1d886d, "f7g7"),
    (0x298cdf694559670e, "f7g7"),
    (0x4d60d2121b942fda, "f7g7"),
    (0x26d4aa5dd60a7b7f, "f7g7"),
    (0x719b7feb217d01b5, "f7g7"),
    (0x6d4f4eb9b21a8ad7, "f7g7"),
    (0x2d6dd308b209f0b9, "f7g7"),
    (0x9f8c426d16942db1, "f7g7"),
    (0xb5a36f77e062af65, "e7e8"),
    (0x1edd2f4d35715ea6, "e7e8"),
    (0x4af0b117d31c01d2, "f7g7"),
    (0x512c6470eb4ca527, "f7g7"),
    (0x1297fac54e4dcdba, "f7g7"),
    (0x4688f4a51c60e400, "f7g7"),
    (0xd279be066a327d6f, "f7g7"),
    (0x2fab413d4dab747d, "c5e6"),
    (0x58d5dc0674af6374, "f7g7"),
    (0x4c24985d42b826ef, "f7h7"),
    (0x39bb81798c69954e, "c6d5"),
    (0x0b5a90d8d9f4d82a, "c6d5"),
    (0xd98dd4c7ef1d84e4, "c6d5"),
    (0x8d92daa7bd30ad5e, "c6d5"),
    (0x19639004cb623431, "c6d5"),
    (0x1b6f3b6e175ef308, "c6d5"),
    (0x489d59859b7b1cb5, "c5e6"),
    (0x93cff204d5ff2a2a, "c5e4"),
    (0xd734691b9c445735, "g5h5"),
    (0xf4e5495aac476f33, "g5h5"),
    (0xe83178083f20e451, "g5h5"),
    (0x27c690128061a1a3, "g5h5"),
    (0x1af274dc9bae4337, "g5h5"),
    (0xcc165c7f6e01505a, "e8h8"),
    (0x2a84b01a4c05e3b9, "e8h8"),
    (0xcf8e87a65e266f54, "g5h5"),
    (0xd45252c16676cba1, "g5h5"),
    (0xfc064c0ef380abca, "g5h5"),
    (0x0b6058f709491355, "d6d7"),
    (0xf8892a79c703c092, "d6d7"),
    (0x6e7aa6af80ca8bac, "d6d7"),
    (0x560f3edd2733cbb9, "e8e7"),
    (0xf8763023081ce7fc, "g5h5"),
    (0x77df99caa003b296, "g5h5"),
    (0xe082e9e01b8fda67, "f2g4"),
    (0x4c2e06d3f0a302c9, "g3h5"),
    (0xddabeab7f9950df2, "c6d5"),
    (0x838de909883071cd, "d8f7"),
    (0x873eb65fe3e86fb1, "d8f7"),
    (0x2354ccd69ebd476e, "b3b4"),
    (0x4e8688387d9018a5, "c5e4"),
    (0xde80e37d78add283, "b6d5"),
    (0xd36cad20041e0a3e, "e7g7"),
    (0x7c22361a19e803be, "e7g7"),
    (0x454aa9e22d0f4d17, "e7g7"),
    (0xa3d845870f0bfef4, "e8f8"),
    (0xe9e4de391e45e85c, "e7g7"),
    (0x714b25de8afd9d9a, "e7g7"),
    (0x5d0ea75c2578d6ec, "e7g7"),
    (0xdc149d50db5fda43, "e4d6"),
    (0x8fa5b4abf41f2620, "g3h5"),
    (0x54f71f2aba9b10bf, "e7a7"),
    (0x40065b718c8c5524, "d8f7"),
    (0x365e8b671b31f665, "b6d5"),
    (0x0560efdfc87e3bb6, "e7g7"),
    (0xf4d98ed24a053ea5, "e7g7"),
    (0x43ffe392ce93d9db, "e7g7"),
    (0x620658eae1a98623, "e7g7"),
    (0xaf5f8d8db46fb4a8, "e7g7"),
    (0xccf6d497c130f1d5, "e7g7"),
    (0x875386be81aac9c0, "e7g7"),
    (0x9d0a6f62a0f7eb9a, "e8f8"),
    (0xd736f4dcb1b9fd32, "e7g7"),
    (0x317fedae3598372b, "e7g7"),
    (0x63dc8db98a84c382, "e7g7"),
    (0xfff5586b5bb48071, "e7g7"),
    (0xe0eb037d67d5b0a2, "e7g7"),
    (0xe08957cf0bfa1bca, "e7g7"),
    (0xe2c6b7b574a3cf2d, "e4d6"),
    (0xb1779e4e5be3334e, "g3h5"),
    (0x6a2535cf156705d1, "e7a7"),
    (0x7ed471942370404a, "d8f7"),
    (0x0c1b9ade039a54d2, "f6d7"),
    (0x1ac96296eec24121, "e4f6"),
    (0xf202aada5c000049, "d7f6"),
    (0x13cb61c9001e41ed, "e4f6"),
    (0x298e707018b5e35a, "e4f6"),
    (0x50f266293193912d, "e4f6"),
    (0x152d31608940905f, "e4f6"),
    (0xcf8b688c74eb0020, "e4f6"),
    (0xd35f59dee78c8b42, "e4f6"),
    (0xc89f7995970587a5, "e4f6"),
    (0x2ed660e713244dbc, "e4f6"),
    (0x5d19ab06741ef50b, "d7f6"),
    (0x8caa32ab576f386b, "e4f6"),
    (0x807d068bd65513e7, "e4f6"),
    (0xb29c172a83c85e83, "e4f6"),
    (0x3576105ae3ea6b92, "e4d6"),
    (0x2da08ece547a373f, "g3h5"),
    (0x758cb88633db7f46, "e4f6"),
    (0x617dfcdd05cc3add, "d8f7"),
    (0x4d13847f787766c6, "f6g4"),
    (0x5e10bc20dd187330, "e4f6"),
    (0x3a4f8146b27ee773, "g4f6"),
    (0x5df7403ea8382316, "e4f6"),
    (0x1e1301fa3155b128, "e4f6"),
    (0xb6ced43abe8457d6, "e4f6"),
    (0xcd3b3a8eb23f1007, "e4f6"),
    (0x8b52b63a47313231, "e4f6"),
    (0x97868768d456b953, "e4f6"),
    (0x20e8d0f8979510ff, "e4f6"),
    (0x5896ab04ee3fd047, "e4f6"),
    (0x8c46a723a4dfb5b4, "e4f6"),
    (0x6a0fbe5120fe7fad, "e4f6"),
    (0x38acde469fe28b04, "e4f6"),
    (0x52c120c62988de73, "g4f6"),
    (0xe79ed470f0dd53f0, "e4f6"),
    (0x1477a6fe3e978037, "e4f6"),
    (0x82842a28795ecb09, "e4f6"),
    (0x6cf6c278620d6490, "e4f6"),
    (0xa5af154205c674c0, "e4f6"),
    (0x9b21154d5997f233, "e4f6"),
    (0xa9c004ec0c0abf57, "e4f6"),
    (0x43f6fac30d5bf4fc, "e4d6"),
    (0x5b206457bacba851, "g5f6"),
    (0x3155663000014d57, "e4f6"),
    (0x25a4226b361608cc, "d8f7"),
    (0xd6b318b008159e83, "f6d5"),
    (0x933ac2f253b3396e, "e4f6"),
    (0x3823f5f29c2128f0, "e4f6"),
    (0xd0fd9de8ffbd0c16, "e4f6"),
    (0xbc1d3f01cf52e214, "e4f6"),
    (0xac31260cace398e0, "e4f6"),
    (0xed61ffe8060869f1, "e4f6"),
    (0x42bfb6d215cabd6f, "e4f6"),
    (0x46db992ad6ac4b3f, "e4f6"),
    (0x116edca900e62b45, "e4f6"),
    (0xb89b05d68ef81fcc, "e4f6"),
    (0x0c3cf78361c7246d, "e4f6"),
    (0x5e9f9794dedbd0c4, "e4f6"),
    (0x7ff33c6206fd9cda, "d5f6"),
    (0x6ea2047ed4b20ff2, "e4d6"),
    (0x3d132d85fbf2f391, "g3h5"),
    (0x57662fe241381697, "e4f6"),
    (0x43976bb9772f530c, "d8f7"),
    (0x591b0a5b274f5e0b, "b6d5"),
    (0xbdb2a6c1cec1d390, "e7g7"),
    (0xa4d3793896947843, "e7g7"),
    (0x556ccedbad5df776, "e7g7"),
    (0x59780e48354e2ae6, "e7g7"),
    (0x68f0acdb54e89291, "e7g7"),
    (0x74249d89c78f19f3, "e7g7"),
    (0xc34aca19844cb05f, "e7g7"),
    (0x25d8267ca64803bc, "e8f8"),
    (0x6fe4bdc2b7061514, "e7g7"),
    (0x89ada4b03327df0d, "e7g7"),
    (0xdb0ec4a78c3b2ba4, "e7g7"),
    (0xfa626f51541d67ba, "e7g7"),
    (0x5a14feab721c270b, "e4d6"),
    (0x09a5d7505d5cdb68, "g3h5"),
    (0xd2f77cd113d8edf7, "e7a7"),
    (0xc606388a25cfa86c, "d8f7"),
    (0xbb3df79e870cb3ac, "g5h5"),
    (0x5bcd998b8a3c80ad, "g4h6"),
    (0x94418ed47f12b36f, "g4h6"),
    (0x7de330a43286c5f6, "g4h6"),
    (0xed3d988d5634c118, "g4h6"),
    (0x37960ede009b890c, "e4f6"),
    (0x80c2036a4a6e959e, "e4f6"),
    (0x213d4d8f381ef264, "e4f6"),
    (0x8e8f93911015c1ac, "g4h6"),
    (0x925ba2c383724ace, "g4h6"),
    (0x711450801f11fc8c, "e8f8"),
    (0x7280b0d016fb8318, "g4h6"),
    (0x899b8288f3fb4629, "g4h6"),
    (0x6fd29bfa77da8c30, "g4h6"),
    (0x3d71fbedc8c67899, "g4h6"),
    (0x1c1d501b10e03487, "g4h6"),
    (0xe243f1dba7f9a06d, "g4h6"),
    (0x11aa835569b373aa, "g4h6"),
    (0x87590f832e7a3894, "g4h6"),
    (0x692be7d33529970d, "g4h6"),
    (0xa07230e952e2875d, "g4h6"),
    (0x30205d933e2c8eb5, "g4h6"),
    (0xaf5dff98d621944a, "g4h6"),
    (0xe8f7b21d6ae0887a, "e4g5"),
    (0xf0212c89dd70d4d7, "h5h6"),
    (0x9a542eee67ba31d1, "e4g5"),
    (0x207907c06132fb51, "g6g7"),
    (0x19813a5fe45a3509, "c8b7"),
    (0xbb5e1e0879f3d1f6, "e7g7"),
    (0xd4848375ca3f79a9, "e7g7"),
    (0x94b04b05755ad318, "e7g7"),
    (0x4625e740275492cf, "e7g7"),
    (0xa7ec2c537b4ad36b, "e7g7"),
    (0x2ff218cba1cd3c5e, "e7g7"),
    (0x1a63d9e123cef6ad, "e4f6"),
    (0x33627b364b213be9, "e4f6"),
    (0x0f8b19940a6d6c9a, "e4f6"),
    (0x52bf52c87b449ad9, "e4f6"),
    (0x3a655ae29e0dab58, "e7g7"),
    (0x8e5bcbdad193ae6d, "e7g7"),
    (0xaa7cefad80b21a66, "e7g7"),
    (0xfdc9aa2e56f87a1c, "e8f8"),
    (0x06d29876b3f8bf2d, "e7g7"),
    (0xe09b810437d97534, "e7g7"),
    (0xb238e11388c5819d, "e7g7"),
    (0x93544ae550e3cd83, "e7g7"),
    (0x820572f982ac5eab, "e4d6"),
    (0xd1b45b02adeca2c8, "g3h5"),
    (0xbbc15965172647ce, "e7c7"),
    (0xaf301d3e21310255, "d8f7"),
    (0xa328490d3e7d1dd5, "f6h7"),
    (0x8d1c2e3613b36ec2, "e4f6"),
    (0x7d7bafbc68817c69, "e4f6"),
    (0xd904a79bda3545d7, "e4f6"),
    (0x41cc216cc3a5009c, "e4f6"),
    (0x1bdd90fda31c45d8, "e4f6"),
    (0x74070d8010d0ed87, "e4f6"),
    (0x9cd9659a734cc961, "e4f6"),
    (0xa69c74236be76bd6, "e4f6"),
    (0xe7946a82100a59c2, "e4f6"),
    (0xef7d85aa728abc20, "e4f6"),
    (0xf39ce4a64f32610f, "e4f6"),
    (0xa09db0851b3f5f31, "e4f6"),
    (0x91680ea36a8bc433, "e4f6"),
    (0xc0fb330de42d5e15, "e4f6"),
    (0x417faeebd109b9eb, "e4f6"),
    (0xe5ab014dc5e5ef71, "e4f6"),
    (0xa651168369172b03, "e4f6"),
    (0x40180ff1ed36e11a, "e4f6"),
    (0x31b7ddeffd0c67d2, "e4f6"),
    (0x2b3ef1120ce42617, "e4f6"),
    (0xb005c44d6120e08c, "e4f6"),
    (0xe328689088bf313a, "e4f6"),
    (0x10c11a1e46f5e2fd, "e4f6"),
    (0x863296c8013ca9c3, "e4f6"),
    (0x68407e981a6f065a, "e4f6"),
    (0x93409d8da0cfbb75, "e4f6"),
    (0x9f97a9ad21f590f9, "e4f6"),
    (0x5bb87f4c1df8c734, "e4d6"),
    (0x6dcfec981fc267e4, "g3h5"),
    (0x21129e8b4e746a06, "d8f7"),
    (0x1716d83571e318e0, "b6d5"),
    (0xc3e225d1e36df5c7, "e7g7"),
    (0x16f7cd3c14e6d72d, "e7g7"),
    (0x406d645e102a9e9a, "e7g7"),
    (0xfdd78bce8503eb90, "e7g7"),
    (0xd2276e7d83925264, "e7g7"),
    (0x44016abfaa83154d, "e7g7"),
    (0xa29386da8887a6ae, "e8f8"),
    (0xe8af1d6499c9b006, "e7g7"),
    (0x2b946d7191514a5e, "e7g7"),
    (0x5c456401a2f48eb6, "e7g7"),
    (0x65c0faf5fc3abd12, "e7g7"),
    (0xfefbcfaa91fe7b89, "e7g7"),
    (0xdd5f5e0d5cd38219, "e4d6"),
    (0x8eee77f673937e7a, "g3h5"),
    (0x55bcdc773d1748e5, "e7a7"),
    (0x414d982c0b000d7e, "d8f7"),
    (0xa0788fa53220b14c, "e8f8"),
    (0x46ea63c0102402af, "e8f8"),
    (0x0cd6f87e016a1407, "b6d5"),
    (0x419316085ff28bdb, "e7g7"),
    (0xeca8d38ba22af479, "e7g7"),
    (0xefaedb1391427937, "e7g7"),
    (0x8dbb236da71083b0, "e7g7"),
    (0x3c1b0bacc14e461d, "e7g7"),
    (0x97023cac0edc5783, "e7g7"),
    (0x7fdc54b66d407365, "e7g7"),
    (0x3d3d5efe72cdd89d, "e7g7"),
    (0x5e9407e407929de0, "e7g7"),
    (0xe9fa50744451344c, "e7g7"),
    (0x0f68bc11665587af, "e8f8"),
    (0x17bacc881c0560bf, "e7g7"),
    (0x0637530c7b486be1, "e7g7"),
    (0x5494331bc4549f48, "e7g7"),
    (0xdb38b1590e863c5e, "e7g7"),
    (0x70a464c6b201a318, "e4d6"),
    (0x23154d3d9d415f7b, "g3h5"),
    (0xf847e6bcd3c569e4, "e7a7"),
    (0xecb6a2e7e5d22c7f, "d8f7"),
    (0xea9fe10c854bde1e, "f6h5"),
    (0xd0ff3921b0cd93c2, "e4f6"),
    (0x2f2a1785260b1dd8, "e4f6"),
    (0x2c2c1f1d15639096, "e4f6"),
    (0x4e39e76323316a11, "e4f6"),
    (0x5480f8a28afdbe22, "e4f6"),
    (0xbc5e90b8e9619ac4, "e4f6"),
    (0x861b8101f1ca3873, "e4f6"),
    (0xc7139fa08a270a67, "e4f6"),
    (0x5cb3036ffa45f222, "e4f6"),
    (0xd31b1184d51f32aa, "e4f6"),
    (0x801a45a781120c94, "e4f6"),
    (0x22a68866e2448a31, "e4f6"),
    (0x292852d2cc2d7174, "e4f6"),
    (0x9d16c3ea83b37441, "e4f6"),
    (0x2a78947ac070dded, "e4f6"),
    (0x7dcdd1f9163abd97, "e4f6"),
    (0x975b7c25947773f8, "e4f6"),
    (0x609ffad3771bb2bf, "e4f6"),
    (0x23b10540af4a4d48, "e4f6"),
    (0x71cf9eba946541b5, "e4f6"),
    (0x2c277f872beb5275, "e4f6"),
    (0xed0e90f2a7389ee2, "e4f6"),
    (0x1ee7e27c69724d25, "e4f6"),
    (0x88146eaa2ebb061b, "e4f6"),
    (0x666686fa35e8a982, "e4f6"),
    (0xaf3f51c05223b9d2, "e4f6"),
    (0x91b151cf0e723f21, "e4f6"),
    (0x25b8744d8ef22e93, "e4f6"),
    (0x4966be415abe39ee, "e4d6"),
    (0x51b020d5ed2e6543, "g5f6"),
    (0x3bc522b257e48045, "e4f6"),
    (0x2f3466e961f3c5de, "d8f7"),
    (0xb83c811b3a572ab7, "b6d5"),
    (0xa797844a0fd144d3, "e7g7"),
    (0x7b4e18e73631b8a8, "e7g7"),
    (0x5b44a276aa7f4787, "e7g7"),
    (0x1a5de801330bcf61, "e7g7"),
    (0x23e845c935e16933, "e7g7"),
    (0xcb362dd3567d4dd5, "e7g7"),
    (0x89d7279b49f0e62d, "e7g7"),
    (0xea7e7e813cafa350, "e7g7"),
    (0xdda9ba0719131db1, "e7g7"),
    (0x91f5b65f7f511a6f, "e7g7"),
    (0x5d1029117f6c0afc, "e7g7"),
    (0xbb82c5745d68b91f, "e8f8"),
    (0x919c13503b512221, "e7g7"),
    (0x665895a6d83de366, "e7g7"),
    (0x77d50a22bf70e838, "e7g7"),
    (0xaf5f8a5f96cd6454, "e7g7"),
    (0xc44e1da3893c9da8, "e4d6"),
    (0x97ff3458a67c61cb, "g3h5"),
    (0x4cad9fd9e8f85754, "e7a7"),
    (0x585cdb82deef12cf, "d8f7"),
    (0x670e8b2d5568f243, "f6d7"),
    (0xdac5446577a2f62e, "e4f6"),
    (0x71dc7365b830e7b0, "e4f6"),
    (0x99021b7fdbacc356, "e4f6"),
    (0xa3470ac6c30761e1, "e4f6"),
    (0xe24f1467b8ea53f5, "e4f6"),
    (0x2b01638fa3965a08, "e4f6"),
    (0xf6479a43e7d26b38, "e4f6"),
    (0x8a8df01f1c3ff5c4, "e4f6"),
    (0x07fa03a1d089d3a3, "e4f6"),
    (0x0c74d915fee028e6, "e4f6"),
    (0xb84a482db17e2dd3, "e4f6"),
    (0xf1648341aae9d08c, "e4f6"),
    (0x45c3711445d6eb2d, "e4f6"),
    (0x17601103faca1f84, "e4f6"),
    (0x2041785a1add88c5, "e4f6"),
    (0xb3a9b44aba3d6ae2, "e4f6"),
    (0x99c36309487bffb8, "e4f6"),
    (0xaaccba493bd711c9, "e4f6"),
    (0x3c3f369f7c1e5af7, "e4f6"),
    (0x8d3df83b179deae6, "e4f6"),
    (0x5e6301a9b518cd03, "e4d6"),
    (0x46b59f3d028891ae, "g3h5"),
    (0x1e99a9756529d9d7, "e4f6"),
    (0x0a68ed2e533e9c4c, "d8f7"),
    (0x94e7f9a39b222184, "f6h7"),
    (0x43c8bd2eb58fd1d6, "e4f6"),
    (0xab16d534d613f530, "e4f6"),
    (0x9153c48dceb85787, "e4f6"),
    (0xd05bda2cb5556593, "e4f6"),
    (0x4bfb46e3c5379dd6, "e4f6"),
    (0xc4535408ea6d5d5e, "e4f6"),
    (0x9752002bbe606360, "e4f6"),
    (0x0b03c185cae1e8e4, "e4f6"),
    (0xa25af90c883f79b0, "e4f6"),
    (0xaf2e11c878c7a3f6, "e4f6"),
    (0x2a232853ef59121b, "e4f6"),
    (0x0c44388c6cc168b4, "e4f6"),
    (0x3e60175ef35f1e80, "e4f6"),
    (0x8a5e8666bcc11bb5, "e4f6"),
    (0x919ea62dcc481752, "e4f6"),
    (0x77d7bf5f4869dd4b, "e4f6"),
    (0x2574df48f77529e2, "e4f6"),
    (0x85ff537a4313f48e, "e4f6"),
    (0xe0e5ad22ca906c77, "e4f6"),
    (0x0e974572d1c3c3ee, "e4f6"),
    (0x588a92768c6cabaf, "e4f6"),
    (0xf597a6676b637ec1, "e4f6"),
    (0xf9409247ea59554d, "e4f6"),
    (0x2c3f8e9de8e785d4, "e4f6"),
    (0x6c77cfe2b8a7fb65, "e4d6"),
    (0x5a005c36ba9d5bb5, "g3h5"),
    (0x16dd2e25eb2b5657, "d8f7"),
    (0x02147575dceb6aba, "f6d7"),
    (0x14c68d3d31b37f49, "e4f6"),
    (0xfc18e527522f5baf, "e4f6"),
    (0xc65df49e4a84f918, "e4f6"),
    (0x8755ea3f3169cb0c, "e4f6"),
    (0x1cf576f0410b3349, "e4f6"),
    (0x935d641b6e51f3c1, "e4f6"),
    (0xef970e4795bc6d3d, "e4f6"),
    (0x62e0fdf9590a4b5a, "e4f6"),
    (0x696e274d7763b01f, "e4f6"),
    (0xdd50b67538fdb52a, "e4f6"),
    (0x9799a8e0d35b8087, "e4f6"),
    (0xd87db7ed8d2bf269, "e4f6"),
    (0x454a032eade42d60, "d7f6"),
    (0x20837efdc1cfa755, "d7f6"),
    (0x8c86641f272aa238, "d7f6"),
    (0xc690963e4874b9cd, "e4f6"),
    (0x20d98f4ccc5573d4, "e4f6"),
    (0x727aef5b7349877d, "e4f6"),
    (0xa13eb1c4495b4b29, "e4f6"),
    (0x52d7c34a871198ee, "e4f6"),
    (0xc4159b48483e3fc4, "e4f6"),
    (0xb3acf8a845dfcad3, "e4f6"),
    (0xd8f71ddb3f3f4bf9, "e4f6"),
    (0x3652a9c80a88f171, "e4f6"),
    (0x3b79fff13c9b55fa, "e4d6"),
    (0x23af61658b0b0957, "g3h5"),
    (0x7b83572decaa412e, "e4f6"),
    (0x6f721376dabd04b5, "d8f7"),
    (0xec669d25c7b8c523, "c8c7"),
    (0xd6d6c7a314e60ae5, "d7g7"),
    (0x3e08afb9777a2e03, "d7g7"),
    (0x044dbe006fd18cb4, "d7g7"),
    (0x4545a0a1143cbea0, "d7g7"),
    (0x6def4406e4fb1919, "d7g7"),
    (0x514d2e854b04866d, "d7g7"),
    (0x9ca0ed3f44a7cb08, "d7g7"),
    (0x3a4310e6622f2bb5, "d7g7"),
    (0xab7e6dd35236c5b3, "d7g7"),
    (0xac4a84839d0d9f7a, "d7g7"),
    (0x66092e67803a5272, "d7g7"),
    (0x4e6fa7690ef6bf5f, "d7g7"),
    (0xa82eab7b5e6b692a, "d7g7"),
    (0xdf20bd480f6f20a2, "e8f8"),
    (0x4ebc471e7c6fdac9, "e8f8"),
    (0x0480dca06d21cc61, "d7g7"),
    (0xe2c9c5d2e9000678, "d7g7"),
    (0xb06aa5c5561cf2d1, "d7g7"),
    (0x7a1728a630eb5eb0, "e4d6"),
    (0x62c1b632877b021d, "g3h5"),
    (0xb9931db3c9ff3482, "e6e7"),
    (0xad6259e8ffe87119, "d8f7"),
    (0x9418e3f9543d06ea, "c8d7"),
    (0x5ac1865af4b7ed6c, "f7g7"),
    (0xb21fee40972bc98a, "f7g7"),
    (0x885afff98f806b3d, "f7g7"),
    (0xc952e158f46d5929, "f7g7"),
    (0x52f27d97840fa16c, "f7g7"),
    (0xdd5a6f7cab5561e4, "f7g7"),
    (0x8e5b3b5fff585fda, "f7g7"),
    (0x2ce7f69e9c0ed97f, "f7g7"),
    (0x9426dcce1d466268, "e4f6"),
    (0x369a110f7e10e4cd, "e4f6"),
    (0x15cdde73a950217b, "e4f6"),
    (0xdcfa57e745cc0e1c, "e4f6"),
    (0x27692c2ab267223a, "f7g7"),
    (0x9357bd12fdf9270f, "f7g7"),
    (0x591417f6e0ceea07, "f7g7"),
    (0xc627d688ab793ece, "f7g7"),
    (0x2439ea82be3a8ea3, "f7g7"),
    (0xe21055571b7014b2, "f7g7"),
    (0x738caf016870eed9, "f7f8"),
    (0x88979d598d702be8, "f7g7"),
    (0x6ede842b0951e1f1, "f7g7"),
    (0x3c7de43cb64d1558, "f7g7"),
    (0x42fac17110aac402, "g3h5"),
    (0x35845c4a29aed30b, "f7e7"),
    (0x217518111fb99690, "f7h7"),
    (0x1bb14a10fc225380, "e6e7"),
    (0x54fdcbffa32cdc33, "d3c4"),
    (0xbc23a3e5c0b0f8d5, "c6d5"),
    (0x8666b25cd81b5a62, "c6d5"),
    (0xc76eacfda3f66876, "c6d5"),
    (0x5cce3032d3949033, "c6d5"),
    (0xd36622d9fcce50bb, "c6d5"),
    (0x1e8be163f36d1dde, "c6d5"),
    (0xd89ba5b2a70b9b77, "c8e6"),
    (0x2955618fe5fc1365, "c6d5"),
    (0x9d6bf0b7aa621650, "c6d5"),
    (0x86abd0fcdaeb1ab7, "c6d5"),
    (0x60e2c98e5ecad0ae, "c6d5"),
    (0x3241a999e1d62407, "c6d5"),
    (0x08348eb92b269793, "e5e6"),
    (0xcbf3f0bc7157ee7f, "g5h5"),
    (0xdf02b4e74740abe4, "d8f7"),
    (0x29505bb1a9bf1ee4, "c8d7"),
    (0xf0878205ff67a59e, "h7g7"),
    (0xe9e65dfca7320e4d, "h7g7"),
    (0x1859ea1f9cfb8178, "h7g7"),
    (0x221cfba6845023cf, "h7g7"),
    (0x6314e507ffbd11db, "h7g7"),
    (0xf8b479c88fdfe99e, "h7g7"),
    (0x522b427d61329d4c, "h7g7"),
    (0x953a96e600c6c4b1, "h7g7"),
    (0x37865b2763904214, "h7g7"),
    (0x8d2f2875b9b76ac8, "h7g7"),
    (0x3911b94df6296ffd, "h7g7"),
    (0xf35213a9eb1ea2f5, "h7g7"),
    (0x6c61d2d7a0a9763c, "h7g7"),
    (0x8e7feeddb5eac651, "h7g7"),
    (0xf971f8eee4ee8fd9, "h7g7"),
    (0x68ed02b897ee75b2, "h7h8"),
    (0x22d1990686a0631a, "h7g7"),
    (0xc49880740281a903, "h7g7"),
    (0x963be063bd9d5daa, "h7g7"),
    (0x5c466d00db6af1cb, "e4d6"),
    (0x4e95d0df19660f95, "g3h5"),
    (0x81363f0561f57c91, "d8f7"),
    (0x72410c785ca086d6, "e4d6"),
    (0x6a9792eceb30da7b, "g3h5"),
    (0xb1c5396da5b4ece4, "g5h5"),
    (0xa5347d3693a3a97f, "d8f7"),
    (0xf42c5d1de702d3e2, "e6d6"),
    (0x82231b93897c4485, "d3c4"),
    (0x2e41fb7ae0200158, "c6d5"),
    (0xa40db2bc31c601de, "c6d5"),
    (0x34d31a9555740530, "c6d5"),
    (0xbb26e20673dc494d, "c6d5"),
    (0x132bdffcb550e6a0, "c6d5"),
    (0x8b2118cc67a04449, "c6d5"),
    (0x5ab129e7e7eae9a9, "c6d5"),
    (0x8a10e05ef9c40885, "c6d5"),
    (0x05b8f2b5d69ec80d, "c6d5"),
    (0x3c07758ce336a337, "c6d5"),
    (0x4522c2b1158ba30f, "c8e6"),
    (0xd7ea458759786252, "c6d5"),
    (0x195124169b288721, "c6d5"),
    (0xa02f34db1215f42b, "c6d5"),
    (0xff8bb1e3cfac8bd3, "c6d5"),
    (0x4bb520db80328ee6, "c6d5"),
    (0x6103f1ea1017711b, "c6d5"),
    (0x001072f2c0a8b6f3, "c6d5"),
    (0x1a499b2ee1f594a9, "c6d5"),
    (0x50750090f0bb8201, "c6d5"),
    (0xb63c19e2749a4818, "c6d5"),
    (0xe49f79f5cb86bcb1, "c6d5"),
    (0xcccb673a5e70dcda, "c8d7"),
    (0x3b7da126392a0d01, "d7f7"),
    (0x971f41cf507648dc, "d7f7"),
    (0x1d5308098190485a, "d7f7"),
    (0x8d8da020e5224cb4, "d7f7"),
    (0xb35ff15537c4d350, "d7f7"),
    (0x1b52ccaff1487cbd, "d7f7"),
    (0x327fa279d7f60dcd, "d7f7"),
    (0xe3ef935257bca02d, "d7f7"),
    (0x334e5aeb49924101, "d7f7"),
    (0xbce6480066c88189, "d7f7"),
    (0x347e66dfa72e392a, "d7f7"),
    (0x4d5bd1e251933912, "d7f7"),
    (0x6eb4ff32e92e2bd6, "d7f7"),
    (0x11283745df301d3c, "d7f7"),
    (0xa8562788560d6e36, "d7f7"),
    (0x860ebcb483bcab75, "d7f7"),
    (0x2e03814e45300498, "d7f7"),
    (0x57263673b38d04a0, "d7f7"),
    (0x3f70ac790a81d87f, "d7f7"),
    (0x418d75228b087fc8, "d7f7"),
    (0x46d50b567ffac257, "d7f7"),
    (0xf2eb9a6e3064c762, "d7f7"),
    (0xd85d4b5fa041389f, "d7f7"),
    (0xb94ec84770feff77, "e8f8"),
    (0x75e80986f8005908, "e8f8"),
    (0x1230887da5ed0eb4, "e8f8"),
    (0xe92bba2540edcb85, "d7f7"),
    (0x0f62a357c4cc019c, "d7f7"),
    (0x5dc1c3407bd0f535, "d7f7"),
    (0xd2c7e6ff40588cab, "d7f7"),
    (0x86d8e89f1275a511, "d7f7"),
    (0xde941936faae2d7d, "d7f7"),
    (0x269be7c5e9698acd, "d7a7"),
    (0x3e4d79515ef9d660, "e6e7"),
    (0x54387b36e4333366, "e6e7"),
    (0x40c93f6dd22476fd, "d8f7"),
    (0x3bad73c3a4b96445, "c6d5"),
    (0xc844014d6af3b782, "c6d5"),
    (0x5eb78d9b2d3afcbc, "f6d5"),
    (0xb7fdceb79fc22831, "d5f6"),
    (0x1b9f2e5ef69e6dec, "d5f6"),
    (0x4644afba1db92d22, "d5f6"),
    (0x010dcfb143ca6984, "d5f6"),
    (0x3fdf9ec4912cf660, "d5f6"),
    (0x40456b1c6d6119c5, "d5f6"),
    (0x35a3588175074eae, "d5f6"),
    (0x9da858d479d8380c, "d5f6"),
    (0x24d64819f0e54b06, "d5f6"),
    (0x1dc2ace5e3d3a72f, "d5f6"),
    (0x7e6bf5ff968ce252, "d5f6"),
    (0x54dd24ce06a91daf, "d5f6"),
    (0x616aeecc8296cf08, "d5f6"),
    (0x2e8ef1c1dce6bde6, "d5f6"),
    (0x3360e96a844d5efb, "c7g7"),
    (0x4adeae60c69f975c, "d5f6"),
    (0x9f65f38862a85b96, "c7g7"),
    (0x35cea7d6d616da47, "d5f6"),
    (0xf96866175ee87c38, "d5f6"),
    (0x9eb0e7ec03052b84, "d5f6"),
    (0x65abd5b4e605eeb5, "d5f6"),
    (0x83e2ccc6622424ac, "d5f6"),
    (0xd141acd1dd38d005, "d5f6"),
    (0x5e47896ee6b0a99b, "d5f6"),
    (0x0a58870eb49d8021, "d5f6"),
    (0xc6b442a08fd4eeb4, "d5e7"),
    (0x521476a75c46084d, "d5f6"),
    (0xc7a26887a9edf693, "d5f6"),
    (0x672ed8c2e64f68bc, "d5f6"),
    (0x91bc210a28ddcbcf, "c7g7"),
    (0x18f419e67d910274, "d5f6"),
    (0x9e7b155144058269, "d5f6"),
    (0x7bcc5e51914e1c81, "d5f6"),
    (0x9d0a488632c639d6, "d5f6"),
    (0xe13f742b74341ced, "d6f6"),
    (0xb2cd16c0f811f350, "c5e6"),
    (0xd8b814a742db1656, "d5f6"),
    (0xcc4950fc74cc53cd, "d8f7"),
    (0x66c215e98ac3bca9, "c6d5"),
    (0xc8bb1b17a5ec90ec, "d6d7"),
    (0x896dc0d28af380f2, "f7g7"),
    (0x250f203be3afc52f, "f7g7"),
    (0xaf4369fd3249c5a9, "f7g7"),
    (0x3f9dc1d456fbc147, "f7g7"),
    (0x644298ebb812eed3, "f7g7"),
    (0xa942ad5b4291f14e, "f7g7"),
    (0x806fc38d642f803e, "f7g7"),
    (0x51fff2a6e4652dde, "f7g7"),
    (0x815e3b1ffa4bccf2, "f7g7"),
    (0x0ef629f4d5110c7a, "f7g7"),
    (0x866e072b14f7b4d9, "f7g7"),
    (0xff4bb016e24ab4e1, "f7g7"),
    (0xdca49ec65af7a625, "f7g7"),
    (0xc6355efb50e620bf, "f7g7"),
    (0x1a46467ce5d4e3c5, "f7g7"),
    (0x5113d50a0c6a96f6, "f7g7"),
    (0x9c13e0baf6e9896b, "f7g7"),
    (0xe536578700548953, "f7g7"),
    (0x8d60cd8db958558c, "f7g7"),
    (0xf39d14d638d1f23b, "f7g7"),
    (0xf4c56aa2cc234fa4, "f7g7"),
    (0x40fbfb9a83bd4a91, "f7g7"),
    (0x6a4d2aab1398b56c, "f7g7"),
    (0x6c1a9e064482c11b, "f7g7"),
    (0x0b5ea9b3c3277284, "f7g7"),
    (0x482d1dd9f48beb67, "f7g7"),
    (0xa020e98916348347, "f7f8"),
    (0x5b3bdbd1f3344676, "f7g7"),
    (0xbd72c2a377158c6f, "f7g7"),
    (0xefd1a2b4c80978c6, "f7g7"),
    (0x60d7870bf3810158, "f7g7"),
    (0x34c8896ba1ac28e2, "f7g7"),
    (0x6c8478c24977a08e, "f7g7"),
    (0x915687f96eeea99c, "c5e6"),
    (0xe6281ac257eabe95, "f7e7"),
    (0xf2d95e9961fdfb0e, "f7h7"),
    (0x4712b2fe0df3c586, "c6d5"),
    (0x75f3a35f586e88e2, "c6d5"),
    (0x6b995c4af00ec52f, "c6d5"),
    (0x3f86522aa223ec95, "c6d5"),
    (0x67caa3834af864f9, "c6d5"),
    (0x65c608e996c4a3c0, "c6d5"),
    (0x36346a021ae14c7d, "c5e6"),
    (0xed66c18354657ae2, "c8e6"),
    (0x56d71475fb9630be, "e7a7"),
    (0xc609bc5c9f243450, "e7a7"),
    (0x49fc44cfb98c782d, "e7a7"),
    (0xce3a474ad0e07702, "e7a7"),
    (0x79fbbe05adf07529, "e7a7"),
    (0xa86b8f2e2dbad8c9, "e7a7"),
    (0x78ca4697339439e5, "e7a7"),
    (0xf762547c1ccef96d, "e7a7"),
    (0xe116ed3a86863295, "e7a7"),
    (0xfc9fd317470b32a1, "e6f7"),
    (0x2530e34e93285332, "e7a7"),
    (0xeb8b82df5178b641, "e7a7"),
    (0x52f59212d845c54b, "e7a7"),
    (0x0d51172a05fcbab3, "e7a7"),
    (0xb96f86124a62bf86, "e7a7"),
    (0x93d95723da47407b, "e7a7"),
    (0xf2cad43b0af88793, "e7a7"),
    (0xe8933de72ba5a5c9, "e8h8"),
    (0xa2afa6593aebb361, "e7a7"),
    (0x0d990a5b39862924, "e7a7"),
    (0x1645df3c01d68dd1, "e7a7"),
    (0x55fe4189a4d7e54c, "e7a7"),
    (0x01e14fe9f6faccf6, "e7a7"),
    (0x9510054a80a85599, "e7a7"),
    (0xde5e61a47f760dae, "f6g4"),
    (0x8e398b2e970344b9, "e7a7"),
    (0x1fbc674a9e354b82, "e7g7"),
    (0x419a64f4ef9037bd, "d8f7"),
    (0xf99785d862723f79, "d8f7"),
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
