#!/usr/bin/env python3

import argparse
import math
from pathlib import Path

WIDTH = 38
HEIGHT = 17
FRAME_COUNT = 36
FRAME_DIR = (
    Path(__file__).resolve().parents[1]
    / "agent9527-rs"
    / "tui"
    / "frames"
    / "agent9527"
)
BAYER_4X4 = (
    (0, 8, 2, 10),
    (12, 4, 14, 6),
    (3, 11, 1, 9),
    (15, 7, 13, 5),
)
DENSITY_CHARS = " ░▒▓█"


def ellipse_distance(
    x: float, y: float, cx: float, cy: float, rx: float, ry: float
) -> float:
    return ((x - cx) / rx) ** 2 + ((y - cy) / ry) ** 2


def hat_density(x: int, y: int) -> float:
    px = x + 0.5
    py = y + 0.5
    center_x = WIDTH / 2
    density = 0.0

    crown_top = 2.0
    crown_bottom = 10.2
    if crown_top <= py <= crown_bottom:
        progress = (py - crown_top) / (crown_bottom - crown_top)
        half_width = 7.2 + 4.4 * progress
        horizontal = abs(px - center_x)
        top_curve = ellipse_distance(px, py, center_x, 3.4, 8.4, 2.1)
        inside_crown = horizontal <= half_width and (py >= 3.3 or top_curve <= 1.0)
        if inside_crown:
            density = 0.56
            edge_distance = half_width - horizontal
            if edge_distance < 1.15 or py < 3.2:
                density = 1.0
            elif py < 4.6:
                density = 0.72

    if 7.9 <= py <= 10.2:
        progress = (py - crown_top) / (crown_bottom - crown_top)
        half_width = 7.2 + 4.4 * progress
        if abs(px - center_x) <= half_width:
            density = max(density, 0.9)

    brim_distance = ellipse_distance(px, py, center_x, 11.4, 18.2, 3.7)
    if brim_distance <= 1.0 and 8.8 <= py <= 14.8:
        density = max(density, 0.6)
        if brim_distance >= 0.72 or py < 9.7:
            density = max(density, 0.95)
        if 11.0 <= py <= 12.4:
            density = max(density, 0.76)

    inner_cutout = ellipse_distance(px, py, center_x, 10.6, 11.4, 1.35)
    if py >= 10.2 and inner_cutout <= 1.0:
        density *= 0.42

    return min(density, 1.0)


def frame_character(base_density: float, x: int, y: int, frame_index: int) -> str:
    if base_density == 0.0:
        return " "

    angle = 2.0 * math.pi * frame_index / FRAME_COUNT
    reveal = 0.5 - 0.5 * math.cos(angle)
    pulse = 0.34 + 0.66 * reveal
    highlight = 0.09 * math.sin(x * 0.72 + y * 1.17 - angle * 2.0)
    value = max(0.0, min(1.0, base_density * pulse + highlight))

    threshold = (BAYER_4X4[y % 4][x % 4] + 0.5) / 16.0
    if value < threshold * 0.58:
        return " "

    level = min(4, max(1, int(round(value * 4))))
    return DENSITY_CHARS[level]


def render_frame(frame_index: int) -> str:
    rows = []
    for y in range(HEIGHT):
        row = "".join(
            frame_character(hat_density(x, y), x, y, frame_index) for x in range(WIDTH)
        )
        rows.append(row)
    return "\n".join(rows)


def expected_frames() -> dict[Path, str]:
    return {
        FRAME_DIR / f"frame_{index + 1}.txt": render_frame(index)
        for index in range(FRAME_COUNT)
    }


def check_frames(frames: dict[Path, str]) -> int:
    stale = []
    for path, expected in frames.items():
        if not path.exists() or path.read_text(encoding="utf-8") != expected:
            stale.append(path)
    if stale:
        for path in stale:
            print(f"stale: {path}")
        return 1
    print(f"verified {len(frames)} fedora animation frames")
    return 0


def write_frames(frames: dict[Path, str]) -> int:
    FRAME_DIR.mkdir(parents=True, exist_ok=True)
    for path, contents in frames.items():
        path.write_text(contents, encoding="utf-8")
    print(f"wrote {len(frames)} fedora animation frames to {FRAME_DIR}")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Generate the Agent9527 fedora TUI animation."
    )
    parser.add_argument(
        "--check", action="store_true", help="Verify generated frames are current."
    )
    args = parser.parse_args()

    frames = expected_frames()
    return check_frames(frames) if args.check else write_frames(frames)


if __name__ == "__main__":
    raise SystemExit(main())
