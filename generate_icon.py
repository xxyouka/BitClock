#!/usr/bin/env python3
"""Generate a simple pixel art icon for BitClock"""
from PIL import Image, ImageDraw
import os

def create_icon(size):
    """Create a pixel art clock icon"""
    img = Image.new('RGBA', (size, size), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)

    s = size // 16
    center = size // 2
    r = size // 2 - s

    # Background circle (dark blue)
    draw.ellipse([center - r, center - r, center + r, center + r], fill=(26, 26, 46, 255))

    # Border (pixel style)
    border_width = max(1, s // 2)
    draw.ellipse([center - r, center - r, center + r, center + r], outline=(48, 71, 94, 255), width=border_width)

    # Clock hands
    hand_color = (233, 69, 96, 255)
    # Hour hand (short, thick)
    draw.rectangle([center - s // 2, center - 3 * s, center + s // 2, center - s], fill=hand_color)

    # Minute hand (long, thin)
    minute_color = (83, 215, 105, 255)
    draw.rectangle([center, center - 5 * s, center + max(1, s // 3), center], fill=minute_color)

    # Center dot
    dot_r = max(1, s)
    draw.ellipse([center - dot_r, center - dot_r, center + dot_r, center + dot_r], fill=(245, 166, 35, 255))

    return img

def main():
    # Create icons directory if it doesn't exist
    os.makedirs('src-tauri/icons', exist_ok=True)

    # Generate different sizes
    sizes = {
        '32x32.png': 32,
        '128x128.png': 128,
        '128x128@2x.png': 256,
    }

    for filename, size in sizes.items():
        img = create_icon(size)
        img.save(f'src-tauri/icons/{filename}')
        print(f'Generated {filename}')

    # Generate ICO file (Windows)
    ico_sizes = [16, 32, 48, 64, 128, 256]
    ico_images = [create_icon(s) for s in ico_sizes]
    ico_images[0].save(
        'src-tauri/icons/icon.ico',
        format='ICO',
        sizes=[(s, s) for s in ico_sizes],
        append_images=ico_images[1:]
    )
    print('Generated icon.ico')

if __name__ == '__main__':
    main()
