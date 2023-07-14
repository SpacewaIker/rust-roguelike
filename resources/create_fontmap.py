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

draw.rectangle((0, SIZE * 11, SIZE - 1, SIZE * 12 - 1), fill=BAR_EMPTY)
draw.line((0, SIZE * 11, SIZE - 1, SIZE * 11), fill=BAR_BORDER)
draw.line((0, SIZE * 12 - 1, SIZE - 1, SIZE * 12 - 1), fill=BAR_BORDER)

draw.rectangle((SIZE * 2, SIZE * 11, SIZE * 3 - 1, SIZE * 12 - 1), fill=BAR_FILL)
draw.line((SIZE * 2, SIZE * 11, SIZE * 3 - 1, SIZE * 11), fill=BAR_BORDER)
draw.line((SIZE * 2, SIZE * 12 - 1, SIZE * 3 - 1, SIZE * 12 - 1), fill=BAR_BORDER)

img.save(OUTPUT)
