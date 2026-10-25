"""Cuts a pet sprite sheet drawn on an invisible grid into aligned, scaled frames.

    grid.py <sheet.png> <rows> <cols> <cells> <out-dir> <name>
            [--size K] [--drop K,...] [--align feet|top|left] [--height PX]
            [--part K:NAME]

Needs Pillow, numpy and scipy. The sheets come from Codex, one action each,
with the character centred in each cell of the grid.

Cells are numbered from 1 in reading order; the first <cells> are used.
Cell K (--size, default 1) shows the standard standing pose: the whole sheet
is scaled so that figure is --height pixels tall (default 200), which keeps
every action at the size of the standing image. Cells in --drop are only
there for that and are left out of the output. Cell K in --part (beyond
<cells>) holds a prop drawn on its own, such as something she holds; it is
written as <NAME>.png at the same scale.

With `feet`, frames stand on one line and are centred on their feet; with
`top`, the raised hand at the top of the figure is the fixed point; with
`left`, frames stand on one line and their left edges (against a wall) line
up. The JSON gets the fixed point's position in the frame as `anchor`, and
with `top`, each frame's feet as `feet`.

Figures drawn a little too big spill over their cell's edge, so each blob
of pixels goes to the cell its centre is in rather than being cut on the
grid line; a blob that joins two figures is split where they meet. Writes
<name>.png (one row of frames) and <name>.json.
"""
import argparse
import json

import numpy as np
from PIL import Image
from scipy import ndimage

parser = argparse.ArgumentParser()
parser.add_argument('sheet')
parser.add_argument('rows', type=int)
parser.add_argument('cols', type=int)
parser.add_argument('cells', type=int)
parser.add_argument('out_dir')
parser.add_argument('name')
parser.add_argument('--size', type=int, default=1)
parser.add_argument('--drop', default='')
parser.add_argument('--align', choices=['feet', 'top', 'left'], default='feet')
parser.add_argument('--height', type=int, default=200)
parser.add_argument('--part')
args = parser.parse_args()
drop = {int(k) for k in args.drop.split(',') if k}

pixels = np.asarray(Image.open(args.sheet).convert('RGBA')).astype(np.float32)
# Nearly transparent haze around the figures would show on a dark page.
pixels[..., 3] = np.where(pixels[..., 3] < 16, 0, pixels[..., 3])
# Transparent pixels keep a colour (a glow in some viewers): clear it so
# resampling cannot bleed it into the edges.
pixels[pixels[..., 3] == 0] = 0
cell_h = pixels.shape[0] // args.rows
cell_w = pixels.shape[1] // args.cols

visible = pixels[..., 3] > 0
labels, count = ndimage.label(visible, structure=np.ones((3, 3)))
ids = np.arange(1, count + 1)
sizes = ndimage.sum(visible, labels, ids)
centres = ndimage.center_of_mass(visible, labels, ids)
ys, xs = np.indices(visible.shape)
cell_at = np.minimum(ys // cell_h, args.rows - 1) * args.cols + np.minimum(xs // cell_w, args.cols - 1) + 1
owner_of = np.zeros(count + 1, int)
for i, (cy, cx) in zip(ids, centres):
    # Specks of a few pixels are noise.
    if sizes[i - 1] >= 20:
        owner_of[i] = min(int(cy) // cell_h, args.rows - 1) * args.cols + min(int(cx) // cell_w, args.cols - 1) + 1
owner = owner_of[labels]


def joins(counts):
    """Whether pixels counted per cell are two figures, not one spilling over:
    a spilling figure puts only its feet or a hand in the next cell."""
    first, second = np.sort(counts)[-1], np.sort(counts)[-2]
    return second > 0.3 * first


def split(blob):
    """Splits a blob joining several figures where they touch.

    Figures usually touch only at their soft edges: each solid core (alpha
    above 240) goes to the cell its centre is in, and every other pixel to
    the nearest core. A core that is itself two figures is cut on the grid
    line.
    """
    ys, xs = np.nonzero(blob)
    box = (slice(ys.min(), ys.max() + 1), slice(xs.min(), xs.max() + 1))
    inside = blob[box]
    cores, n = ndimage.label(inside & (pixels[box][..., 3] > 240), structure=np.ones((3, 3)))
    seeds = np.zeros(inside.shape, int)
    for j, (cy, cx) in zip(range(1, n + 1), ndimage.center_of_mass(cores > 0, cores, range(1, n + 1))):
        core = cores == j
        counts = np.bincount(cell_at[box][core], minlength=args.rows * args.cols + 1)
        if joins(counts):
            print('warning: two figures share solid pixels; cut on the grid line')
            seeds[core] = cell_at[box][core]
        else:
            seeds[core] = cell_at[box][int(cy), int(cx)]
    if n == 0:
        return cell_at[box][inside]
    _, (iy, ix) = ndimage.distance_transform_edt(seeds == 0, return_indices=True)
    return seeds[iy, ix][inside]


for i in ids:
    blob = labels == i
    counts = np.bincount(cell_at[blob], minlength=args.rows * args.cols + 1)
    if sizes[i - 1] >= 20 and joins(counts):
        print(f'note: figures in cells {sorted(np.argsort(counts)[-2:].tolist())} touch; split where they meet')
        owner[blob] = split(blob)

part = None
if args.part:
    k, part_name = args.part.split(':')
    part = (int(k), part_name)

figures = {}
for k in list(range(1, args.cells + 1)) + ([part[0]] if part else []):
    own = owner == k
    solid = own & (pixels[..., 3] > 32)
    if not solid.any():
        raise SystemExit(f'cell {k} is empty')
    ys, xs = np.nonzero(solid)
    y0, y1, x0, x1 = ys.min(), ys.max() + 1, xs.min(), xs.max() + 1
    row, col = divmod(k - 1, args.cols)
    spill = {
        'top': row * cell_h - y0,
        'bottom': y1 - (row + 1) * cell_h,
        'left': col * cell_w - x0,
        'right': x1 - (col + 1) * cell_w,
    }
    over = [f'{side} by {n}px' for side, n in spill.items() if n > 0]
    if over:
        print(f'note: cell {k} spills over its {", ".join(over)}')
    figure = pixels[y0:y1, x0:x1].copy()
    figure[~own[y0:y1, x0:x1]] = 0
    figures[k] = figure

scale = args.height / figures[args.size].shape[0]


def scaled(figure):
    image = Image.fromarray(figure.clip(0, 255).astype(np.uint8))
    return image.resize((max(1, round(image.width * scale)), max(1, round(image.height * scale))), Image.LANCZOS)


if part:
    prop = scaled(figures.pop(part[0]))
    # Padded like the frames, to an even size.
    size = (prop.width + 4 + prop.width % 2, prop.height + 4 + prop.height % 2)
    canvas = Image.new('RGBA', size)
    canvas.paste(prop, (2, 2))
    canvas.save(f'{args.out_dir}/{part[1]}.png')
    json.dump({'width': size[0], 'height': size[1], 'display': 0.5}, open(f'{args.out_dir}/{part[1]}.json', 'w'))
    print(f'{part[1]}: {size[0]}x{size[1]}')

frames = []
for k, figure in figures.items():
    if k in drop:
        continue
    crop = np.asarray(scaled(figure)).astype(np.float32)
    alpha = crop[..., 3] > 32
    # The fixed point: the middle of the feet (bottom tenth), at the bottom
    # edge; or the middle of the raised hand. The ears reach nearly as high
    # as the hand, so the hand is the blob in the top band that holds the
    # topmost pixel.
    feet = alpha[-max(1, crop.shape[0] // 10) :]
    feet_x = np.nonzero(feet)[1].mean()
    if args.align == 'feet':
        fx, fy = feet_x, crop.shape[0]
    elif args.align == 'left':
        fx, fy = np.nonzero(alpha.any(axis=0))[0][0], crop.shape[0]
    else:
        band = alpha[: max(1, crop.shape[0] * 8 // 100)]
        blobs, _ = ndimage.label(band)
        row = np.nonzero(band.any(axis=1))[0][0]
        ys, xs = np.nonzero(blobs == blobs[row, np.nonzero(band[row])[0][0]])
        fx, fy = xs.mean(), ys.mean()
    frames.append((k, crop, fx, fy, feet_x))

# One frame size for all: the furthest any figure reaches from its fixed point.
left = max(fx for _, _, fx, _, _ in frames)
right = max(c.shape[1] - fx for _, c, fx, _, _ in frames)
up = max(fy for _, _, _, fy, _ in frames)
down = max(c.shape[0] - fy for _, c, _, fy, _ in frames)
pad = 2
# Shown at half size, an odd size would be half a CSS pixel and blur.
width = int(np.ceil(left + right)) + 2 * pad
width += width % 2
height = int(np.ceil(up + down)) + 2 * pad
height += height % 2
point = (pad + left, pad + up)

strip = np.zeros((height, width * len(frames), 4), np.float32)
for i, (_, crop, fx, fy, _) in enumerate(frames):
    x = int(round(point[0] - fx)) + i * width
    y = int(round(point[1] - fy))
    strip[y : y + crop.shape[0], x : x + crop.shape[1]] = crop
Image.fromarray(strip.clip(0, 255).astype(np.uint8)).save(f'{args.out_dir}/{args.name}.png')

meta = {
    'frames': len(frames),
    'width': width,
    'height': height,
    'display': 0.5,
    'anchor': {'x': round(point[0] / width, 3), 'y': round(point[1] / height, 3)},
}
if args.align == 'top':
    # Where each frame's feet are, to set her down or pick her up there.
    meta['feet'] = [
        {'x': round((point[0] - fx + feet_x) / width, 3), 'y': round((point[1] - fy + crop.shape[0]) / height, 3)}
        for _, crop, fx, fy, feet_x in frames
    ]
json.dump(meta, open(f'{args.out_dir}/{args.name}.json', 'w'))
print(f'{args.name}: cells {[k for k, *_ in frames]} -> {len(frames)} frames of {width}x{height}, scale {scale:.3f}')
