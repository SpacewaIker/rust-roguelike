from PIL import Image, ImageEnhance

PREFIX = "resources/glyphs/"

glyphs = {
    # items ------------------------------
    16: "PotionRed.PNG",
    17: "PotionRoundedTopaz.PNG",
    19: "Sword01.PNG",
    20: "SwordTwoHanded.PNG",
    21: "SwordMedievalMagical.PNG",
    22: "chest.gif",
    23: "ArmorLeatherSoft.PNG",
    24: "ArmorChainMailDouble.PNG",
    25: "ArmorPlatemailRibbed.PNG",
    # monsters -----------------------------
    48: "ImpFighter.PNG",
    49: "ImpFighter2.PNG",
    50: "ImpFighter4.PNG",
    51: "Ghost2Grey.PNG",
    52: "Ghost2GreyRed.PNG",
    53: "Ghost3Grey.PNG",
    54: "Ghost3Blue.PNG",
    55: "OrcFighter4.PNG",
    56: "OrcFighter6.PNG",
    57: "OrcGreenFighter.PNG",
    58: "SkeletonFighter.PNG",
    59: "SkeletonFighter10.PNG",
    60: "SkeletonFighter11.PNG",
    61: "SpectreBlack.PNG",
    62: "SpectreOrange.PNG",
    63: "SpectreSilver.PNG",
    # tiles -------------------------------
    # grass theme
    96: ("dg_grounds32.gif", 0, 1, "darken", 0.8),  # grass
    97: ("dg_grounds32.gif", 3, 6, "darken", 0.8),  # tree1
    98: ("dg_grounds32.gif", 3, 6, "darken", 0.8),  # tree2
    99: ("dg_grounds32.gif", 6, 6, "darken", 0.8),  # tree3
    100: ("dg_grounds32.gif", 6, 15, "darken", 0.8),  # tree4
    101: ("dg_grounds32.gif", 6, 18, "darken", 0.8),  # rocks
    102: ("openSecretDoor21.gif", 0, 0, "darken", 0.9),  # exit
    103: ("openDoor21.gif", 0, 0, "darken", 0.9),  # exit
    # cave theme
    112: ("floor13.gif", 0, 0, "darken", 0.5),  # cave floor
    113: ("dg_features32.gif", 4, 1),  # cave wall
    114: ("dg_features32.gif", 5, 1, "darken", 1.8),  # cave wall
    115: ("dg_features32.gif", 1, 11),  # exit
    116: ("dg_features32.gif", 2, 11),  # exit
    117: ("dg_features32.gif", 1, 12),  # exit
    118: ("dg_features32.gif", 2, 12),  # exit
    # dungeon theme
    128: ("roomFloor32.gif", 0, 0, "darken", 0.5),  # dungeon floor
    129: ("roomFloor32.gif", 0, 0, "darken", 0.6),  # dungeon floor
    130: ("dg_features32.gif", 3, 10),  # dungeon wall
    131: ("dg_features32.gif", 1, 2),  # exit
    132: ("dg_features32.gif", 2, 2),  # exit
    133: ("dg_features32.gif", 1, 7),  # exit
    134: ("dg_features32.gif", 2, 7),  # exit
    # player ------------------------------
    224: ("Soldier 03-3.png", 1, 0),  # front
    225: ("Soldier 03-3.png", 1, 3),  # back
    226: ("Soldier 03-3.png", 1, 2),  # right
    227: ("Soldier 03-3.png", 1, 1),  # left
    # amulet ------------------------------
    240: "sirref.png",
}

dungeonfont = Image.new("RGBA", (32 * 16, 32 * 16), (0, 0, 0, 0))

for index, glyph in glyphs.items():
    x = (index % 16) * 32
    y = (index // 16) * 32

    if isinstance(glyph, str):
        img = Image.open(PREFIX + glyph)
    else:
        glyph, x_, y_, *args = glyph
        img = Image.open(PREFIX + glyph)
        x_ *= 32
        y_ *= 32
        img = img.crop((x_, y_, x_ + 32, y_ + 32))

        if "flip" in args:
            img = img.transpose(Image.FLIP_LEFT_RIGHT)

        if "darken" in args:
            amount = args[args.index("darken") + 1]
            img = img.convert("RGBA")
            img = ImageEnhance.Brightness(img).enhance(amount)

    if img.size != (32, 32):
        border = (img.size[0] - 32) // 2
        img = img.crop((border, border, border + 32, border + 32))

    for i in range(img.size[0]):
        for j in range(img.size[1]):
            if img.getpixel((i, j)) in [(255, 0, 255), (255, 0, 255, 255)]:
                img.putpixel((i, j), (0, 0, 0, 0))

    dungeonfont.paste(img, (x, y))

dungeonfont.save("resources/mydungeonfont.png")
