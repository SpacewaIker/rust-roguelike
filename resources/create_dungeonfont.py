from PIL import Image

PREFIX = "resources/glyphs/"

glyphs = {
    # items ------------------------------
    16: "PotionRed.PNG",
    19: "Sword01.PNG",
    20: "SwordTwoHanded.PNG",
    21: "SwordMedievalMagical.PNG",
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
    96: ("dg_grounds32.gif", 1, 1),  # grass
    97: ("dg_grounds32.gif", 1, 6),  # tree1
    98: ("dg_grounds32.gif", 4, 6),  # tree2
    99: ("dg_grounds32.gif", 7, 6),  # tree3
    100: ("dg_grounds32.gif", 7, 15),  # tree4
    101: ("dg_grounds32.gif", 7, 18),  # rocks
    # cave theme
    102: ("dg_grounds32.gif", 6, 0),  # cave floor
    103: ("dg_features32.gif", 2, 10),  # cave wall
    104: ("dg_features32.gif", 4, 1),  # cave wall2
    # dungeon theme
    105: ("dg_grounds32.gif", 7, 0),  # dungeon floor
    106: ("dg_features32.gif", 3, 10),  # dungeon wall
    # player ------------------------------
    160: ("RPGCharacterSprites32x32.png", 1, 11),  # front
    161: ("RPGCharacterSprites32x32.png", 5, 11),  # back
    162: ("RPGCharacterSprites32x32.png", 9, 11),  # right
    163: ("RPGCharacterSprites32x32.png", 9, 11, "flip"),  # left
}

dungeonfont = Image.new("RGBA", (32 * 16, 32 * 16), (0, 0, 0, 0))

for index, glyph in glyphs.items():
    x = (index % 16) * 32
    y = (index // 16) * 32

    if isinstance(glyph, str):
        img = Image.open(PREFIX + glyph)
    else:
        glyph, x_, y_, *flip = glyph
        img = Image.open(PREFIX + glyph)
        x_ *= 32
        y_ *= 32
        img = img.crop((x_, y_, x_ + 32, y_ + 32))
        if len(flip) > 0:
            img = img.transpose(Image.FLIP_LEFT_RIGHT)

    if img.size != (32, 32):
        border = (img.size[0] - 32) // 2
        img = img.crop((border, border, border + 32, border + 32))

    for i in range(img.size[0]):
        for j in range(img.size[1]):
            if img.getpixel((i, j)) in [(255, 0, 255), (255, 0, 255, 255)]:
                img.putpixel((i, j), (0, 0, 0, 0))

    dungeonfont.paste(img, (x, y))

dungeonfont.save("resources/mydungeonfont.png")
