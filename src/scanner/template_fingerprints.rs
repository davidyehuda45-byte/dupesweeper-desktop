use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemplateCategory {
    /// Informational template / documentation / boilerplate (Safe to delete, enters auto-suggest)
    Documentation,
    /// Executable code / migration / config / seeder (Protected from auto-selection, informational only)
    Functional,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateFingerprint {
    pub framework: &'static str,
    pub file_relative_path: &'static str,
    pub version: Option<&'static str>,
    pub blake3_hash: &'static str,
    pub category: TemplateCategory,
}

/// Hardcoded database of official starter kit fingerprints.
pub static TEMPLATE_DATABASE: &[TemplateFingerprint] = &[
    // === Laravel (11.x / 12.x official starter kit) ===
    TemplateFingerprint {
        framework: "Laravel",
        file_relative_path: "README.md",
        version: Some("11.x/12.x"),
        blake3_hash: "c35d26f139fc16e2aaaa252e5d9eb98ae120275b594027d5282d5b1be67fee3b",
        category: TemplateCategory::Documentation,
    },
    TemplateFingerprint {
        framework: "Laravel",
        file_relative_path: ".gitignore",
        version: Some("11.x/12.x"),
        blake3_hash: "f3276ff9118bc26974bd21e874e27f1a09bc4afd5e8da217acdcf3a154b55189",
        category: TemplateCategory::Documentation,
    },
    TemplateFingerprint {
        framework: "Laravel",
        file_relative_path: ".editorconfig",
        version: Some("11.x/12.x"),
        blake3_hash: "44ce68bb443065dbece10597524be6d810f2ab095f0330971869240dbb266bbe",
        category: TemplateCategory::Documentation,
    },
    TemplateFingerprint {
        framework: "Laravel",
        file_relative_path: "resources/views/welcome.blade.php",
        version: Some("11.x/12.x"),
        blake3_hash: "533be867d1282be0a1bd4bcdb5b806d5e72efc7f3a7809ac9bb23ee204263075",
        category: TemplateCategory::Documentation,
    },
    TemplateFingerprint {
        framework: "Laravel",
        file_relative_path: "database/migrations/0001_01_01_000000_create_users_table.php",
        version: Some("11.x/12.x"),
        blake3_hash: "e4b76df2d2048e6a373a8f88001a06930f08de587011a84300386679bd08b082",
        category: TemplateCategory::Functional,
    },
    TemplateFingerprint {
        framework: "Laravel",
        file_relative_path: "database/migrations/0001_01_01_000001_create_cache_table.php",
        version: Some("11.x/12.x"),
        blake3_hash: "93e0ebc733c2fbedc1c63912f65767ab5b9e90bcb2ed0409fc3aefc4c19867f3",
        category: TemplateCategory::Functional,
    },
    TemplateFingerprint {
        framework: "Laravel",
        file_relative_path: "database/migrations/0001_01_01_000002_create_jobs_table.php",
        version: Some("11.x/12.x"),
        blake3_hash: "5694d90a1538161a13b77c5370c4f4e277531cd1fc1ea0ab51249da2a877ed9e",
        category: TemplateCategory::Functional,
    },
    TemplateFingerprint {
        framework: "Laravel",
        file_relative_path: "database/seeders/DatabaseSeeder.php",
        version: Some("11.x/12.x"),
        blake3_hash: "eb94381c3a40ce5ca75e3574e0af0b9acfde855d1dc4cf16a049c3aacf3a0e24",
        category: TemplateCategory::Functional,
    },
    TemplateFingerprint {
        framework: "Laravel",
        file_relative_path: "routes/web.php",
        version: Some("11.x/12.x"),
        blake3_hash: "0d907339a16924acea3eab550c0ab54c13308c37225a23ae840393815dba55d7",
        category: TemplateCategory::Functional,
    },
    TemplateFingerprint {
        framework: "Laravel",
        file_relative_path: "routes/console.php",
        version: Some("11.x/12.x"),
        blake3_hash: "3c74d8ea6432f0128f97b438e4ab5d347ecc0b1cd3c34155be9d027efd2f42c1",
        category: TemplateCategory::Functional,
    },
    TemplateFingerprint {
        framework: "Laravel",
        file_relative_path: "bootstrap/app.php",
        version: Some("11.x/12.x"),
        blake3_hash: "9f3f720902985e3e98742195319f44f9033783f6523e57e673ddb27594ddb6cf",
        category: TemplateCategory::Functional,
    },

    // === Next.js / React (14.x / 15.x official starter kit) ===
    TemplateFingerprint {
        framework: "Next.js",
        file_relative_path: "README.md",
        version: Some("14.x/15.x"),
        blake3_hash: "516b4567b54983fef95143c2886d6e208838827596bab651a8bdabdfd1abb325",
        category: TemplateCategory::Documentation,
    },
    TemplateFingerprint {
        framework: "Next.js",
        file_relative_path: ".gitignore",
        version: Some("14.x/15.x"),
        blake3_hash: "4479a78b2b8ee64bd61c9903ab1f4b69ef09779c2a24376639de277b0c013a36",
        category: TemplateCategory::Documentation,
    },
    TemplateFingerprint {
        framework: "Next.js",
        file_relative_path: "public/next.svg",
        version: Some("14.x/15.x"),
        blake3_hash: "e3b3a22e1602686c634aece1fa6025d3fcb38c5aef558227bfb7b4f8c2335512",
        category: TemplateCategory::Documentation,
    },
    TemplateFingerprint {
        framework: "Next.js",
        file_relative_path: "public/vercel.svg",
        version: Some("14.x/15.x"),
        blake3_hash: "bcad5f0d811587a2f12b6774acc6b73017e2b5c8d42e00d58e2d0bfcdd5e7bb8",
        category: TemplateCategory::Documentation,
    },
    TemplateFingerprint {
        framework: "Next.js",
        file_relative_path: "src/app/favicon.ico",
        version: Some("14.x/15.x"),
        blake3_hash: "17937a2e497790de39dadb9b2534127347c942e3b8d76379545933a01e7798b0",
        category: TemplateCategory::Documentation,
    },
    TemplateFingerprint {
        framework: "Next.js",
        file_relative_path: "next.config.ts",
        version: Some("14.x/15.x"),
        blake3_hash: "cffa485cc39e35da1ff735d42c7a4e13fddb619d7d284a6ecced9e971a1e7f12",
        category: TemplateCategory::Functional,
    },
    TemplateFingerprint {
        framework: "Next.js",
        file_relative_path: "src/app/layout.tsx",
        version: Some("14.x/15.x"),
        blake3_hash: "5ee9c1a92a3844b2f11d8855beca05038e7e62e29ad78bbfaccb52ce8b397d20",
        category: TemplateCategory::Functional,
    },
    TemplateFingerprint {
        framework: "Next.js",
        file_relative_path: "src/app/page.tsx",
        version: Some("14.x/15.x"),
        blake3_hash: "0814bb266ca18a08b24aa6f22e542a30e32ba1d25e282bf18f41e27eb4ad5022",
        category: TemplateCategory::Functional,
    },

    // === Vite + React (Official create-vite starter kit) ===
    TemplateFingerprint {
        framework: "Vite + React",
        file_relative_path: "README.md",
        version: Some("Latest"),
        blake3_hash: "d5284473d8d8734ee17a6aa7577131c3b7f043022cb2c4c9f09a0a60d1eccfd3",
        category: TemplateCategory::Documentation,
    },
    TemplateFingerprint {
        framework: "Vite + React",
        file_relative_path: ".gitignore",
        version: Some("Latest"),
        blake3_hash: "c88d7994f219daeb89538b989d43875f2f495c736f89e2ea9082407f87b962f0",
        category: TemplateCategory::Documentation,
    },
    TemplateFingerprint {
        framework: "Vite + React",
        file_relative_path: "src/assets/vite.svg",
        version: Some("Latest"),
        blake3_hash: "a0deb7a045a850a804d70b7637a8736292c40eb9aab8fb8b5ae8d6bb2acd2cc6",
        category: TemplateCategory::Documentation,
    },
    TemplateFingerprint {
        framework: "Vite + React",
        file_relative_path: "src/assets/react.svg",
        version: Some("Latest"),
        blake3_hash: "679fc68d782e2ec79add7891c0181421808eecff388149ad3cd73e9bf41ab113",
        category: TemplateCategory::Documentation,
    },
    TemplateFingerprint {
        framework: "Vite + React",
        file_relative_path: "index.html",
        version: Some("Latest"),
        blake3_hash: "195ef9bf9ecb2b21fed84aee14392000eb53ca9dcb89f39b4c5f6ae83e37b041",
        category: TemplateCategory::Functional,
    },
    TemplateFingerprint {
        framework: "Vite + React",
        file_relative_path: "vite.config.js",
        version: Some("Latest"),
        blake3_hash: "ac27405af2ff179977a631ac417520dba920c38fb52b4b1a87ab97ea76bf767e",
        category: TemplateCategory::Functional,
    },
    TemplateFingerprint {
        framework: "Vite + React",
        file_relative_path: "src/main.jsx",
        version: Some("Latest"),
        blake3_hash: "d4f4b5fb020e03fbebdcd802b0e927d47770d38f517df46ca0c7c44c16861181",
        category: TemplateCategory::Functional,
    },

    // === Vite + Vue (Official create-vite starter kit) ===
    TemplateFingerprint {
        framework: "Vite + Vue",
        file_relative_path: "README.md",
        version: Some("Latest"),
        blake3_hash: "7525bbd60992b43e7132baa3c123b3250ec09d3d3743beea08dfb61d20abfd60",
        category: TemplateCategory::Documentation,
    },
    TemplateFingerprint {
        framework: "Vite + Vue",
        file_relative_path: "src/assets/vue.svg",
        version: Some("Latest"),
        blake3_hash: "fa22d4892e466066fc8f68676b4a87cf8ab40f1f328dcbb0d488bd7ab03b5d4f",
        category: TemplateCategory::Documentation,
    },
    TemplateFingerprint {
        framework: "Vite + Vue",
        file_relative_path: "vite.config.js",
        version: Some("Latest"),
        blake3_hash: "fde17da78074d86bb3149563deefd05bb31d694488399ab091d8b04a83d4dbfe",
        category: TemplateCategory::Functional,
    },
];

static FINGERPRINT_INDEX: OnceLock<HashMap<&'static str, &'static TemplateFingerprint>> =
    OnceLock::new();

/// Performs fast O(1) hash lookup to check if a file matches any known framework template.
pub fn match_template(blake3_hash: &str) -> Option<&'static TemplateFingerprint> {
    let index = FINGERPRINT_INDEX.get_or_init(|| {
        let mut map = HashMap::with_capacity(TEMPLATE_DATABASE.len());
        for fp in TEMPLATE_DATABASE {
            map.insert(fp.blake3_hash, fp);
        }
        map
    });

    index.get(blake3_hash).copied()
}
