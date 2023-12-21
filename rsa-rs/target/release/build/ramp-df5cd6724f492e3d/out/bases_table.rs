static BASES : [Base; 257] = [
    /*   0 */ Base { digits_per_limb: 0, big_base: ::ll::limb::Limb(0) },
    /*   1 */ Base { digits_per_limb: 0, big_base: ::ll::limb::Limb(0) },
    /*   2 */ Base { digits_per_limb: 64, big_base: ::ll::limb::Limb(0x1) },
    /*   3 */ Base { digits_per_limb: 40, big_base: ::ll::limb::Limb(0xa8b8b452291fe821) },
    /*   4 */ Base { digits_per_limb: 32, big_base: ::ll::limb::Limb(0x2) },
    /*   5 */ Base { digits_per_limb: 27, big_base: ::ll::limb::Limb(0x6765c793fa10079d) },
    /*   6 */ Base { digits_per_limb: 24, big_base: ::ll::limb::Limb(0x41c21cb8e1000000) },
    /*   7 */ Base { digits_per_limb: 22, big_base: ::ll::limb::Limb(0x3642798750226111) },
    /*   8 */ Base { digits_per_limb: 21, big_base: ::ll::limb::Limb(0x3) },
    /*   9 */ Base { digits_per_limb: 20, big_base: ::ll::limb::Limb(0xa8b8b452291fe821) },
    /*  10 */ Base { digits_per_limb: 19, big_base: ::ll::limb::Limb(0x8ac7230489e80000) },
    /*  11 */ Base { digits_per_limb: 18, big_base: ::ll::limb::Limb(0x4d28cb56c33fa539) },
    /*  12 */ Base { digits_per_limb: 17, big_base: ::ll::limb::Limb(0x1eca170c00000000) },
    /*  13 */ Base { digits_per_limb: 17, big_base: ::ll::limb::Limb(0x780c7372621bd74d) },
    /*  14 */ Base { digits_per_limb: 16, big_base: ::ll::limb::Limb(0x1e39a5057d810000) },
    /*  15 */ Base { digits_per_limb: 16, big_base: ::ll::limb::Limb(0x5b27ac993df97701) },
    /*  16 */ Base { digits_per_limb: 16, big_base: ::ll::limb::Limb(0x4) },
    /*  17 */ Base { digits_per_limb: 15, big_base: ::ll::limb::Limb(0x27b95e997e21d9f1) },
    /*  18 */ Base { digits_per_limb: 15, big_base: ::ll::limb::Limb(0x5da0e1e53c5c8000) },
    /*  19 */ Base { digits_per_limb: 15, big_base: ::ll::limb::Limb(0xd2ae3299c1c4aedb) },
    /*  20 */ Base { digits_per_limb: 14, big_base: ::ll::limb::Limb(0x16bcc41e90000000) },
    /*  21 */ Base { digits_per_limb: 14, big_base: ::ll::limb::Limb(0x2d04b7fdd9c0ef49) },
    /*  22 */ Base { digits_per_limb: 14, big_base: ::ll::limb::Limb(0x5658597bcaa24000) },
    /*  23 */ Base { digits_per_limb: 14, big_base: ::ll::limb::Limb(0xa0e2073737609371) },
    /*  24 */ Base { digits_per_limb: 13, big_base: ::ll::limb::Limb(0xc29e98000000000) },
    /*  25 */ Base { digits_per_limb: 13, big_base: ::ll::limb::Limb(0x14adf4b7320334b9) },
    /*  26 */ Base { digits_per_limb: 13, big_base: ::ll::limb::Limb(0x226ed36478bfa000) },
    /*  27 */ Base { digits_per_limb: 13, big_base: ::ll::limb::Limb(0x383d9170b85ff80b) },
    /*  28 */ Base { digits_per_limb: 13, big_base: ::ll::limb::Limb(0x5a3c23e39c000000) },
    /*  29 */ Base { digits_per_limb: 13, big_base: ::ll::limb::Limb(0x8e65137388122bcd) },
    /*  30 */ Base { digits_per_limb: 13, big_base: ::ll::limb::Limb(0xdd41bb36d259e000) },
    /*  31 */ Base { digits_per_limb: 12, big_base: ::ll::limb::Limb(0xaee5720ee830681) },
    /*  32 */ Base { digits_per_limb: 12, big_base: ::ll::limb::Limb(0x5) },
    /*  33 */ Base { digits_per_limb: 12, big_base: ::ll::limb::Limb(0x172588ad4f5f0981) },
    /*  34 */ Base { digits_per_limb: 12, big_base: ::ll::limb::Limb(0x211e44f7d02c1000) },
    /*  35 */ Base { digits_per_limb: 12, big_base: ::ll::limb::Limb(0x2ee56725f06e5c71) },
    /*  36 */ Base { digits_per_limb: 12, big_base: ::ll::limb::Limb(0x41c21cb8e1000000) },
    /*  37 */ Base { digits_per_limb: 12, big_base: ::ll::limb::Limb(0x5b5b57f8a98a5dd1) },
    /*  38 */ Base { digits_per_limb: 12, big_base: ::ll::limb::Limb(0x7dcff8986ea31000) },
    /*  39 */ Base { digits_per_limb: 12, big_base: ::ll::limb::Limb(0xabd4211662a6b2a1) },
    /*  40 */ Base { digits_per_limb: 12, big_base: ::ll::limb::Limb(0xe8d4a51000000000) },
    /*  41 */ Base { digits_per_limb: 11, big_base: ::ll::limb::Limb(0x7a32956ad081b79) },
    /*  42 */ Base { digits_per_limb: 11, big_base: ::ll::limb::Limb(0x9f49aaff0e86800) },
    /*  43 */ Base { digits_per_limb: 11, big_base: ::ll::limb::Limb(0xce583bb812d37b3) },
    /*  44 */ Base { digits_per_limb: 11, big_base: ::ll::limb::Limb(0x109b79a654c00000) },
    /*  45 */ Base { digits_per_limb: 11, big_base: ::ll::limb::Limb(0x1543beff214c8b95) },
    /*  46 */ Base { digits_per_limb: 11, big_base: ::ll::limb::Limb(0x1b149a79459a3800) },
    /*  47 */ Base { digits_per_limb: 11, big_base: ::ll::limb::Limb(0x224edfb5434a830f) },
    /*  48 */ Base { digits_per_limb: 11, big_base: ::ll::limb::Limb(0x2b3fb00000000000) },
    /*  49 */ Base { digits_per_limb: 11, big_base: ::ll::limb::Limb(0x3642798750226111) },
    /*  50 */ Base { digits_per_limb: 11, big_base: ::ll::limb::Limb(0x43c33c1937564800) },
    /*  51 */ Base { digits_per_limb: 11, big_base: ::ll::limb::Limb(0x54411b2441c3cd8b) },
    /*  52 */ Base { digits_per_limb: 11, big_base: ::ll::limb::Limb(0x6851455acd400000) },
    /*  53 */ Base { digits_per_limb: 11, big_base: ::ll::limb::Limb(0x80a23b117c8feb6d) },
    /*  54 */ Base { digits_per_limb: 11, big_base: ::ll::limb::Limb(0x9dff7d32d5dc1800) },
    /*  55 */ Base { digits_per_limb: 11, big_base: ::ll::limb::Limb(0xc155af6faeffe6a7) },
    /*  56 */ Base { digits_per_limb: 11, big_base: ::ll::limb::Limb(0xebb7392e00000000) },
    /*  57 */ Base { digits_per_limb: 10, big_base: ::ll::limb::Limb(0x50633659656d971) },
    /*  58 */ Base { digits_per_limb: 10, big_base: ::ll::limb::Limb(0x5fa8624c7fba400) },
    /*  59 */ Base { digits_per_limb: 10, big_base: ::ll::limb::Limb(0x717d9faa73c5679) },
    /*  60 */ Base { digits_per_limb: 10, big_base: ::ll::limb::Limb(0x86430aac6100000) },
    /*  61 */ Base { digits_per_limb: 10, big_base: ::ll::limb::Limb(0x9e64d9944b57f29) },
    /*  62 */ Base { digits_per_limb: 10, big_base: ::ll::limb::Limb(0xba5ca5392cb0400) },
    /*  63 */ Base { digits_per_limb: 10, big_base: ::ll::limb::Limb(0xdab2ce1d022cd81) },
    /*  64 */ Base { digits_per_limb: 10, big_base: ::ll::limb::Limb(0x6) },
    /*  65 */ Base { digits_per_limb: 10, big_base: ::ll::limb::Limb(0x12aeed5fd3e2d281) },
    /*  66 */ Base { digits_per_limb: 10, big_base: ::ll::limb::Limb(0x15c3da1572d50400) },
    /*  67 */ Base { digits_per_limb: 10, big_base: ::ll::limb::Limb(0x194c05534f75ee29) },
    /*  68 */ Base { digits_per_limb: 10, big_base: ::ll::limb::Limb(0x1d56299ada100000) },
    /*  69 */ Base { digits_per_limb: 10, big_base: ::ll::limb::Limb(0x21f2a089a4ff4f79) },
    /*  70 */ Base { digits_per_limb: 10, big_base: ::ll::limb::Limb(0x2733896c68d9a400) },
    /*  71 */ Base { digits_per_limb: 10, big_base: ::ll::limb::Limb(0x2d2cf2c33b533c71) },
    /*  72 */ Base { digits_per_limb: 10, big_base: ::ll::limb::Limb(0x33f506e440000000) },
    /*  73 */ Base { digits_per_limb: 10, big_base: ::ll::limb::Limb(0x3ba43bec1d062211) },
    /*  74 */ Base { digits_per_limb: 10, big_base: ::ll::limb::Limb(0x4455872d8fd4e400) },
    /*  75 */ Base { digits_per_limb: 10, big_base: ::ll::limb::Limb(0x4e2694539f2f6c59) },
    /*  76 */ Base { digits_per_limb: 10, big_base: ::ll::limb::Limb(0x5938006c18900000) },
    /*  77 */ Base { digits_per_limb: 10, big_base: ::ll::limb::Limb(0x65ad9912474aa649) },
    /*  78 */ Base { digits_per_limb: 10, big_base: ::ll::limb::Limb(0x73ae9ff4241ec400) },
    /*  79 */ Base { digits_per_limb: 10, big_base: ::ll::limb::Limb(0x836612ee9c4ce1e1) },
    /*  80 */ Base { digits_per_limb: 10, big_base: ::ll::limb::Limb(0x9502f90000000000) },
    /*  81 */ Base { digits_per_limb: 10, big_base: ::ll::limb::Limb(0xa8b8b452291fe821) },
    /*  82 */ Base { digits_per_limb: 10, big_base: ::ll::limb::Limb(0xbebf59a07dab4400) },
    /*  83 */ Base { digits_per_limb: 10, big_base: ::ll::limb::Limb(0xd7540d4093bc3109) },
    /*  84 */ Base { digits_per_limb: 10, big_base: ::ll::limb::Limb(0xf2b96616f1900000) },
    /*  85 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x336de62af2bca35) },
    /*  86 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x39235ec33d49600) },
    /*  87 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x3f674e539585a17) },
    /*  88 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x4645b6958000000) },
    /*  89 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x4dcb74afbc49c19) },
    /*  90 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x56064e1d18d9a00) },
    /*  91 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x5f04fe2cd8a39fb) },
    /*  92 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x68d74421f5c0000) },
    /*  93 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x738df1f6ab4827d) },
    /*  94 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x7f3afbc9cfb5e00) },
    /*  95 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x8bf187fba88f35f) },
    /*  96 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x99c600000000000) },
    /*  97 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0xa8ce21eb6531361) },
    /*  98 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0xb92112c1a0b6200) },
    /*  99 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0xcad7718b8747c43) },
    /* 100 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0xde0b6b3a7640000) },
    /* 101 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0xf2d8cf5fe6d74c5) },
    /* 102 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x1095d25bfa712600) },
    /* 103 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x121b7c4c3698faa7) },
    /* 104 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x13c09e8d68000000) },
    /* 105 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x15876ccb0b709ca9) },
    /* 106 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x17723c2976da2a00) },
    /* 107 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x198384e9c259048b) },
    /* 108 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x1bbde41dfeec0000) },
    /* 109 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x1e241d6e3337910d) },
    /* 110 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x20b91cee9901ee00) },
    /* 111 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x237ff9079863dfef) },
    /* 112 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x267bf47000000000) },
    /* 113 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x29b08039fbeda7f1) },
    /* 114 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x2d213df34f65f200) },
    /* 115 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x30d201d957a7c2d3) },
    /* 116 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x34c6d52160f40000) },
    /* 117 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x3903f855d8f4c755) },
    /* 118 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x3d8de5c8ec59b600) },
    /* 119 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x4269541d1ff01337) },
    /* 120 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x479b38e478000000) },
    /* 121 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x4d28cb56c33fa539) },
    /* 122 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x5317871fa13aba00) },
    /* 123 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x596d2f44de9fa71b) },
    /* 124 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x602fd125c47c0000) },
    /* 125 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x6765c793fa10079d) },
    /* 126 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x6f15be069b847e00) },
    /* 127 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x7746b3e82a77047f) },
    /* 128 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x7) },
    /* 129 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x894953f7ea890481) },
    /* 130 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x932abffea4848200) },
    /* 131 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0x9dacb687d3d6a163) },
    /* 132 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0xa8d8102a44840000) },
    /* 133 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0xb4b60f9d140541e5) },
    /* 134 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0xc15065d4856e4600) },
    /* 135 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0xceb1363f396d23c7) },
    /* 136 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0xdce31b2488000000) },
    /* 137 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0xebf12a24bca135c9) },
    /* 138 */ Base { digits_per_limb: 9, big_base: ::ll::limb::Limb(0xfbe6f8dbf88f4a00) },
    /* 139 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x1ef156c084ce761) },
    /* 140 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x20c4e3b94a10000) },
    /* 141 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x22b0695a08ba421) },
    /* 142 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x24b4f35d7a4c100) },
    /* 143 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x26d397284975781) },
    /* 144 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x290d74100000000) },
    /* 145 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x2b63b3a37866081) },
    /* 146 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x2dd789f4d894100) },
    /* 147 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x306a35e51b58721) },
    /* 148 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x331d01712e10000) },
    /* 149 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x35f14200a827c61) },
    /* 150 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x38e858b62216100) },
    /* 151 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x3c03b2c13176a41) },
    /* 152 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x3f44c9b21000000) },
    /* 153 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x42ad23cef3113c1) },
    /* 154 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x463e546b19a2100) },
    /* 155 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x49f9fc3f96684e1) },
    /* 156 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x4de1c9c5dc10000) },
    /* 157 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x51f77994116d2a1) },
    /* 158 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x563cd6bb3398100) },
    /* 159 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x5ab3bb270beeb01) },
    /* 160 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x5f5e10000000000) },
    /* 161 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x643dce0ec16f501) },
    /* 162 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x6954fe21e3e8100) },
    /* 163 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x6ea5b9755f440a1) },
    /* 164 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x74322a1c0410000) },
    /* 165 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x79fc8b6ae8a46e1) },
    /* 166 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x80072a66d512100) },
    /* 167 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x86546633b42b9c1) },
    /* 168 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x8ce6b0861000000) },
    /* 169 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x93c08e16a022441) },
    /* 170 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x9ae49717f026100) },
    /* 171 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0xa25577ae24c1a61) },
    /* 172 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0xaa15f068e610000) },
    /* 173 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0xb228d6bf7577921) },
    /* 174 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0xba91158ef5c4100) },
    /* 175 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0xc351ad9aec0b681) },
    /* 176 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0xcc6db6100000000) },
    /* 177 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0xd5e85d09025c181) },
    /* 178 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0xdfc4e816401c100) },
    /* 179 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0xea06b4c72947221) },
    /* 180 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0xf4b139365210000) },
    /* 181 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0xffc80497d520961) },
    /* 182 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x10b4ebfca1dee100) },
    /* 183 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x117492de921fc141) },
    /* 184 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x123bb2ce41000000) },
    /* 185 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x130a8b6157bdecc1) },
    /* 186 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x13e15dede0e8a100) },
    /* 187 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x14c06d941c0ca7e1) },
    /* 188 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x15a7ff487a810000) },
    /* 189 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x169859ddc5c697a1) },
    /* 190 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x1791c60f6fed0100) },
    /* 191 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x18948e8c0e6fba01) },
    /* 192 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x19a1000000000000) },
    /* 193 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x1ab769203dafc601) },
    /* 194 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x1bd81ab557f30100) },
    /* 195 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x1d0367a69fed1ba1) },
    /* 196 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x1e39a5057d810000) },
    /* 197 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x1f7b2a18f29ac3e1) },
    /* 198 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x20c850694c2aa100) },
    /* 199 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x222173cc014980c1) },
    /* 200 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x2386f26fc1000000) },
    /* 201 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x24f92ce8af296d41) },
    /* 202 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x2678863cd0ece100) },
    /* 203 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x280563f0a9472d61) },
    /* 204 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x29a02e1406210000) },
    /* 205 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x2b494f4efe6d2e21) },
    /* 206 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x2d0134ef21cbc100) },
    /* 207 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x2ec84ef4da2ef581) },
    /* 208 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x309f102100000000) },
    /* 209 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x3285ee02a1420281) },
    /* 210 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x347d6104fc324100) },
    /* 211 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x3685e47dade53d21) },
    /* 212 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x389ff6bb15610000) },
    /* 213 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x3acc1912ebb57661) },
    /* 214 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x3d0acff111946100) },
    /* 215 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x3f5ca2e692eaf841) },
    /* 216 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x41c21cb8e1000000) },
    /* 217 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x443bcb714399a5c1) },
    /* 218 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x46ca406c81af2100) },
    /* 219 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x496e106ac22aaae1) },
    /* 220 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x4c27d39fa5410000) },
    /* 221 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x4ef825c296e43ca1) },
    /* 222 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x51dfa61f5ad88100) },
    /* 223 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x54def7a6d2f16901) },
    /* 224 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x57f6c10000000000) },
    /* 225 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x5b27ac993df97701) },
    /* 226 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x5e7268b9bbdf8100) },
    /* 227 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x61d7a7932ff3d6a1) },
    /* 228 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x65581f53c8c10000) },
    /* 229 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x68f48a385b8320e1) },
    /* 230 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x6cada69ed07c2100) },
    /* 231 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x70843718cdbf27c1) },
    /* 232 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x7479027ea1000000) },
    /* 233 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x788cd40268f39641) },
    /* 234 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x7cc07b437ecf6100) },
    /* 235 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x8114cc6220762061) },
    /* 236 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x858aa0135be10000) },
    /* 237 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x8a22d3b53c54c321) },
    /* 238 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x8ede496339f34100) },
    /* 239 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x93bde80aec3a1481) },
    /* 240 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x98c29b8100000000) },
    /* 241 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x9ded549671832381) },
    /* 242 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0xa33f092e0b1ac100) },
    /* 243 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0xa8b8b452291fe821) },
    /* 244 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0xae5b564ac3a10000) },
    /* 245 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0xb427f4b3be74c361) },
    /* 246 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0xba1f9a938041e100) },
    /* 247 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0xc0435871d1110f41) },
    /* 248 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0xc694446f01000000) },
    /* 249 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0xcd137a5b57ac3ec1) },
    /* 250 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0xd3c21bcecceda100) },
    /* 251 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0xdaa150410b788de1) },
    /* 252 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0xe1b24521be010000) },
    /* 253 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0xe8f62df12777c1a1) },
    /* 254 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0xf06e445906fc0100) },
    /* 255 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0xf81bc845c81bf801) },
    /* 256 */ Base { digits_per_limb: 8, big_base: ::ll::limb::Limb(0x8) },
];
