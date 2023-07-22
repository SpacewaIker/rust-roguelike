from PIL import Image, ImageDraw, ImageFont

FONT = "resources/alagard.ttf"
SIZE = 16
OUTPUT = "resources/alagard_fontmap.png"

BAR_BORDER = (255, 255, 255, 255)
BAR_FILL = (255, 255, 255, 150)
BAR_EMPTY = (255, 255, 255, 50)

font = ImageFont.truetype(FONT, SIZE)
img = Image.new("RGBA", (SIZE * 16, SIZE * 16), color=(0, 0, 0, 0))
draw = ImageDraw.Draw(img)


def index_to_rect(index):
    x = (index % 16) * SIZE
    y = (index // 16) * SIZE
    return (x, y, x + SIZE - 1, y + SIZE - 1)


for c in range(256):
    x = (c % 16) * SIZE + SIZE // 2
    y = (c // 16) * SIZE + SIZE // 2
    char = chr(c)
    draw.text(
        (x, y),
        char,
        font=font,
        anchor="mm",
        fill=(255, 255, 255, 255),
    )

# loading/health bar
draw.rectangle((0, SIZE * 11, SIZE - 1, SIZE * 12 - 1), fill=BAR_EMPTY)
draw.line((0, SIZE * 11, SIZE - 1, SIZE * 11), fill=BAR_BORDER)
draw.line((0, SIZE * 12 - 1, SIZE - 1, SIZE * 12 - 1), fill=BAR_BORDER)

draw.rectangle((SIZE * 2, SIZE * 11, SIZE * 3 -
               1, SIZE * 12 - 1), fill=BAR_FILL)
draw.line((SIZE * 2, SIZE * 11, SIZE * 3 - 1, SIZE * 11), fill=BAR_BORDER)
draw.line((SIZE * 2, SIZE * 12 - 1, SIZE * 3 -
          1, SIZE * 12 - 1), fill=BAR_BORDER)

# rectangle glyphs
# upper left: 218
# upper right: 191
# lower left: 192
# lower right: 217
# horizontal: 196
# vertical: 179
x1, y1, x2, y2 = index_to_rect(218)
draw.rectangle((x1, y1, x2, y2), fill=(255, 255, 255, 0))
draw.line(
    (x1 + SIZE // 2, y1 + SIZE // 2, x2 + 1 - SIZE // 2, y2),
    fill=(255, 255, 255, 255),
    width=4,
)
draw.line(
    (x1 + SIZE // 2, y1 + SIZE // 2, x2, y2 + 1 - SIZE // 2),
    fill=(255, 255, 255, 255),
    width=4,
)

x1, y1, x2, y2 = index_to_rect(191)
draw.rectangle((x1, y1, x2, y2), fill=(255, 255, 255, 0))
draw.line(
    (x1 + SIZE // 2, y1 + SIZE // 2, x2 + 1 - SIZE // 2, y2),
    fill=(255, 255, 255, 255),
    width=4,
)
draw.line(
    (x1, y1 + SIZE // 2, x2 - SIZE // 2, y2 + 1 - SIZE // 2),
    fill=(255, 255, 255, 255),
    width=4,
)

x1, y1, x2, y2 = index_to_rect(192)
draw.rectangle((x1, y1, x2, y2), fill=(255, 255, 255, 0))
draw.line(
    (x1 + SIZE // 2, y1, x2 + 1 - SIZE // 2, y2 + 1 - SIZE // 2),
    fill=(255, 255, 255, 255),
    width=4,
)
draw.line(
    (x1 + SIZE // 2, y1 + SIZE // 2, x2, y2 + 1 - SIZE // 2),
    fill=(255, 255, 255, 255),
    width=4,
)

x1, y1, x2, y2 = index_to_rect(217)
draw.rectangle((x1, y1, x2, y2), fill=(255, 255, 255, 0))
draw.line(
    (x1 + SIZE // 2, y1, x2 + 1 - SIZE // 2, y2 + 1 - SIZE // 2),
    fill=(255, 255, 255, 255),
    width=4,
)
draw.line(
    (x1, y1 + SIZE // 2, x2 + 1 - SIZE // 2, y2 + 1 - SIZE // 2),
    fill=(255, 255, 255, 255),
    width=4,
)

x1, y1, x2, y2 = index_to_rect(196)
draw.rectangle((x1, y1, x2, y2), fill=(255, 255, 255, 0))
draw.line(
    (x1, y1 + SIZE // 2, x2, y2 + 1 - SIZE // 2),
    fill=(255, 255, 255, 255),
    width=4,
)

x1, y1, x2, y2 = index_to_rect(179)
draw.rectangle((x1, y1, x2, y2), fill=(255, 255, 255, 0))
draw.line(
    (x1 + SIZE // 2, y1, x2 + 1 - SIZE // 2, y2),
    fill=(255, 255, 255, 255),
    width=4,
)


img.save(OUTPUT)
