#!/usr/bin/env python3
"""Generate welcome images for NSIS installer"""
import math
import os
from PIL import Image, ImageDraw, ImageFont

def create_welcome_image(width, height, filename):
    """Create a welcome image for the installer"""
    # Create image with gradient background
    img = Image.new('RGB', (width, height), (26, 26, 46))
    draw = ImageDraw.Draw(img)

    # Draw gradient background
    for y in range(height):
        r = int(26 + (y / height) * 20)
        g = int(26 + (y / height) * 15)
        b = int(46 + (y / height) * 30)
        draw.line([(0, y), (width, y)], fill=(r, g, b))

    # Draw pixel art clock (simplified)
    center_x = width // 2
    center_y = height // 2 - 20

    # Clock circle
    radius = 60
    for angle in range(0, 360, 2):
        x = center_x + int(radius * math.cos(math.radians(angle)))
        y = center_y + int(radius * math.sin(math.radians(angle)))
        draw.rectangle([x-2, y-2, x+2, y+2], fill=(48, 71, 94))

    # Clock hands
    # Hour hand
    for i in range(30):
        x = center_x + i // 2
        y = center_y - i
        draw.rectangle([x, y, x+2, y+2], fill=(233, 69, 96))

    # Minute hand
    for i in range(45):
        x = center_x + i // 3
        y = center_y - i * 2 // 3
        draw.rectangle([x, y, x+1, y+1], fill=(83, 215, 105))

    # Center dot
    draw.rectangle([center_x-4, center_y-4, center_x+4, center_y+4], fill=(245, 166, 35))

    # Text
    try:
        font_path = os.path.join(os.environ.get("WINDIR", r"C:\Windows"), "Fonts", "arial.ttf")
        font = ImageFont.truetype(font_path, 24)
        small_font = ImageFont.truetype(font_path, 14)
    except OSError:
        font = ImageFont.load_default()
        small_font = font

    # Title
    title = "BitClock"
    draw.text((center_x - 50, center_y + 80), title, fill=(233, 69, 96), font=font)

    # Subtitle
    subtitle = "像素风使用时长追踪器"
    draw.text((center_x - 80, center_y + 110), subtitle, fill=(200, 200, 200), font=small_font)

    # Save
    img.save(filename)
    print(f"Generated {filename}")

def main():
    os.makedirs('src-tauri/nsis', exist_ok=True)

    # Header image (150x57 for NSIS)
    create_welcome_image(150, 57, 'src-tauri/nsis/header.bmp')

    # Sidebar image (164x314 for NSIS)
    create_welcome_image(164, 314, 'src-tauri/nsis/sidebar.bmp')

if __name__ == '__main__':
    main()
