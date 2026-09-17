"""design.py — design-everything 单一 CLI (L4, stdlib-only)。

子命令:
  list                        列出全部模块与条目数
  search Q [--module M]       FTS 检索 (bm25 排序), 无结果自动回退 LIKE
  read MODULE [--limit N]     读取模块全部条目
  contrast FG BG [--large]    WCAG 对比度与 AA/AAA 判定
  palette HEX [--scheme S]    色相旋转生成配色 (60/30/10 建议角色)
  export BRIEF --format F     由 design-brief.json 导出 css/tailwind/godot
  gate BRIEF [--up-to STAGE]  简报校验: pass/fail + 缺失清单 (无分数)
  init [--out PATH]           生成空 design-brief.json 模板

库不存在时先运行: uv run python scripts/build_db.py
"""
from __future__ import annotations

import argparse
import colorsys
import json
import re
import sqlite3
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DB = ROOT / "data" / "design_knowledge.db"

STAGES = ["analyze", "propose", "color", "layout", "elements", "avoid"]

HEX_RE = re.compile(r"^#(?:[0-9a-fA-F]{3}|[0-9a-fA-F]{6})$")
CJK = re.compile(r"[\u3400-\u9fff\uf900-\ufaff]+")


def connect() -> sqlite3.Connection:
    if not DB.exists():
        sys.exit(f"库不存在: {DB}\n先运行: uv run python scripts/build_db.py")
    conn = sqlite3.connect(DB)
    conn.row_factory = sqlite3.Row
    return conn


def cjk_tokens(text: str) -> list[str]:
    tokens: list[str] = []
    for part in re.split(r"([^\u3400-\u9fff\uf900-\ufaff]+)", text):
        if not part:
            continue
        if CJK.fullmatch(part):
            if len(part) == 1:
                tokens.append(part)
            else:
                tokens.extend(part[i:i + 2] for i in range(len(part) - 1))
        else:
            tokens.extend(w for w in re.split(r"[\s,.;:!?|，。；：！？、]+", part) if w)
    return tokens


def cmd_list(_: argparse.Namespace) -> None:
    with connect() as conn:
        for r in conn.execute("SELECT module_id, entry_count FROM modules ORDER BY module_id"):
            print(f"{r['module_id']:<28} {r['entry_count']}")


def cmd_search(args: argparse.Namespace) -> None:
    tokens = cjk_tokens(args.query)
    if not tokens:
        sys.exit("空查询")
    match = " ".join(tokens)
    sql = [
        "SELECT e.module_id, e.title, e.content, e.tags, bm25(entries_fts) AS rank",
        "FROM entries_fts f JOIN entries e ON e.id = f.rowid",
        "WHERE entries_fts MATCH ?",
    ]
    params: list = [match]
    if args.module:
        sql.append("AND e.module_id = ?")
        params.append(args.module)
    sql.append("ORDER BY rank LIMIT ?")
    params.append(args.limit)
    with connect() as conn:
        rows = conn.execute(" ".join(sql), params).fetchall()
        if not rows:  # 回退: LIKE 精确子串
            like = f"%{args.query}%"
            sql_fb = ("SELECT module_id, title, content, tags FROM entries "
                      "WHERE (content LIKE ? OR title LIKE ? OR tags LIKE ?)")
            p_fb: list = [like, like, like]
            if args.module:
                sql_fb += " AND module_id = ?"
                p_fb.append(args.module)
            rows = conn.execute(sql_fb + " LIMIT ?", p_fb + [args.limit]).fetchall()
    for r in rows:
        i = r["content"].find(args.query)
        window = r["content"][max(0, i - 24):i + 56].replace("\n", " ") if i >= 0 else r["content"][:80]
        print(f"[{r['module_id']}] {r['title']}\n    {window}\n")
    print(f"{len(rows)} 条" if rows else "未命中")


def cmd_read(args: argparse.Namespace) -> None:
    with connect() as conn:
        rows = conn.execute(
            "SELECT title, content, tags, metadata FROM entries WHERE module_id = ? LIMIT ?",
            (args.module, args.limit),
        ).fetchall()
    if not rows:
        sys.exit(f"模块不存在或为空: {args.module} (用 list 查看全部)")
    for r in rows:
        print(f"# {r['title']}\n{r['content']}\n  tags: {r['tags']}  meta: {r['metadata']}\n")


def _srgb_luminance(hex_color: str) -> float:
    h = hex_color.lstrip("#")
    if len(h) == 3:
        h = "".join(c * 2 for c in h)
    rgb = [int(h[i:i + 2], 16) / 255 for i in (0, 2, 4)]
    lin = [c / 12.92 if c <= 0.03928 else ((c + 0.055) / 1.055) ** 2.4 for c in rgb]
    return 0.2126 * lin[0] + 0.7152 * lin[1] + 0.0722 * lin[2]


def contrast_ratio(fg: str, bg: str) -> float:
    lo, hi = sorted((_srgb_luminance(fg), _srgb_luminance(bg)))
    return (hi + 0.05) / (lo + 0.05)


def cmd_contrast(args: argparse.Namespace) -> None:
    for c in (args.fg, args.bg):
        if not HEX_RE.match(c):
            sys.exit(f"非法 HEX: {c}")
    ratio = contrast_ratio(args.fg, args.bg)
    aa, aaa = (3.0, 4.5) if args.large else (4.5, 7.0)
    verdict = "pass" if ratio >= aa else "fail"
    print(f"{args.fg} on {args.bg}  ratio={ratio:.2f}  AA({'大字号' if args.large else '正文'})={verdict}"
          f"  AAA={'pass' if ratio >= aaa else 'fail'}")


def _hex_to_hls(hex_color: str) -> tuple[float, float, float]:
    h = hex_color.lstrip("#")
    if len(h) == 3:
        h = "".join(c * 2 for c in h)
    r, g, b = (int(h[i:i + 2], 16) / 255 for i in (0, 2, 4))
    return colorsys.rgb_to_hls(r, g, b)


def _hls_to_hex(hls: tuple[float, float, float]) -> str:
    r, g, b = colorsys.hls_to_rgb(*hls)
    return "#{:02X}{:02X}{:02X}".format(round(r * 255), round(g * 255), round(b * 255))


SCHEMES: dict[str, list[float]] = {
    "analogous": [-30, 30],
    "complementary": [180],
    "split": [150, 210],
    "tetradic": [90, 180, 270],
}


def cmd_palette(args: argparse.Namespace) -> None:
    if not HEX_RE.match(args.base):
        sys.exit(f"非法 HEX: {args.base}")
    hue, light, sat = _hex_to_hls(args.base)
    derived = [_hls_to_hex(((hue + d / 360) % 1, light, sat)) for d in SCHEMES[args.scheme]]
    secondary, accent = derived[0], derived[-1]
    print(f"主色 60%: {args.base}")
    print(f"辅色 30%: {secondary}   (基色色相 {SCHEMES[args.scheme][0]:+d}°)")
    print(f"点缀 10%: {accent}   (基色色相 {SCHEMES[args.scheme][-1]:+d}°)")
    for h in derived:
        print(f"  候选: {h}")


def _load_brief(path: str) -> dict:
    p = Path(path)
    if not p.exists():
        sys.exit(f"文件不存在: {p}")
    return json.loads(p.read_text(encoding="utf-8"))


def cmd_export(args: argparse.Namespace) -> None:
    brief = _load_brief(args.brief)
    c = brief.get("color", {})
    colors = {k: c.get(k, {}).get("hex", "") for k in ("primary", "secondary", "accent")}
    colors["background"] = c.get("background", "")
    missing = [k for k, v in colors.items() if not v]
    if missing:
        sys.exit(f"brief 缺少色值: {', '.join(missing)}")
    if args.format == "css":
        print(":root {\n" + "\n".join(f"  --color-{k}: {v};" for k, v in colors.items()) + "\n}")
    elif args.format == "tailwind":
        obj = {k: {"DEFAULT": v} for k, v in colors.items()}
        print(json.dumps({"theme": {"extend": {"colors": obj}}}, indent=2))
    elif args.format == "godot":
        print("const COLORS := {")
        for k, v in colors.items():
            print(f'    "{k}": Color("{v}"),')
        print("}")
    else:
        sys.exit(f"未知格式: {args.format} (支持 css/tailwind/godot)")


def _is_hex(s: str) -> bool:
    return bool(s) and bool(HEX_RE.match(s))


def cmd_gate(args: argparse.Namespace) -> None:
    brief = _load_brief(args.brief)
    upto = STAGES.index(args.up_to) + 1 if args.up_to else len(STAGES)
    missing: dict[str, list[str]] = {}

    def need(stage: str, cond: bool, what: str) -> None:
        if not cond:
            missing.setdefault(stage, []).append(what)

    a = brief.get("analyze", {})
    if "analyze" in STAGES[:upto]:
        need("analyze", bool(a.get("keywords")), "keywords 为空")
        need("analyze", bool(a.get("emotion")), "emotion 未填")
        need("analyze", bool(a.get("scene")), "scene 未填")
        need("analyze", bool(a.get("not_convey")), "not_convey (最不想传达什么) 未填")
    p = brief.get("propose", {})
    if "propose" in STAGES[:upto]:
        need("propose", any(o.get("name") for o in p.get("options", [])), "风格选项名称为空")
        need("propose", bool(p.get("selected")), "用户未确认风格")
    c = brief.get("color", {})
    contrast_out: dict = {}
    if "color" in STAGES[:upto]:
        for k in ("primary", "secondary", "accent"):
            need("color", _is_hex(c.get(k, {}).get("hex", "")), f"{k}.hex 非法或未填")
        need("color", bool(c.get("rationale")), "配色理由未填")
        bg = c.get("background", "")
        if not _is_hex(bg) and bg != "":
            need("color", False, "background 非法 HEX")
        elif bg:
            for k in ("primary", "accent"):
                if _is_hex(c.get(k, {}).get("hex", "")):
                    ratio = contrast_ratio(c[k]["hex"], bg)
                    contrast_out[f"{k}_on_background"] = {"ratio": round(ratio, 2), "aa_pass": ratio >= 4.5}
            if contrast_out and not all(v["aa_pass"] for v in contrast_out.values()):
                missing.setdefault("color", []).append("对比度未达 WCAG AA (4.5)")
    lay = brief.get("layout", {})
    if "layout" in STAGES[:upto]:
        need("layout", bool(lay.get("composition_rule")), "构图法则未填")
        need("layout", bool(lay.get("visual_hierarchy")), "视觉层级为空")
        need("layout", bool(lay.get("description")), "布局描述未填")
    e = brief.get("elements", {})
    if "elements" in STAGES[:upto]:
        need("elements", bool(e.get("required")), "必需元素为空")
    v = brief.get("avoid", {})
    if "avoid" in STAGES[:upto]:
        taboos = v.get("taboos", [])
        need("avoid", bool(taboos), "禁忌清单为空")
        for i, t in enumerate(taboos):
            need("avoid", bool(t.get("avoid")) and bool(t.get("alternative")), f"taboos[{i}] 缺 avoid 或 alternative")

    result = {"pass": not missing, "missing": missing, "contrast": contrast_out}
    print(json.dumps(result, ensure_ascii=False, indent=2))
    sys.exit(0 if not missing else 1)


TEMPLATE = {
    "version": 1,
    "stage": "analyze",
    "analyze": {"keywords": [], "emotion": "", "scene": "", "audience": "", "not_convey": ""},
    "propose": {"options": [{"name": "", "keywords": [], "color_features": ""}], "selected": ""},
    "color": {
        "primary": {"hex": "", "name": "", "emotion": ""},
        "secondary": {"hex": "", "name": "", "emotion": ""},
        "accent": {"hex": "", "name": "", "emotion": ""},
        "background": "",
        "rationale": "",
    },
    "layout": {"composition_rule": "", "visual_hierarchy": [], "description": ""},
    "elements": {"required": [], "recommended": [], "optional": []},
    "avoid": {"taboos": [{"avoid": "", "reason": "", "alternative": ""}]},
}


def cmd_init(args: argparse.Namespace) -> None:
    out = Path(args.out)
    if out.exists():
        sys.exit(f"已存在, 不覆盖: {out}")
    out.write_text(json.dumps(TEMPLATE, ensure_ascii=False, indent=2) + "\n", encoding="utf-8")
    print(f"已生成: {out}")


def main() -> None:
    ap = argparse.ArgumentParser(description="design-everything CLI")
    sub = ap.add_subparsers(required=True)

    sub.add_parser("list").set_defaults(fn=cmd_list)

    s = sub.add_parser("search")
    s.add_argument("query")
    s.add_argument("--module")
    s.add_argument("--limit", type=int, default=5)
    s.set_defaults(fn=cmd_search)

    s = sub.add_parser("read")
    s.add_argument("module")
    s.add_argument("--limit", type=int, default=10)
    s.set_defaults(fn=cmd_read)

    s = sub.add_parser("contrast")
    s.add_argument("fg")
    s.add_argument("bg")
    s.add_argument("--large", action="store_true", help="大字号阈值 (AA=3.0)")
    s.set_defaults(fn=cmd_contrast)

    s = sub.add_parser("palette")
    s.add_argument("base")
    s.add_argument("--scheme", choices=sorted(SCHEMES), default="analogous")
    s.set_defaults(fn=cmd_palette)

    s = sub.add_parser("export")
    s.add_argument("brief")
    s.add_argument("--format", choices=["css", "tailwind", "godot"], default="css")
    s.set_defaults(fn=cmd_export)

    s = sub.add_parser("gate")
    s.add_argument("brief")
    s.add_argument("--up-to", choices=STAGES, help="只校验到指定阶段")
    s.set_defaults(fn=cmd_gate)

    s = sub.add_parser("init")
    s.add_argument("--out", default="design-brief.json")
    s.set_defaults(fn=cmd_init)

    args = ap.parse_args()
    args.fn(args)


if __name__ == "__main__":
    main()
