#!/usr/bin/env python3
"""
Setup script for Design Everything skill
Initialize SQLite index database and verify TSV files
"""

import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).parent.parent.resolve()
sys.path.insert(0, str(SCRIPT_DIR / "scripts"))

from index_manager import IndexDBManager
from tsv_reader import TSVReader

CATEGORIES = [
    ("Interface Design", "interface", "用户界面与体验设计相关模块"),
    ("Creative Design", "creative", "创意设计相关模块"),
]

MODULES = [
    ("UI Components", "ui", "用户界面组件设计，包括按钮、输入框、卡片、导航等常见UI元素的规范与最佳实践", "interface", "ui,components,按钮,输入框,卡片,导航,模态框,表单", "ui.tsv"),
    ("UX Design", "ux", "用户体验设计理论、研究方法、设计原则与评估框架", "interface", "ux,用户体验,用户研究,交互设计,信息架构,可用性,设计原则", "ux.tsv"),
    ("Game Design", "game", "游戏设计理论、机制设计、玩家心理与数值平衡", "creative", "game,游戏设计,游戏机制,玩家心理,数值设计,心流,游戏化", "game.tsv"),
    ("Manga Storyboard", "manga_storyboard", "漫画分镜设计、构图法则、镜头语言与叙事技巧", "creative", "manga,分镜,漫画,构图,画格,镜头语言,叙事,漫画分镜", "manga_storyboard.tsv"),
    ("Video Editing", "video_editing", "视频剪辑理论、蒙太奇手法、节奏控制与叙事剪辑", "creative", "video,剪辑,蒙太奇,节奏,叙事,镜头,转场,爱森斯坦", "video_editing.tsv"),
    ("Cinematography", "cinematography", "镜头语言、摄影构图、光影设计与镜头运动", "creative", "cinematography,镜头,摄影,构图,光影,运动,景别,角度", "cinematography.tsv"),
    ("Screenplay", "screenplay", "剧本结构、人物塑造、对话写作与叙事技巧", "creative", "screenplay,剧本,编剧,人物,对话,叙事,结构,三幕式", "screenplay.tsv"),
    ("Character Pose", "pose", "人物姿势参考、动作设计、人体力学与动态表现", "creative", "pose,姿势,人物,动作,动态,人体,重心,运动", "pose.tsv"),
    ("Scene Design", "scene", "场景设计原理、环境氛围、光影构图与空间叙事", "creative", "scene,场景,环境,氛围,光影,构图,空间,叙事", "scene.tsv"),
    ("Character Motion", "character_motion", "角色动作动画原理、运动规律、表情动画与物理法则", "creative", "motion,动作,动画,运动,表情,迪士尼,运动原理,动画规律", "character_motion.tsv"),
]

def main():
    index_db = IndexDBManager()
    tsv_reader = TSVReader()
    tsv_dir = SCRIPT_DIR / "data" / "tsv"

    print("=" * 50)
    print("Design Everything - Skill Setup")
    print("=" * 50)

    print(f"\n[1/3] Database: {index_db.db_path}")
    print(f"[2/3] TSV Directory: {tsv_dir}")
    print(f"[3/3] Categories: {len(CATEGORIES)}, Modules: {len(MODULES)}")

    print("\n" + "-" * 50)
    print("Registering categories...")
    print("-" * 50)
    for name, slug, desc in CATEGORIES:
        index_db.register_category(name, slug, desc)
        print(f"  ✓ {name} ({slug})")

    print("\n" + "-" * 50)
    print("Registering modules...")
    print("-" * 50)
    for name, slug, desc, category, keywords, tsv in MODULES:
        index_db.register_module(name, slug, desc, category, keywords, tsv)
        print(f"  ✓ {name} ({slug})")

    print("\n" + "-" * 50)
    print("Counting TSV items and updating index...")
    print("-" * 50)
    total_items = 0
    for _, slug, _, _, _, tsv_file in MODULES:
        if (tsv_dir / tsv_file).exists():
            count = tsv_reader.count_items(tsv_file)
            index_db.update_module_item_count(slug, count)
            total_items += count
            print(f"  ✓ {tsv_file}: {count} items")
        else:
            print(f"  ✗ {tsv_file}: FILE NOT FOUND")

    stats = index_db.get_module_stats()
    print("\n" + "=" * 50)
    print("Setup Complete!")
    print(f"  Categories: {stats['total_categories']}")
    print(f"  Modules: {stats['total_modules']}")
    print(f"  Total Items: {total_items}")
    print("=" * 50)

    print("\nTSV files are read directly, not imported into database.")
    print("Use tsv_reader.py to search content:")
    print("  python scripts/tsv_reader.py --list")
    print("  python scripts/tsv_reader.py --search 配色")
    print("  python scripts/tsv_reader.py --read ui")

if __name__ == "__main__":
    main()
