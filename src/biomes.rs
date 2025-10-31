pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

pub struct Biome {
    pub asset_path: &'static str,
    pub sea_color: Color,
    pub shore_color: Color,
    pub land_color: Color,
}

pub const ALPINE: Biome = Biome {
    asset_path: "alpine/",
    sea_color: Color {
        r: 120,
        g: 150,
        b: 220,
        a: 255,
    },
    shore_color: Color {
        r: 160,
        g: 130,
        b: 90,
        a: 255,
    },
    land_color: Color {
        r: 40,
        g: 80,
        b: 50,
        a: 255,
    },
};

pub const DESERT: Biome = Biome {
    asset_path: "desert/",
    sea_color: Color {
        r: 120,
        g: 150,
        b: 220,
        a: 255,
    },
    shore_color: Color {
        r: 130,
        g: 100,
        b: 60,
        a: 255,
    },
    land_color: Color {
        r: 160,
        g: 130,
        b: 90,
        a: 255,
    },
};

pub const TUNDRA: Biome = Biome {
    asset_path: "tundra/",
    sea_color: Color {
        r: 120,
        g: 150,
        b: 220,
        a: 255,
    },
    shore_color: Color {
        r: 140,
        g: 145,
        b: 150,
        a: 255,
    },
    land_color: Color {
        r: 248,
        g: 248,
        b: 255,
        a: 255,
    },
};

pub const ALIEN: Biome = Biome {
    asset_path: "alien/",
    sea_color: Color {
        r: 200,
        g: 50,
        b: 30,
        a: 255,
    },
    shore_color: Color {
        r: 65,
        g: 70,
        b: 75,
        a: 255,
    },
    land_color: Color {
        r: 25,
        g: 15,
        b: 35,
        a: 255,
    },
};